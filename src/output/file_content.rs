use crate::epub::{Content, ContentReference, Epub, ReferenceType};

#[derive(Debug, PartialEq, Eq)]
pub struct FileContent<F, B> {
    pub filepath: F,
    pub bytes: B,
}

impl<F, B> FileContent<F, B>
where
    F: Into<String>,
    B: AsRef<[u8]>,
{
    pub fn new(filepath: F, bytes: B) -> FileContent<F, B> {
        Self { filepath, bytes }
    }

    pub fn format(&mut self, bytes: B) {
        self.bytes = bytes;
    }
}

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

pub fn mimetype<'a>() -> FileContent<&'a str, &'a [u8]> {
    FileContent::new("mimetype", b"application/epub+zip")
}

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

#[derive(Debug)]
pub struct ContentBuilder(String);

impl ContentBuilder {
    pub fn add<S: Into<String>>(&mut self, value: S) {
        self.0.push_str(&value.into());
    }

    pub fn add_optional<S: Into<String>>(&mut self, value: Option<S>) {
        if let Some(value) = value {
            self.0.push_str(&value.into());
        }
    }

    pub fn add_if_some<T, S: Into<String>>(&mut self, value: S, some: Option<T>) {
        if some.is_some() {
            self.0.push_str(&value.into());
        }
    }

    pub fn build(self) -> String {
        self.0
    }
}

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

    recursive_content(
        &mut 0,
        &mut content_builder,
        epub.contents.as_deref(),
        &|cb, filename, _| {
            cb.add(format!(
                r#"<item id="{filename}" href="{filename}" media-type="application/xhtml+xml"/>"#
            ));
        },
    )?;

    content_builder.add(r#"</manifest><spine toc="ncx">"#);

    recursive_content(
        &mut 0,
        &mut content_builder,
        epub.contents.as_deref(),
        &|cb, filename, _| {
            cb.add(format!(r#"<itemref idref="{filename}"/>"#));
        },
    )?;

    content_builder.add(r#"</spine><guide>"#);

    recursive_content(
        &mut 0,
        &mut content_builder,
        epub.contents.as_deref(),
        &|cb, filename, reference_type| {
            let (ref_type, title) = reference_type.type_and_title();
            cb.add(format!(
                r#"<reference type="{ref_type}" title="{title}" href="{filename}"/>"#,
            ));
        },
    )?;

    content_builder.add(r#"</guide></package>"#);

    Ok(FileContent::new(
        "OEBPS/content.opf".to_string(),
        content_builder.build(),
    ))
}

fn recursive_content<F>(
    file_number: &mut usize,
    cb: &mut ContentBuilder,
    contents: Option<&[Content<'_>]>,
    f: &F,
) -> crate::Result
where
    F: Fn(&mut ContentBuilder, String, ReferenceType),
{
    if let Some(contents) = contents {
        for con in contents {
            *file_number += 1;
            let filename = con.filename(*file_number);
            if !filename.ends_with(".xhtml") {
                return Err(crate::Error::ContentFilename(filename));
            }
            f(cb, filename, con.reference_type.clone());

            recursive_content(file_number, cb, con.subcontents.as_deref(), f)?;
        }
    }
    Ok(())
}

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
            .add_content_reference(ContentReference::new("Ref A".to_string()))
            .add_content_reference(ContentReference::new("Ref B".to_string()))
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
            ContentReference::new("Level 1 Ref 1".to_string()).add_child(
                ContentReference::new("Level 2 Ref 1".to_string())
                    .add_child(ContentReference::new("Level 3 Ref 1".to_string())),
            ),
            ContentReference::new("Level 1 Ref 2".to_string()),
        ];

        let current_xhtml = (5, "c05.xhtml");
        let mut play_order = 10;
        let toc_index = "";
        let mut link_number = 0;

        let result = content_references_to_nav_point(
            current_xhtml,
            &mut play_order,
            toc_index,
            &content_references,
            &mut link_number,
        );

        assert!(result.is_some());
        let xml = cleaner(result.unwrap());

        assert!(xml.contains(r#"<navPoint id="navPoint-5-1" playOrder="11"><navLabel><text>Level 1 Ref 1</text></navLabel><content src="c05.xhtml#id01"/>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-5-1-1" playOrder="12"><navLabel><text>Level 2 Ref 1</text></navLabel><content src="c05.xhtml#id02"/>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-5-1-1-1" playOrder="13"><navLabel><text>Level 3 Ref 1</text></navLabel><content src="c05.xhtml#id03"/></navPoint>"#));
        assert!(xml.contains(r#"<navPoint id="navPoint-5-2" playOrder="14"><navLabel><text>Level 1 Ref 2</text></navLabel><content src="c05.xhtml#id04"/></navPoint>"#));
        assert_eq!(play_order, 14);
        assert_eq!(link_number, 4);
    }
}
