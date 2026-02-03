package body

import "os"

// Body defines the interface for content that can be rendered into an EPUB section.
// It supports both raw string data and external file paths.
type Body interface {
	// ToBytes returns the content as a byte slice.
	ToBytes() ([]byte, error)
	// ToString returns the content as a string.
	ToString() (string, error)
	// isBody is a marker method to ensure only valid body types are used.
	isBody()
}

// Raw represents content that is already held in memory as a string.
// This is useful for dynamically generated HTML or text.
type Raw string

func (Raw) isBody() {}

// ToBytes converts the raw string into a byte slice.
func (r Raw) ToBytes() ([]byte, error) {
	return []byte(r), nil
}

// ToString returns the raw string as-is.
func (r Raw) ToString() (string, error) {
	return string(r), nil
}

// File represents a path to a file on the local file system.
// The content is read only when a conversion method is called.
type File string

func (File) isBody() {}

// ToString reads the file from the disk and returns its contents as a string.
func (p File) ToString() (string, error) {
	b, err := p.ToBytes()
	return string(b), err
}

// ToBytes reads the file from the disk and returns its contents as a byte slice.
// It returns an error if the file cannot be found or read.
func (p File) ToBytes() ([]byte, error) {
	content, err := os.ReadFile(string(p))
	if err != nil {
		return nil, err
	}

	return content, nil
}
