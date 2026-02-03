package epub

import (
	"testing"

	"github.com/javiorfo/nilo"
)

func TestContentReference_Level(t *testing.T) {
	tests := []struct {
		name     string
		ref      ContentReference
		expected int
	}{
		{
			name:     "No children",
			ref:      ContentReference{Title: "Root"},
			expected: 0,
		},
		{
			name: "One level deep",
			ref: ContentReference{
				Title: "Root",
				SubContentReferences: []ContentReference{
					{Title: "Child"},
				},
			},
			expected: 1,
		},
		{
			name: "Three levels deep",
			ref: ContentReference{
				Title: "Root",
				SubContentReferences: []ContentReference{
					{
						SubContentReferences: []ContentReference{
							{
								SubContentReferences: []ContentReference{
									{Title: "Leaf"},
								},
							},
						},
					},
				},
			},
			expected: 3,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.ref.level(); got != tt.expected {
				t.Errorf("level() = %d, want %d", got, tt.expected)
			}
		})
	}
}

func TestContentReference_ReferenceName(t *testing.T) {
	xhtml := "chapter01.xhtml"

	t.Run("With ID present", func(t *testing.T) {
		ref := ContentReference{
			ID: nilo.Value("my-anchor"),
		}
		expected := "chapter01.xhtml#my-anchor"
		if got := ref.ReferenceName(xhtml, 1); got != expected {
			t.Errorf("got %s, want %s", got, expected)
		}
	})

	t.Run("Without ID (fallback to number)", func(t *testing.T) {
		ref := ContentReference{
			ID: nilo.Nil[string](),
		}
		expected := "chapter01.xhtml#id05"
		if got := ref.ReferenceName(xhtml, 5); got != expected {
			t.Errorf("got %s, want %s", got, expected)
		}
	})
}
