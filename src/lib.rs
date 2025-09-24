mod epub;
mod metadata;
mod output;

pub use epub::EpubBuilder;
pub use metadata::{Identifier, Language, MetadataBuilder};

/// Error type for all fallible operations in this crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error("Filename not found: {0}")]
    FilenameNotFound(String),
}

/// A convenient alias for `Result` with the crate's [`Error`] type.
///
/// Defaults to `()` for the success type if not specified.
pub type Result<T = ()> = std::result::Result<T, Error>;
