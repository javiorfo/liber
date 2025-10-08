use std::{io::Write, path::Path};

use crate::ZipCompression;
use crate::{
    epub::{Content, ImageType, Resource, metadata::Metadata},
    output::creator::EpubFile,
};

#[derive(Debug, Clone)]
pub(crate) struct Epub<'a> {
    pub metadata: Metadata<'a>,
    pub stylesheet: Option<&'a [u8]>,
    pub cover_image: Option<Resource<'a>>,
    pub resources: Option<Vec<Resource<'a>>>,
    pub contents: Option<Vec<Content<'a>>>,
}

impl<'a> Epub<'a> {
    fn new(metadata: Metadata<'a>) -> Epub<'a> {
        Self {
            metadata,
            stylesheet: None,
            cover_image: None,
            resources: None,
            contents: None,
        }
    }

    pub fn level_as_toc_xml(&self) -> String {
        format!(r#"<meta name="dtb:depth" content="{}"/>"#, self.level())
    }

    pub fn cover_image_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            r#"<meta name="cover" content="{}"/>"#,
            self.cover_image.as_ref()?.filename().ok()?
        ))
    }

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

#[derive(Debug)]
pub struct EpubBuilder<'a>(Epub<'a>);

impl<'a> EpubBuilder<'a> {
    #[must_use]
    pub fn new(metadata: Metadata<'a>) -> Self {
        Self(Epub::new(metadata))
    }

    pub fn stylesheet(mut self, stylesheet: &'a [u8]) -> Self {
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

    pub fn add_content(mut self, content: Content<'a>) -> Self {
        if let Some(ref mut contents) = self.0.contents {
            contents.push(content);
        } else {
            self.0.contents = Some(vec![content]);
        }
        self
    }

    pub fn contents(mut self, contents: Vec<Content<'a>>) -> Self {
        self.0.contents = Some(contents);
        self
    }

    pub fn create<W: Write>(self, writer: &mut W) -> crate::Result {
        self.create_with_compression(writer, ZipCompression::Stored)
    }

    pub fn create_with_compression<W: Write>(
        self,
        writer: &mut W,
        compression: ZipCompression,
    ) -> crate::Result {
        EpubFile::new(self.0, writer, compression).create()
    }

    #[cfg(feature = "async")]
    pub async fn async_create<W: tokio::io::AsyncWrite + Unpin>(
        self,
        writer: &mut W,
    ) -> crate::Result {
        self.async_create_with_compression(writer, ZipCompression::Stored)
            .await
    }

    #[cfg(feature = "async")]
    pub async fn async_create_with_compression<W: tokio::io::AsyncWrite + Unpin>(
        self,
        writer: &mut W,
        compression: ZipCompression,
    ) -> crate::Result {
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
        let metadata = MetadataBuilder::title("Title").build();

        let temp_dir = tempdir().expect("Error creating tempdir");
        let cover_image = temp_dir.path().join("cover.png");
        let font = temp_dir.path().join("SomeFont.ttf");

        let mut file = File::create(&cover_image).expect("Error creating mock cover image");
        file.write_all(b"dummy image data")
            .expect("Error writing to mock cover image");

        let mut file = File::create(&font).expect("Error creating mock mock font");
        file.write_all(b"dummy font data")
            .expect("Error writing to mock font");

        let epub_result = EpubBuilder::new(metadata)
            .stylesheet(b"body { color: red; }")
            .cover_image(&cover_image, ImageType::Png)
            .add_resource(Resource::Font(&font))
            .add_content(
                ContentBuilder::new(
                    "<body><h1>Part I</h1></body>".as_bytes(),
                    ReferenceType::TitlePage("Part I"),
                )
                .add_subcontent(
                    ContentBuilder::new(
                        "<body><h1>Chapter 1</h1></body>".as_bytes(),
                        ReferenceType::Text("Chapter 1"),
                    )
                    .add_content_reference(ContentReference::new("Content 1.1"))
                    .add_content_reference(
                        ContentReference::new("Content 1.2")
                            .add_subcontent_reference(ContentReference::new("Content 1.2.1")),
                    )
                    .build(),
                )
                .build(),
            )
            .add_content(
                ContentBuilder::new(
                    "<body><h1>Part II</h1></body>".as_bytes(),
                    ReferenceType::TitlePage("Part II"),
                )
                .add_content_reference(ContentReference::new("Content 2.1"))
                .build(),
            )
            .create(&mut std::io::stdout());

        assert!(epub_result.is_ok());
    }
}
