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

#[derive(Debug, Clone)]
pub enum ZipCompression {
    Deflated,
    Stored,
}

#[derive(Debug)]
pub struct EpubFile<'a, W> {
    epub: Epub<'a>,
    options: FileOptions<'a, ()>,
    writer: W,
    zip_writer: ZipWriter<Cursor<Vec<u8>>>,
}

impl<'a, W> EpubFile<'a, W>
where
    W: Write,
{
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

    pub fn create(mut self) -> crate::Result {
        self.add_file(file_content::mimetype())?;
        self.add_file(file_content::container())?;
        self.add_file(file_content::display_options())?;

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

        if let Some(ref contents) = self.epub.contents {
            let mut file_number: usize = 0;
            let mut file_contents: Vec<FileContent<String, String>> = Vec::new();
            for content in contents {
                let res = content.file_content(&mut file_number, self.epub.stylesheet.is_some())?;
                file_contents.extend(res);
            }

            self.add_files(file_contents)?;
        }

        let mut content_opf = file_content::content_opf(&self.epub)?;
        content_opf.format(xml::format(&content_opf.bytes)?);
        self.add_file(content_opf)?;

        let mut toc_ncx = file_content::toc_ncx(&self.epub)?;
        toc_ncx.format(xml::format(&toc_ncx.bytes)?);
        self.add_file(toc_ncx)?;

        let buffer = self.zip_writer.finish()?;
        self.writer.write_all(&buffer.into_inner())?;

        Ok(())
    }

    fn add_file<F, B>(&mut self, file_content: FileContent<F, B>) -> crate::Result
    where
        F: ToString,
        B: AsRef<[u8]>,
    {
        self.zip_writer
            .start_file(file_content.filepath, self.options)?;
        self.zip_writer.write_all(file_content.bytes.as_ref())?;
        Ok(())
    }

    fn add_files<F, B>(&mut self, file_contents: Vec<FileContent<F, B>>) -> crate::Result
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
