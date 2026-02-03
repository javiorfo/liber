package epub

import (
	"fmt"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/internal/output/files"
	"github.com/javiorfo/liber/reftype"
	"github.com/javiorfo/nilo"
)

const LinkCSS = `<link href="style.css" rel="stylesheet" type="text/css"/>`

type Content struct {
	Body              body.Body
	ReferenceType     reftype.ReferenceType
	SubContents       []Content
	ContentReferences []ContentReference
	Filename          nilo.Option[string]
}

func (c Content) Level() int {
	if len(c.SubContents) == 0 {
		return 0
	}
	return 1 + c.SubContents[0].Level()
}

func (c Content) LevelReferenceContent() int {
	contentRefsLevel := 0
	if len(c.ContentReferences) > 0 {
		contentRefsLevel = 1 + c.ContentReferences[0].level()
	}

	subContentsLevel := 0
	if len(c.SubContents) > 0 {
		subContentsLevel = 1 + c.SubContents[0].LevelReferenceContent()
	}

	if contentRefsLevel > subContentsLevel {
		return contentRefsLevel
	}
	return subContentsLevel
}

func (c Content) GetFilename(number int) string {
	return c.Filename.Or(fmt.Sprintf("c%02d.xhtml", number))
}

func (c Content) CreateFileContent(number *int, stylesheet string) ([]files.FileContent[string], error) {
	*number++
	var fileContents []files.FileContent[string]

	text, err := c.Body.ToString()
	if err != nil {
		return nil, err
	}

	xml := fmt.Sprintf(`<?xml version="1.0" encoding="utf-8"?>
	<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
	<html xmlns="http://www.w3.org/1999/xhtml"><head><title>%s</title>%s</head>%s</html>`,
		c.ReferenceType,
		stylesheet,
		text,
	)

	fileContents = append(fileContents, files.NewFileContent("OEBPS/"+c.GetFilename(*number), files.FormatXML(xml)))

	for _, subc := range c.SubContents {
		contents, err := subc.CreateFileContent(number, stylesheet)
		if err != nil {
			return nil, err
		}
		fileContents = append(fileContents, contents...)
	}

	return fileContents, nil
}
