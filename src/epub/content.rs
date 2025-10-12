use crate::{
    epub::ContentReference,
    output::{file_content::FileContent, xml},
};

#[derive(Debug, Clone)]
pub enum ReferenceType {
    Acknowledgements(String),
    Bibliography(String),
    Colophon(String),
    Copyright(String),
    Cover(String),
    Dedication(String),
    Epigraph(String),
    Foreword(String),
    Glossary(String),
    Index(String),
    Loi(String),
    Lot(String),
    Notes(String),
    Preface(String),
    Text(String),
    TitlePage(String),
    Toc(String),
}

impl ReferenceType {
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

#[derive(Debug, Clone)]
pub struct Content<'a> {
    body: &'a [u8],
    pub(crate) reference_type: ReferenceType,
    pub(crate) subcontents: Option<Vec<Content<'a>>>,
    pub(crate) content_references: Option<Vec<ContentReference>>,
    filename: Option<String>,
}

impl<'a> Content<'a> {
    fn new(body: &'a [u8], reference_type: ReferenceType) -> Self {
        Self {
            body,
            reference_type,
            subcontents: None,
            content_references: None,
            filename: None,
        }
    }

    pub(crate) fn level(&self) -> usize {
        self.subcontents
            .as_ref()
            .map_or(0, |subcontents| 1 + subcontents[0].level())
    }

    pub(crate) fn level_reference_content(&self) -> usize {
        let content_references_level = self
            .content_references
            .as_ref()
            .map_or(0, |content_references| 1 + content_references[0].level());

        let subcontents_cont_ref_level = self.subcontents.as_ref().map_or(0, |subcontents| {
            1 + subcontents[0].level_reference_content()
        });

        content_references_level.max(subcontents_cont_ref_level)
    }

    pub(crate) fn file_content(
        &self,
        number: &mut usize,
        add_stylesheet: bool,
    ) -> crate::Result<Vec<FileContent<String, String>>> {
        *number += 1;
        let filepath = format!("OEBPS/{}", self.filename(*number));
        let mut file_contents = Vec::new();

        let xhtml_content =
            xml::format(&self.xhtml(std::str::from_utf8(self.body)?, add_stylesheet))?;

        file_contents.push(FileContent::new(filepath.to_string(), xhtml_content));

        if let Some(ref subcontents) = self.subcontents {
            for content in subcontents {
                let contents = content.file_content(number, add_stylesheet)?;
                file_contents.extend(contents);
            }
        }
        Ok(file_contents)
    }

    #[cfg(feature = "async")]
    pub(crate) async fn async_file_content(
        &self,
        number: &mut usize,
        add_stylesheet: bool,
    ) -> crate::Result<Vec<FileContent<String, String>>> {
        *number += 1;
        let filepath = format!("OEBPS/{}", self.filename(*number));
        let mut file_contents = Vec::new();

        let xhtml_content =
            xml::async_format(self.xhtml(std::str::from_utf8(self.body)?, add_stylesheet)).await?;

        file_contents.push(FileContent::new(filepath.to_string(), xhtml_content));

        if let Some(ref subcontents) = self.subcontents {
            for content in subcontents {
                let contents = content.file_content(number, add_stylesheet)?;
                file_contents.extend(contents);
            }
        }
        Ok(file_contents)
    }

    pub(crate) fn filename(&self, number: usize) -> String {
        if let Some(ref filename) = self.filename {
            filename.clone()
        } else {
            format!("c{number:02}.xhtml")
        }
    }

    pub(crate) fn title(&self) -> &str {
        self.reference_type.type_and_title().1
    }

    fn xhtml(&self, text: &str, add_stylesheet: bool) -> String {
        let stylesheet = if add_stylesheet {
            r#"<link href="style.css" rel="stylesheet" type="text/css"/>"#
        } else {
            ""
        };

        format!(
            r#"<?xml version="1.0" encoding="utf-8"?><!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
            <html xmlns="http://www.w3.org/1999/xhtml"><head><title>{}</title>{}</head>{}</html>"#,
            self.title(),
            stylesheet,
            text
        )
    }
}

#[derive(Debug)]
pub struct ContentBuilder<'a>(Content<'a>);

impl<'a> ContentBuilder<'a> {
    #[must_use]
    pub fn new(body: &'a [u8], reference_type: ReferenceType) -> Self {
        Self(Content::new(body, reference_type))
    }

    pub fn add_child(mut self, content: Content<'a>) -> Self {
        if let Some(ref mut subcontents) = self.0.subcontents {
            subcontents.push(content);
        } else {
            self.0.subcontents = Some(vec![content]);
        }
        self
    }

    pub fn add_children(mut self, contents: Vec<Content<'a>>) -> Self {
        if let Some(ref mut subcontents) = self.0.subcontents {
            subcontents.extend(contents);
        } else {
            self.0.subcontents = Some(contents);
        }
        self
    }

    pub fn add_content_reference(mut self, content_reference: ContentReference) -> Self {
        if let Some(ref mut content_references) = self.0.content_references {
            content_references.push(content_reference);
        } else {
            self.0.content_references = Some(vec![content_reference]);
        }
        self
    }

    pub fn add_content_references(mut self, content_references: Vec<ContentReference>) -> Self {
        if let Some(ref mut self_content_references) = self.0.content_references {
            self_content_references.extend(content_references);
        } else {
            self.0.content_references = Some(content_references);
        }
        self
    }

    pub fn filename<S: Into<String>>(mut self, name: S) -> Self {
        self.0.filename = Some(name.into());
        self
    }

    pub fn build(self) -> Content<'a> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_content(body: &'static str, title: &'static str) -> Content<'static> {
        ContentBuilder::new(body.as_bytes(), ReferenceType::Text(title.to_string())).build()
    }

    fn make_cr(title: &'static str) -> ContentReference {
        ContentReference::new(title)
    }

    #[test]
    fn test_content_builder_add_child() {
        let parent_body = b"parent";
        let child_content = make_content("child", "Child Title");

        let parent_content =
            ContentBuilder::new(parent_body, ReferenceType::Text("Parent".to_string()))
                .add_child(child_content.clone())
                .build();

        let subs = parent_content.subcontents.unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].body, b"child");
    }

    #[test]
    fn test_content_builder_add_children() {
        let parent_body = b"parent";
        let children = vec![make_content("child1", "C1"), make_content("child2", "C2")];

        let parent_content =
            ContentBuilder::new(parent_body, ReferenceType::TitlePage("Parent".to_string()))
                .add_children(children)
                .build();

        assert_eq!(parent_content.subcontents.unwrap().len(), 2);
    }

    #[test]
    fn test_content_builder_add_content_reference() {
        let content_ref = make_cr("Reference 1");

        let content = ContentBuilder::new(b"", ReferenceType::Text("T".to_string()))
            .add_content_reference(content_ref.clone())
            .build();

        let refs = content.content_references.unwrap();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].title, "Reference 1");
    }

    #[test]
    fn test_content_builder_add_content_references() {
        let refs = vec![make_cr("R1"), make_cr("R2")];

        let content = ContentBuilder::new(b"", ReferenceType::Text("T".to_string()))
            .add_content_references(refs)
            .filename("filename1")
            .build();

        assert_eq!(content.content_references.unwrap().len(), 2);
    }

    #[test]
    fn test_content_level_no_subcontents() {
        let content = make_content("body", "Leaf");
        assert_eq!(content.level(), 0);
    }

    #[test]
    fn test_content_level_one_deep() {
        let child = make_content("child", "C");
        let parent = ContentBuilder::new(b"parent", ReferenceType::Preface("P".to_string()))
            .add_child(child)
            .build();
        assert_eq!(parent.level(), 1);
    }

    #[test]
    fn test_content_level_two_deep() {
        let grandchild = make_content("gc", "GC");
        let child = ContentBuilder::new(b"c", ReferenceType::Preface("C".to_string()))
            .add_child(grandchild)
            .build();
        let parent = ContentBuilder::new(b"p", ReferenceType::TitlePage("P".to_string()))
            .add_child(child)
            .build();
        assert_eq!(parent.level(), 2);
    }

    #[test]
    fn test_level_reference_content_only_content_references() {
        let deep_cr = make_cr("Deep CR").add_child(make_cr("Sub"));

        let content = ContentBuilder::new(b"", ReferenceType::Text("T".to_string()))
            .add_content_reference(deep_cr)
            .build();

        assert_eq!(content.level_reference_content(), 2);
    }

    #[test]
    fn test_level_reference_content_only_subcontents() {
        let child_cr = make_cr("Child CR");
        let child = ContentBuilder::new(b"c", ReferenceType::Text("C".to_string()))
            .add_content_reference(child_cr)
            .build();

        let parent = ContentBuilder::new(b"p", ReferenceType::Text("P".to_string()))
            .add_child(child)
            .build();

        assert_eq!(parent.level_reference_content(), 2);
    }

    #[test]
    fn test_level_reference_content_mixed_max_from_subcontents() {
        let parent_cr = make_cr("P CR");
        let deep_child_cr = make_cr("DCR").add_child(make_cr("Sub"));
        let child = ContentBuilder::new(b"c", ReferenceType::Text("C".to_string()))
            .add_content_reference(deep_child_cr)
            .build();

        let parent = ContentBuilder::new(b"p", ReferenceType::Text("P".to_string()))
            .add_content_reference(parent_cr)
            .add_child(child)
            .build();

        assert_eq!(parent.level_reference_content(), 3);
    }

    #[test]
    fn test_content_xhtml_no_stylesheet() {
        let content = make_content("<body>Content</body>", "Test");
        let expected = r#"<?xml version="1.0" encoding="utf-8"?><!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
            <html xmlns="http://www.w3.org/1999/xhtml"><head><title>Test</title></head><body>Content</body></html>"#;
        assert_eq!(content.xhtml("<body>Content</body>", false), expected);
    }

    #[test]
    fn test_content_xhtml_with_stylesheet() {
        let content = make_content("<body>Content</body>", "Test");
        let expected = r#"<?xml version="1.0" encoding="utf-8"?><!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
            <html xmlns="http://www.w3.org/1999/xhtml"><head><title>Test</title><link href="style.css" rel="stylesheet" type="text/css"/></head><body>Content</body></html>"#;
        assert_eq!(content.xhtml("<body>Content</body>", true), expected);
    }

    #[test]
    fn test_content_file_content_no_subcontents() {
        let content = make_content("body text", "Chapter 1");
        let mut number = 0;
        let files = content.file_content(&mut number, false).unwrap();

        assert_eq!(number, 1);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].filepath, "OEBPS/c01.xhtml");

        assert!(files[0].bytes.contains("<title>Chapter 1</title>"));
        assert!(files[0].bytes.contains("body text"));
    }

    #[test]
    fn test_content_file_content_with_subcontents() {
        let child1 = make_content("c1", "Section 1.1");
        let child2 = make_content("c2", "Section 1.2");
        let parent = ContentBuilder::new(b"p", ReferenceType::Text("Chapter 1".to_string()))
            .add_child(child1)
            .add_child(child2)
            .build();

        let mut number = 0;
        let files = parent.file_content(&mut number, false).unwrap();

        assert_eq!(number, 3);
        assert_eq!(files.len(), 3);

        assert_eq!(files[0].filepath, "OEBPS/c01.xhtml");
        assert_eq!(files[1].filepath, "OEBPS/c02.xhtml");
        assert_eq!(files[2].filepath, "OEBPS/c03.xhtml");

        assert!(files[0].bytes.contains("<title>Chapter 1</title>"));
        assert!(files[1].bytes.contains("<title>Section 1.1</title>"));
        assert!(files[2].bytes.contains("<title>Section 1.2</title>"));
    }
}
