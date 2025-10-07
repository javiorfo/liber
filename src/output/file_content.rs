use crate::epub::{Content, ContentReference, Epub};

#[derive(Debug)]
pub struct FileContent<F, B> {
    pub filepath: F,
    pub bytes: B,
}

impl<F: ToString, B: AsRef<[u8]>> FileContent<F, B> {
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
    pub fn add<S: ToString>(&mut self, value: S) {
        self.0.push_str(&value.to_string());
    }

    pub fn add_optional<S: ToString>(&mut self, value: Option<S>) {
        if let Some(value) = value {
            self.0.push_str(&value.to_string());
        }
    }

    pub fn add_if_some<T, S: ToString>(&mut self, value: S, some: Option<T>) {
        if some.is_some() {
            self.0.push_str(&value.to_string());
        }
    }

    pub fn build(self) -> String {
        self.0
    }
}

pub fn content_opf(
    epub: &Epub<'_>,
    file_number: usize,
) -> crate::Result<FileContent<String, String>> {
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

    if let Some(ref cover_image) = epub.cover_image {
        let filename = cover_image.filename()?;
        let media_type = cover_image.media_type();
        content_builder.add(format!(
            r#"<item id="{filename}" href="{filename}" media-type="{media_type}"/>"#
        ));
    }

    if let Some(ref resources) = epub.resources {
        for resource in resources {
            let filename = resource.filename()?;
            let media_type = resource.media_type();
            content_builder.add(format!(
                r#"<item id="{filename}" href="{filename}" media-type="{media_type}"/>"#,
            ));
        }
    }

    if epub.contents.is_some() {
        for i in 1..=file_number {
            let filename = Content::filename(i);
            content_builder.add(format!(
                r#"<item id="{filename}" href="{filename}" media-type="application/xhtml+xml"/>"#,
            ));
        }
    }

    content_builder.add(r#"</manifest><spine toc="ncx">"#);

    if epub.contents.is_some() {
        for i in 1..=file_number {
            let filename = Content::filename(i);
            content_builder.add(format!(r#"<itemref idref="{filename}"/>"#));
        }
    }

    content_builder.add(r#"</spine><guide>"#);

    if let Some(ref contents) = epub.contents {
        let mut file_number = 1;
        for con in contents {
            let filename = Content::filename(file_number);
            file_number += 1;
            let (ref_type, title) = con.reference_type.type_and_title();
            content_builder.add(format!(
                r#"<reference type="{ref_type}" title="{title}" href="{filename}"/>"#,
            ));

            if let Some(ref subcontents) = con.subcontents {
                for subcon in subcontents {
                    let filename = Content::filename(file_number);
                    file_number += 1;
                    let (ref_type, title) = subcon.reference_type.type_and_title();
                    content_builder.add(format!(
                        r#"<reference type="{ref_type}" title="{title}" href="{filename}"/>"#,
                    ));
                }
            }
        }
    }

    content_builder.add(r#"</guide></package>"#);

    Ok(FileContent::new(
        "OEBPS/content.opf".to_string(),
        content_builder.build(),
    ))
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

        let nav_point = format!(
            r#"<navPoint id="navPoint-{current_play_order}" playOrder="{current_play_order}">
            <navLabel><text>{text}</text></navLabel>
            <content src="{file}"/>{subs}{content_references}</navPoint>"#,
            text = content.title(),
            file = Content::filename(current_play_order),
            subs = content
                .subcontents
                .as_ref()
                .and_then(|s| contents_to_nav_point(play_order, s))
                .unwrap_or_default(),
            content_references = content
                .content_references
                .as_ref()
                .and_then(|content_references| content_references_to_nav_point(
                    current_play_order,
                    play_order,
                    "",
                    content_references,
                    &mut 0
                ))
                .unwrap_or_default()
        );
        result.push_str(&nav_point);
    }

    Some(result)
}

fn content_references_to_nav_point(
    current_xhtml: usize,
    play_order: &mut usize,
    toc_index: &str,
    content_references: &[ContentReference<'_>],
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
            r#"<navPoint id="navPoint-{current_xhtml}{current_toc}" playOrder="{current_play_order}">
            <navLabel><text>{text}</text></navLabel>
            <content src="{xhtml}#id{current_link:02}"/>{subcontent_references}</navPoint>"#,
            text = content_reference.title,
            xhtml = Content::filename(current_xhtml),
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
