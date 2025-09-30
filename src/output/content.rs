use crate::epub::Epub;

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
    let mut content = vec![
        r#"<?xml version="1.0" encoding="utf-8"?>
<package version="2.0" unique-identifier="BookId" xmlns="http://www.idpf.org/2007/opf">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:opf="http://www.idpf.org/2007/opf">"#
            .to_string(),
    ];

    let metadata = &epub.metadata;

    content.push(format!("    <dc:title>{}</dc:title>", metadata.title));

    content.push(format!(
        "    <dc:language>{}</dc:language>",
        metadata.language.as_ref()
    ));

    content.push(format!(
        r#"    <dc:identifier id="BookId" opf:scheme="{}">{}</dc:identifier>"#,
        metadata.identifier,
        metadata.identifier.as_ref()
    ));

    if let Some(ref creator) = metadata.creator {
        content.push(format!(
            r#"    <dc:creator opf:role="aut">{}</dc:creator>"#,
            creator
        ));
    }

    if let Some(ref contributor) = metadata.contributor {
        content.push(format!(
            r#"    <dc:contributor opf:role="trl">{}</dc:contributor>"#,
            contributor
        ));
    }

    if let Some(ref publisher) = metadata.publisher {
        content.push(format!("    <dc:publisher>{}</dc:publisher>", publisher));
    }

    if metadata.date.is_some() {
        content.push(format!(
            r#"    <dc:date opf:event="publication">{}</dc:date>"#,
            metadata.format_date()
        ));
    }

    if let Some(ref subject) = metadata.subject {
        content.push(format!("    <dc:subject>{}</dc:subject>", subject));
    }

    if let Some(ref description) = metadata.description {
        content.push(format!(
            "    <dc:description>{}</dc:description>",
            description
        ));
    }

    if let Some(ref cover_image) = epub.cover_image {
        content.push(format!(
            r#"    <meta name="cover" content="{}"/>"#,
            cover_image.filename()?
        ));
    }

    content.push(
        r#"  </metadata> 
  <manifest>
    <item href="toc.ncx" id="ncx" media-type="application/x-dtbncx+xml" />"#
            .to_string(),
    );

    if epub.stylesheet.is_some() {
        content.push(
            r#"    <item id="style.css" href="style.css" media-type="text/css"/>"#.to_string(),
        );
    }

    if let Some(ref cover_image) = epub.cover_image {
        let filename = cover_image.filename()?;
        let media_type = cover_image.media_type();
        content.push(format!(
            r#"    <item id="{filename}" href="{filename}" media-type="{media_type}"/>"#
        ));
    }

    if let Some(ref resources) = epub.resources {
        for resource in resources {
            let filename = resource.filename()?;
            let media_type = resource.media_type();
            content.push(format!(
                r#"    <item id="{filename}" href="{filename}" media-type="{media_type}"/>"#,
            ));
        }
    }

    if epub.sections.is_some() {
        for i in 1..=file_number {
            let filename = format!("{:02}.xhtml", i);
            content.push(format!(
                r#"    <item id="{filename}" href="{filename}" media-type="application/xhtml+xml"/>"#,
            ));
        }
    }

    content.push(
        r#"  </manifest>
  <spine toc="ncx">"#
            .to_string(),
    );

    if epub.sections.is_some() {
        for i in 1..=file_number {
            let filename = format!("{:02}.xhtml", i);
            content.push(format!(r#"    <itemref idref="{filename}"/>"#));
        }
    }

    content.push(
        r#"  </spine>
  <guide>"#
            .to_string(),
    );

    if let Some(ref sections) = epub.sections {
        let mut file_number = 1;
        for section in sections {
            let filename = section.filename(file_number);
            file_number += 1;
            let (ref_type, title) = section.reference_type.type_and_title();
            content.push(format!(
                r#"    <reference type="{ref_type}" title="{title}" href="{filename}"/>"#,
            ));

            if let Some(ref sections) = section.subsections {
                for section in sections {
                    let filename = section.filename(file_number);
                    file_number += 1;
                    let (ref_type, title) = section.reference_type.type_and_title();
                    content.push(format!(
                        r#"    <reference type="{ref_type}" title="{title}" href="{filename}"/>"#,
                    ));
                }
            }
        }
    }

    content.push(
        r#"  </guide>
<package>"#
            .to_string(),
    );

    Ok(FileContent::new(
        "OEBPS/content.opf",
        content.join("\n").as_bytes().to_vec(),
    ))
}

pub fn toc_ncx<'a>(epub: &Epub<'a>) -> crate::Result<FileContent<&'a str, Vec<u8>>> {
    let mut content = vec![
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE ncx PUBLIC "-//NISO//DTD ncx 2005-1//EN"
   "http://www.daisy.org/z3986/2005/ncx-2005-1.dtd">
<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/" version="2005-1">
  <head>"#
            .to_string(),
    ];

    let metadata = &epub.metadata;

    content.push(format!(
        r#"    <meta name="dtb:uid" content="{}"/>"#,
        metadata.identifier.as_ref()
    ));

    content.push(format!(
        r#"    <meta name="dtb:depth" content="{}"/>"#,
        epub.level()
    ));

    content.push(format!(
        r#"    <meta content="0" name="dtb:totalPageCount"/>
    <meta content="0" name="dtb:maxPageNumber"/>
  </head>
  <docTitle>
    <text>{}</text>
  </docTitle>
  <navMap>"#,
        metadata.title
    ));

    Ok(FileContent::new(
        "OEBPS/toc.ncx",
        content.join("\n").as_bytes().to_vec(),
    ))
}
