use crate::epub::{Content, ContentReference, Epub, ReferenceType};

/// A generic struct representing a file within the EPUB archive.
///
/// It holds the **path** of the file and its **content bytes**. The type
/// parameters allow flexibility for the path (`F`) and the content (`B`).
#[derive(Debug, PartialEq, Eq)]
pub struct FileContent<F, B> {
    /// The path of the file, e.g., "OEBPS/content.opf".
    pub filepath: F,
    /// The binary or text content of the file.
    pub bytes: B,
}

impl<F, B> FileContent<F, B>
where
    F: Into<String>,
    B: AsRef<[u8]>,
{
    /// Creates a new `FileContent` instance.
    ///
    /// # Arguments
    ///
    /// * `filepath`: The path of the file. Must be convertible to `String`.
    /// * `bytes`: The content of the file. Must be convertible to a byte slice.
    pub fn new(filepath: F, bytes: B) -> FileContent<F, B> {
        Self { filepath, bytes }
    }

    /// Replaces the current content bytes with new ones.
    ///
    /// # Arguments
    ///
    /// * `bytes`: The new content bytes.
    pub fn format(&mut self, bytes: B) {
        self.bytes = bytes;
    }
}

/// Creates a `FileContent` for the mandatory EPUB **container.xml** file.
///
/// This file specifies the location of the OPF package document.
pub fn container<'a>() -> FileContent<&'a str, &'a [u8]> {
    FileContent::new(
        "META-INF/container.xml",
        r#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
    <rootfiles>
        <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
   </rootfiles>
</container>
        "#
        .as_bytes(),
    )
}

/// Creates a `FileContent` for the mandatory EPUB **mimetype** file.
///
/// This file *must* be the first file in the EPUB ZIP archive and must not be compressed.
pub fn mimetype<'a>() -> FileContent<&'a str, &'a [u8]> {
    FileContent::new("mimetype", b"application/epub+zip")
}

/// Creates a `FileContent` for the **com.apple.ibooks.display-options.xml** file.
///
/// This is a non-mandatory file used by iBooks to specify display options,
/// in this case, enabling specified fonts.
pub fn display_options<'a>() -> FileContent<&'a str, &'a [u8]> {
    FileContent::new(
        "META-INF/com.apple.ibooks.display-options.xml",
        r#"<?xml version="1.0" encoding="utf-8"?>
<display_options>
	<platform name="*">
		<option name="specified-fonts">true</option>
	</platform>
</display_options>
        "#
        .as_bytes(),
    )
}

/// A helper struct for efficiently building the content of XML files as a `String`.
///
/// It wraps a single `String` and provides methods for appending various values,
/// including conditional and optional strings, which is useful for generating XML dynamically.
#[derive(Debug)]
pub struct ContentBuilder(String);

impl ContentBuilder {
    /// Appends a string-like value to the builder's content.
    ///
    /// # Type Parameters
    ///
    /// * `S`: Any type that can be converted into a `String` (e.g., `&str` or `String`).
    pub fn add<S: Into<String>>(&mut self, value: S) {
        self.0.push_str(&value.into());
    }

    /// Appends an optional string-like value to the builder's content if it is `Some`.
    ///
    /// If `value` is `None`, nothing is appended.
    ///
    /// # Type Parameters
    ///
    /// * `S`: Any type that can be converted into a `String`.
    pub fn add_optional<S: Into<String>>(&mut self, value: Option<S>) {
        if let Some(value) = value {
            self.0.push_str(&value.into());
        }
    }

    /// Appends a specific string-like value only if the condition-providing `Option` is `Some`.
    ///
    /// This is useful for including fixed XML tags only when a related field exists.
    ///
    /// # Type Parameters
    ///
    /// * `S`: Any type that can be converted into a `String`.
    /// * `T`: The inner type of the condition `Option`.
    pub fn add_if_some<T, S: Into<String>>(&mut self, value: S, some: Option<T>) {
        if some.is_some() {
            self.0.push_str(&value.into());
        }
    }

    /// Consumes the builder and returns the assembled content as a `String`.
    pub fn build(self) -> String {
        self.0
    }
}

/// Generates the **content.opf** (Open Packaging Format) file for the EPUB.
///
/// This file is the spine of the EPUB, containing the full manifest of all
/// files, the linear reading order (`spine`), and the essential metadata.
///
/// # Arguments
///
/// * `epub`: A reference to the main `Epub` structure containing all book data.
///
/// # Returns
///
/// Returns a `crate::Result` wrapping a `FileContent<String, String>` for
/// "OEBPS/content.opf" with the generated XML content.
pub fn content_opf(epub: &Epub<'_>) -> crate::Result<FileContent<String, String>> {
    let metadata = &epub.metadata;

    let mut content_builder = ContentBuilder(String::from(
        r#"<?xml version="1.0" encoding="utf-8"?><package version="2.0" unique-identifier="BookId" xmlns="http://www.idpf.org/2007/opf">
        <metadata xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:opf="http://www.idpf.org/2007/opf">"#,
    ));

    content_builder.add(metadata.title_as_metadata_xml());
    content_builder.add(metadata.language.as_metadata_xml());
    content_builder.add(metadata.identifier.as_metadata_xml());
    content_builder.add_optional(metadata.creator_as_metadata_xml());
    content_builder.add_optional(metadata.contributor_as_metadata_xml());
    content_builder.add_optional(metadata.publisher_as_metadata_xml());
    content_builder.add_optional(metadata.date_as_metadata_xml());
    content_builder.add_optional(metadata.subject_as_metadata_xml());
    content_builder.add_optional(metadata.description_as_metadata_xml());
    content_builder.add_optional(epub.cover_image_as_metadata_xml());
    content_builder.add(
        r#"</metadata><manifest><item id="ncx" href="toc.ncx" media-type="application/x-dtbncx+xml" />"#,
    );

    content_builder.add_if_some(
        r#"<item id="style.css" href="style.css" media-type="text/css"/>"#,
        epub.stylesheet.as_ref(),
    );

    content_builder.add_optional(epub.cover_image_as_manifest_xml());

    if let Some(ref resources) = epub.resources {
        for resource in resources {
            content_builder.add_optional(resource.as_manifest_xml());
        }
    }

    create_content_chain(
        &mut 0,
        &mut content_builder,
        epub.contents.as_deref(),
        &|filename, _| {
            format!(
                r#"<item id="{filename}" href="{filename}" media-type="application/xhtml+xml"/>"#
            )
        },
    )?;

    content_builder.add(r#"</manifest><spine toc="ncx">"#);

    create_content_chain(
        &mut 0,
        &mut content_builder,
        epub.contents.as_deref(),
        &|filename, _| format!(r#"<itemref idref="{filename}"/>"#),
    )?;

    content_builder.add(r#"</spine><guide>"#);

    create_content_chain(
        &mut 0,
        &mut content_builder,
        epub.contents.as_deref(),
        &|filename, reference_type| {
            let (ref_type, title) = reference_type.type_and_title();
            format!(r#"<reference type="{ref_type}" title="{title}" href="{filename}"/>"#,)
        },
    )?;

    content_builder.add(r#"</guide></package>"#);

    Ok(FileContent::new(
        "OEBPS/content.opf".to_string(),
        content_builder.build(),
    ))
}

/// A recursive private helper function used by `content_opf` to traverse the
/// hierarchical content structure (`epub.contents`) and generate repeated XML
/// elements (manifest items, spine references, or guide references).
///
/// # Arguments
///
/// * `file_number`: A mutable counter to assign unique filenames/IDs to content documents.
/// * `cb`: A mutable reference to the `ContentBuilder` to append the generated XML.
/// * `contents`: An `Option` containing a slice of the current level of `Content` to process.
/// * `f`: A closure that takes the generated filename and its `ReferenceType` and
///   returns the specific XML element string to be added (e.g., a `<item>` tag).
///
/// # Returns
///
/// Returns `crate::Result<()>`, signaling an error if a content filename is invalid
/// (not ending with `.xhtml`).
fn create_content_chain<F>(
    file_number: &mut usize,
    cb: &mut ContentBuilder,
    contents: Option<&[Content<'_>]>,
    f: &F,
) -> crate::Result
where
    F: Fn(String, &ReferenceType) -> String,
{
    if let Some(contents) = contents {
        for con in contents {
            *file_number += 1;
            let filename = con.filename(*file_number);
            if !filename.ends_with(".xhtml") {
                return Err(crate::Error::ContentFilename(filename));
            }

            cb.add(f(filename, &con.reference_type));

            create_content_chain(file_number, cb, con.subcontents.as_deref(), f)?;
        }
    }
    Ok(())
}

/// Generates the **toc.ncx** (Navigation Control File for XML) file for the EPUB.
///
/// This file defines the EPUB's table of contents, including the hierarchical
/// structure of the book's sections and subsections (`navMap`).
///
/// # Arguments
///
/// * `epub`: A reference to the main `Epub` structure.
///
/// # Returns
///
/// Returns a `crate::Result` wrapping a `FileContent<String, String>` for
/// "OEBPS/toc.ncx" with the generated XML content.
pub fn toc_ncx(epub: &Epub<'_>) -> crate::Result<FileContent<String, String>> {
    let metadata = &epub.metadata;

    let mut content_builder = ContentBuilder(String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE ncx PUBLIC "-//NISO//DTD ncx 2005-1//EN" "http://www.daisy.org/z3986/2005/ncx-2005-1.dtd">
        <ncx xmlns="http://www.daisy.org/z3986/2005/ncx/" version="2005-1"><head>"#,
    ));

    content_builder.add(metadata.identifier.as_toc_xml());
    content_builder.add(epub.level_as_toc_xml());

    content_builder.add(format!(r#"<meta name="dtb:totalPageCount" content="0"/><meta name="dtb:maxPageNumber" content="0"/></head>
                        <docTitle><text>{}</text></docTitle><navMap>"#, metadata.title));

    content_builder.add_optional(
        epub.contents
            .as_ref()
            .and_then(|contents| contents_to_nav_point(&mut 0, contents)),
    );

    content_builder.add(r#"</navMap></ncx>"#);

    Ok(FileContent::new(
        "OEBPS/toc.ncx".to_string(),
        content_builder.build(),
    ))
}

/// A recursive private helper function to generate the `navPoint` elements for the `toc.ncx` file.
///
/// It traverses the hierarchical content structure and creates the corresponding
/// nested `<navPoint>` XML tags for the table of contents.
///
/// # Arguments
///
/// * `play_order`: A mutable counter used to generate the unique sequential `playOrder` attribute.
/// * `contents`: A slice of `Content` items at the current hierarchy level.
///
/// # Returns
///
/// Returns an `Option<String>`: `Some(String)` containing the generated XML for the
/// navigation points, or `None` if the input slice is empty.
fn contents_to_nav_point(play_order: &mut usize, contents: &[Content<'_>]) -> Option<String> {
    let mut result = String::new();
    for content in contents {
        *play_order += 1;
        let current_play_order = *play_order;
        let filename = &content.filename(current_play_order);

        let nav_point = format!(
            r#"<navPoint id="navPoint-{current_play_order}" playOrder="{current_play_order}">
            <navLabel><text>{text}</text></navLabel>
            <content src="{filename}"/>{content_references}{subs}</navPoint>"#,
            text = content.title(),
            content_references = content
                .content_references
                .as_ref()
                .and_then(|content_references| content_references_to_nav_point(
                    (current_play_order, filename),
                    play_order,
                    "",
                    content_references,
                    &mut 0
                ))
                .unwrap_or_default(),
            subs = content
                .subcontents
                .as_ref()
                .and_then(|s| contents_to_nav_point(play_order, s))
                .unwrap_or_default(),
        );
        result.push_str(&nav_point);
    }

    Some(result)
}

/// A recursive private helper function to generate nested `navPoint` elements
/// for **content references** (i.e., internal links/subheadings within a single XHTML file).
///
/// This function is called from `contents_to_nav_point` to handle the deeper
/// hierarchy of links within a specific file.
///
/// # Arguments
///
/// * `current_xhtml`: A tuple containing the unique index and filename of the current XHTML file.
/// * `play_order`: A mutable counter to continue the sequential `playOrder` across all entries.
/// * `toc_index`: A string representing the current hierarchical index path (e.g., "1-2-").
/// * `content_references`: A slice of `ContentReference` items to process.
/// * `link_number`: A mutable counter to generate unique link IDs/names within the file.
///
/// # Returns
///
/// Returns an `Option<String>`: `Some(String)` containing the generated XML for the
/// reference navigation points, or `None` if the input slice is empty.
fn content_references_to_nav_point(
    current_xhtml: (usize, &str),
    play_order: &mut usize,
    toc_index: &str,
    content_references: &[ContentReference],
    link_number: &mut usize,
) -> Option<String> {
    let mut result = String::new();

    let (prefix, mut toc_number) = toc_index
        .rsplit_once('-')
        .map(|(prefix, number)| (prefix, number.parse::<usize>().unwrap_or(0)))
        .unwrap_or(("", 0));

    for content_reference in content_references {
        *link_number += 1;
        let current_link = *link_number;

        toc_number += 1;
        let current_toc = format!("{prefix}-{toc_number}");

        *play_order += 1;
        let current_play_order = *play_order;

        let nav_point = format!(
            r#"<navPoint id="navPoint-{xhtml_number}{current_toc}" playOrder="{current_play_order}">
            <navLabel><text>{text}</text></navLabel>
            <content src="{src}"/>{subcontent_references}</navPoint>"#,
            xhtml_number = current_xhtml.0,
            text = content_reference.title,
            src = content_reference.reference_name(current_xhtml.1, current_link),
            subcontent_references = content_reference
                .subcontent_references
                .as_ref()
                .and_then(|subcontent_references| content_references_to_nav_point(
                    current_xhtml,
                    play_order,
                    &format!("{current_toc}-"),
                    subcontent_references,
                    link_number,
                ))
                .unwrap_or_default()
        );
        result.push_str(&nav_point);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::epub::{
        ContentBuilder, ContentReference, EpubBuilder, Identifier, MetadataBuilder, ReferenceType,
    };

    use super::{content_references_to_nav_point, contents_to_nav_point, toc_ncx};

    fn cleaner(xml: String) -> String {
        xml.replace("\n", "").replace(" ".repeat(12).as_str(), "")
    }

    #[test]
    fn test_toc_ncx_simple_content() {
        let mock_epub = EpubBuilder::new(
            MetadataBuilder::title("Title")
                .identifier(Identifier::UUID("mock-epub-id".to_string()))
                .build(),
        )
        .add_content(
            ContentBuilder::new(
                "<body><h1>Chapter I</h1></body>".as_bytes(),
                ReferenceType::Text("Chapter I".to_string()),
            )
            .build(),
        )
        .add_content(
            ContentBuilder::new(
                "<body><h1>Chapter II</h1></body>".as_bytes(),
                ReferenceType::Text("Chapter II".to_string()),
            )
            .build(),
        );

        let result = toc_ncx(&mock_epub.0);

        assert!(result.is_ok());
        let file_content = result.unwrap();

        assert_eq!(file_content.filepath, "OEBPS/toc.ncx");

        let content = cleaner(file_content.bytes);
        assert!(content.contains(r#"<meta name="dtb:uid" content="urn:uuid:mock-epub-id"/>"#));
        assert!(content.contains(r#"<meta name="dtb:depth" content="1"/>"#));
        assert!(content.contains(r#"<docTitle><text>Title</text></docTitle>"#));
        assert!(content.contains(r#"<navPoint id="navPoint-1" playOrder="1"><navLabel><text>Chapter I</text></navLabel><content src="c01.xhtml"/></navPoint>"#));
        assert!(content.contains(r#"<navPoint id="navPoint-2" playOrder="2"><navLabel><text>Chapter II</text></navLabel><content src="c02.xhtml"/></navPoint>"#));
        assert!(content.ends_with(r#"</navMap></ncx>"#));
    }

    #[test]
    fn test_toc_ncx_no_content() {
        let mock_epub = EpubBuilder::new(MetadataBuilder::title("Empty Book").build());
        let result = toc_ncx(&mock_epub.0);

        assert!(result.is_ok());
        let file_content = result.unwrap();

        let content = file_content.bytes;
        assert!(content.contains(r#"<docTitle><text>Empty Book</text></docTitle><navMap>"#));
        assert!(content.contains(r#"<meta name="dtb:totalPageCount" content="0"/>"#));
        assert!(
            content.ends_with(
                r#"<docTitle><text>Empty Book</text></docTitle><navMap></navMap></ncx>"#
            )
        );
    }

    #[test]
    fn test_contents_to_nav_point_nested() {
        let mock_epub = EpubBuilder::new(MetadataBuilder::title("Title").build())
            .add_content(
                ContentBuilder::new(
                    "<body><h1>Main Chapter</h1></body>".as_bytes(),
                    ReferenceType::Text("Main Chapter".to_string()),
                )
                .add_children(vec![
                    ContentBuilder::new(
                        "<body><h1>Section 1.1</h1></body>".as_bytes(),
                        ReferenceType::Text("Section 1.1".to_string()),
                    )
                    .build(),
                    ContentBuilder::new(
                        "<body><h1>Section 1.2</h1></body>".as_bytes(),
                        ReferenceType::Text("Section 1.2".to_string()),
                    )
                    .build(),
                ])
                .build(),
            )
            .add_content(
                ContentBuilder::new(
                    "<body><h1>Next Chapter</h1></body>".as_bytes(),
                    ReferenceType::Text("Next Chapter".to_string()),
                )
                .build(),
            );

        let mut play_order = 0;

        let result = contents_to_nav_point(&mut play_order, &mock_epub.0.contents.unwrap());

        assert!(result.is_some());
        let xml = cleaner(result.unwrap());

        assert!(xml.contains(r#"<navPoint id="navPoint-1" playOrder="1"><navLabel><text>Main Chapter</text></navLabel><content src="c01.xhtml"/>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-2" playOrder="2"><navLabel><text>Section 1.1</text></navLabel><content src="c02.xhtml"/></navPoint>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-3" playOrder="3"><navLabel><text>Section 1.2</text></navLabel><content src="c03.xhtml"/></navPoint>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-4" playOrder="4"><navLabel><text>Next Chapter</text></navLabel><content src="c04.xhtml"/></navPoint>"#));

        assert_eq!(play_order, 4);
    }

    #[test]
    fn test_contents_to_nav_point_with_references() {
        let mock_epub = EpubBuilder::new(MetadataBuilder::title("With Refs").build()).add_content(
            ContentBuilder::new(
                "<body><h1>Chapter with Refs</h1></body>".as_bytes(),
                ReferenceType::Text("Chapter with Refs".to_string()),
            )
            .add_content_references(vec![
                ContentReference::new("Ref A"),
                ContentReference::new("Ref B"),
            ])
            .build(),
        );

        let mut play_order = 0;

        let result = contents_to_nav_point(&mut play_order, &mock_epub.0.contents.unwrap());
        assert!(result.is_some());
        let xml = cleaner(result.unwrap());

        assert!(xml.contains(r#"<navPoint id="navPoint-1" playOrder="1"><navLabel><text>Chapter with Refs</text></navLabel><content src="c01.xhtml"/>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-1-1" playOrder="2"><navLabel><text>Ref A</text></navLabel><content src="c01.xhtml#id01"/></navPoint>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-1-2" playOrder="3"><navLabel><text>Ref B</text></navLabel><content src="c01.xhtml#id02"/></navPoint>"#));
        assert_eq!(play_order, 3);
    }

    #[test]
    fn test_content_references_to_nav_point_nested() {
        let content_references = vec![
            ContentReference::new("Level 1 Ref 1").add_child(
                ContentReference::new("Level 2 Ref 1")
                    .add_child(ContentReference::new("Level 3 Ref 1")),
            ),
            ContentReference::new("Level 1 Ref 2").id("four"),
        ];

        let mut play_order = 10;
        let mut link_number = 0;

        let result = content_references_to_nav_point(
            (5, "some.xhtml"),
            &mut play_order,
            "",
            &content_references,
            &mut link_number,
        );

        assert!(result.is_some());
        let xml = cleaner(result.unwrap());

        assert!(xml.contains(r#"<navPoint id="navPoint-5-1" playOrder="11"><navLabel><text>Level 1 Ref 1</text></navLabel><content src="some.xhtml#id01"/>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-5-1-1" playOrder="12"><navLabel><text>Level 2 Ref 1</text></navLabel><content src="some.xhtml#id02"/>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-5-1-1-1" playOrder="13"><navLabel><text>Level 3 Ref 1</text></navLabel><content src="some.xhtml#id03"/></navPoint>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-5-2" playOrder="14"><navLabel><text>Level 1 Ref 2</text></navLabel><content src="some.xhtml#four"/></navPoint>"#));
        assert_eq!(play_order, 14);
        assert_eq!(link_number, 4);
    }
}
