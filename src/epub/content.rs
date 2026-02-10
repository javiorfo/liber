use crate::{
    epub::ContentReference,
    output::{file_content::FileContent, xml},
};

/// Defines the **semantically meaningful type** and **display title** for a piece of content.
///
/// Each variant carries a `String` which serves as the **display title** (e.g., "Chapter 1", "Glossary").
/// The variant name itself maps to a machine-readable type string (e.g., `toc`, `foreword`).
#[derive(Debug, Clone)]
pub enum ReferenceType {
    /// Content preceding the main text, like a thank you section.
    Acknowledgements(String),
    /// A list of sources or works consulted.
    Bibliography(String),
    /// A page containing publishing information and details.
    Colophon(String),
    /// The copyright notice page.
    Copyright(String),
    /// The cover image or page content.
    Cover(String),
    /// A dedication page.
    Dedication(String),
    /// A short quotation at the beginning of a book or chapter.
    Epigraph(String),
    /// A preliminary introduction to the book, usually written by someone other than the author.
    Foreword(String),
    /// A list of terms and their definitions.
    Glossary(String),
    /// A list of names, subjects, etc., with references to where they occur.
    Index(String),
    /// List of Illustrations (LOI).
    Loi(String),
    /// List of Tables (LOT).
    Lot(String),
    /// Section for end-notes or footnotes.
    Notes(String),
    /// An introductory statement or essay, usually written by the author.
    Preface(String),
    /// The main, continuous textual content of the book.
    Text(String),
    /// The dedicated title page content.
    TitlePage(String),
    /// The Table of Contents (TOC).
    Toc(String),
}

impl ReferenceType {
    /// Retrieves the tuple containing the machine-readable **type string** and the **display title**.
    ///
    /// The type string is used for standard structural semantics in formats like EPUB.
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

/// Represents a single hierarchical content unit within a document structure.
///
/// This structure can hold raw XHTML body bytes, be nested via `subcontents`,
/// and reference other content units via `content_references`.
#[derive(Debug, Clone)]
pub struct Content<'a> {
    /// A byte slice containing the raw body of the content (assumed to be XHTML fragments).
    body: &'a [u8],
    /// The semantic type and display title of this content unit.
    pub(crate) reference_type: ReferenceType,
    /// An optional vector of children, enabling hierarchical (chapter/section) nesting.
    pub(crate) subcontents: Option<Vec<Content<'a>>>,
    /// An optional vector of references to other content units (e.g., links in a TOC).
    pub(crate) content_references: Option<Vec<ContentReference>>,
    /// An optional, user-defined filename. If `None`, a sequential name is generated.
    filename: Option<String>,
}

impl<'a> Content<'a> {
    /// Creates a new `Content` instance with mandatory fields and uninitialized optional fields.
    fn new(body: &'a [u8], reference_type: ReferenceType) -> Self {
        Self {
            body,
            reference_type,
            subcontents: None,
            content_references: None,
            filename: None,
        }
    }

    /// Recursively calculates the maximum nesting depth of **subcontents**.
    ///
    /// Returns `0` for leaf nodes.
    pub(crate) fn level(&self) -> usize {
        self.subcontents
            .as_ref()
            .map_or(0, |subcontents| 1 + subcontents[0].level())
    }

    /// Recursively calculates the maximum nesting depth considering both **subcontents** and **content references**.
    ///
    /// This is typically used for determining the necessary depth of the final document structure (e.g., NCX/TOC).
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

    /// Recursively converts this content unit and all subcontents into a vector of [`FileContent`] structs.
    ///
    /// This handles serialization to final XHTML files and assigns sequential filenames.
    ///
    /// # Arguments
    /// * `number`: A mutable counter to generate sequential filenames.
    /// * `add_stylesheet`: Flag to include a CSS link in the generated XHTML header.
    ///
    /// # Errors
    /// Returns a [`crate::Result`] if the body is not valid UTF-8 or if XML formatting fails.
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

    /// Asynchronously converts content and subcontents into a vector of [`FileContent`] structs.
    ///
    /// This method requires the **`async` feature** to be enabled.
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

    /// Gets the final output filename for this content unit.
    ///
    /// If `filename` is set, it uses that; otherwise, it formats a sequential name like `c01.xhtml`.
    pub(crate) fn filename(&self, number: usize) -> String {
        if let Some(ref filename) = self.filename {
            filename.clone()
        } else {
            format!("c{number:02}.xhtml")
        }
    }

    /// Gets the display title of this content unit from its `ReferenceType`.
    pub(crate) fn title(&self) -> &str {
        self.reference_type.type_and_title().1
    }

    /// Wraps the content body and necessary boilerplate into a complete XHTML 1.1 document string.
    fn xhtml(&self, text: &str, add_stylesheet: bool) -> String {
        if !text.starts_with(r#"<?xml version="1.0" encoding="utf-8"?>"#) {
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
        } else {
            text.to_string()
        }
    }
}

/// A builder for creating and configuring hierarchical [`Content`] structures.
///
/// This provides a **fluent interface** to manage children and references.
#[derive(Debug)]
pub struct ContentBuilder<'a>(Content<'a>);

impl<'a> ContentBuilder<'a> {
    /// Creates a new builder instance, initializing the content with the raw body and required type.
    #[must_use]
    pub fn new(body: &'a [u8], reference_type: ReferenceType) -> Self {
        Self(Content::new(body, reference_type))
    }

    /// Adds a single [`Content`] unit as a **child** (subcontent) of the current unit.
    pub fn add_child(mut self, content: Content<'a>) -> Self {
        if let Some(ref mut subcontents) = self.0.subcontents {
            subcontents.push(content);
        } else {
            self.0.subcontents = Some(vec![content]);
        }
        self
    }

    /// Adds a vector of [`Content`] units as **children** (subcontents) of the current unit.
    pub fn add_children(mut self, contents: Vec<Content<'a>>) -> Self {
        if let Some(ref mut subcontents) = self.0.subcontents {
            subcontents.extend(contents);
        } else {
            self.0.subcontents = Some(contents);
        }
        self
    }

    /// Adds a single [`ContentReference`] to the current unit's reference list.
    ///
    /// A content reference usually points to another content unit in the document tree.
    pub fn add_content_reference(mut self, content_reference: ContentReference) -> Self {
        if let Some(ref mut content_references) = self.0.content_references {
            content_references.push(content_reference);
        } else {
            self.0.content_references = Some(vec![content_reference]);
        }
        self
    }

    /// Adds a vector of [`ContentReference`] structs to the current unit's reference list.
    pub fn add_content_references(mut self, content_references: Vec<ContentReference>) -> Self {
        if let Some(ref mut self_content_references) = self.0.content_references {
            self_content_references.extend(content_references);
        } else {
            self.0.content_references = Some(content_references);
        }
        self
    }

    /// Sets a custom **filename** for the final output file corresponding to this content unit.
    pub fn filename<S: Into<String>>(mut self, name: S) -> Self {
        self.0.filename = Some(name.into());
        self
    }

    /// Consumes the builder and returns the final [`Content`] instance.
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
