package reftype

type ReferenceType interface {
	Type() string
	isReferenceType()
}

type Acknowledgements string

func (Acknowledgements) isReferenceType() {}
func (Acknowledgements) Type() string     { return "acknowledgements" }

type Bibliography string

func (Bibliography) isReferenceType() {}
func (Bibliography) Type() string     { return "bibliography" }

type Colophon string

func (Colophon) isReferenceType() {}
func (Colophon) Type() string     { return "colophon" }

type Copyright string

func (Copyright) isReferenceType() {}
func (Copyright) Type() string     { return "copyright-page" }

type Cover string

func (Cover) isReferenceType() {}
func (Cover) Type() string     { return "cover" }

type Dedication string

func (Dedication) isReferenceType() {}
func (Dedication) Type() string     { return "dedication" }

type Epigraph string

func (Epigraph) isReferenceType() {}
func (Epigraph) Type() string     { return "epigraph" }

type Foreword string

func (Foreword) isReferenceType() {}
func (Foreword) Type() string     { return "foreword" }

type Glossary string

func (Glossary) isReferenceType() {}
func (Glossary) Type() string     { return "glossary" }

type Index string

func (Index) isReferenceType() {}
func (Index) Type() string     { return "index" }

type Loi string

func (Loi) isReferenceType() {}
func (Loi) Type() string     { return "loi" }

type Lot string

func (Lot) isReferenceType() {}
func (Lot) Type() string     { return "lot" }

type Notes string

func (Notes) isReferenceType() {}
func (Notes) Type() string     { return "notes" }

type Preface string

func (Preface) isReferenceType() {}
func (Preface) Type() string     { return "preface" }

type Text string

func (Text) isReferenceType() {}
func (Text) Type() string     { return "text" }

type TitlePage string

func (TitlePage) isReferenceType() {}
func (TitlePage) Type() string     { return "title-page" }

type Toc string

func (Toc) isReferenceType() {}
func (Toc) Type() string     { return "toc" }
