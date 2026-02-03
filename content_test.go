package liber

import (
	"testing"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/internal/epub"
	"github.com/javiorfo/liber/reftype"
)

func TestContentBuilder(t *testing.T) {
	mockBody := body.Raw("test-body")
	rt := reftype.Text("test-ref")
	filename := "test.epub"

	builder := ContentBuilder(mockBody, rt).Filename(filename)
	content := builder.Build()

	if content.Body != mockBody {
		t.Errorf("expected body %v, got %v", mockBody, content.Body)
	}
	if content.ReferenceType != rt {
		t.Errorf("expected reference type %v, got %v", rt, content.ReferenceType)
	}
	if content.Filename.IsNil() {
		t.Errorf("expected filename %s, got %s", filename, content.Filename)
	}

	child := epub.Content{Body: body.Raw("child")}
	builder.AddChildren(child)
	if len(builder.SubContents) != 1 {
		t.Errorf("expected 1 child, got %d", len(builder.SubContents))
	}

	ref := epub.ContentReference{Title: "ref-title"}
	builder.AddContentReferences(ref)
	if len(builder.ContentReferences) != 1 {
		t.Errorf("expected 1 content reference, got %d", len(builder.ContentReferences))
	}
}

func TestContentReferenceBuilder(t *testing.T) {
	title := "Chapter 1"
	id := "ch1-id"

	builder := ContentReferenceBuilder(title).ID(id)
	ref := builder.Build()

	if ref.Title != title {
		t.Errorf("expected title %s, got %s", title, ref.Title)
	}
	if ref.ID.AsValue() != id {
		t.Errorf("expected ID %s, got %s", id, ref.ID)
	}

	subRef := epub.ContentReference{Title: "Sub-section"}
	builder.AddChildren(subRef)

	if len(builder.SubContentReferences) != 1 {
		t.Errorf("expected 1 sub-reference, got %d", len(builder.SubContentReferences))
	}
	if builder.SubContentReferences[0].Title != "Sub-section" {
		t.Errorf("expected sub-reference title 'Sub-section', got %s", builder.SubContentReferences[0].Title)
	}
}

func TestBuilderChaining(t *testing.T) {
	res := ContentReferenceBuilder("Root").
		ID("root-id").
		AddChildren(epub.ContentReference{Title: "Child 1"}).
		AddChildren(epub.ContentReference{Title: "Child 2"}).
		Build()

	if len(res.SubContentReferences) != 2 {
		t.Errorf("expected 2 children from chained calls, got %d", len(res.SubContentReferences))
	}
}
