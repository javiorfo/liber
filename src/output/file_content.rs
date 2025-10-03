use crate::epub::{Content, ContentReference, Epub};
use crate::output::xml;

#[derive(Debug)]
pub struct FileContent<F, B> {
    pub filepath: F,
    pub bytes: B,
}

impl<F: ToString, B: AsRef<[u8]>> FileContent<F, B> {
    pub fn new(filepath: F, bytes: B) -> FileContent<F, B> {
        Self { filepath, bytes }
    }
}

pub fn container<'a>() -> FileContent<&'a str, &'a [u8]> {
    FileContent::new(
        "META-INF/container.xml",
        r#"
<?xml version="1.0" encoding="UTF-8"?>
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
        r#"
<?xml version="1.0" encoding="utf-8"?>
<display_options>
	<platform name="*">
		<option name="specified-fonts">true</option>
	</platform>
</display_options>
        "#
        .as_bytes(),
    )
}

pub fn content_opf<'a>(
    epub: &Epub<'a>,
    file_number: usize,
) -> crate::Result<FileContent<&'a str, Vec<u8>>> {
    let metadata = &epub.metadata;

    let mut content = vec![
        r#"<?xml version="1.0" encoding="utf-8"?><package version="2.0" unique-identifier="BookId" xmlns="http://www.idpf.org/2007/opf">
        <metadata xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:opf="http://www.idpf.org/2007/opf">"#.to_string(),
        metadata.title_as_metadata_xml(),
        metadata.language.as_metadata_xml(),
        metadata.identifier.as_metadata_xml()
    ];

    if let Some(creator) = metadata.creator_as_metadata_xml() {
        content.push(creator);
    }

    if let Some(contributor) = metadata.contributor_as_metadata_xml() {
        content.push(contributor);
    }

    if let Some(ref publisher) = metadata.publisher {
        content.push(format!("<dc:publisher>{}</dc:publisher>", publisher));
    }

    if metadata.date.is_some() {
        content.push(format!(
            r#"<dc:date opf:event="publication">{}</dc:date>"#,
            metadata.format_date()
        ));
    }

    if let Some(ref subject) = metadata.subject {
        content.push(format!("<dc:subject>{}</dc:subject>", subject));
    }

    if let Some(ref description) = metadata.description {
        content.push(format!("<dc:description>{}</dc:description>", description));
    }

    if let Some(ref cover_image) = epub.cover_image {
        content.push(format!(
            r#"<meta name="cover" content="{}"/>"#,
            cover_image.filename()?
        ));
    }

    content.push(
        r#"</metadata><manifest><item href="toc.ncx" id="ncx" media-type="application/x-dtbncx+xml" />"#.to_string(),
    );

    if epub.stylesheet.is_some() {
        content
            .push(r#"<item id="style.css" href="style.css" media-type="text/css"/>"#.to_string());
    }

    if let Some(ref cover_image) = epub.cover_image {
        let filename = cover_image.filename()?;
        let media_type = cover_image.media_type();
        content.push(format!(
            r#"<item id="{filename}" href="{filename}" media-type="{media_type}"/>"#
        ));
    }

    if let Some(ref resources) = epub.resources {
        for resource in resources {
            let filename = resource.filename()?;
            let media_type = resource.media_type();
            content.push(format!(
                r#"<item id="{filename}" href="{filename}" media-type="{media_type}"/>"#,
            ));
        }
    }

    if epub.contents.is_some() {
        for i in 1..=file_number {
            let filename = Content::filename(i);
            content.push(format!(
                r#"<item id="{filename}" href="{filename}" media-type="application/xhtml+xml"/>"#,
            ));
        }
    }

    content.push(r#"</manifest><spine toc="ncx">"#.to_string());

    if epub.contents.is_some() {
        for i in 1..=file_number {
            let filename = Content::filename(i);
            content.push(format!(r#"<itemref idref="{filename}"/>"#));
        }
    }

    content.push(r#"</spine><guide>"#.to_string());

    if let Some(ref contents) = epub.contents {
        let mut file_number = 1;
        for con in contents {
            let filename = Content::filename(file_number);
            file_number += 1;
            let (ref_type, title) = con.reference_type.type_and_title();
            content.push(format!(
                r#"<reference type="{ref_type}" title="{title}" href="{filename}"/>"#,
            ));

            if let Some(ref subcontents) = con.subcontents {
                for subcon in subcontents {
                    let filename = Content::filename(file_number);
                    file_number += 1;
                    let (ref_type, title) = subcon.reference_type.type_and_title();
                    content.push(format!(
                        r#"<reference type="{ref_type}" title="{title}" href="{filename}"/>"#,
                    ));
                }
            }
        }
    }

    content.push(r#"</guide></package>"#.to_string());

    Ok(FileContent::new(
        "OEBPS/content.opf",
        xml::format(&content.join("\n"))?.as_bytes().to_vec(),
    ))
}

pub fn toc_ncx<'a>(epub: &Epub<'a>) -> crate::Result<FileContent<&'a str, Vec<u8>>> {
    let metadata = &epub.metadata;

    let mut content = vec![
        r#"<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE ncx PUBLIC "-//NISO//DTD ncx 2005-1//EN" "http://www.daisy.org/z3986/2005/ncx-2005-1.dtd">
        <ncx xmlns="http://www.daisy.org/z3986/2005/ncx/" version="2005-1"><head>"#.to_string(),
        metadata.identifier.as_toc_xml(),
        epub.level_as_toc_xml(),
    ];

    content.push(format!(r#"<meta name="dtb:totalPageCount" content="0"/><meta name="dtb:maxPageNumber" content="0"/></head>
                        <docTitle><text>{}</text></docTitle><navMap>"#, metadata.title));

    if let Some(ref contents) = epub.contents {
        content.push(process_contents(&mut 0, contents).unwrap());
    }

    content.push(r#"</navMap></ncx>"#.to_string());

    Ok(FileContent::new(
        "OEBPS/toc.ncx",
        xml::format(&content.join("\n"))?.as_bytes().to_vec(),
    ))
}

fn process_contents(play_order: &mut usize, contents: &[Content<'_>]) -> Option<String> {
    let mut result = String::new();
    for content in contents {
        *play_order += 1;
        let current_play_order = *play_order;

        let nav_point = format!(
            r#"<navPoint id="navPoint-{current_play_order}" playOrder="{current_play_order}"><navLabel><text>{text}</text></navLabel><content src="{file}"/>{subs}{content_references}</navPoint>"#,
            text = content.title(),
            file = Content::filename(current_play_order),
            subs = content
                .subcontents
                .as_ref()
                .and_then(|s| process_contents(play_order, s))
                .unwrap_or_default(),
            content_references = content
                .content_references
                .as_ref()
                .and_then(|content_references| process_content_references(
                    current_play_order,
                    play_order,
                    "",
                    content_references
                ))
                .unwrap_or_default()
        );
        result.push_str(&nav_point);
    }

    Some(result)
}

fn process_content_references(
    current_xhtml: usize,
    play_order: &mut usize,
    toc_index: &str,
    content_references: &[ContentReference<'_>],
) -> Option<String> {
    let mut result = String::new();

    let (prefix, mut toc_number) = toc_index
        .rsplit_once('-')
        .map(|(prefix, number)| (prefix, number.parse::<usize>().unwrap_or(0)))
        .unwrap_or(("", 0));

    for content_reference in content_references {
        toc_number += 1;
        let current_toc = format!("{prefix}-{toc_number}");
        *play_order += 1;
        let current_play_order = *play_order;

        let nav_point = format!(
            r#"<navPoint id="navPoint-{current_xhtml}{current_toc}" playOrder="{current_play_order}"><navLabel><text>{text}</text></navLabel><content src="{xhtml}#{current_xhtml}{current_toc}"/>{subcontent_references}</navPoint>"#,
            text = content_reference.title,
            xhtml = Content::filename(current_xhtml),
            subcontent_references = content_reference
                .subcontent_references
                .as_ref()
                .and_then(|subcontent_references| process_content_references(
                    current_xhtml,
                    play_order,
                    &format!("{current_toc}-"),
                    subcontent_references
                ))
                .unwrap_or_default()
        );
        result.push_str(&nav_point);
    }

    Some(result)
}
