package main

import (
	"log"
	"os"

	"github.com/javiorfo/liber"
	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/ident"
	"github.com/javiorfo/liber/lang"
	"github.com/javiorfo/liber/reftype"
)

func main() {
	e := liber.EpubBuilder(
		liber.MetadataBuilder("My Book", lang.Spanish, ident.Default()).
			Creator("author").
			Build(),
	).
		Stylesheet(body.Raw("body {}")).
		AddContents(
			liber.ContentBuilder(body.Raw(
				`<h1>Chapter 1</h1>
			<h2 id="id01">Section 1.1</h2>
			<h2 id="id02">Section 1.1.1</h2>
			<h2 id="id03">Section 1.2</h2>`,
			), reftype.Text("Chapter 1")).
				AddContentReferences(liber.ContentReferenceBuilder("Section 1.1").
					AddChildren(liber.ContentReferenceBuilder("Section 1.1.1").Build()).
					Build()).
				AddContentReferences(liber.ContentReferenceBuilder("Section 1.2").Build()).
				AddChildren(
					liber.ContentBuilder(body.Raw("<h1>Chapter 2</h1>"), reftype.Text("Chapter 2")).Build(),
					liber.ContentBuilder(body.Raw("<h1>Chapter 3</h1>"), reftype.Text("Chapter 3")).
						AddChildren(liber.ContentBuilder(body.Raw("<h1>Chapter 4</h1>"), reftype.Text("Chapter 4")).
							Build()).
						Build(),
				).
				Build(),
		).
		Build()

	if err := liber.Create(&e, os.Stdout); err != nil {
		log.Fatal(err)
	}
}
