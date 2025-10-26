//! # A library for creating (sync/async) EPUB files
//!
//! This crate provides a high-level, ergonomic API for creating EPUB files (2.0.1).
//! It offers both asynchronous and blocking (synchronous) implementations, with flexible builders and output options.
//!
//! ## Features
//!
//! - Create an Epub with multiple settings (contents, references, resources, etc).
//! - Both async and blocking APIs (enable via crate features).
//! - Strong error handling with [`Error`] and [`Result`] types.
//!
//!
//! ### Example
//! ```rust
//! use std::path::Path;
//!
//! use liber::epub::{
//!     ContentBuilder, ContentReference, EpubBuilder, ImageType, MetadataBuilder, ReferenceType,
//!     Resource,
//! };
//!
//! fn main() {
//!     match create() {
//!         Err(e) => eprintln!("{e}"),
//!         Ok(_) => println!("ok"),
//!     }
//! }
//!
//! fn create() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut file = std::fs::File::create("book.epub")?;
//!     let title = "My Book";
//!
//!     let contents = vec![
//!         ContentBuilder::new(
//!             r#"<body><h1>Chapter 2</h1></body>"#.as_bytes(),
//!             ReferenceType::Text("Chapter 2".to_string()),
//!         )
//!         .build(),
//!         ContentBuilder::new(
//!             r#"<body><h1>Chapter 3</h1></body>"#.as_bytes(),
//!             ReferenceType::Text("Chapter 3".to_string()),
//!         )
//!         .add_child(
//!             ContentBuilder::new(
//!                 r#"<body><h1>Chapter 4</h1></body>"#.as_bytes(),
//!                 ReferenceType::TitlePage("Chapter 4".to_string()),
//!             )
//!             .build(),
//!         )
//!         .build(),
//!     ];
//!
//!     let epub_builder = EpubBuilder::new(MetadataBuilder::title(title).creator("author").build())
//!         .stylesheet("body {}".as_bytes())
//!         .cover_image(Path::new("/path/to/img.jpg"), ImageType::Jpg)
//!         .add_resource(Resource::Font(Path::new("/path/to/some_font.otf")))
//!         .add_content(
//!             ContentBuilder::new(
//!                 r#"<body><h1>Chapter 1</h1><h2 id="id01">Section 1.1</h2><h2 id="id02">Section 1.1.1</h2><h2 id="id03">Section 1.2</h2></body>"#.as_bytes(),
//!                 ReferenceType::Text("Chapter 1".to_string()))
//!             .add_content_reference(ContentReference::new("Section 1.1").add_child(ContentReference::new("Section 1.1.1")))
//!             .add_content_reference(ContentReference::new("Section 1.2"))
//!             .add_children(contents)
//!             .build(),
//!         );
//!
//!     epub_builder.create(&mut file)?;
//!
//!     Ok(())
//! }
//! ```
//!
//!
//! ## Modules & Re-exports
//!
//! - [`epub`] — Core types to model the epub.
//! - [`epub::Content`], [`epub::ContentReference`], [`epub::Resource`], [`epub::Language`], [`epub::Identifier`], [`epub::Metadata`] — Main data structures.
//! - [`epub::EpubBuilder`], [`epub::ContentBuilder`], [`epub::MetadataBuilder`] — Builders.
//!
//! ## Error Handling
//!
//! All fallible operations return [`Result<T>`](Result) with a custom [`Error`] enum that wraps
//! errors from underlying dependencies.
//!
//! ## Feature Flags
//!
//! - `async` — Enables the asynchronous API (`search`).
//!
//! ## License
//!
//! This is free software, published under the [MIT License](https://mit-license.org/).

pub mod epub;
mod output;

pub use output::creator::ZipCompression;

/// Error type for all fallible operations in this crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[cfg(feature = "async")]
    #[error(transparent)]
    AsyncZip(#[from] async_zip::error::ZipError),

    #[cfg(feature = "async")]
    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    StringUtf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Xml(#[from] quick_xml::Error),

    #[error("Filename not found: {0}")]
    FilenameNotFound(String),

    #[error("Content filename must end with '.xhtml'. Got '{0}'")]
    ContentFilename(String),

    #[error("Error at position {0}: {1:?}")]
    XmlParser(u64, quick_xml::Error),
}

/// A convenient alias for `Result` with the crate's [`Error`] type.
///
/// Defaults to `()` for the success type if not specified.
pub type Result<T = ()> = std::result::Result<T, Error>;
