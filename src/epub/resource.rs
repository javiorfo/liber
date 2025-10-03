use std::{fmt::Display, fs, path::Path};

use crate::output::file_content::FileContent;

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
    pub(crate) fn media_type(&self) -> &str {
        match self {
            Resource::Image(_, img_type) => img_type.into(),
            Resource::Font(_) => "application/vnd.ms-opentype",
            Resource::Audio(_) => "audio/mpeg",
            Resource::Video(_) => "video/mp4",
        }
    }

    pub(crate) fn file_content(&self) -> crate::Result<FileContent<String, Vec<u8>>> {
        match self {
            Self::Image(path, _) | Self::Font(path) | Self::Audio(path) | Self::Video(path) => Ok(
                FileContent::new(format!("OEBPS/{}", self.filename()?), fs::read(path)?),
            ),
        }
    }

    pub(crate) fn filename(&self) -> crate::Result<String> {
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
