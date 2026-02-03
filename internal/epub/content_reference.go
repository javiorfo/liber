package epub

import (
	"fmt"

	"github.com/javiorfo/nilo"
)

// ContentReference represents a navigational entry point within a content file.
// It is used to generate Table of Contents (ToC) entries that can point to
// specific anchors or headers within a section.
type ContentReference struct {
	// Title is the text displayed in the navigation menu.
	Title string
	// SubContentReferences allows for nested navigation levels (e.g., Chapter 1 > Section A).
	SubContentReferences []ContentReference
	// ID is an optional HTML anchor ID within the target file.
	ID nilo.Option[string]
}

// level calculates the maximum depth of the nested navigation hierarchy.
// A reference with no children returns 0.
func (cr ContentReference) level() int {
	if len(cr.SubContentReferences) == 0 {
		return 0
	}
	return 1 + cr.SubContentReferences[0].level()
}

// ReferenceName constructs a full URI for the reference, combining the XHTML
// filename with an anchor fragment. If no ID is provided, it generates a
// default fragment (e.g., "filename.xhtml#id01").
func (cr ContentReference) ReferenceName(xhtml string, number int) string {
	return cr.ID.Map(func(s string) string {
		return fmt.Sprintf("%s#%s", xhtml, s)
	}).Or(fmt.Sprintf("%s#id%02d", xhtml, number))
}
