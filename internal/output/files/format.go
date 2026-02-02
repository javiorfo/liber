package files

import (
	"bytes"
	"encoding/xml"
	"io"
	"strings"
)

func FormatXml(r io.Reader) (string, error) {
	var out bytes.Buffer

	decoder := xml.NewDecoder(r)
	encoder := xml.NewEncoder(&out)
	encoder.Indent("", "  ")

	for {
		token, err := decoder.Token()
		if err == io.EOF {
			break
		}

		if err != nil {
			return "", err
		}

		switch t := token.(type) {
		case xml.StartElement:
			t.Name.Space = ""
			for i := range t.Attr {
				t.Attr[i].Name.Space = ""
			}
			token = t
		case xml.EndElement:
			t.Name.Space = ""
			token = t
		case xml.CharData:
			if len(bytes.TrimSpace(t)) == 0 {
				continue
			}
		}

		err = encoder.EncodeToken(token)
		if err != nil {
			return "", err
		}
	}

	encoder.Flush()
	return out.String(), nil
}

func FormatXmlString(s string) (string, error) {
	return FormatXml(strings.NewReader(s))
}
