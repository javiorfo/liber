package parser

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/internal/output/files"
	"github.com/javiorfo/liber/reftype"
	"github.com/javiorfo/liber/resource"
	"github.com/javiorfo/nilo"
)

// Xhtml internal helper to track file numbering and naming during parsing.
type Xhtml struct {
	number   int
	filename string
}

// CreateResourceFileContent reads a physical file from the disk based on a resource
// and wraps it in a FileContent struct for the archive.
func CreateResourceFileContent(r resource.Resource) (*files.FileContent[[]byte], error) {
	path := fmt.Sprint(r)
	content, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("read resource file %s: %w", path, err)
	}

	fc := files.NewFileContent("OEBPS/"+filepath.Base(path), content)
	return &fc, nil
}

// ContentOpf generates the Open Packaging Format (OPF) file.
// This is the "brain" of the EPUB, containing the manifest of all files,
// the spine (reading order), and the guide (semantic sections).
func ContentOpf(e *epub.Epub) (*files.FileContent[string], error) {
	metadata := e.Metadata
	var builder strings.Builder

	builder.WriteString(`<?xml version="1.0" encoding="utf-8"?>
	<package version="2.0" unique-identifier="BookId" xmlns="http://www.idpf.org/2007/opf">
	<metadata xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:opf="http://www.idpf.org/2007/opf">`)

	// Standard Dublin Core Metadata
	builder.WriteString("<dc:title>" + metadata.Title + "</dc:title>")
	builder.WriteString("<dc:language>" + metadata.Language.Code() + "</dc:language>")
	fmt.Fprintf(&builder, `<dc:identifier id="BookId" opf:scheme="%s">%s</dc:identifier>`, metadata.Identifier.Label(), metadata.Identifier.String())

	metadata.Creator.Consume(func(s string) {
		builder.WriteString(`<dc:creator opf:role="aut">` + s + `</dc:creator>`)
	})

	metadata.Contributor.Consume(func(s string) {
		builder.WriteString(`<dc:contributor opf:role="trl">` + s + `</dc:contributor>`)
	})

	metadata.Publisher.Consume(func(s string) {
		builder.WriteString(`<dc:publisher>` + s + `</dc:publisher>`)
	})

	metadata.Date.Consume(func(t time.Time) {
		builder.WriteString(`<dc:date opf:event="publication">` + t.Format("2006-01-02") + `</dc:date>`)
	})

	metadata.Subject.Consume(func(s string) {
		builder.WriteString(`<dc:subject>` + s + `</dc:subject>`)
	})

	metadata.Description.Consume(func(s string) {
		builder.WriteString(`<dc:description>` + s + `</dc:description>`)
	})

	e.CoverImage.Consume(func(i resource.Image) {
		builder.WriteString(`<meta name="cover" content="` + filepath.Base(fmt.Sprint(i)) + `"/>`)
	})

	// Manifest: Lists every file included in the EPUB
	builder.WriteString(`</metadata><manifest><item id="ncx" href="toc.ncx" media-type="application/x-dtbncx+xml" />`)

	if e.Stylesheet.IsValue() {
		builder.WriteString(`<item id="style.css" href="style.css" media-type="text/css"/>`)
	}

	e.CoverImage.Consume(func(i resource.Image) {
		builder.WriteString(resourceAsManifestXml(i))
	})

	for _, res := range e.Resources {
		builder.WriteString(resourceAsManifestXml(res))
	}

	manifestNumber := 0
	if err := createContentChain(
		&manifestNumber,
		&builder,
		e.Contents,
		func(filename string, _ reftype.ReferenceType) string {
			return fmt.Sprintf(`<item id="%s" href="%s" media-type="application/xhtml+xml"/>`, filename, filename)
		},
	); err != nil {
		return nil, err
	}

	builder.WriteString(`</manifest><spine toc="ncx">`)

	tocNumber := 0
	if err := createContentChain(
		&tocNumber,
		&builder,
		e.Contents,
		func(filename string, _ reftype.ReferenceType) string {
			return fmt.Sprintf(`<itemref idref="%s"/>`, filename)
		},
	); err != nil {
		return nil, err
	}

	builder.WriteString("</spine><guide>")

	guideNumber := 0
	if err := createContentChain(
		&guideNumber,
		&builder,
		e.Contents,
		func(filename string, rf reftype.ReferenceType) string {
			return fmt.Sprintf(`<reference type="%s" title="%s" href="%s"/>`,
				rf.Type(),
				fmt.Sprint(rf),
				filename,
			)
		},
	); err != nil {
		return nil, err
	}

	builder.WriteString("</guide></package>")

	fc := files.NewFileContent("OEBPS/content.opf", files.FormatXML(builder.String()))
	return &fc, nil
}

// createContentChain is a recursive helper that traverses the Content tree
// to populate manifest, spine, or guide sections based on a provided formatting function.
func createContentChain(
	fileNumber *int,
	builder *strings.Builder,
	contents []epub.Content,
	f func(string, reftype.ReferenceType) string,
) error {
	for _, con := range contents {
		*fileNumber++
		filename := con.GetFilename(*fileNumber)
		if !strings.HasSuffix(filename, ".xhtml") {
			return fmt.Errorf("content filename must end with '.xhtml'. Got '%s'", filename)
		}

		builder.WriteString(f(filename, con.ReferenceType))

		// Recursively appends content strings to the builder
		if err := createContentChain(fileNumber, builder, con.SubContents, f); err != nil {
			return err
		}
	}
	return nil
}

// TocNcx generates the Navigation Control file for XML (NCX).
// This provides the Table of Contents that allows e-readers to display a
// navigation menu outside of the book content.
func TocNcx(e *epub.Epub) files.FileContent[string] {
	metadata := e.Metadata
	var builder strings.Builder

	builder.WriteString(`<?xml version="1.0" encoding="UTF-8"?>
	<!DOCTYPE ncx PUBLIC "-//NISO//DTD ncx 2005-1//EN" "http://www.daisy.org/z3986/2005/ncx-2005-1.dtd">
	<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/" version="2005-1"><head>`)

	fmt.Fprintf(&builder, `<meta name="dtb:uid" content="%s"/>`, metadata.Identifier.String())
	fmt.Fprintf(&builder, `<meta name="dtb:depth" content="%d"/>`, e.Level())

	fmt.Fprintf(&builder, `<meta name="dtb:totalPageCount" content="0"/>
	<meta name="dtb:maxPageNumber" content="0"/></head><docTitle><text>%s</text></docTitle><navMap>`, metadata.Title)

	playOrder := 0
	fileNumber := 0
	contentsToNavPoint(&playOrder, &fileNumber, e.Contents).Consume(func(s string) {
		builder.WriteString(s)
	})

	builder.WriteString("</navMap></ncx>")

	return files.NewFileContent("OEBPS/toc.ncx", files.FormatXML(builder.String()))
}

// contentsToNavPoint recursively converts the Content tree into <navPoint>
// elements required for the NCX navigation map.
func contentsToNavPoint(playOrder *int, fileNumber *int, contents []epub.Content) nilo.Option[string] {
	var builder strings.Builder

	for _, content := range contents {
		*playOrder++
		currentPlayOrder := *playOrder

		*fileNumber++
		filename := content.GetFilename(*fileNumber)

		contentRefNumber := 0
		navPoint := fmt.Sprintf(`<navPoint id="navPoint-%d" playOrder="%d">
			<navLabel><text>%s</text></navLabel>
			<content src="%s"/>%s%s</navPoint>`,
			currentPlayOrder,
			currentPlayOrder,
			content.ReferenceType,
			filename,
			contentReferencesToNavPoint(
				Xhtml{number: currentPlayOrder, filename: filename},
				playOrder,
				"",
				content.ContentReferences,
				&contentRefNumber,
			).Or(""),
			// Recursive call to sub contents
			contentsToNavPoint(playOrder, fileNumber, content.SubContents).Or(""),
		)

		builder.WriteString(navPoint)
	}

	return nilo.Value(builder.String())
}

// contentReferencesToNavPoint converts internal ContentReferences (anchors)
// into nested <navPoint> elements within a specific file.
func contentReferencesToNavPoint(
	currentXhtml Xhtml,
	playOrder *int,
	tocIndex string,
	contentReferences []epub.ContentReference,
	linkNumber *int,
) nilo.Option[string] {
	var builder strings.Builder

	var prefix string
	var tocNumber int
	if i := strings.LastIndex(tocIndex, "-"); i != -1 {
		prefix = tocIndex[:i]
		tocNumber = nilo.FromResult(strconv.Atoi(tocIndex[i+1:])).Or(0)
	}

	for _, conRef := range contentReferences {
		*linkNumber++
		currentLink := *linkNumber

		tocNumber++
		currentToc := fmt.Sprintf("%s-%d", prefix, tocNumber)

		*playOrder++
		currentPlayOrder := *playOrder

		navPoint := fmt.Sprintf(`<navPoint id="navPoint-%d%s" playOrder="%d">
			<navLabel><text>%s</text></navLabel>
			<content src="%s"/>%s</navPoint>`,
			currentXhtml.number,
			currentToc,
			currentPlayOrder,
			conRef.Title,
			conRef.ReferenceName(currentXhtml.filename, currentLink),
			// Recursive call sub content references
			contentReferencesToNavPoint(
				currentXhtml,
				playOrder,
				currentToc+"-",
				conRef.SubContentReferences,
				linkNumber,
			).Or(""),
		)

		builder.WriteString(navPoint)
	}

	return nilo.Value(builder.String())
}

// resourceAsManifestXml converts a resource into a valid manifest <item/> tag.
func resourceAsManifestXml(r resource.Resource) string {
	filename := filepath.Base(fmt.Sprint(r))
	return fmt.Sprintf(`<item id="%s" href="%s" media-type="%s"/>`,
		filename,
		filename,
		r.Mediatype(),
	)
}
