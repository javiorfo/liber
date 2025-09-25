use std::io::{Cursor, Write};

use zip::{
    CompressionMethod, ZipWriter,
    write::{FileOptions, SimpleFileOptions},
};

use crate::{
    epub::Epub,
    output::content::{self, FileContent},
};

#[derive(Debug)]
pub struct EpubFile<'a, W, S: AsRef<str>>
where
    W: Write,
{
    epub: Epub<'a, S>,
    options: FileOptions<'a, ()>,
    writer: W,
    zip_writer: ZipWriter<Cursor<Vec<u8>>>,
}

impl<'a, W: Write, S: AsRef<str>> EpubFile<'a, W, S> {
    pub fn new(epub: Epub<'a, S>, writer: W) -> EpubFile<'a, W, S> {
        Self {
            epub,
            writer,
            options: SimpleFileOptions::default()
                .compression_method(CompressionMethod::Stored)
                .unix_permissions(0o755),
            zip_writer: ZipWriter::new(Cursor::new(Vec::new())),
        }
    }

    pub fn create(mut self) -> crate::Result {
        self.add_file(content::container())?;
        self.add_file(content::mimetype())?;
        self.add_file(content::display_options())?;

        if let Some(ref stylesheet) = self.epub.stylesheet {
            self.add_file(stylesheet.content()?)?;
        }

        if let Some(ref cover_image) = self.epub.cover_image {
            self.add_file(cover_image.content()?)?;
        }

        if let Some(ref images) = self.epub.images {
            let contents = images
                .iter()
                .map(|img| img.content())
                .collect::<crate::Result<Vec<FileContent<String>>>>()?;

            self.add_files(contents)?;
        }

        self.add_file(content::content_opf(&self.epub)?)?;

        let buffer = self.zip_writer.finish()?;
        self.writer.write_all(&buffer.into_inner())?;

        Ok(())
    }

    fn add_file<P: ToString>(&mut self, file_content: FileContent<P>) -> crate::Result {
        self.zip_writer
            .start_file(file_content.filepath, self.options)?;
        self.zip_writer.write_all(&file_content.bytes)?;
        Ok(())
    }

    fn add_files<P: ToString>(&mut self, file_contents: Vec<FileContent<P>>) -> crate::Result {
        for fc in file_contents {
            self.zip_writer.start_file(fc.filepath, self.options)?;
            self.zip_writer.write_all(&fc.bytes)?;
        }
        Ok(())
    }
}
