use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    path::Path,
};

use crate::{
    metadata::Metadata,
    output::{content::FileContent, creator::EpubFile},
};

#[derive(Debug)]
pub struct Epub<'a, S: AsRef<str>> {
    pub metadata: Metadata<S>,
    pub stylesheet: Option<Stylesheet<'a, S>>,
    pub cover_image: Option<Image<'a>>,
    pub images: Option<Vec<Image<'a>>>,
}

impl<'a, S: AsRef<str>> Epub<'a, S> {
    fn new(metadata: Metadata<S>) -> Epub<'a, S> {
        Self {
            metadata,
            stylesheet: None,
            cover_image: None,
            images: None,
        }
    }
}

#[derive(Debug)]
pub struct EpubBuilder<'a, S: AsRef<str>>(Epub<'a, S>);

impl<'a, S: AsRef<str>> EpubBuilder<'a, S> {
    #[must_use]
    pub fn new(metadata: Metadata<S>) -> Self {
        Self(Epub::new(metadata))
    }

    pub fn stylesheet(mut self, stylesheet: Stylesheet<'a, S>) -> Self {
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
        EpubFile::new(self.0, writer).create()
    }
}

#[derive(Debug, Clone)]
pub enum Stylesheet<'a, R: AsRef<str>> {
    File(&'a Path),
    Raw(R),
}

impl<'a, R: AsRef<str>> Stylesheet<'a, R> {
    pub fn content(&self) -> io::Result<FileContent<String>> {
        let filepath = "OEBPS/styles/style.css";

        match self {
            Stylesheet::Raw(text) => Ok(FileContent::new(
                filepath.to_string(),
                text.as_ref().as_bytes().to_vec(),
            )),
            Stylesheet::File(path) => Ok(FileContent::new(filepath.to_string(), fs::read(path)?)),
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
    pub fn media_type(&self) -> &str {
        match self {
            Image::Jpg(_) => "image/jpeg",
            Image::Png(_) => "image/png",
            Image::Gif(_) => "image/gif",
        }
    }

    pub fn content(&self) -> crate::Result<FileContent<String>> {
        match self {
            Self::Jpg(path) | Self::Png(path) | Self::Gif(path) => Ok(FileContent::new(
                format!("OEBPS/images/{}", self.filename()?),
                fs::read(path)?,
            )),
        }
    }

    pub fn filename(&self) -> crate::Result<String> {
        match self {
            Self::Jpg(path) | Self::Png(path) | Self::Gif(path) => {
                let filename = path
                    .file_name()
                    .and_then(|filename| filename.to_str())
                    .ok_or(crate::Error::FilenameNotFound(self.to_string()))?;

                Ok(filename.to_string())
            }
        }
    }
}

impl Display for Image<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jpg(path) | Self::Png(path) | Self::Gif(path) => {
                write!(f, "{}", path.to_str().unwrap_or_default())
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
            assert!(stylesheet.content().is_ok());
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
            .cover_image(Image::Png(&cover_image))
            .create(&mut io::stdout());

        assert!(epub_result.is_ok());
    }
}
