package ident

import (
	"fmt"

	"github.com/google/uuid"
)

type Identifier interface {
	fmt.Stringer
	Label() string
	isIdentifier()
}

type UUID string

func (UUID) isIdentifier()    {}
func (u UUID) String() string { return "urn:uuid:" + string(u) }
func (u UUID) Label() string  { return "UUID" }

func Default() UUID {
	return UUID(uuid.NewString())
}

type ISBN string

func (ISBN) isIdentifier()    {}
func (i ISBN) String() string { return "urn:isbn:" + string(i) }
func (i ISBN) Label() string  { return "ISBN" }
