package liber

import (
	"io"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/internal/output"
	"github.com/javiorfo/liber/resource"
	"github.com/javiorfo/nilo"
)

type epubBuilder struct {
	epub.Epub
}

func EpubBuilder(m epub.Metadata) *epubBuilder {
	return &epubBuilder{epub.Epub{Metadata: m}}
}

func (b *epubBuilder) AddContents(contents ...epub.Content) *epubBuilder {
	if len(contents) > 0 {
		b.Contents = append(b.Contents, contents...)
	}
	return b
}

func (b *epubBuilder) AddResources(resources ...resource.Resource) *epubBuilder {
	if len(resources) > 0 {
		b.Resources = append(b.Resources, resources...)
	}
	return b
}

func (b *epubBuilder) CoverImage(ci resource.Image) *epubBuilder {
	b.Epub.CoverImage = nilo.Value(ci)
	return b
}

func (b *epubBuilder) Stylesheet(r body.Body) *epubBuilder {
	b.Epub.Stylesheet = nilo.Value(r)
	return b
}

func (b *epubBuilder) Build() epub.Epub {
	return b.Epub
}

func Create(e epub.Epub, writer io.Writer) error {
	return output.NewCreator(e, writer).Create()
}
