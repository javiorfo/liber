package files

import (
	"html"
	"regexp"
	"runtime"
	"strings"
)

var (
	reg           = regexp.MustCompile(`<([/!]?)([^>]+?)(/?)>`)
	reXMLComments = regexp.MustCompile(`(?s)(<!--)(.*?)(-->)`)
	reSpaces      = regexp.MustCompile(`(?s)>\s+<`)
	reNewlines    = regexp.MustCompile(`\r*\n`)
	NL            = "\n"
)

func init() {
	if runtime.GOOS == "windows" {
		NL = "\r\n"
	}
}

func FormatXML(xmlString string) string {
	nestedTagsInComment := false
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
