use crate::output::file_content::FileContent;

#[derive(Debug)]
pub struct Stylesheet<'a> {
    pub body: &'a [u8],
}

impl<'a> Stylesheet<'a> {
    pub fn new(body: &'a [u8]) -> Stylesheet<'a> {
        Self { body }
    }

    pub(crate) fn file_content(&self) -> FileContent<&'a str, &'a [u8]> {
        FileContent::new("OEBPS/style.css", self.body)
    }
}
