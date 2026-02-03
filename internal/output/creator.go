package output

import (
	"archive/zip"
	"io"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/internal/output/files"
	"github.com/javiorfo/liber/internal/output/files/parser"
)

// Creator handles the physical assembly of the EPUB file.
// It coordinates the writing of structural files, stylesheets, images,
// and content into a ZIP-compressed stream.
type Creator struct {
	// Epub is the data model containing all book information.
	Epub *epub.Epub
	// zipWriter manages the low-level ZIP archive creation.
	zipWriter *zip.Writer
}

// NewCreator initializes a new Creator with a target io.Writer (e.g., an os.File)
// and prepares the underlying ZIP engine.
func NewCreator(e *epub.Epub, writer io.Writer) *Creator {
	return &Creator{
		Epub:      e,
		zipWriter: zip.NewWriter(writer),
	}
}

// Create executes the multi-step process of building an EPUB.
// It handles mandatory files (mimetype, container), resources (CSS, images),
// recursive content generation, and final manifest (OPF/NCX) creation.
// It automatically closes the ZIP writer upon completion.
func (c *Creator) Create() error {
	e := c.Epub
	defer c.zipWriter.Close()

	// Step 1: Write mandatory OCF (Open Container Format) files.
	if err := c.AddFile(files.Mimetype()); err != nil {
		return err
	}
	if err := c.AddFile(files.Container()); err != nil {
		return err
	}
	if err := c.AddFile(files.DisplayOptions()); err != nil {
		return err
	}

	// Step 2: Process Global Stylesheet.
	if e.Stylesheet.IsValue() {
		bytes, err := e.Stylesheet.AsValue().ToBytes()
		if err != nil {
			return err
		}
		if err := c.AddFile(files.NewFileContent("OEBPS/style.css", bytes)); err != nil {
			return err
		}
	}

	// Step 3: Embed Cover.
	if e.CoverImage.IsValue() {
		fc, err := parser.CreateResourceFileContent(e.CoverImage.AsValue())
		if err != nil {
			return err
		}
		if err := c.AddFile(*fc); err != nil {
			return err
		}
	}

	// Step 4: Embed other resources (images, fonts, etc.).
	for _, res := range e.Resources {
		fc, err := parser.CreateResourceFileContent(res)
		if err != nil {
			return err
		}
		if err := c.AddFile(*fc); err != nil {
			return err
		}
	}

	fileNumber := 0
	stylesheet := e.Stylesheet.MapToString(func(b body.Body) string {
		return epub.LinkCSS
	}).Or("")

	// Step 5: Generate XHTML content files from the recursive tree.
	for _, con := range e.Contents {
		fileContents, err := con.CreateFileContent(&fileNumber, stylesheet)
		if err != nil {
			return err
		}

		for _, fc := range fileContents {
			if err := c.AddFile(fc.ToBytes()); err != nil {
				return err
			}
		}
	}

	// Step 6: Generate and write the Package Document (OPF).
	opfFileContent, err := parser.ContentOpf(c.Epub)
	if err != nil {
		return err
	}
	if err := c.AddFile(opfFileContent.ToBytes()); err != nil {
		return err
	}

	// Step 7: Generate and write the Navigation Control file (NCX).
	tocFileContent, err := parser.TocNcx(c.Epub)
	if err != nil {
		return err
	}
	if err := c.AddFile(tocFileContent.ToBytes()); err != nil {
		return err
	}

	return nil
}

// AddFile is a method that creates a new file entry in the ZIP archive
// and writes the provided byte content to it.
func (c *Creator) AddFile(fileContent files.FileContent[[]byte]) error {
	writer, err := c.zipWriter.Create(fileContent.Filepath)
	if err != nil {
		return err
	}

	_, err = writer.Write(fileContent.Bytes)
	if err != nil {
		return err
	}

	return nil
}
