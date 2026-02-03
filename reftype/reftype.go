package reftype

// ReferenceType defines the semantic meaning of a content section within an EPUB.
// These types help reading systems identify the purpose of a file (e.g., a cover or a glossary).
type ReferenceType interface {
	Type() string
	// isReferenceType is a marker method used to enforce a sealed interface.
	isReferenceType()
}

// Acknowledgements represents a section containing credits or thanks.
type Acknowledgements string

func (Acknowledgements) isReferenceType() {}
func (Acknowledgements) Type() string     { return "acknowledgements" }

// Bibliography represents a list of cited works or references.
type Bibliography string

func (Bibliography) isReferenceType() {}
func (Bibliography) Type() string     { return "bibliography" }

// Colophon represents a brief description of publication facts (printer, fonts, etc.).
type Colophon string

func (Colophon) isReferenceType() {}
func (Colophon) Type() string     { return "colophon" }

// Copyright represents the copyright notice and legal information page.
type Copyright string

func (Copyright) isReferenceType() {}
func (Copyright) Type() string     { return "copyright-page" }

// Cover represents the primary cover image or page of the book.
type Cover string

func (Cover) isReferenceType() {}
func (Cover) Type() string     { return "cover" }

// Dedication represents the author's dedication section.
type Dedication string

func (Dedication) isReferenceType() {}
func (Dedication) Type() string     { return "dedication" }

// Epigraph represents a short quotation at the beginning of the book or a chapter.
type Epigraph string

func (Epigraph) isReferenceType() {}
func (Epigraph) Type() string     { return "epigraph" }

// Foreword represents an introduction written by someone other than the author.
type Foreword string

func (Foreword) isReferenceType() {}
func (Foreword) Type() string     { return "foreword" }

// Glossary represents a list of specialized terms and their definitions.
type Glossary string

func (Glossary) isReferenceType() {}
func (Glossary) Type() string     { return "glossary" }

// Index represents the back-matter index of names, places, or topics.
type Index string

func (Index) isReferenceType() {}
func (Index) Type() string     { return "index" }

// Loi represents the List of Illustrations.
type Loi string

func (Loi) isReferenceType() {}
func (Loi) Type() string     { return "loi" }

// Lot represents the List of Tables.
type Lot string

func (Lot) isReferenceType() {}
func (Lot) Type() string     { return "lot" }

// Notes represents a section for endnotes or footnotes.
type Notes string

func (Notes) isReferenceType() {}
func (Notes) Type() string     { return "notes" }

// Preface represents an introduction written by the author.
type Preface string

func (Preface) isReferenceType() {}
func (Preface) Type() string     { return "preface" }

// Text represents the main body of the work.
type Text string

func (Text) isReferenceType() {}
func (Text) Type() string     { return "text" }

// TitlePage represents the page containing the book's title and author information.
type TitlePage string

func (TitlePage) isReferenceType() {}
func (TitlePage) Type() string     { return "title-page" }

// Toc represents the Table of Contents.
type Toc string

func (Toc) isReferenceType() {}
func (Toc) Type() string     { return "toc" }
