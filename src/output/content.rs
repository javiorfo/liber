use crate::epub::Epub;

#[derive(Debug)]
pub struct FileContent<F> {
    pub filepath: F,
    pub bytes: Vec<u8>,
}

impl<F: ToString> FileContent<F> {
    pub fn new(filepath: F, bytes: Vec<u8>) -> FileContent<F> {
        Self { filepath, bytes }
    }
}

pub fn container<'a>() -> FileContent<&'a str> {
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
        .as_bytes()
        .to_vec(),
    )
}

pub fn mimetype<'a>() -> FileContent<&'a str> {
    FileContent::new("mimetype", b"application/epub+zip".to_vec())
}

pub fn display_options<'a>() -> FileContent<&'a str> {
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
        .as_bytes()
        .to_vec(),
    )
}

pub fn content_opf<'a, S: AsRef<str>>(epub: &Epub<'a, S>) -> crate::Result<FileContent<&'a str>> {
    let mut content = vec![r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string(),
                        r#"<package version="2.0" unique-identifier="BookId" xmlns="http://www.idpf.org/2007/opf">"#.to_string(),
                        r#"  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:opf="http://www.idpf.org/2007/opf">"#.to_string()];

    let metadata = &epub.metadata;

    content.push(format!(
        "    <dc:title>{}</dc:title>",
        metadata.title.as_ref()
    ));

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
            creator.as_ref()
        ));
    }

    if let Some(ref contributor) = metadata.contributor {
        content.push(format!(
            r#"    <dc:contributor opf:role="trl">{}</dc:contributor>"#,
            contributor.as_ref()
        ));
    }

    if let Some(ref publisher) = metadata.publisher {
        content.push(format!(
            "    <dc:publisher>{}</dc:publisher>",
            publisher.as_ref()
        ));
    }

    if metadata.date.is_some() {
        content.push(format!(
            r#"    <dc:date opf:event="publication">{}</dc:date>"#,
            metadata.format_date()
        ));
    }

    if let Some(ref subject) = metadata.subject {
        content.push(format!("    <dc:subject>{}</dc:subject>", subject.as_ref()));
    }

    if let Some(ref description) = metadata.description {
        content.push(format!(
            "    <dc:description>{}</dc:description>",
            description.as_ref()
        ));
    }

    if let Some(ref cover_image) = epub.cover_image {
        content.push(format!(
            r#"    <meta name="cover" content="{}"/>"#,
            cover_image.filename()?
        ));
    }

    content.push("  </metadata>".to_string());
    content.push("  <manifest>".to_string());

    if epub.stylesheet.is_some() {
        content.push(
            r#"    <item id="style.css" href="style.css" media-type="text/css"/>"#.to_string(),
        );
    }

    if let Some(ref cover_image) = epub.cover_image {
        let filename = cover_image.filename()?;
        content.push(format!(
            r#"    <item id="{}" href="{}" media-type="{}"/>"#,
            filename,
            filename,
            cover_image.media_type()
        ));
    }

    Ok(FileContent::new(
        "OEBPS/content.opf",
        content.join("\n").as_bytes().to_vec(),
    ))
}
