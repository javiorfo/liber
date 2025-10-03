use crate::{
    epub::ContentReference,
    output::{file_content::FileContent, xml},
};

#[derive(Debug, Clone)]
pub enum ReferenceType<'a> {
    Acknowledgements(&'a str),
    Bibliography(&'a str),
    Colophon(&'a str),
    Copyright(&'a str),
    Cover(&'a str),
    Dedication(&'a str),
    Epigraph(&'a str),
    Foreword(&'a str),
    Glossary(&'a str),
    Index(&'a str),
    Loi(&'a str),
    Lot(&'a str),
    Notes(&'a str),
    Preface(&'a str),
    Text(&'a str),
    TitlePage(&'a str),
    Toc(&'a str),
}

impl ReferenceType<'_> {
    pub(crate) fn type_and_title(&self) -> (&str, &str) {
        match self {
            Self::Acknowledgements(s) => ("acknowledgements", s),
            Self::Bibliography(s) => ("bibliography", s),
            Self::Colophon(s) => ("colophon", s),
            Self::Copyright(s) => ("copyright-page", s),
            Self::Cover(s) => ("cover", s),
            Self::Dedication(s) => ("dedication", s),
            Self::Epigraph(s) => ("epigraph", s),
            Self::Foreword(s) => ("foreword", s),
            Self::Glossary(s) => ("glossary", s),
            Self::Index(s) => ("index", s),
            Self::Loi(s) => ("loi", s),
            Self::Lot(s) => ("lot", s),
            Self::Notes(s) => ("notes", s),
            Self::Preface(s) => ("preface", s),
            Self::Text(s) => ("text", s),
            Self::TitlePage(s) => ("title-page", s),
            Self::Toc(s) => ("toc", s),
        }
    }
}

#[derive(Debug)]
pub struct Content<'a> {
    body: &'a [u8],
    pub(crate) reference_type: ReferenceType<'a>,
    pub(crate) subcontents: Option<Vec<Content<'a>>>,
    pub(crate) content_references: Option<Vec<ContentReference<'a>>>,
}

impl<'a> Content<'a> {
    fn new(body: &'a [u8], reference_type: ReferenceType<'a>) -> Self {
        Self {
            body,
            reference_type,
            subcontents: None,
            content_references: None,
        }
    }

    pub(crate) fn level(&self) -> usize {
        match self.subcontents {
            Some(ref subcontents) if subcontents.is_empty() => 0,
            Some(ref subcontents) => 1 + subcontents[0].level(),
            None => 0,
        }
    }

    pub(crate) fn level_reference_content(&self) -> usize {
        let content_references_level = match self.content_references {
            Some(ref content_references) if content_references.is_empty() => 0,
            Some(ref content_references) => 1 + content_references[0].level(),
            None => 0,
        };

        let subcontents_cont_ref_level = match self.subcontents {
            Some(ref subcontents) if subcontents.is_empty() => 0,
            Some(ref subcontents) => 1 + subcontents[0].level_reference_content(),
            None => 0,
        };

        content_references_level.max(subcontents_cont_ref_level)
    }

    pub(crate) fn file_content(
        &self,
        number: &mut usize,
        add_stylesheet: bool,
    ) -> crate::Result<Vec<FileContent<String, Vec<u8>>>> {
        *number += 1;
        let filepath = Self::filename(*number);
        let mut file_contents = Vec::new();

        let xhtml_content = self.xhtml(std::str::from_utf8(self.body)?, add_stylesheet)?;

        file_contents.push(FileContent::new(
            filepath.to_string(),
            xhtml_content.as_bytes().to_vec(),
        ));

        if let Some(ref subcontents) = self.subcontents {
            for content in subcontents {
                let contents = content.file_content(number, add_stylesheet)?;
                file_contents.extend(contents);
            }
        }
        Ok(file_contents)
    }

    pub(crate) fn filename(number: usize) -> String {
        format!("{:02}.xhtml", number)
    }

    pub(crate) fn title(&self) -> &str {
        self.reference_type.type_and_title().1
    }

    fn xhtml(&self, text: &str, add_stylesheet: bool) -> crate::Result<String> {
        let stylesheet = if add_stylesheet {
            r#"<link href="style.css" rel="stylesheet" type="text/css"/>"#
        } else {
            ""
        };

        xml::format(&format!(
            r#"<?xml version="1.0" encoding="utf-8"?><!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
            <html xmlns="http://www.w3.org/1999/xhtml"><head><title>{}</title>{}</head>{}</html>"#,
            self.title(),
            stylesheet,
            text
        ))
    }
}

#[derive(Debug)]
pub struct ContentBuilder<'a>(Content<'a>);

impl<'a> ContentBuilder<'a> {
    #[must_use]
    pub fn new(body: &'a [u8], reference_type: ReferenceType<'a>) -> Self {
        Self(Content::new(body, reference_type))
    }

    pub fn add_subcontent(mut self, content: Content<'a>) -> Self {
        if let Some(ref mut subcontents) = self.0.subcontents {
            subcontents.push(content);
        } else {
            self.0.subcontents = Some(vec![content]);
        }
        self
    }

    pub fn subcontents(mut self, contents: Vec<Content<'a>>) -> Self {
        self.0.subcontents = Some(contents);
        self
    }

    pub fn add_content_reference(mut self, content_reference: ContentReference<'a>) -> Self {
        if let Some(ref mut content_references) = self.0.content_references {
            content_references.push(content_reference);
        } else {
            self.0.content_references = Some(vec![content_reference]);
        }
        self
    }

    pub fn content_references(mut self, content_references: Vec<ContentReference<'a>>) -> Self {
        self.0.content_references = Some(content_references);
        self
    }

    pub fn build(self) -> Content<'a> {
        self.0
    }
}
