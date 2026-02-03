package lang

// Language defines the interface for language types supported by the EPUB standard.
// It uses a marker method to ensure only valid internal types are used.
type Language interface {
	isLanguage()
	// Code returns the ISO 639-1 two-letter language code (e.g., "en", "es").
	Code() string
}

// Lang represents a specific language using an internal integer enumeration.
type Lang int

const (
	Arabic Lang = iota
	Bulgarian
	Chinese
	Croatian
	Czech
	Danish
	Dutch
	English
	Estonian
	Finnish
	French
	Greek
	German
	Hebrew
	Hungarian
	Icelandic
	Indonesian
	Irish
	Italian
	Japanese
	Korean
	Latvian
	Lithuanian
	Macedonian
	Malay
	Maltese
	Norwegian
	Persian
	Polish
	Portuguese
	Romanian
	Russian
	Serbian
	Slovak
	Slovenian
	Spanish
	Swahili
	Swedish
	Tagalog
	Thai
	Turkish
	Ukrainian
	Urdu
	Vietnamese
	Welsh
	Yiddish
)

func (l Lang) isLanguage() {}

// Code returns the corresponding ISO 639-1 string for the Lang value.
// If the language is unrecognized, it defaults to "en" (English).
func (l Lang) Code() string {
	switch l {
	case Arabic:
		return "ar"
	case Bulgarian:
		return "bg"
	case Chinese:
		return "zh"
	case Croatian:
		return "hr"
	case Czech:
		return "cs"
	case Danish:
		return "da"
	case Dutch:
		return "nl"
	case English:
		return "en"
	case Estonian:
		return "et"
	case Finnish:
		return "fi"
	case French:
		return "fr"
	case German:
		return "de"
	case Greek:
		return "el"
	case Hebrew:
		return "he"
	case Hungarian:
		return "hu"
	case Icelandic:
		return "is"
	case Indonesian:
		return "id"
	case Irish:
		return "ga"
	case Italian:
		return "it"
	case Japanese:
		return "ja"
	case Korean:
		return "ko"
	case Latvian:
		return "lv"
	case Lithuanian:
		return "lt"
	case Macedonian:
		return "mk"
	case Malay:
		return "ms"
	case Maltese:
		return "mt"
	case Norwegian:
		return "no"
	case Persian:
		return "fa"
	case Polish:
		return "pl"
	case Portuguese:
		return "pt"
	case Romanian:
		return "ro"
	case Russian:
		return "ru"
	case Serbian:
		return "sr"
	case Slovak:
		return "sk"
	case Slovenian:
		return "sl"
	case Spanish:
		return "es"
	case Swahili:
		return "sw"
	case Swedish:
		return "sv"
	case Tagalog:
		return "tl"
	case Thai:
		return "th"
	case Turkish:
		return "tr"
	case Ukrainian:
		return "uk"
	case Urdu:
		return "ur"
	case Vietnamese:
		return "vi"
	case Welsh:
		return "cy"
	case Yiddish:
		return "yi"
	default:
		return "en"
	}
}
