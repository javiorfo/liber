use async_zip::{Compression, ZipEntryBuilder, tokio::write::ZipFileWriter};
use futures::future;

use crate::{
    epub::Epub,
    output::{
        file_content::{self, FileContent},
        xml,
        zip::ZipCompression,
    },
};

pub struct EpubFile<'a, W>
where
    W: tokio::io::AsyncWrite + Unpin,
{
    epub: Epub<'a>,
    zip_writer: ZipFileWriter<W>,
    compression: async_zip::Compression,
}

impl<'a, W: tokio::io::AsyncWrite + Unpin> EpubFile<'a, W> {
    pub fn new(epub: Epub<'a>, writer: W, compression: ZipCompression) -> EpubFile<'a, W> {
        let compression = match compression {
            ZipCompression::Stored => Compression::Stored,
            ZipCompression::Deflated => Compression::Deflate,
        };

        Self {
            epub,
            zip_writer: ZipFileWriter::with_tokio(writer),
            compression,
        }
    }

    pub async fn create(mut self) -> crate::Result {
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

        if let Some(ref resources) = self.epub.resources {
            let contents = resources
                .iter()
                .map(|resource| resource.async_file_content())
                .collect::<Vec<_>>();

            let contents = future::try_join_all(contents).await?;
            self.add_files(contents).await?;
        }

        let mut file_number: usize = 0;
        if let Some(ref contents) = self.epub.contents {
            let mut file_contents: Vec<FileContent<String, String>> = Vec::new();
            for content in contents {
                let res = content
                    .async_file_content(&mut file_number, self.epub.stylesheet.is_some())
                    .await?;
                file_contents.extend(res);
            }

            self.add_files(file_contents).await?;
        }

        let mut content_opf = file_content::content_opf(&self.epub, file_number)?;
        content_opf.format(xml::async_format(content_opf.bytes.clone()).await?);
        self.add_file(content_opf).await?;

        let mut toc_ncx = file_content::toc_ncx(&self.epub)?;
        toc_ncx.format(xml::async_format(toc_ncx.bytes.clone()).await?);
        self.add_file(toc_ncx).await?;

        self.zip_writer.close().await?;

        Ok(())
    }

    async fn add_file<F: ToString, B: AsRef<[u8]>>(
        &mut self,
        file_content: FileContent<F, B>,
    ) -> crate::Result {
        let builder =
            ZipEntryBuilder::new(file_content.filepath.to_string().into(), self.compression);

        self.zip_writer
            .write_entry_whole(builder, file_content.bytes.as_ref())
            .await?;
        Ok(())
    }

    async fn add_files<F: ToString, B: AsRef<[u8]>>(
        &mut self,
        file_contents: Vec<FileContent<F, B>>,
    ) -> crate::Result {
        for fc in file_contents {
            self.add_file(fc).await?;
        }
        Ok(())
    }
}
