use std::io::Cursor;

use async_zip::{Compression, ZipEntryBuilder, tokio::write::ZipFileWriter};
use futures::future;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::{
    ZipCompression,
    epub::Epub,
    output::{
        file_content::{self, FileContent},
        xml,
    },
};

/// A builder responsible for asynchronously creating and writing all components
/// of an EPUB book into a standard ZIP archive format using `tokio` and `async_zip`.
///
/// This struct is suitable for non-blocking I/O operations where the final
/// EPUB archive is written to an asynchronous writer (`W`).
pub struct EpubFile<'a, W> {
    /// The source data structure containing all metadata and content of the EPUB.
    epub: Epub<'a>,
    /// The external asynchronous writer where the final compressed EPUB bytes will be written to.
    writer: W,
    /// The internal asynchronous ZIP writer, buffering the content before flushing.
    zip_writer: ZipFileWriter<Cursor<Vec<u8>>>,
    /// The configured compression method for the ZIP entries.
    compression: async_zip::Compression,
}

impl<'a, W> EpubFile<'a, W>
where
    W: AsyncWrite + Unpin + Send,
{
    /// Creates a new asynchronous `EpubFile` builder.
    ///
    /// This sets up the internal asynchronous ZIP writer and configures the
    /// compression method to be used for most files (excluding `mimetype`, which is stored).
    ///
    /// # Type Parameters
    ///
    /// * `W`: A type that implements `tokio::io::AsyncWrite`, `Unpin`, and `Send`.
    ///
    /// # Arguments
    ///
    /// * `epub`: The EPUB data structure to be written.
    /// * `writer`: The output asynchronous stream where the final EPUB bytes will be written.
    /// * `compression`: The default compression method to use for the files.
    pub fn new(epub: Epub<'a>, writer: W, compression: ZipCompression) -> EpubFile<'a, W> {
        Self {
            epub,
            writer,
            zip_writer: ZipFileWriter::with_tokio(Cursor::new(Vec::new())),
            compression: match compression {
                ZipCompression::Stored => Compression::Stored,
                ZipCompression::Deflated => Compression::Deflate,
            },
        }
    }

    /// Asynchronously generates all necessary EPUB files, zips them, and writes the
    /// final archive to the output writer.
    ///
    /// This method leverages asynchronous I/O and uses `future::try_join_all`
    /// to concurrently load content from resources. It also uses the asynchronous
    /// XML formatting function to ensure non-blocking operation.
    ///
    /// # Returns
    ///
    /// Returns `crate::Result<()>` indicating success or failure in any step
    /// (async file generation, XML formatting, or asynchronous ZIP writing).
    pub async fn create(mut self) -> crate::Result<()> {
        self.add_file(file_content::mimetype()).await?;
        self.add_file(file_content::container()).await?;
        self.add_file(file_content::display_options()).await?;

        if let Some(stylesheet) = self.epub.stylesheet {
            self.add_file(FileContent::new("OEBPS/style.css", stylesheet))
                .await?;
        }

        if let Some(ref cover_image) = self.epub.cover_image {
            self.add_file(cover_image.async_file_content().await?)
                .await?;
        }

        // Concurrently load resources and add them
        if let Some(ref resources) = self.epub.resources {
            // Map resources to a vector of futures
            let contents = resources
                .iter()
                .map(|resource| resource.async_file_content())
                .collect::<Vec<_>>();

            // Wait for all resource futures to complete
            let contents = future::try_join_all(contents).await?;
            self.add_files(contents).await?;
        }

        // Generate and add content XHTML files
        if let Some(ref contents) = self.epub.contents {
            let mut file_number: usize = 0;
            let mut file_contents: Vec<FileContent<String, String>> = Vec::new();
            for content in contents {
                let res = content
                    .async_file_content(&mut file_number, self.epub.stylesheet.is_some())
                    .await?;
                file_contents.extend(res);
            }

            self.add_files(file_contents).await?;
        }

        // Generate, format (async), and add OPF file
        let mut content_opf = file_content::content_opf(&self.epub)?;
        content_opf.format(xml::async_format(content_opf.bytes.clone()).await?);
        self.add_file(content_opf).await?;

        // Generate, format (async), and add NCX file
        let mut toc_ncx = file_content::toc_ncx(&self.epub)?;
        toc_ncx.format(xml::async_format(toc_ncx.bytes.clone()).await?);
        self.add_file(toc_ncx).await?;

        // Finalize the ZIP archive and write the internal buffer to the external writer
        let compat_cursor = self.zip_writer.close().await?;
        self.writer
            .write_all(&compat_cursor.into_inner().into_inner())
            .await?;

        Ok(())
    }

    /// Asynchronously adds a single `FileContent` item to the internal ZIP archive.
    ///
    /// Uses `ZipEntryBuilder` to configure the file and `write_entry_whole` to write
    /// the entire content buffer in one asynchronous operation.
    ///
    /// # Arguments
    ///
    /// * `file_content`: The structure holding the file path and content bytes.
    async fn add_file<F, B>(&mut self, file_content: FileContent<F, B>) -> crate::Result<()>
    where
        F: Into<String>,
        B: AsRef<[u8]>,
    {
        // Use the configured compression for all files added here
        let builder = ZipEntryBuilder::new(file_content.filepath.into().into(), self.compression)
            .unix_permissions(0o755)
            .build();

        self.zip_writer
            .write_entry_whole(builder, file_content.bytes.as_ref())
            .await?;
        Ok(())
    }

    /// Asynchronously adds a vector of `FileContent` items to the internal ZIP archive.
    ///
    /// # Arguments
    ///
    /// * `file_contents`: A vector of file contents to add to the archive.
    async fn add_files<F, B>(&mut self, file_contents: Vec<FileContent<F, B>>) -> crate::Result<()>
    where
        F: Into<String>,
        B: AsRef<[u8]>,
    {
        for fc in file_contents {
            self.add_file(fc).await?;
        }
        Ok(())
    }
}
