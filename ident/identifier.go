package ident

import (
	"fmt"

	"github.com/google/uuid"
)

// Identifier defines the interface for a unique publication ID.
// It implements fmt.Stringer to provide the formatted URN required for the EPUB package document.
type Identifier interface {
	fmt.Stringer
	// Label returns the human-readable name of the identifier type (e.g., "UUID", "ISBN").
	Label() string
	// isIdentifier is a marker method to enforce a sealed interface.
	isIdentifier()
}

// UUID represents a Universally Unique Identifier.
// When stringified, it prepends the "urn:uuid:" prefix.
type UUID string

func (UUID) isIdentifier() {}

// String returns the identifier as a URN string (e.g., "urn:uuid:550e8400-e29b...").
func (u UUID) String() string { return "urn:uuid:" + string(u) }

// Label returns the identifier type name "UUID".
func (u UUID) Label() string { return "UUID" }

// Default generates a new, random version 4 UUID using the underlying uuid library.
func Default() UUID {
	return UUID(uuid.NewString())
}

// ISBN represents an International Standard Book Number.
// When stringified, it prepends the "urn:isbn:" prefix.
type ISBN string

func (ISBN) isIdentifier() {}

// String returns the identifier as a URN string (e.g., "urn:isbn:9783161484100").
func (i ISBN) String() string { return "urn:isbn:" + string(i) }

// Label returns the identifier type name "ISBN".
func (i ISBN) Label() string { return "ISBN" }
