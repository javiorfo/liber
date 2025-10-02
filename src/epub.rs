use std::{fmt::Display, fs, io::Write, path::Path};

use crate::{
    metadata::Metadata,
    output::{content::FileContent, creator::EpubFile},
};

#[derive(Debug)]
pub struct Epub<'a> {
    pub metadata: Metadata<'a>,
    pub stylesheet: Option<Stylesheet<'a>>,
    pub cover_image: Option<Resource<'a>>,
    pub resources: Option<Vec<Resource<'a>>>,
    pub sections: Option<Vec<Section<'a>>>,
}

impl<'a> Epub<'a> {
    fn new(metadata: Metadata<'a>) -> Epub<'a> {
        Self {
            metadata,
            stylesheet: None,
            cover_image: None,
            resources: None,
            sections: None,
        }
    }

    pub fn level(&self) -> usize {
        if let Some(ref sections) = self.sections {
            let level_subsections = sections
                .iter()
                .map(|section| section.level_subsections() + 1)
                .max()
                .unwrap_or(1);

            let level_tocs = sections
                .iter()
                .map(|section| section.level_tocs() + 1)
                .max()
                .unwrap_or(1);

            level_subsections.max(level_tocs)
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub struct EpubBuilder<'a>(Epub<'a>);

impl<'a> EpubBuilder<'a> {
    #[must_use]
    pub fn new(metadata: Metadata<'a>) -> Self {
        Self(Epub::new(metadata))
    }

    pub fn stylesheet(mut self, stylesheet: Stylesheet<'a>) -> Self {
        self.0.stylesheet = Some(stylesheet);
        self
    }

    pub fn cover_image(mut self, path: &'a Path, image_type: ImageType) -> Self {
        self.0.cover_image = Some(Resource::Image(path, image_type));
        self
    }

    pub fn add_resource(mut self, resource: Resource<'a>) -> Self {
        if let Some(ref mut resources) = self.0.resources {
            resources.push(resource);
        } else {
            self.0.resources = Some(vec![resource]);
        }
        self
    }

    pub fn resources(mut self, resources: Vec<Resource<'a>>) -> Self {
        self.0.resources = Some(resources);
        self
    }

    pub fn add_section(mut self, section: Section<'a>) -> Self {
        if let Some(ref mut sections) = self.0.sections {
            sections.push(section);
        } else {
            self.0.sections = Some(vec![section]);
        }
        self
    }

    pub fn sections(mut self, sections: Vec<Section<'a>>) -> Self {
        self.0.sections = Some(sections);
        self
    }

    pub fn create<W: Write>(self, writer: &mut W) -> crate::Result {
        EpubFile::new(self.0, writer).create()
    }
}

#[derive(Debug)]
pub struct Stylesheet<'a> {
    pub content: &'a [u8],
}

impl<'a> Stylesheet<'a> {
    pub fn new(content: &'a [u8]) -> Stylesheet<'a> {
        Self { content }
    }

    pub fn content(&self) -> FileContent<&'a str, &'a [u8]> {
        FileContent::new("OEBPS/style.css", self.content)
    }
}

#[derive(Debug, Clone)]
pub enum ImageType {
    Jpg,
    Png,
    Gif,
    Svg,
}

impl From<&ImageType> for &str {
    fn from(value: &ImageType) -> Self {
        match value {
            ImageType::Jpg => "image/jpeg",
            ImageType::Png => "image/png",
            ImageType::Gif => "image/gif",
            ImageType::Svg => "image/svg+xml",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Resource<'a> {
    Image(&'a Path, ImageType),
    Font(&'a Path),
    Audio(&'a Path),
    Video(&'a Path),
}

impl<'a> Resource<'a> {
    pub fn media_type(&self) -> &str {
        match self {
            Resource::Image(_, img_type) => img_type.into(),
            Resource::Font(_) => "application/vnd.ms-opentype",
            Resource::Audio(_) => "audio/mpeg",
            Resource::Video(_) => "video/mp4",
        }
    }

    pub fn content(&self) -> crate::Result<FileContent<String, Vec<u8>>> {
        match self {
            Self::Image(path, _) | Self::Font(path) | Self::Audio(path) | Self::Video(path) => Ok(
                FileContent::new(format!("OEBPS/{}", self.filename()?), fs::read(path)?),
            ),
        }
    }

    pub fn filename(&self) -> crate::Result<String> {
        match self {
            Self::Image(path, _) | Self::Font(path) | Self::Audio(path) | Self::Video(path) => {
                let filename = path
                    .file_name()
                    .and_then(|filename| filename.to_str())
                    .ok_or(crate::Error::FilenameNotFound(self.to_string()))?;

                Ok(filename.to_string())
            }
        }
    }
}

impl Display for Resource<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Image(path, _) | Self::Font(path) | Self::Audio(path) | Self::Video(path) => {
                write!(f, "{}", path.to_str().unwrap_or_default())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReferenceType<'a> {
    Acknowledgements(&'a str),
    Bibliography(&'a str),
    Colophon(&'a str),
    Copyright(&'a str),
    Cover(&'a str),
    Dedication(&'a str),
    Epigraph(&'a str),
    Foreword(&'a str),
    Glossary(&'a str),
    Index(&'a str),
    Loi(&'a str),
    Lot(&'a str),
    Notes(&'a str),
    Preface(&'a str),
    Text(&'a str),
    TitlePage(&'a str),
    Toc(&'a str),
}

impl ReferenceType<'_> {
    pub fn type_and_title(&self) -> (&str, &str) {
        match self {
            Self::Acknowledgements(s) => ("acknowledgements", s),
            Self::Bibliography(s) => ("bibliography", s),
            Self::Colophon(s) => ("colophon", s),
            Self::Copyright(s) => ("copyright-page", s),
            Self::Cover(s) => ("cover", s),
            Self::Dedication(s) => ("dedication", s),
            Self::Epigraph(s) => ("epigraph", s),
            Self::Foreword(s) => ("foreword", s),
            Self::Glossary(s) => ("glossary", s),
            Self::Index(s) => ("index", s),
            Self::Loi(s) => ("loi", s),
            Self::Lot(s) => ("lot", s),
            Self::Notes(s) => ("notes", s),
            Self::Preface(s) => ("preface", s),
            Self::Text(s) => ("text", s),
            Self::TitlePage(s) => ("title-page", s),
            Self::Toc(s) => ("toc", s),
        }
    }
}

#[derive(Debug)]
pub struct Toc<'a>(&'a str);

#[derive(Debug)]
pub struct Section<'a> {
    content: &'a [u8],
    pub reference_type: ReferenceType<'a>,
    pub subsections: Option<Vec<Section<'a>>>,
    pub tocs: Option<Vec<Toc<'a>>>,
}

impl<'a> Section<'a> {
    fn new(content: &'a [u8], reference_type: ReferenceType<'a>) -> Self {
        Self {
            content,
            reference_type,
            subsections: None,
            tocs: None,
        }
    }

    pub fn level_subsections(&self) -> usize {
        match self.subsections {
            Some(ref subsections) if subsections.is_empty() => 0,
            Some(ref subsections) => 1 + subsections[0].level_subsections(),
            None => 0,
        }
    }

    pub fn level_tocs(&self) -> usize {
        self.tocs.as_ref().map(|toc| toc.len()).unwrap_or(0)
    }

    pub fn content(
        &self,
        number: &mut usize,
        add_stylesheet: bool,
    ) -> crate::Result<Vec<FileContent<String, Vec<u8>>>> {
        *number += 1;
        let filepath = Self::filename(*number);
        let mut file_contents = Vec::new();

        let xhtml_content = self.xhtml(std::str::from_utf8(self.content)?, add_stylesheet);

        file_contents.push(FileContent::new(
            filepath.to_string(),
            xhtml_content.as_bytes().to_vec(),
        ));

        if let Some(ref subsections) = self.subsections {
            for section in subsections {
                let contents = section.content(number, add_stylesheet)?;
                file_contents.extend(contents);
            }
        }
        Ok(file_contents)
    }

    pub fn filename(number: usize) -> String {
        format!("{:02}.xhtml", number)
    }

    fn xhtml(&self, text: &str, add_stylesheet: bool) -> String {
        let stylesheet = if add_stylesheet {
            r#"
  <link href="style.css" rel="stylesheet" type="text/css"/>"#
        } else {
            ""
        };

        format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN"
  "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <title>{}</title>{}
</head>
{}
</html>"#,
            self.title(),
            stylesheet,
            text
        )
    }

    pub fn title(&self) -> &str {
        self.reference_type.type_and_title().1
    }
}

#[derive(Debug)]
pub struct SectionBuilder<'a>(Section<'a>);

impl<'a> SectionBuilder<'a> {
    #[must_use]
    pub fn new(content: &'a [u8], reference_type: ReferenceType<'a>) -> Self {
        Self(Section::new(content, reference_type))
    }

    pub fn add_subsection(mut self, section: Section<'a>) -> Self {
        if let Some(ref mut subsections) = self.0.subsections {
            subsections.push(section);
        } else {
            self.0.subsections = Some(vec![section]);
        }
        self
    }

    pub fn subsections(mut self, sections: Vec<Section<'a>>) -> Self {
        self.0.subsections = Some(sections);
        self
    }

    pub fn add_toc(mut self, toc: Toc<'a>) -> Self {
        if let Some(ref mut tocs) = self.0.tocs {
            tocs.push(toc);
        } else {
            self.0.tocs = Some(vec![toc]);
        }
        self
    }

    pub fn tocs(mut self, tocs: Vec<Toc<'a>>) -> Self {
        self.0.tocs = Some(tocs);
        self
    }

    pub fn build(self) -> Section<'a> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use tempfile::tempdir;

    use super::*;
    use crate::metadata::MetadataBuilder;

    #[test]
    fn test_epub_builder_new() {
        let metadata = MetadataBuilder::title("Title").build();
        let builder = EpubBuilder::new(metadata);

        assert!(builder.0.stylesheet.is_none());
    }

    #[test]
    fn test_epub_builder_stylesheet_file() {
        let metadata = MetadataBuilder::title("Title").build();

        let temp_dir = tempdir().unwrap();
        let stylesheet_path = temp_dir.path().join("style.css");
        let mut file = File::create(&stylesheet_path).unwrap();
        file.write_all(b"dummy font data").unwrap();

        let stylesheet = std::fs::read(stylesheet_path).unwrap();

        let builder = EpubBuilder::new(metadata).stylesheet(Stylesheet::new(&stylesheet));

        assert!(builder.0.stylesheet.is_some());
    }

    #[test]
    fn test_epub_builder_stylesheet_raw() {
        let metadata = MetadataBuilder::title("Title").build();

        let stylesheet_content = "body { color: red; }";
        let builder =
            EpubBuilder::new(metadata).stylesheet(Stylesheet::new(stylesheet_content.as_bytes()));

        if let Some(stylesheet) = builder.0.stylesheet {
            assert_eq!(stylesheet.content().filepath, "OEBPS/style.css");
        } else {
            panic!("Stylesheet was not set to raw content");
        }
    }

    #[test]
    fn test_epub_builder_complete() {
        let metadata = MetadataBuilder::title("Title").build();

        let temp_dir = tempdir().unwrap();
        let cover_image = temp_dir.path().join("cover.png");
        let font = temp_dir.path().join("SomeFont.ttf");

        let mut file = File::create(&cover_image).unwrap();
        file.write_all(b"dummy image data").unwrap();

        let mut file = File::create(&font).unwrap();
        file.write_all(b"dummy font data").unwrap();

        let epub_result = EpubBuilder::new(metadata)
            .stylesheet(Stylesheet::new(b"body { color: red; }"))
            .cover_image(&cover_image, ImageType::Png)
            .add_resource(Resource::Font(&font))
            .add_section(
                SectionBuilder::new(
                    r#"<body>
  <h1>Part I</h1>
</body>"#
                        .as_bytes(),
                    ReferenceType::TitlePage("Part I"),
                )
                .add_subsection(
                    SectionBuilder::new(
                        r#"<body>
  <h1>Chapter 1</h1>
</body>"#
                            .as_bytes(),
                        ReferenceType::Text("Chapter 1"),
                    )
                    .add_toc(Toc("Toc 1"))
                    .build(),
                )
                .build(),
            )
            .add_section(
                SectionBuilder::new(
                    r#"<body>
  <h1>Part II</h1>
</body>"#
                        .as_bytes(),
                    ReferenceType::TitlePage("Part II"),
                )
                .build(),
            )
            .create(&mut std::io::stdout());

        assert!(epub_result.is_ok());
    }
}
