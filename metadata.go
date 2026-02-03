package liber

import (
	"time"

	"github.com/javiorfo/liber/ident"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/lang"
	"github.com/javiorfo/nilo"
)

// metadataBuilder is a wrapper around epub.Metadata that provides a fluent
// interface for defining an EPUB's bibliographic information.
type metadataBuilder struct {
	epub.Metadata
}

// MetadataBuilder initializes a new builder with the mandatory Dublin Core
// elements: title, language, and a unique identifier (UUID or ISBN).
func MetadataBuilder(title string, l lang.Language, i ident.Identifier) *metadataBuilder {
	return &metadataBuilder{
		epub.Metadata{
			Title:      title,
			Language:   l,
			Identifier: i,
		},
	}
}

// Creator sets the primary author or entity responsible for the content.
func (b *metadataBuilder) Creator(c string) *metadataBuilder {
	b.Metadata.Creator = nilo.Value(c)
	return b
}

// Publisher sets the entity responsible for making the resource available.
func (b *metadataBuilder) Publisher(p string) *metadataBuilder {
	b.Metadata.Publisher = nilo.Value(p)
	return b
}

// Contributor sets an entity that made secondary contributions to the resource.
func (b *metadataBuilder) Contributor(c string) *metadataBuilder {
	b.Metadata.Contributor = nilo.Value(c)
	return b
}

// Subject sets the topic, keywords, or classification codes for the EPUB.
func (b *metadataBuilder) Subject(s string) *metadataBuilder {
	b.Metadata.Subject = nilo.Value(s)
	return b
}

// Date sets the publication or creation date of the resource.
func (b *metadataBuilder) Date(t time.Time) *metadataBuilder {
	b.Metadata.Date = nilo.Value(t)
	return b
}

// Description sets a brief summary or abstract of the EPUB content.
func (b *metadataBuilder) Description(d string) *metadataBuilder {
	b.Metadata.Description = nilo.Value(d)
	return b
}

// Build returns the constructed epub.Metadata instance.
func (b *metadataBuilder) Build() epub.Metadata {
	return b.Metadata
}
