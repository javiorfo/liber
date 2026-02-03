package parser

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/javiorfo/liber/ident"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/lang"
	"github.com/javiorfo/liber/reftype"
	"github.com/javiorfo/liber/resource"
)

func TestCreateResourceFileContent(t *testing.T) {
	tmpDir := t.TempDir()
	tmpFile := filepath.Join(tmpDir, "image.png")
	expectedContent := []byte("fake-image-data")
	err := os.WriteFile(tmpFile, expectedContent, 0644)
	if err != nil {
		t.Fatalf("Failed to write temp file: %v", err)
	}

	res := resource.PngFile(tmpFile)

	fc, err := CreateResourceFileContent(res)
	if err != nil {
		t.Fatalf("Failed to create resource file: %v", err)
	}

	if fc.Filepath != "OEBPS/image.png" {
		t.Errorf("expected path OEBPS/image.png, got %s", fc.Filepath)
	}
	if string(fc.Bytes) != string(expectedContent) {
		t.Errorf("content mismatch")
	}
}

func TestContentOpf(t *testing.T) {
	e := &epub.Epub{
		Metadata: epub.Metadata{
			Title:      "Go Testing",
			Language:   lang.English,
			Identifier: ident.ISBN("123"),
		},
		Contents: []epub.Content{
			{
				ReferenceType: reftype.Text("Text"),
				SubContents: []epub.Content{
					{ReferenceType: reftype.Preface("Chapter")},
				},
			},
		},
	}

	fc, err := ContentOpf(e)
	if err != nil {
		t.Fatalf("ContentOpf error: %v", err)
	}

	xml := fc.Bytes

	checks := []string{
		"<dc:title>Go Testing</dc:title>",
		"<dc:identifier id=\"BookId\" opf:scheme=\"ISBN\">urn:isbn:123</dc:identifier>",
		"<item id=\"c01.xhtml\" href=\"c01.xhtml\"",
		"<item id=\"c02.xhtml\" href=\"c02.xhtml\"",
		"<itemref idref=\"c01.xhtml\"/>",
		"<reference type=\"text\"",
	}

	for _, check := range checks {
		if !strings.Contains(xml, check) {
			t.Errorf("OPF missing expected string: %s", check)
		}
	}
}

func TestTocNcx(t *testing.T) {
	e := &epub.Epub{
		Metadata: epub.Metadata{
			Title:      "TOC Test",
			Identifier: ident.ISBN("456"),
		},
		Contents: []epub.Content{
			{
				ReferenceType: reftype.TitlePage("Title"),
				ContentReferences: []epub.ContentReference{
					{Title: "Anchor 1"},
				},
			},
		},
	}

	fc, err := TocNcx(e)
	if err != nil {
		t.Fatalf("TocNcx error: %v", err)
	}

	xml := fc.Bytes

	if !strings.Contains(xml, `playOrder="1"`) {
		t.Error("Missing playOrder 1")
	}
	if !strings.Contains(xml, `playOrder="2"`) {
		t.Error("Missing playOrder 2 (nested reference)")
	}
	if !strings.Contains(xml, `navPoint-1-1`) {
		t.Error("Nested NavPoint ID format incorrect")
	}
}
