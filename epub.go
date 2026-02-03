package liber

import (
	"io"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/internal/output"
	"github.com/javiorfo/liber/resource"
	"github.com/javiorfo/nilo"
)

// epubBuilder is a wrapper around epub.Epub that provides a fluent interface
// for assembling a complete EPUB document, including metadata, content, and resources.
type epubBuilder struct {
	epub.Epub
}

// EpubBuilder initializes a new builder with the provided metadata.
func EpubBuilder(m epub.Metadata) *epubBuilder {
	return &epubBuilder{epub.Epub{Metadata: m}}
}

// AddContents appends one or more Content items to the EPUB document.
func (b *epubBuilder) AddContents(contents ...epub.Content) *epubBuilder {
	if len(contents) > 0 {
		b.Contents = append(b.Contents, contents...)
	}
	return b
}

// AddResources appends external resources (like font, image, video and audio files) to the EPUB.
func (b *epubBuilder) AddResources(resources ...resource.Resource) *epubBuilder {
	if len(resources) > 0 {
		b.Resources = append(b.Resources, resources...)
	}
	return b
}

// CoverImage sets the primary cover image for the EPUB document.
func (b *epubBuilder) CoverImage(ci resource.Image) *epubBuilder {
	b.Epub.CoverImage = nilo.Value(ci)
	return b
}

// Stylesheet sets the global CSS stylesheet for the EPUB body.
func (b *epubBuilder) Stylesheet(r body.Body) *epubBuilder {
	b.Epub.Stylesheet = nilo.Value(r)
	return b
}

// Build returns the fully constructed epub.Epub instance.
func (b *epubBuilder) Build() epub.Epub {
	return b.Epub
}

// Create takes a populated Epub structure and writes the finalized EPUB file
// to the provided io.Writer using the internal output creator.
func Create(e *epub.Epub, writer io.Writer) error {
	return output.NewCreator(e, writer).Create()
}
