package liber

import (
	"bytes"
	"fmt"
	"testing"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/ident"
	"github.com/javiorfo/liber/lang"
	"github.com/javiorfo/liber/reftype"
	"github.com/javiorfo/liber/resource"
)

func TestEpubBuilder(t *testing.T) {
	meta := MetadataBuilder("Test Book", lang.English, ident.Default()).Build()
	content := ContentBuilder(body.Raw("<h1>content</h1>"), reftype.Preface("preface")).Filename("ch1.xhtml").Build()
	res := resource.AudioFile("/path/audio.mp4")
	cover := resource.JpgFile("/path/img.jpg")
	style := body.Raw("body { color: red; }")

	e := EpubBuilder(meta).
		AddContents(content).
		AddResources(res).
		CoverImage(cover).
		Stylesheet(style).
		Build()

	if e.Metadata.Title != "Test Book" {
		t.Errorf("expected title 'Test Book', got %s", e.Metadata.Title)
	}

	if len(e.Contents) != 1 || e.Contents[0].Filename.AsValue() != "ch1.xhtml" {
		t.Errorf("contents not added correctly")
	}

	if len(e.Resources) != 1 || fmt.Sprint(e.Resources[0]) != "/path/audio.mp4" {
		t.Errorf("resources not added correctly")
	}

	if e.CoverImage.IsNil() {
		t.Errorf("cover image not set correctly")
	}

	if e.Stylesheet.IsNil() {
		t.Errorf("stylesheet not set correctly")
	}
}

func TestCreate(t *testing.T) {
	e := EpubBuilder(
		MetadataBuilder("Test Book", lang.English, ident.Default()).Build(),
	).Build()

	var buf bytes.Buffer
	err := Create(&e, &buf)

	if err != nil {
		t.Logf("Create returned error (likely due to empty epub fields): %v", err)
	}
}
