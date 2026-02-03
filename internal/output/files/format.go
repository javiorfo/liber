package files

import (
	"html"
	"regexp"
	"runtime"
	"strings"
)

var (
	// reg identifies XML tags, including self-closing and special tags like processing instructions.
	reg = regexp.MustCompile(`<([/!]?)([^>]+?)(/?)>`)
	// reXMLComments identifies XML comment blocks ().
	reXMLComments = regexp.MustCompile(`(?s)()`)
	// reSpaces identifies whitespace between tags for collapsing.
	reSpaces = regexp.MustCompile(`(?s)>\s+<`)
	// reNewlines identifies various newline characters.
	reNewlines = regexp.MustCompile(`\r*\n`)
	// NL is the system-specific newline character (initialized in init()).
	NL = "\n"
)

func init() {
	// Adjusts the newline character based on the host operating system to ensure
	// file compatibility on Windows platforms.
	if runtime.GOOS == "windows" {
		NL = "\r\n"
	}
}

// FormatXML takes a raw XML string and returns a formatted, indented version.
// It collapses existing whitespace and applies a hierarchical indentation
// based on the tag nesting level.
func FormatXML(xmlString string) string {
	nestedTagsInComment := false
	// Strip all existing whitespace between tags to ensure a clean slate for formatting.
	src := reSpaces.ReplaceAllString(xmlString, "><")
	if nestedTagsInComment {
		src = reXMLComments.ReplaceAllStringFunc(src, func(m string) string {
			parts := reXMLComments.FindStringSubmatch(m)
			p2 := reNewlines.ReplaceAllString(parts[2], " ")
			return parts[1] + html.EscapeString(p2) + parts[3]
		})
	}
	rf := replaceTag()
	r := reg.ReplaceAllStringFunc(src, rf)
	if nestedTagsInComment {
		r = reXMLComments.ReplaceAllStringFunc(r, func(m string) string {
			parts := reXMLComments.FindStringSubmatch(m)
			return parts[1] + html.UnescapeString(parts[2]) + parts[3]
		})
	}

	return r
}

// replaceTag returns a stateful function used for regex replacement that
// tracks indentation levels and tag types during the formatting process.
func replaceTag() func(string) string {
	indent := "  "
	indentLevel := 0
	lastEndElem := true
	return func(m string) string {
		if strings.HasPrefix(m, "<?xml") {
			return strings.Repeat(indent, indentLevel) + m
		}
		if strings.HasSuffix(m, "/>") {
			lastEndElem = true
			return NL + strings.Repeat(indent, indentLevel) + m
		}
		if strings.HasPrefix(m, "<!") {
			return NL + strings.Repeat(indent, indentLevel) + m
		}
		if strings.HasPrefix(m, "</") {
			indentLevel--
			if lastEndElem {
				return NL + strings.Repeat(indent, indentLevel) + m
			}
			lastEndElem = true
			return m
		} else {
			lastEndElem = false
		}
		defer func() {
			indentLevel++
		}()
		return NL + strings.Repeat(indent, indentLevel) + m
	}
}
