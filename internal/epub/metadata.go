package epub

import (
	"time"

	"github.com/javiorfo/liber/ident"
	"github.com/javiorfo/liber/lang"
	"github.com/javiorfo/nilo"
)

type Metadata struct {
	Title       string
	Language    lang.Language
	Identifier  ident.Identifier
	Creator     nilo.Option[string]
	Contributor nilo.Option[string]
	Publisher   nilo.Option[string]
	Date        nilo.Option[time.Time]
	Subject     nilo.Option[string]
	Description nilo.Option[string]
}
