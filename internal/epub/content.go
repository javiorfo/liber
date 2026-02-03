package epub

import (
	"fmt"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/internal/output/files"
	"github.com/javiorfo/liber/reftype"
	"github.com/javiorfo/nilo"
)

const LinkCSS = `<link href="style.css" rel="stylesheet" type="text/css"/>`

// Content represents a chapter, section, or specific semantic block within the EPUB.
// It supports recursive nesting via SubContents and internal cross-linking via ContentReferences.
type Content struct {
	// Body contains the actual HTML/text data for this section.
	Body body.Body
	// ReferenceType defines the semantic nature (e.g., Preface, Text, Index).
	ReferenceType reftype.ReferenceType
	// SubContents allows for hierarchical nesting of chapters (e.g., Part > Chapter).
	SubContents []Content
	// ContentReferences defines internal navigational points within this content.
	ContentReferences []ContentReference
	// Filename is an optional custom name for the generated XHTML file.
	Filename nilo.Option[string]
}

// Level calculates the maximum depth of the nested SubContents tree.
// A content block with no children returns 0.
func (c Content) Level() int {
	if len(c.SubContents) == 0 {
		return 0
	}
	return 1 + c.SubContents[0].Level()
}

// LevelReferenceContent calculates the maximum nesting depth of the tree,
// accounting for both SubContents and ContentReferences.
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

// GetFilename returns the user-defined filename or generates a default
// sequential one (e.g., "c01.xhtml") based on the provided index.
func (c Content) GetFilename(number int) string {
	return c.Filename.Or(fmt.Sprintf("c%02d.xhtml", number))
}

// CreateFileContent recursively generates a slice of FileContent objects.
// It wraps the raw body in a valid XHTML 1.1 template, applies the stylesheet,
// and places the resulting files into the "OEBPS/" directory.
func (c Content) CreateFileContent(number *int, stylesheet string) ([]files.FileContent[string], error) {
	*number++
	var fileContents []files.FileContent[string]

	text, err := c.Body.ToString()
	if err != nil {
		return nil, err
	}

	// Generating a standard XHTML 1.1 header required for EPUB compatibility.
	xml := fmt.Sprintf(`<?xml version="1.0" encoding="utf-8"?>
	<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
	<html xmlns="http://www.w3.org/1999/xhtml"><head><title>%s</title>%s</head><body>%s</body></html>`,
		c.ReferenceType,
		stylesheet,
		text,
	)

	fileContents = append(fileContents, files.NewFileContent("OEBPS/"+c.GetFilename(*number), files.FormatXML(xml)))

	// Recursively process child content.
	for _, subc := range c.SubContents {
		contents, err := subc.CreateFileContent(number, stylesheet)
		if err != nil {
			return nil, err
		}
		fileContents = append(fileContents, contents...)
	}

	return fileContents, nil
}
