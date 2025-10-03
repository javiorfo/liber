use std::io::Cursor;

use quick_xml::{Reader, Writer, events::Event};

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
