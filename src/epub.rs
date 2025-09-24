use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use crate::{metadata::Metadata, output::creator::create};

#[derive(Debug)]
pub struct Epub<'a> {
    pub metadata: Metadata<'a>,
    pub stylesheet: Option<Stylesheet<'a>>,
    pub cover_image: Option<Image<'a>>,
    pub images: Option<Vec<Image<'a>>>,
}

impl<'a> Epub<'a> {
    fn new(metadata: Metadata<'a>) -> Epub<'a> {
        Self {
            metadata,
            stylesheet: None,
            cover_image: None,
            images: None,
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

    pub fn cover_image(mut self, cover_image: Image<'a>) -> Self {
        self.0.cover_image = Some(cover_image);
        self
    }

    pub fn add_image(mut self, image: Image<'a>) -> Self {
        if let Some(ref mut images) = self.0.images {
            images.push(image);
        } else {
            self.0.images = Some(vec![image]);
        }
        self
    }

    pub fn create<W: Write>(self, writer: &mut W) -> crate::Result {
        create(self.0, writer)
    }
}

#[derive(Debug, Clone)]
pub enum Stylesheet<'a> {
    File(&'a Path),
    Raw(&'a str),
}

impl<'a> Stylesheet<'a> {
    pub const FILE: &'a str = "OEBPS/styles/style.css";

    pub fn content(&self) -> io::Result<String> {
        match self {
            Stylesheet::Raw(text) => Ok(text.to_string()),
            Stylesheet::File(path) => Ok(fs::read_to_string(path)?),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Image<'a> {
    Jpg(&'a Path),
    Png(&'a Path),
    Gif(&'a Path),
}

impl<'a> Image<'a> {
    pub const PATH: &'a str = "OEBPS/images";

    pub fn media_type(&self) -> &str {
        match self {
            Image::Jpg(_) => "image/jpeg",
            Image::Png(_) => "image/png",
            Image::Gif(_) => "image/gif",
        }
    }

    pub fn content(&self) -> io::Result<Vec<u8>> {
        match self {
            Self::Jpg(path) | Self::Png(path) | Self::Gif(path) => Ok(fs::read(path)?),
        }
    }

    pub fn file_name(&self) -> Option<&str> {
        match self {
            Self::Jpg(path) | Self::Png(path) | Self::Gif(path) => {
                path.file_name().and_then(|filename| filename.to_str())
            }
        }
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

        let builder = EpubBuilder::new(metadata).stylesheet(Stylesheet::File(&stylesheet_path));

        if let Some(Stylesheet::File(path)) = builder.0.stylesheet {
            assert_eq!(path, stylesheet_path);
        } else {
            panic!("Stylesheet was not set to a file");
        }
    }

    #[test]
    fn test_epub_builder_stylesheet_raw() {
        let metadata = MetadataBuilder::title("Title").build();

        let stylesheet_content = "body { color: red; }";
        let builder = EpubBuilder::new(metadata).stylesheet(Stylesheet::Raw(stylesheet_content));

        if let Some(stylesheet) = builder.0.stylesheet {
            assert_eq!(
                stylesheet.content().expect("Must be OK text content"),
                stylesheet_content
            );
        } else {
            panic!("Stylesheet was not set to raw content");
        }
    }

    #[test]
    fn test_epub_builder_with_cover_image() {
        let metadata = MetadataBuilder::title("Title").build();

        let temp_dir = tempdir().unwrap();
        let cover_image = temp_dir.path().join("cover.png");

        let mut file = File::create(&cover_image).unwrap();
        file.write_all(b"dummy image data").unwrap();

        let epub_result = EpubBuilder::new(metadata)
            .stylesheet(Stylesheet::Raw("body { color: red; }"))
            .cover_image(Image::Jpg(&cover_image))
            .create(&mut io::stdout());

        assert!(epub_result.is_ok());
    }
}
