use std::io::Cursor;

use quick_xml::{Reader, Writer, events::Event};

/// Formats an XML string, adding indentation and trimming text content.
///
/// This function uses the `quick_xml` crate to parse the input XML string
/// and then write it back out with a specified indentation (two spaces)
/// to improve readability. It also trims leading/trailing whitespace
/// from text nodes during parsing.
///
/// # Arguments
///
/// * `xml_data`: The XML content to be formatted, as a string slice (`&str`).
///
/// # Returns
///
/// Returns a `crate::Result<String>`:
/// * `Ok(String)`: The formatted XML string.
/// * `Err(crate::Error)`: If an error occurs during XML parsing or writing,
///   or if the resulting bytes are not valid UTF-8.
///
/// # Errors
///
/// The primary error is `crate::Error::XmlParser` if the input XML is invalid.
pub fn format(xml_data: &str) -> crate::Result<String> {
    let mut reader = Reader::from_str(xml_data);
    reader.config_mut().trim_text(true);

    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(event) => {
                writer.write_event(event)?;
            }
            Err(e) => return Err(crate::Error::XmlParser(reader.buffer_position(), e)),
        }
        buf.clear();
    }

    let result = writer.into_inner().into_inner();

    Ok(String::from_utf8(result)?)
}

/// Asynchronously formats an XML string by spawning the blocking
/// `format` function onto a Tokio blocking thread pool.
///
/// This is a convenience function for use in asynchronous contexts.
/// It consumes the input string and returns the formatted XML string.
///
/// This function is only compiled when the `"async"` feature is enabled.
///
/// # Arguments
///
/// * `xml_data`: The XML content to be formatted, as an owned `String`.
///
/// # Returns
///
/// Returns a `crate::Result<String>`:
/// * `Ok(String)`: The formatted XML string.
/// * `Err(crate::Error)`: If the internal `format` function fails, or
///   if the `spawn_blocking` task panics.
#[cfg(feature = "async")]
pub async fn async_format(xml_data: String) -> crate::Result<String> {
    tokio::task::spawn_blocking(move || format(&xml_data)).await?
}
