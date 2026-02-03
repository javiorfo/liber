package files

// ByteSeq is a generic constraint that allows for types that are
// underlyingly strings or byte slices.
type ByteSeq interface{ ~string | ~[]byte }

// FileContent represents a file to be included in the EPUB archive,
// pairing its destination path with its raw data.
type FileContent[T ByteSeq] struct {
	// Filepath is the relative path within the EPUB zip container.
	Filepath string
	// Bytes is the content of the file, either as a string or a byte slice.
	Bytes T
}

// ToBytes converts a string-based FileContent into a byte-slice-based
// FileContent, which is often necessary for the final compression stage.
func (f FileContent[string]) ToBytes() FileContent[[]byte] {
	return FileContent[[]byte]{
		Filepath: f.Filepath,
		Bytes:    []byte(f.Bytes),
	}
}

// NewFileContent is a constructor that initializes a FileContent with
// the given path and data.
func NewFileContent[T ByteSeq](filepath string, bytes T) FileContent[T] {
	return FileContent[T]{filepath, bytes}
}

// Container returns the mandatory "META-INF/container.xml" file.
// This file tells the e-reader where the root metadata (OPF) file is located.
func Container() FileContent[[]byte] {
	return FileContent[[]byte]{
		Filepath: "META-INF/container.xml",
		Bytes: []byte(`<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
    <rootfiles>
        <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
   </rootfiles>
</container>
        `),
	}
}

// Mimetype returns the mandatory "mimetype" file.
// According to EPUB specs, this must be the first file in the ZIP archive
// and must be uncompressed.
func Mimetype() FileContent[[]byte] {
	return FileContent[[]byte]{"mimetype", []byte("application/epub+zip")}
}

// DisplayOptions returns a configuration file specifically for Apple iBooks
// to ensure custom embedded fonts are enabled by default.
func DisplayOptions() FileContent[[]byte] {
	return FileContent[[]byte]{
		Filepath: "META-INF/com.apple.ibooks.display-options.xml",
		Bytes: []byte(`<?xml version="1.0" encoding="utf-8"?>
<display_options>
    <platform name="*">
        <option name="specified-fonts">true</option>
    </platform>
</display_options>
        `),
	}
}
