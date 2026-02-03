package liber

import (
	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/reftype"
	"github.com/javiorfo/nilo"
)

//-------------------- ContentBuilder --------------------------//

// contentBuilder is a wrapper around epub.Content that facilitates
// a fluent API for building complex content structures.
type contentBuilder struct {
	epub.Content
}

// ContentBuilder initializes a new builder with the provided body and reference type.
func ContentBuilder(body body.Body, rt reftype.ReferenceType) *contentBuilder {
	return &contentBuilder{
		epub.Content{
			Body:          body,
			ReferenceType: rt,
		},
	}
}

// AddChildren appends multiple epub.Content items to the SubContents of the current builder.
func (b *contentBuilder) AddChildren(contents ...epub.Content) *contentBuilder {
	if len(contents) > 0 {
		b.SubContents = append(b.SubContents, contents...)
	}
	return b
}

// AddContentReferences appends multiple ContentReference items to the current builder.
func (b *contentBuilder) AddContentReferences(contents ...epub.ContentReference) *contentBuilder {
	if len(contents) > 0 {
		b.ContentReferences = append(b.ContentReferences, contents...)
	}
	return b
}

// Filename sets the filename for the content otherwise the files will be named "c{number}.xhtml"
func (b *contentBuilder) Filename(f string) *contentBuilder {
	b.Content.Filename = nilo.Value(f)
	return b
}

// Build returns the constructed epub.Content instance.
func (b *contentBuilder) Build() epub.Content {
	return b.Content
}

//-------------------- ContentReferenceBuilder --------------------------//

// contentReferenceBuilder is a wrapper around epub.ContentReference
// used to fluently construct hierarchical content references.
type contentReferenceBuilder struct {
	epub.ContentReference
}

// ContentReferenceBuilder initializes a new builder with a specific title.
func ContentReferenceBuilder(title string) *contentReferenceBuilder {
	return &contentReferenceBuilder{
		epub.ContentReference{Title: title},
	}
}

// AddChildren appends sub-references to the current content reference.
func (b *contentReferenceBuilder) AddChildren(children ...epub.ContentReference) *contentReferenceBuilder {
	if len(children) > 0 {
		b.SubContentReferences = append(b.SubContentReferences, children...)
	}
	return b
}

// ID sets the unique identifier for the content reference.
// Otherwise ID will increase automatically with format "id{number}".
func (b *contentReferenceBuilder) ID(f string) *contentReferenceBuilder {
	b.ContentReference.ID = nilo.Value(f)
	return b
}

// Build returns the constructed epub.ContentReference instance.
func (b *contentReferenceBuilder) Build() epub.ContentReference {
	return b.ContentReference
}
