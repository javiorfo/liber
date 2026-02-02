package liber

import (
	"time"

	"github.com/javiorfo/liber/ident"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/lang"
	"github.com/javiorfo/nilo"
)

type metadataBuilder struct {
	epub.Metadata
}

func MetadataBuilder(title string, l lang.Language, i ident.Identifier) *metadataBuilder {
	return &metadataBuilder{
		epub.Metadata{
			Title:      title,
			Language:   l,
			Identifier: i,
		},
	}
}

func (b *metadataBuilder) Creator(c string) *metadataBuilder {
	b.Metadata.Creator = nilo.Value(c)
	return b
}

func (b *metadataBuilder) Publisher(p string) *metadataBuilder {
	b.Metadata.Publisher = nilo.Value(p)
	return b
}

func (b *metadataBuilder) Contributor(c string) *metadataBuilder {
	b.Metadata.Contributor = nilo.Value(c)
	return b
}

func (b *metadataBuilder) Subject(s string) *metadataBuilder {
	b.Metadata.Subject = nilo.Value(s)
	return b
}

func (b *metadataBuilder) Date(t time.Time) *metadataBuilder {
	b.Metadata.Date = nilo.Value(t)
	return b
}

func (b *metadataBuilder) Description(d string) *metadataBuilder {
	b.Metadata.Description = nilo.Value(d)
	return b
}

func (b *metadataBuilder) Build() epub.Metadata {
	return b.Metadata
}
