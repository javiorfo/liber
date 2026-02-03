package output

import (
	"archive/zip"
	"bytes"
	"testing"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/ident"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/internal/output/files"
	"github.com/javiorfo/liber/lang"
	"github.com/javiorfo/liber/reftype"
)

func TestCreator_Create(t *testing.T) {
	e := &epub.Epub{
		Metadata: epub.Metadata{
			Title:      "Test Book",
			Language:   lang.English,
			Identifier: ident.ISBN("12345"),
		},
		Contents: []epub.Content{
			{
				Body:          body.Raw("<body>Hello World</body>"),
				ReferenceType: reftype.Foreword("some"),
			},
		},
	}

	buf := new(bytes.Buffer)
	creator := NewCreator(e, buf)

	err := creator.Create()
	if err != nil {
		t.Fatalf("Create() failed: %v", err)
	}

	reader, err := zip.NewReader(bytes.NewReader(buf.Bytes()), int64(buf.Len()))
	if err != nil {
		t.Fatalf("Failed to read back ZIP: %v", err)
	}

	expectedFiles := map[string]bool{
		"mimetype":               false,
		"META-INF/container.xml": false,
		"META-INF/com.apple.ibooks.display-options.xml": false,
		"OEBPS/content.opf":                             false,
		"OEBPS/toc.ncx":                                 false,
		"OEBPS/c01.xhtml":                               false,
	}

	for _, file := range reader.File {
		if _, found := expectedFiles[file.Name]; found {
			expectedFiles[file.Name] = true
		}
	}

	for name, found := range expectedFiles {
		if !found {
			t.Errorf("Expected file %s was not found in the ZIP archive", name)
		}
	}
}

func TestCreator_AddFile(t *testing.T) {
	buf := new(bytes.Buffer)
	creator := NewCreator(&epub.Epub{}, buf)

	testPath := "test/file.txt"
	testData := []byte("content")

	fc := files.FileContent[[]byte]{
		Filepath: testPath,
		Bytes:    testData,
	}

	err := creator.AddFile(fc)
	if err != nil {
		t.Fatalf("AddFile failed: %v", err)
	}

	creator.zipWriter.Close()

	reader, _ := zip.NewReader(bytes.NewReader(buf.Bytes()), int64(buf.Len()))
	if len(reader.File) != 1 || reader.File[0].Name != testPath {
		t.Errorf("AddFile did not create the correct entry in ZIP")
	}
}
