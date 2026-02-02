package resource

type Resource interface {
	Mediatype() string
	isResource()
}

type Image interface {
	Resource
	isImage()
}

type JpgFile string

func (JpgFile) isResource()       {}
func (JpgFile) isImage()          {}
func (JpgFile) Mediatype() string { return "image/jpeg" }

type GifFile string

func (GifFile) isResource()       {}
func (GifFile) isImage()          {}
func (GifFile) Mediatype() string { return "image/gif" }

type SvgFile string

func (SvgFile) isResource()       {}
func (SvgFile) isImage()          {}
func (SvgFile) Mediatype() string { return "image/svg+xml" }

type PngFile string

func (PngFile) isResource()       {}
func (PngFile) isImage()          {}
func (PngFile) Mediatype() string { return "image/png" }

type FontFile string

func (FontFile) isResource()       {}
func (FontFile) Mediatype() string { return "application/vnd.ms-opentype" }

type AudioFile string

func (AudioFile) isResource()       {}
func (AudioFile) Mediatype() string { return "audio/mpeg" }

type VideoFile string

func (VideoFile) isResource()       {}
func (VideoFile) Mediatype() string { return "video/mp4" }
