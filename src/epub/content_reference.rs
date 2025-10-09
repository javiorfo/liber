#[derive(Debug, Clone)]
pub struct ContentReference {
    pub title: String,
    pub subcontent_references: Option<Vec<ContentReference>>,
}

impl ContentReference {
    pub fn new<S: ToString>(title: S) -> Self {
        Self {
            title: title.to_string(),
            subcontent_references: None,
        }
    }

    pub fn nest(mut self, content_reference: ContentReference) -> Self {
        if let Some(ref mut subcontent_references) = self.subcontent_references {
            subcontent_references.push(content_reference);
        } else {
            self.subcontent_references = Some(vec![content_reference]);
        }
        self
    }

    pub(crate) fn level(&self) -> usize {
        self.subcontent_references
            .as_ref()
            .map_or(0, |subcontent_references| {
                1 + subcontent_references[0].level()
            })
    }
}
