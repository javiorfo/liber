package files

type ByteSeq interface{ ~string | ~[]byte }

type FileContent[T ByteSeq] struct {
	Filepath string
	Bytes    T
}

func (f FileContent[string]) ToBytes() FileContent[[]byte] {
	return FileContent[[]byte]{
		Filepath: f.Filepath,
		Bytes:    []byte(f.Bytes),
	}
}

func NewFileContent[T ByteSeq](filepath string, bytes T) FileContent[T] {
	return FileContent[T]{filepath, bytes}
}

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

func Mimetype() FileContent[[]byte] {
	return FileContent[[]byte]{"mimetype", []byte("application/epub+zip")}
}

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
