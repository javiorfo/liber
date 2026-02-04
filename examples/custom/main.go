package main

import (
	"log"
	"os"

	"github.com/javiorfo/liber"
	"github.com/javiorfo/liber/body"
	"github.com/javiorfo/liber/ident"
	"github.com/javiorfo/liber/lang"
	"github.com/javiorfo/liber/reftype"
	"github.com/javiorfo/liber/resource"
)

func main() {
	book, err := os.Create("book.epub")
	if err != nil {
		panic(err)
	}
	defer book.Close()

	e := liber.EpubBuilder(
		liber.MetadataBuilder("My Book", lang.Spanish, ident.Default()).
			Creator("author").
			Build(),
	).
		Stylesheet(body.File("./files/styles.css")).
		CoverImage(resource.PngFile("./files/mock.png")).
		AddResources(resource.FontFile("./files/mock.otf")).
		AddContents(
			liber.ContentBuilder(body.File("./files/ch1.xhtml"), reftype.Text("Chapter 1")).
				Filename("chapter1.xhtml").
				AddContentReferences(
					liber.ContentReferenceBuilder("Section 1.1").ID("s1-1").
						AddChildren(liber.ContentReferenceBuilder("Section 1.1.1").ID("s1-1-1").Build()).
						Build(),
				).
				AddContentReferences(liber.ContentReferenceBuilder("Section 1.2").ID("s1-2").Build()).
				AddChildren(
					liber.ContentBuilder(
						body.Raw("<h1>Chapter 2</h1>"), reftype.Text("Chapter 2")).
						Filename("chapter2.xhtml").
						Build(),
					liber.ContentBuilder(
						body.Raw("<h1>Chapter 3</h1>"), reftype.Text("Chapter 3")).
						Filename("chapter3.xhtml").
						AddChildren(
							liber.ContentBuilder(body.Raw("<h1>Chapter 4</h1>"), reftype.Epigraph("Chapter 4")).
								Filename("chapter4.xhtml").
								Build()).
						Build(),
				).
				Build(),
		).
		Build()

	if err := liber.Create(&e, book); err != nil {
		os.Remove("book.epub")
		log.Fatal(err)
	}
}
