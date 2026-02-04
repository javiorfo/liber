package resource

import "strings"

// Resource represents any asset that can be included in an EPUB.
// It requires a valid MIME media type.
type Resource interface {
	Mediatype() string
	// isResource is a marker method to ensure type safety.
	isResource()
}

// Image represents a resource specifically classified as an image format,
// such as JPEG, PNG, GIF, or SVG.
type Image interface {
	Resource
	// isImage is a marker method to differentiate images from other resources.
	isImage()
}

// JpgFile represents a JPEG image resource.
type JpgFile string

func (JpgFile) isResource()       {}
func (JpgFile) isImage()          {}
func (JpgFile) Mediatype() string { return "image/jpeg" }

// GifFile represents a GIF image resource.
type GifFile string

func (GifFile) isResource()       {}
func (GifFile) isImage()          {}
func (GifFile) Mediatype() string { return "image/gif" }

// SvgFile represents a Scalable Vector Graphics resource.
type SvgFile string

func (SvgFile) isResource()       {}
func (SvgFile) isImage()          {}
func (SvgFile) Mediatype() string { return "image/svg+xml" }

// PngFile represents a Portable Network Graphics resource.
type PngFile string

func (PngFile) isResource()       {}
func (PngFile) isImage()          {}
func (PngFile) Mediatype() string { return "image/png" }

// FontFile represents an OpenType font resource used for document styling.
type FontFile string

func (FontFile) isResource() {}
func (f FontFile) Mediatype() string {
	if strings.HasSuffix(string(f), "ttf") {
		return "application/x-font-ttf"
	}
	return "application/vnd.ms-opentype"
}

// AudioFile represents an MPEG audio resource (MP3).
type AudioFile string

func (AudioFile) isResource()       {}
func (AudioFile) Mediatype() string { return "audio/mpeg" }

// VideoFile represents an MP4 video resource.
type VideoFile string

func (VideoFile) isResource()       {}
func (VideoFile) Mediatype() string { return "video/mp4" }
