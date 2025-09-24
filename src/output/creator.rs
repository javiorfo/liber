use std::io::{Cursor, Write};

use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

use crate::{
    epub::{Epub, Image, Stylesheet},
    output::xhtml,
};

pub fn create<W: Write>(epub: Epub, writer: &mut W) -> crate::Result {
    let mut zip_writer = ZipWriter::new(Cursor::new(Vec::new()));

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);

    let (container_file, container_content) = xhtml::container();
    zip_writer.start_file(container_file, options)?;
    zip_writer.write_all(container_content)?;

    let (mimetype_file, mimetype_content) = xhtml::mimetype();
    zip_writer.start_file(mimetype_file, options)?;
    zip_writer.write_all(mimetype_content)?;

    let (display_options_file, display_options_content) = xhtml::display_options();
    zip_writer.start_file(display_options_file, options)?;
    zip_writer.write_all(display_options_content)?;

    if let Some(stylesheet) = epub.stylesheet {
        zip_writer.start_file(Stylesheet::FILE, options)?;
        zip_writer.write_all(stylesheet.content()?.as_bytes())?;
    }

    if let Some(cover_image) = epub.cover_image {
        let filename = cover_image
            .file_name()
            .ok_or(crate::Error::FilenameNotFound(String::from("cover image")))?;
        zip_writer.start_file(format!("{}/{}", Image::PATH, filename), options)?;
        zip_writer.write_all(&cover_image.content()?)?;
    }

    let buffer = zip_writer.finish()?;
    writer.write_all(&buffer.into_inner())?;

    Ok(())
}
