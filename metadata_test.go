package liber

import (
	"testing"
	"time"

	"github.com/javiorfo/liber/ident"
	"github.com/javiorfo/liber/lang"
)

func TestMetadataBuilder(t *testing.T) {
	title := "The Go Programming Language"
	language := lang.English
	identifier := ident.ISBN("123456789")
	testTime := time.Date(2024, 5, 20, 0, 0, 0, 0, time.UTC)

	builder := MetadataBuilder(title, language, identifier).
		Creator("Alan Donovan").
		Publisher("Addison-Wesley").
		Contributor("Brian Kernighan").
		Subject("Software Engineering").
		Date(testTime).
		Description("A comprehensive guide to Go.")

	metadata := builder.Build()

	if metadata.Title != title {
		t.Errorf("expected title %s, got %s", title, metadata.Title)
	}
	if metadata.Language != language {
		t.Errorf("expected language %v, got %v", language, metadata.Language)
	}
	if metadata.Identifier != identifier {
		t.Errorf("expected identifier %v, got %v", identifier, metadata.Identifier)
	}

	if metadata.Creator.IsNil() {
		t.Errorf("expected creator got %s", metadata.Creator)
	}

	if metadata.Publisher.IsNil() {
		t.Errorf("expected publisher got %s", metadata.Publisher)
	}

	if metadata.Contributor.IsNil() {
		t.Errorf("expected contributor got %s", metadata.Contributor)
	}

	if metadata.Subject.IsNil() {
		t.Errorf("expected subject got %s", metadata.Subject)
	}

	if metadata.Description.IsNil() {
		t.Errorf("expected description got %s", metadata.Description)
	}

	if !metadata.Date.AsValue().Equal(testTime) {
		t.Errorf("expected date %v, got %v", testTime, metadata.Date)
	}
}
