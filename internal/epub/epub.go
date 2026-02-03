package epub

import (
	"time"

	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/ident"
	"github.com/javiorfo/liber/lang"
	"github.com/javiorfo/liber/resource"
	"github.com/javiorfo/nilo"
)

//-------------------- Epub --------------------------//

// Epub represents the complete structure of an EPUB document.
// It serves as the root container that holds metadata, styling,
// embedded resources, and the hierarchical content tree.
type Epub struct {
	// Metadata contains the bibliographic information for the publication.
	Metadata Metadata
	// Stylesheet is an optional global CSS body applied to all content sections.
	Stylesheet nilo.Option[body.Body]
	// CoverImage is the optional image designated as the book's front cover.
	CoverImage nilo.Option[resource.Image]
	// Resources is a collection of secondary assets like fonts, audio, or video.
	Resources []resource.Resource
	// Contents holds the primary reading order and structural sections of the book.
	Contents []Content
}

// Level calculates the maximum depth of the entire EPUB's navigation and content tree.
// It evaluates both nested SubContents and ContentReferences across all root-level items
// to determine the structural complexity of the document.
func (e Epub) Level() int {
	if len(e.Contents) == 0 {
		return 0
	}

	maxSub := 1
	maxRef := 1

	for _, content := range e.Contents {
		if lvl := content.Level() + 1; lvl > maxSub {
			maxSub = lvl
		}

		if refLvl := content.LevelReferenceContent() + 1; refLvl > maxRef {
			maxRef = refLvl
		}
	}

	if maxSub > maxRef {
		return maxSub
	}
	return maxRef
}

//-------------------- Metadata --------------------------//

// Metadata represents the Dublin Core metadata elements for the EPUB.
// It includes mandatory fields like Title, Language, and Identifier,
// along with optional bibliographic descriptors.
type Metadata struct {
	// Title is the primary name of the publication.
	Title string
	// Language defines the primary language of the content (ISO 639-1).
	Language lang.Language
	// Identifier is the unique ID for the book (e.g., UUID or ISBN).
	Identifier ident.Identifier
	// Creator is the primary author or organization responsible for the work.
	Creator nilo.Option[string]
	// Contributor is an entity that made secondary contributions.
	Contributor nilo.Option[string]
	// Publisher is the entity responsible for publication.
	Publisher nilo.Option[string]
	// Date is the publication or creation timestamp.
	Date nilo.Option[time.Time]
	// Subject defines the keywords or categories for the book.
	Subject nilo.Option[string]
	// Description is a summary or abstract of the content.
	Description nilo.Option[string]
}
