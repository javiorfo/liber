use std::{fmt::Display, fs, path::Path};

use crate::output::file_content::FileContent;

/// Represents the common image file types supported for inclusion as resources.
///
/// This enum automatically maps to the correct **MIME (media) type**.
#[derive(Debug, Clone)]
pub enum ImageType {
    /// JPEG image format, mapping to `image/jpeg`.
    Jpg,
    /// PNG image format, mapping to `image/png`.
    Png,
    /// GIF image format, mapping to `image/gif`.
    Gif,
    /// Scalable Vector Graphics, mapping to `image/svg+xml`.
    Svg,
}

/// Implements conversion from `ImageType` to its standard MIME type string slice.
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

/// Represents a single external file resource (like an image, font, or video)
/// that must be included in the final output file.
///
/// The `'a` lifetime indicates that the resource only holds a reference to the file's path.
#[derive(Debug, Clone)]
pub enum Resource<'a> {
    /// An image resource, holding a reference to the file path and its type.
    Image(&'a Path, ImageType),
    /// A font resource, holding a reference to the file path. Assumed to be **OpenType**.
    Font(&'a Path),
    /// An audio resource, holding a reference to the file path. Assumed to be **MPEG Audio (MP3)**.
    Audio(&'a Path),
    /// A video resource, holding a reference to the file path. Assumed to be **MP4**.
    Video(&'a Path),
}

impl<'a> Resource<'a> {
    /// Gets the appropriate **MIME media type** string for the resource variant.
    ///
    /// This is required for manifest generation (e.g., in EPUB).
    pub(crate) fn media_type(&self) -> &str {
        match self {
            Resource::Image(_, img_type) => img_type.into(),
            Resource::Font(_) => "application/vnd.ms-opentype",
            Resource::Audio(_) => "audio/mpeg",
            Resource::Video(_) => "video/mp4",
        }
    }

    /// Reads the file content synchronously and wraps it in a [`FileContent`] structure.
    ///
    /// The output path is prefixed with `OEBPS/` and the filename.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or if the filename cannot be extracted.
    pub(crate) fn file_content(&self) -> crate::Result<FileContent<String, Vec<u8>>> {
        match self {
            Self::Image(path, _) | Self::Font(path) | Self::Audio(path) | Self::Video(path) => Ok(
                FileContent::new(format!("OEBPS/{}", self.filename()?), fs::read(path)?),
            ),
        }
    }

    /// Reads the file content asynchronously (using `tokio::fs`) and wraps it in a [`FileContent`] structure.
    ///
    /// This method is only compiled when the **`async` feature** is enabled.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read asynchronously or if the filename cannot be extracted.
    #[cfg(feature = "async")]
    pub(crate) async fn async_file_content(&self) -> crate::Result<FileContent<String, Vec<u8>>> {
        match self {
            Self::Image(path, _) | Self::Font(path) | Self::Audio(path) | Self::Video(path) => {
                Ok(FileContent::new(
                    format!("OEBPS/{}", self.filename()?),
                    tokio::fs::read(path).await?,
                ))
            }
        }
    }

    /// Extracts the final filename (e.g., `image.png`) from the full path reference.
    ///
    /// # Errors
    /// Returns a [`crate::Error::FilenameNotFound`] if the path does not contain a valid filename.
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

    /// Generates the **XML `<item>` tag** used in the package manifest (e.g., EPUB's `content.opf`).
    ///
    /// Returns `None` if the filename cannot be extracted.
    pub(crate) fn as_manifest_xml(&self) -> Option<String> {
        Some(format!(
            r#"<item id="{filename}" href="{filename}" media-type="{media_type}"/>"#,
            filename = self.filename().ok()?,
            media_type = self.media_type()
        ))
    }
}

/// Implements display for [`Resource`], outputting the file's full path string.
impl Display for Resource<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Image(path, _) | Self::Font(path) | Self::Audio(path) | Self::Video(path) => {
                write!(f, "{}", path.to_str().unwrap_or_default())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    fn create_temp_file(dir: &Path, filename: &str, content: &[u8]) -> PathBuf {
        let temp_dir = tempdir().expect("Error creating tempdir");
        let file_path = temp_dir.path().join(dir).join(filename);
        let mut file = fs::File::create(&file_path).expect("Error creating mock file");
        file.write_all(content).expect("Error writing to mock file");

        file_path
    }

    #[test]
    fn test_resource_media_type_image() {
        let path = Path::new("test.jpg");
        let resource = Resource::Image(path, ImageType::Jpg);
        assert_eq!(resource.media_type(), "image/jpeg");

        let resource = Resource::Image(path, ImageType::Png);
        assert_eq!(resource.media_type(), "image/png");
    }

    #[test]
    fn test_resource_media_type_other() {
        let path = Path::new("test.otf");
        assert_eq!(
            Resource::Font(path).media_type(),
            "application/vnd.ms-opentype"
        );

        let path = Path::new("test.mp3");
        assert_eq!(Resource::Audio(path).media_type(), "audio/mpeg");

        let path = Path::new("test.mp4");
        assert_eq!(Resource::Video(path).media_type(), "video/mp4");
    }

    #[test]
    fn test_resource_filename_valid() {
        let path = Path::new("/path/to/some/file.png");
        let resource = Resource::Image(path, ImageType::Png);
        assert_eq!(resource.filename().unwrap(), "file.png");

        let path = Path::new("just_a_file.gif");
        let resource = Resource::Image(path, ImageType::Gif);
        assert_eq!(resource.filename().unwrap(), "just_a_file.gif");

        let path = Path::new("assets/font.otf");
        let resource = Resource::Font(path);
        assert_eq!(resource.filename().unwrap(), "font.otf");
    }

    #[test]
    fn test_resource_file_content_success() {
        let temp_dir = tempfile::tempdir().unwrap();
        let filename = "test.jpg";
        let content: Vec<u8> = vec![0x11, 0x22, 0x33, 0x44];
        let file_path = create_temp_file(temp_dir.path(), filename, &content);

        let resource = Resource::Image(&file_path, ImageType::Jpg);

        let file_content = resource.file_content().unwrap();

        let expected_filepath = format!("OEBPS/{}", filename);
        let expected_content = FileContent::new(expected_filepath, content);

        assert_eq!(file_content, expected_content);
    }

    #[test]
    fn test_resource_file_content_io_error() {
        let non_existent_path = Path::new("non_existent_file_for_test.mp4");
        let resource = Resource::Video(non_existent_path);

        match resource.file_content() {
            Err(e) => assert!(matches!(e, crate::Error::Io(_))),
            _ => panic!("Expected Io error when reading non-existent file"),
        }
    }

    #[test]
    fn test_resource_display_trait() {
        let path = Path::new("/some/long/path/file.svg");
        let resource = Resource::Image(path, ImageType::Svg);
        assert_eq!(format!("{}", resource), "/some/long/path/file.svg");

        let path = Path::new("font.otf");
        let resource = Resource::Font(path);
        assert_eq!(format!("{}", resource), "font.otf");
    }
}
