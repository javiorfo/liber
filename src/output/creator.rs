use std::io::{Cursor, Write};

use zip::{
    CompressionMethod, ZipWriter,
    write::{FileOptions, SimpleFileOptions},
};

use crate::{
    epub::Epub,
    output::file_content::{self, FileContent},
};

#[derive(Debug)]
pub struct EpubFile<'a, W: Write> {
    epub: Epub<'a>,
    options: FileOptions<'a, ()>,
    writer: W,
    zip_writer: ZipWriter<Cursor<Vec<u8>>>,
}

impl<'a, W: Write> EpubFile<'a, W> {
    pub fn new(epub: Epub<'a>, writer: W) -> EpubFile<'a, W> {
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
        self.add_file(file_content::container())?;
        self.add_file(file_content::mimetype())?;
        self.add_file(file_content::display_options())?;

        let mut add_stylesheet = false;
        if let Some(ref stylesheet) = self.epub.stylesheet {
            self.add_file(stylesheet.file_content())?;
            add_stylesheet = true;
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

        let mut file_number: usize = 0;
        if let Some(ref contents) = self.epub.contents {
            let mut file_contents: Vec<FileContent<String, Vec<u8>>> = Vec::new();
            for content in contents {
                let res = content.file_content(&mut file_number, add_stylesheet)?;
                file_contents.extend(res);
            }

            self.add_files(file_contents)?;
        }

        self.add_file(file_content::content_opf(&self.epub, file_number)?)?;
        self.add_file(file_content::toc_ncx(&self.epub)?)?;

        let buffer = self.zip_writer.finish()?;
        self.writer.write_all(&buffer.into_inner())?;

        Ok(())
    }

    fn add_file<F: ToString, B: AsRef<[u8]>>(
        &mut self,
        file_content: FileContent<F, B>,
    ) -> crate::Result {
        self.zip_writer
            .start_file(file_content.filepath, self.options)?;
        self.zip_writer.write_all(file_content.bytes.as_ref())?;
        Ok(())
    }

    fn add_files<F: ToString, B: AsRef<[u8]>>(
        &mut self,
        file_contents: Vec<FileContent<F, B>>,
    ) -> crate::Result {
        for fc in file_contents {
            self.add_file(fc)?;
        }
        Ok(())
    }
}
