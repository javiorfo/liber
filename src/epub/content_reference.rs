/// Represents a single entry in a hierarchical list of references (e.g., a Table of Contents entry).
///
/// This structure links a title to a specific location (via `id`) and supports nested sub-references.
#[derive(Debug, Clone)]
pub struct ContentReference {
    /// The display title for this reference entry (e.g., "Section 1.1: The Beginning").
    pub(crate) title: String,
    /// An optional vector of children, creating nested sub-entries (e.g., sections within a chapter).
    pub(crate) subcontent_references: Option<Vec<ContentReference>>,
    /// An optional, user-defined ID corresponding to an anchor within a content file.
    /// If `None`, a sequential ID will be generated when building the output structure.
    id: Option<String>,
}

impl ContentReference {
    /// Creates a new `ContentReference` with the mandatory display **title**.
    ///
    /// # Arguments
    /// * `title`: The string that will be displayed for this entry in the navigation structure.
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            title: title.into(),
            subcontent_references: None,
            id: None,
        }
    }

    /// Sets the **anchor ID** (the target fragment, e.g., `#section-1`) for this reference.
    ///
    /// This is a fluent method, returning `Self`.
    pub fn id<S: Into<String>>(mut self, name: S) -> Self {
        self.id = Some(name.into());
        self
    }

    /// Adds a single [`ContentReference`] as a nested **child** (sub-entry).
    ///
    /// This is a fluent method, returning `Self`.
    pub fn add_child(mut self, content_reference: ContentReference) -> Self {
        if let Some(ref mut subcontent_references) = self.subcontent_references {
            subcontent_references.push(content_reference);
        } else {
            self.subcontent_references = Some(vec![content_reference]);
        }
        self
    }

    /// Adds a vector of [`ContentReference`] structs as nested **children**.
    ///
    /// This is a fluent method, returning `Self`.
    pub fn add_children(mut self, content_references: Vec<ContentReference>) -> Self {
        if let Some(ref mut subcontent_references) = self.subcontent_references {
            subcontent_references.extend(content_references);
        } else {
            self.subcontent_references = Some(content_references);
        }
        self
    }

    /// Recursively calculates the maximum nesting depth of subcontent references.
    ///
    /// Returns `0` for leaf nodes.
    pub(crate) fn level(&self) -> usize {
        self.subcontent_references
            .as_ref()
            .map_or(0, |subcontent_references| {
                1 + subcontent_references[0].level()
            })
    }

    /// Generates the full file-path anchor string for this reference.
    ///
    /// It combines the provided XHTML filename with either the custom `id` or a sequential one.
    ///
    /// # Arguments
    /// * `xhtml`: The base filename (e.g., `c01.xhtml`) this reference points to.
    /// * `number`: A sequential number used for generating a default anchor ID if `self.id` is `None`.
    pub(crate) fn reference_name(&self, xhtml: &str, number: usize) -> String {
        if let Some(ref id) = self.id {
            format!("{xhtml}#{id}")
        } else {
            format!("{xhtml}#id{number:02}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cr<S: Into<String>>(title: S) -> ContentReference {
        ContentReference::new(title)
    }

    #[test]
    fn test_add_subcontent_reference_initial() {
        let parent_title = "Section A";
        let child_title = "Subsection A.1";

        let child_ref = cr(child_title);
        let parent_ref = cr(parent_title).add_child(child_ref.clone());

        assert!(parent_ref.subcontent_references.is_some());

        let subs = parent_ref.subcontent_references.as_ref().unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].title, child_title);
    }

    #[test]
    fn test_add_subcontent_reference_multiple() {
        let parent_title = "Book";
        let child1_title = "Chapter 1";
        let child2_title = "Chapter 2";

        let child1_ref = cr(child1_title);
        let child2_ref = cr(child2_title);

        let parent_ref = cr(parent_title).add_child(child1_ref).add_child(child2_ref);

        let subs = parent_ref.subcontent_references.as_ref().unwrap();
        assert_eq!(subs.len(), 2);

        assert_eq!(subs[0].title, child1_title);
        assert_eq!(subs[1].title, child2_title);
    }

    #[test]
    fn test_level_no_subcontent() {
        let reference = cr("Leaf Node");
        assert_eq!(reference.level(), 0);
    }

    #[test]
    fn test_level_one_deep() {
        let child = cr("Child");
        let parent = cr("Parent").add_child(child);

        assert_eq!(parent.level(), 1);
    }

    #[test]
    fn test_level_two_deep() {
        let child = cr("Child");
        let parent = cr("Parent").add_child(child);
        let grandparent = cr("Grandparent").add_child(parent);

        assert_eq!(grandparent.level(), 2);
    }

    #[test]
    fn test_level_sibling_doesnt_change_depth() {
        let child1 = cr("Child1");
        let child2 = cr("Child2");
        let root = cr("Root").add_child(child1).add_child(child2);

        assert_eq!(root.level(), 1);
    }

    #[test]
    fn test_level_mixed_depth_only_first_matters() {
        let sub_deep = cr("SubDeep");
        let deep_child = cr("DeepChild").add_child(sub_deep);
        let shallow_child = cr("ShallowChild");

        let root = cr("Root").add_child(deep_child).add_child(shallow_child);

        assert_eq!(root.level(), 2);

        let sub_deep_2 = cr("SubDeep_2");
        let deep_child_2 = cr("DeepChild_2").add_child(sub_deep_2);
        let shallow_child_2 = cr("ShallowChild_2");

        let root_2 = cr("Root_2")
            .add_child(shallow_child_2)
            .add_child(deep_child_2);

        assert_eq!(root_2.level(), 1);
    }
}
