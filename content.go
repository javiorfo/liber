package liber

import (
	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/reftype"
	"github.com/javiorfo/nilo"
)

type contentBuilder struct {
	epub.Content
}

func ContentBuilder(body body.Body, rt reftype.ReferenceType) *contentBuilder {
	return &contentBuilder{
		epub.Content{
			Body:          body,
			ReferenceType: rt,
		},
	}
}

func (b *contentBuilder) AddChildren(contents ...epub.Content) *contentBuilder {
	if len(contents) > 0 {
		b.SubContents = append(b.SubContents, contents...)
	}
	return b
}

func (b *contentBuilder) AddContentReferences(contents ...epub.ContentReference) *contentBuilder {
	if len(contents) > 0 {
		b.ContentReferences = append(b.ContentReferences, contents...)
	}
	return b
}

func (b *contentBuilder) Filename(f string) *contentBuilder {
	b.Content.Filename = nilo.Value(f)
	return b
}

func (b *contentBuilder) Build() epub.Content {
	return b.Content
}

type contentReferenceBuilder struct {
	epub.ContentReference
}

func ContentReferenceBuilder(title string) *contentReferenceBuilder {
	return &contentReferenceBuilder{
		epub.ContentReference{Title: title},
	}
}

func (b *contentReferenceBuilder) AddChildren(children ...epub.ContentReference) *contentReferenceBuilder {
	if len(children) > 0 {
		b.SubContentReferences = append(b.SubContentReferences, children...)
	}
	return b
}

func (b *contentReferenceBuilder) ID(f string) *contentReferenceBuilder {
	b.ContentReference.ID = nilo.Value(f)
	return b
}

func (b *contentReferenceBuilder) Build() epub.ContentReference {
	return b.ContentReference
}
