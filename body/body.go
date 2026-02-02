package body

import "os"

type Body interface {
	ToBytes() ([]byte, error)
	ToString() (string, error)
	isBody()
}

type Raw string

func (Raw) isBody() {}

func (r Raw) ToBytes() ([]byte, error) {
	return []byte(r), nil
}

func (r Raw) ToString() (string, error) {
	return string(r), nil
}

type File string

func (File) isBody() {}

func (p File) ToString() (string, error) {
	b, err := p.ToBytes()
	return string(b), err
}

func (p File) ToBytes() ([]byte, error) {
	content, err := os.ReadFile(string(p))
	if err != nil {
		return nil, err
	}

	return content, nil
}
