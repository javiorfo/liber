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
