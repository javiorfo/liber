package epub

import "testing"

func TestEpub_Level(t *testing.T) {
	tests := []struct {
		name     string
		contents []Content
		expected int
	}{
		{
			name:     "Empty contents",
			contents: []Content{},
			expected: 0,
		},
		{
			name: "Flat contents (Level 1)",
			contents: []Content{
				{SubContents: nil},
				{SubContents: nil},
			},
			expected: 1,
		},
		{
			name: "Deeply nested SubContents (Level 3)",
			contents: []Content{
				{
					SubContents: []Content{
						{
							SubContents: []Content{{}},
						},
					},
				},
			},
			expected: 3,
		},
		{
			name: "Deeply nested ContentReferences (Level 4)",
			contents: []Content{
				{
					ContentReferences: []ContentReference{
						{
							SubContentReferences: []ContentReference{
								{
									SubContentReferences: []ContentReference{{}},
								},
							},
						},
					},
				},
			},
			expected: 4,
		},
		{
			name: "Mixed nesting - References win",
			contents: []Content{
				{
					SubContents: []Content{{}}, // Level 2
					ContentReferences: []ContentReference{
						{SubContentReferences: []ContentReference{{}}},
					},
				},
			},
			expected: 3,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			e := Epub{
				Contents: tt.contents,
			}
			got := e.Level()
			if got != tt.expected {
				t.Errorf("Epub.Level() = %d; want %d", got, tt.expected)
			}
		})
	}
}
