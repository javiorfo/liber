package epub

import (
	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/resource"
	"github.com/javiorfo/nilo"
)

type Epub struct {
	Metadata   Metadata
	Stylesheet nilo.Option[body.Body]
	CoverImage nilo.Option[resource.Image]
	Resources  []resource.Resource
	Contents   []Content
}

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
