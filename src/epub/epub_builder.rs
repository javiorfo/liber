use std::{io::Write, path::Path};

use crate::ZipCompression;
use crate::{
    epub::{Content, ImageType, Resource, metadata::Metadata},
    output::creator::EpubFile,
};

/// The main structure representing a complete EPUB document ready for generation.
///
/// It holds all the necessary components: metadata, styling, resources, and ordered content.
/// Instances of `Epub` should generally be created using the [`EpubBuilder`].
#[derive(Debug, Clone)]
pub(crate) struct Epub<'a> {
    /// The descriptive metadata for the EPUB (title, author, publisher, etc.).
    pub metadata: Metadata,
    /// Optional stylesheet content (CSS bytes) to be included in the EPUB.
    pub stylesheet: Option<&'a [u8]>,
    /// Optional resource designated as the cover image.
    pub cover_image: Option<Resource<'a>>,
    /// Optional list of external resources (images, fonts, audio) used by the content.
    pub resources: Option<Vec<Resource<'a>>>,
    /// Optional, ordered list of main content units (chapters, sections, appendices).
    pub contents: Option<Vec<Content<'a>>>,
}

impl<'a> Epub<'a> {
    /// Creates a new `Epub` instance with the mandatory [`Metadata`] and all optional fields set to `None`.
    fn new(metadata: Metadata) -> Epub<'a> {
        Self {
            metadata,
            stylesheet: None,
            cover_image: None,
            resources: None,
            contents: None,
        }
    }

    /// Generates the XML `<meta>` tag for the EPUB's NCX file, specifying the maximum **navigation depth**.
    pub fn level_as_toc_xml(&self) -> String {
        format!(r#"<meta name="dtb:depth" content="{}"/>"#, self.level())
    }

    /// Generates the XML `<meta>` tag for the **cover image**, used in the content package metadata.
    ///
    /// Returns `None` if no cover image is set.
    pub fn cover_image_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            r#"<meta name="cover" content="{}"/>"#,
            self.cover_image.as_ref()?.filename().ok()?
        ))
    }

    /// Generates the XML `<item>` tag for the **cover image**, used in the manifest section.
    ///
    /// Returns `None` if no cover image is set.
    pub fn cover_image_as_manifest_xml(&self) -> Option<String> {
        self.cover_image.as_ref()?.as_manifest_xml()
    }

    /// Calculates the maximum nesting level based on all content and content references.
    ///
    /// This value is used to set the `dtb:depth` property in the TOC/NCX file.
    fn level(&self) -> usize {
        if let Some(ref contents) = self.contents {
            let level_subcontents = contents
                .iter()
                .map(|content| content.level() + 1)
                .max()
                .unwrap_or(1);

            let level_content_references = contents
                .iter()
                .map(|content| content.level_reference_content() + 1)
                .max()
                .unwrap_or(1);

            level_subcontents.max(level_content_references)
        } else {
            0
        }
    }
}

/// A fluent builder for creating and configuring an Epub.
///
/// Use the `create()` method to serialize the EPUB to a file.
#[derive(Debug)]
pub struct EpubBuilder<'a>(pub(crate) Epub<'a>);

impl<'a> EpubBuilder<'a> {
    /// Starts the builder by providing the mandatory descriptive metadata.
    #[must_use]
    pub fn new(metadata: Metadata) -> Self {
        Self(Epub::new(metadata))
    }

    /// Sets the raw byte content for the required stylesheet (`style.css`).
    pub fn stylesheet(mut self, stylesheet: &'a [u8]) -> Self {
        self.0.stylesheet = Some(stylesheet);
        self
    }

    /// Sets the primary **cover image** for the EPUB.
    ///
    /// The cover image is automatically registered as a resource.
    pub fn cover_image(mut self, path: &'a Path, image_type: ImageType) -> Self {
        self.0.cover_image = Some(Resource::Image(path, image_type));
        self
    }

    /// Adds a single external [`Resource`] (e.g., a font or extra image) to the EPUB package.
    pub fn add_resource(mut self, resource: Resource<'a>) -> Self {
        if let Some(ref mut resources) = self.0.resources {
            resources.push(resource);
        } else {
            self.0.resources = Some(vec![resource]);
        }
        self
    }

    /// Adds a collection of external [`Resource`] items to the EPUB package.
    pub fn add_resources(mut self, resources: Vec<Resource<'a>>) -> Self {
        if let Some(ref mut self_resources) = self.0.resources {
            self_resources.extend(resources);
        } else {
            self.0.resources = Some(resources);
        }
        self
    }

    /// Adds a single [`Content`] unit (like a chapter or section) to the main book flow.
    pub fn add_content(mut self, content: Content<'a>) -> Self {
        if let Some(ref mut contents) = self.0.contents {
            contents.push(content);
        } else {
            self.0.contents = Some(vec![content]);
        }
        self
    }

    /// Adds a collection of [`Content`] units to the main book flow.
    pub fn add_contents(mut self, contents: Vec<Content<'a>>) -> Self {
        if let Some(ref mut self_contents) = self.0.contents {
            self_contents.extend(contents);
        } else {
            self.0.contents = Some(contents);
        }
        self
    }

    /// Finalizes the builder and **synchronously** generates the EPUB file, writing the contents to the provided writer.
    ///
    /// Uses the default zip compression method.
    ///
    /// # Errors
    /// Returns a [`crate::Result`] if there are any I/O issues or errors during XML generation.
    pub fn create<W>(self, writer: &mut W) -> crate::Result
    where
        W: Write + Send,
    {
        self.create_with_compression(writer, ZipCompression::default())
    }

    /// Finalizes the builder and **synchronously** generates the EPUB file, using a specified zip compression method.
    ///
    /// # Errors
    /// Returns a [`crate::Result`] if there are any I/O issues or errors during XML generation.
    pub fn create_with_compression<W>(
        self,
        writer: &mut W,
        compression: ZipCompression,
    ) -> crate::Result
    where
        W: Write + Send,
    {
        EpubFile::new(self.0, writer, compression).create()
    }

    /// **Asynchronously** generates the EPUB file, writing the contents to the provided `tokio::io::AsyncWrite` writer.
    ///
    /// This method is only available when the **`async` feature** is enabled.
    #[cfg(feature = "async")]
    pub async fn async_create<W>(self, writer: &mut W) -> crate::Result
    where
        W: tokio::io::AsyncWrite + Unpin + Send,
    {
        self.async_create_with_compression(writer, ZipCompression::default())
            .await
    }

    /// **Asynchronously** generates the EPUB file with a specified zip compression method.
    ///
    /// This method is only available when the **`async` feature** is enabled.
    #[cfg(feature = "async")]
    pub async fn async_create_with_compression<W>(
        self,
        writer: &mut W,
        compression: ZipCompression,
    ) -> crate::Result
    where
        W: tokio::io::AsyncWrite + Unpin + Send,
    {
        use crate::output::creator_async::EpubFile;

        EpubFile::new(self.0, writer, compression).create().await
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use tempfile::tempdir;

    use super::*;
    use crate::epub::{ContentBuilder, ContentReference, ReferenceType, metadata::MetadataBuilder};

    #[test]
    fn test_epub_builder_new() {
        let metadata = MetadataBuilder::title("Title").build();
        let builder = EpubBuilder::new(metadata);

        assert!(builder.0.stylesheet.is_none());
    }

    #[test]
    fn test_epub_builder_stylesheet_file() {
        let metadata = MetadataBuilder::title("Title").build();

        let temp_dir = tempdir().expect("Error creating tempdir");
        let stylesheet_path = temp_dir.path().join("style.css");
        let mut file = File::create(&stylesheet_path).expect("Error creating mock css");
        file.write_all(b"dummy font data")
            .expect("Error writing to mock css");

        let stylesheet = std::fs::read(stylesheet_path).expect("Error reading mock css");

        let builder = EpubBuilder::new(metadata).stylesheet(&stylesheet);

        assert!(builder.0.stylesheet.is_some());
    }

    #[test]
    fn test_epub_builder_stylesheet_raw() {
        let metadata = MetadataBuilder::title("Title").build();

        let stylesheet_content = "body { color: red; }";
        let builder = EpubBuilder::new(metadata).stylesheet(stylesheet_content.as_bytes());

        if let Some(stylesheet) = builder.0.stylesheet {
            assert_eq!(stylesheet, "body { color: red; }".as_bytes());
        } else {
            panic!("Stylesheet was not set to raw content");
        }
    }

    #[test]
    fn test_epub_builder_complete() {
        let temp_dir = tempdir().expect("Error creating tempdir");
        let cover_image = temp_dir.path().join("cover.png");
        let font = temp_dir.path().join("SomeFont.ttf");

        let mut file = File::create(&cover_image).expect("Error creating mock cover image");
        file.write_all(b"dummy image data")
            .expect("Error writing to mock cover image");

        let mut file = File::create(&font).expect("Error creating mock mock font");
        file.write_all(b"dummy font data")
            .expect("Error writing to mock font");

        let epub_result = EpubBuilder::new(MetadataBuilder::title("Title").build())
            .stylesheet(b"body { color: red; }")
            .cover_image(&cover_image, ImageType::Png)
            .add_resource(Resource::Font(&font))
            .add_content(
                ContentBuilder::new(
                    "<body><h1>Part I</h1></body>".as_bytes(),
                    ReferenceType::TitlePage("Part I".to_string()),
                )
                .add_child(
                    ContentBuilder::new(
                        "<body><h1>Chapter 1</h1></body>".as_bytes(),
                        ReferenceType::Text("Chapter 1".to_string()),
                    )
                    .add_content_reference(ContentReference::new("Content 1.1"))
                    .add_content_reference(
                        ContentReference::new("Content 1.2")
                            .add_child(ContentReference::new("Content 1.2.1")),
                    )
                    .build(),
                )
                .build(),
            )
            .add_content(
                ContentBuilder::new(
                    "<body><h1>Part II</h1></body>".as_bytes(),
                    ReferenceType::TitlePage("Part II".to_string()),
                )
                .add_content_reference(ContentReference::new("Content 2.1"))
                .build(),
            )
            .create(&mut std::io::stdout());

        assert!(epub_result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "async")]
    async fn test_async_epub_builder_complete() {
        let epub_result = EpubBuilder::new(MetadataBuilder::title("Title").build())
            .stylesheet(b"body { color: red; }")
            .add_content(
                ContentBuilder::new(
                    "<body><h1>Part I</h1></body>".as_bytes(),
                    ReferenceType::TitlePage("Part I".to_string()),
                )
                .add_child(
                    ContentBuilder::new(
                        "<body><h1>Chapter 1</h1></body>".as_bytes(),
                        ReferenceType::Text("Chapter 1".to_string()),
                    )
                    .add_content_reference(ContentReference::new("Content 1.1"))
                    .add_content_reference(
                        ContentReference::new("Content 1.2")
                            .add_child(ContentReference::new("Content 1.2.1")),
                    )
                    .build(),
                )
                .build(),
            )
            .add_content(
                ContentBuilder::new(
                    "<body><h1>Part II</h1></body>".as_bytes(),
                    ReferenceType::TitlePage("Part II".to_string()),
                )
                .add_content_reference(ContentReference::new("Content 2.1"))
                .build(),
            )
            .async_create(&mut tokio::io::stdout())
            .await;

        assert!(epub_result.is_ok());
    }
}
