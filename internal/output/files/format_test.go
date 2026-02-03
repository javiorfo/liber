package files

import (
	"strings"
	"testing"
)

func TestFormat(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
		wantErr  bool
	}{
		{
			name:     "Simple XML",
			input:    `<root><child>hello</child></root>`,
			expected: "<root>\n  <child>hello</child>\n</root>",
			wantErr:  false,
		},
		{
			name:     "Nested Elements",
			input:    `<user><id>1</id><profile><name>John</name></profile></user>`,
			expected: "<user>\n  <id>1</id>\n  <profile>\n    <name>John</name>\n  </profile>\n</user>",
			wantErr:  false,
		},
		{
			name:    "Invalid XML",
			input:   `<root><unclosed>`,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := FormatXML(tt.input)

			if !tt.wantErr && strings.TrimSpace(got) != strings.TrimSpace(tt.expected) {
				t.Errorf("FormatXML() got:\n%s\nwant:\n%s", got, tt.expected)
			}
		})
	}
}
