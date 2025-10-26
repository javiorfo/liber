use std::io::{Cursor, Write};

use zip::{
    CompressionMethod, ZipWriter,
    write::{FileOptions, SimpleFileOptions},
};

use crate::{
    epub::Epub,
    output::{
        file_content::{self, FileContent},
        xml,
    },
};

/// Defines the compression method used when creating the EPUB ZIP archive.
#[derive(Debug, Clone, Default)]
pub enum ZipCompression {
    /// Use **Deflated** compression. This is generally preferred for smaller file sizes.
    Deflated,
    /// Use **Stored** compression (no compression). This is mandatory for the
    /// `mimetype` file according to EPUB specifications.
    #[default]
    Stored,
}

/// A builder responsible for creating and writing all components of an EPUB book
/// into a standard ZIP archive format.
///
/// This struct manages the final serialization step, taking the high-level
/// `Epub` data structure and writing all necessary files (`.opf`, `.ncx`, `.xhtml`, etc.)
/// to an underlying writer.
#[derive(Debug)]
pub struct EpubFile<'a, W> {
    /// The source data structure containing all metadata and content of the EPUB.
    epub: Epub<'a>,
    /// The file options (including compression method) used for writing files into the ZIP archive.
    options: FileOptions<'a, ()>,
    /// The external writer where the final compressed EPUB bytes will be written to.
    writer: W,
    /// The internal ZIP writer, buffering the content before flushing to `self.writer`.
    zip_writer: ZipWriter<Cursor<Vec<u8>>>,
}

impl<'a, W> EpubFile<'a, W>
where
    W: Write + Send,
{
    /// Creates a new `EpubFile` builder.
    ///
    /// This sets up the internal ZIP writer and configures the file options
    /// based on the chosen compression method.
    ///
    /// # Arguments
    ///
    /// * `epub`: The EPUB data structure to be written.
    /// * `writer`: The output stream (e.g., a `File` or `Vec<u8>`) where the final `.epub` bytes will go.
    /// * `compression`: The default compression method to use for the files inside the ZIP archive.
    pub fn new(epub: Epub<'a>, writer: W, compression: ZipCompression) -> EpubFile<'a, W> {
        let compression = match compression {
            ZipCompression::Stored => CompressionMethod::Stored,
            ZipCompression::Deflated => CompressionMethod::Deflated,
        };

        Self {
            epub,
            writer,
            options: SimpleFileOptions::default()
                .compression_method(compression)
                .unix_permissions(0o755),
            zip_writer: ZipWriter::new(Cursor::new(Vec::new())),
        }
    }

    /// Generates all necessary EPUB files, zips them up, and writes the final
    /// archive to the output writer provided during initialization.
    ///
    /// The process involves:
    /// 1. Adding mandatory fixed files (`mimetype`, `container.xml`).
    /// 2. Adding optional files (stylesheet, cover image, generic resources).
    /// 3. Generating and adding all content XHTML files.
    /// 4. Generating, formatting, and adding the central XML files (`content.opf` and `toc.ncx`).
    /// 5. Finalizing the internal ZIP archive and writing the resulting bytes to the
    ///    external `writer`.
    ///
    /// # Returns
    ///
    /// Returns `crate::Result<()>` indicating success or failure in any step
    /// (file generation, XML formatting, or ZIP writing).
    pub fn create(mut self) -> crate::Result<()> {
        // 1. Add mandatory files
        self.add_file(file_content::mimetype())?;
        self.add_file(file_content::container())?;
        self.add_file(file_content::display_options())?;

        // 2. Add optional files (stylesheet, cover image, resources)
        if let Some(stylesheet) = self.epub.stylesheet {
            self.add_file(FileContent::new("OEBPS/style.css", stylesheet))?;
        }

        if let Some(ref cover_image) = self.epub.cover_image {
            self.add_file(cover_image.file_content()?)?;
        }

        if let Some(ref resources) = self.epub.resources {
            let contents = resources
                .iter()
                .map(|resource| resource.file_content())
                .collect::<crate::Result<Vec<FileContent<String, Vec<u8>>>>>()?;

            self.add_files(contents)?;
        }

        // 3. Generate and add content XHTML files
        if let Some(ref contents) = self.epub.contents {
            let mut file_number: usize = 0;
            let mut file_contents: Vec<FileContent<String, String>> = Vec::new();
            for content in contents {
                let res = content.file_content(&mut file_number, self.epub.stylesheet.is_some())?;
                file_contents.extend(res);
            }

            self.add_files(file_contents)?;
        }

        // 4. Generate, format, and add OPF and NCX files
        let mut content_opf = file_content::content_opf(&self.epub)?;
        content_opf.format(xml::format(&content_opf.bytes)?);
        self.add_file(content_opf)?;

        let mut toc_ncx = file_content::toc_ncx(&self.epub)?;
        toc_ncx.format(xml::format(&toc_ncx.bytes)?);
        self.add_file(toc_ncx)?;

        // 5. Finalize ZIP and flush to external writer
        let buffer = self.zip_writer.finish()?;
        self.writer.write_all(&buffer.into_inner())?;

        Ok(())
    }

    /// Adds a single `FileContent` item to the internal ZIP archive.
    ///
    /// This starts a new file entry in the ZIP using the configured compression
    /// options and writes the file's content bytes.
    ///
    /// # Arguments
    ///
    /// * `file_content`: The structure holding the file path and content bytes.
    fn add_file<F, B>(&mut self, file_content: FileContent<F, B>) -> crate::Result<()>
    where
        F: ToString,
        B: AsRef<[u8]>,
    {
        self.zip_writer
            .start_file(file_content.filepath.to_string(), self.options)?;
        self.zip_writer.write_all(file_content.bytes.as_ref())?;
        Ok(())
    }

    /// Adds a vector of `FileContent` items to the internal ZIP archive.
    ///
    /// # Arguments
    ///
    /// * `file_contents`: A vector of file contents to add to the archive.
    fn add_files<F, B>(&mut self, file_contents: Vec<FileContent<F, B>>) -> crate::Result<()>
    where
        F: ToString,
        B: AsRef<[u8]>,
    {
        for fc in file_contents {
            self.add_file(fc)?;
        }
        Ok(())
    }
}
