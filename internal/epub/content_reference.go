package epub

import (
	"fmt"

	"github.com/javiorfo/nilo"
)

type ContentReference struct {
	Title                string
	SubContentReferences []ContentReference
	ID                   nilo.Option[string]
}

func (cr ContentReference) level() int {
	if len(cr.SubContentReferences) == 0 {
		return 0
	}
	return 1 + cr.SubContentReferences[0].level()
}

func (cr ContentReference) ReferenceName(xhtml string, number int) string {
	return cr.ID.Map(func(s string) string {
		return fmt.Sprintf("%s#%s", xhtml, s)
	}).Or(fmt.Sprintf("%s#id%02d", xhtml, number))
}
