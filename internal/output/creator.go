package output

import (
	"archive/zip"
	"io"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/internal/output/files"
	"github.com/javiorfo/liber/internal/output/files/parser"
)

type Creator struct {
	Epub      *epub.Epub
	zipWriter *zip.Writer
}

func NewCreator(e *epub.Epub, writer io.Writer) *Creator {
	return &Creator{
		Epub:      e,
		zipWriter: zip.NewWriter(writer),
	}
}

func (c *Creator) Create() error {
	e := c.Epub
	defer c.zipWriter.Close()

	if err := c.AddFile(files.Mimetype()); err != nil {
		return err
	}
	if err := c.AddFile(files.Container()); err != nil {
		return err
	}
	if err := c.AddFile(files.DisplayOptions()); err != nil {
		return err
	}

	if e.Stylesheet.IsValue() {
		bytes, err := e.Stylesheet.AsValue().ToBytes()
		if err != nil {
			return err
		}
		if err := c.AddFile(files.NewFileContent("OEBPS/style.css", bytes)); err != nil {
			return err
		}
	}

	if e.CoverImage.IsValue() {
		fc, err := parser.CreateResourceFileContent(e.CoverImage.AsValue())
		if err != nil {
			return err
		}
		if err := c.AddFile(*fc); err != nil {
			return err
		}
	}

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

	opfFileContent, err := parser.ContentOpf(c.Epub)
	if err != nil {
		return err
	}
	if err := c.AddFile(opfFileContent.ToBytes()); err != nil {
		return err
	}

	tocFileContent, err := parser.TocNcx(c.Epub)
	if err != nil {
		return err
	}
	if err := c.AddFile(tocFileContent.ToBytes()); err != nil {
		return err
	}

	return nil
}

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
