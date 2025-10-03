#[derive(Debug)]
pub struct ContentReference<'a> {
    pub title: &'a str,
    pub subcontent_references: Option<Vec<ContentReference<'a>>>,
}

impl<'a> ContentReference<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            subcontent_references: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, content_reference: ContentReference<'a>) -> Self {
        if let Some(ref mut subcontent_references) = self.subcontent_references {
            subcontent_references.push(content_reference);
        } else {
            self.subcontent_references = Some(vec![content_reference]);
        }
        self
    }

    pub(crate) fn level(&self) -> usize {
        match self.subcontent_references {
            Some(ref subcontent_references) if subcontent_references.is_empty() => 0,
            Some(ref subcontent_references) => 1 + subcontent_references[0].level(),
            None => 0,
        }
    }
}
