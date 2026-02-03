package epub

import (
	"strings"
	"testing"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/reftype"
	"github.com/javiorfo/nilo"
)

func TestContent_Levels(t *testing.T) {
	t.Run("Level depth", func(t *testing.T) {
		c := Content{
			SubContents: []Content{
				{
					SubContents: []Content{{}},
				},
			},
		}
		if got := c.Level(); got != 2 {
			t.Errorf("Level() = %d, want 2", got)
		}
	})

	t.Run("LevelReferenceContent mixed", func(t *testing.T) {
		c := Content{
			ContentReferences: []ContentReference{
				{SubContentReferences: []ContentReference{{}}},
			},
			SubContents: []Content{
				{
					ContentReferences: []ContentReference{
						{SubContentReferences: []ContentReference{{}, {}}},
					},
				},
			},
		}
		if got := c.LevelReferenceContent(); got != 3 {
			t.Errorf("LevelReferenceContent() = %d, want 3", got)
		}
	})
}

func TestContent_GetFilename(t *testing.T) {
	t.Run("With Filename", func(t *testing.T) {
		c := Content{Filename: nilo.Value("intro.xhtml")}
		if got := c.GetFilename(1); got != "intro.xhtml" {
			t.Errorf("got %s, want intro.xhtml", got)
		}
	})

	t.Run("Fallback to default", func(t *testing.T) {
		c := Content{Filename: nilo.Nil[string]()}
		if got := c.GetFilename(5); got != "c05.xhtml" {
			t.Errorf("got %s, want c05.xhtml", got)
		}
	})
}

func TestContent_CreateFileContent(t *testing.T) {
	mockBody := body.Raw("<body>Content</body>")

	c := Content{
		ReferenceType: reftype.Text("Chapter"),
		Body:          mockBody,
		SubContents: []Content{
			{
				ReferenceType: reftype.Text("SubChapter"),
				Body:          body.Raw("<body>Subcontent</body>"),
			},
		},
	}

	number := 0
	stylesheet := "<link rel=\"stylesheet\" href=\"style.css\"/>"

	results, err := c.CreateFileContent(&number, stylesheet)
	if err != nil {
		t.Fatalf("failed to create content: %v", err)
	}

	if len(results) != 2 {
		t.Errorf("expected 2 file contents, got %d", len(results))
	}

	if number != 2 {
		t.Errorf("expected counter to be 2, got %d", number)
	}

	parentFile := results[0]
	if parentFile.Filepath != "OEBPS/c01.xhtml" {
		t.Errorf("wrong path for parent: %s", parentFile.Filepath)
	}

	contentStr := parentFile.Bytes
	if !strings.Contains(contentStr, "Chapter") {
		t.Error("ReferenceType missing from XML")
	}
	if !strings.Contains(contentStr, stylesheet) {
		t.Error("Stylesheet missing from XML")
	}
}
