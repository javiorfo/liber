use std::fmt::Display;

use uuid::Uuid;

#[derive(Debug)]
pub struct Metadata<S: AsRef<str>> {
    pub title: S,
    pub language: Language,
    pub identifier: Identifier,
    pub creator: Option<S>,
    pub contributor: Option<S>,
    pub publisher: Option<S>,
    pub date: Option<S>,
    pub subject: Option<S>,
    pub description: Option<S>,
}

impl<S: AsRef<str>> Metadata<S> {
    fn new(title: S, language: Language, identifier: Identifier) -> Self {
        Self {
            title,
            language,
            identifier,
            creator: None,
            contributor: None,
            publisher: None,
            date: None,
            subject: None,
            description: None,
        }
    }
}

#[derive(Debug)]
pub struct MetadataBuilder<S: AsRef<str>>(Metadata<S>);

impl<S: AsRef<str>> MetadataBuilder<S> {
    #[must_use]
    pub fn title(title: S) -> Self {
        Self(Metadata::new(
            title,
            Language::default(),
            Identifier::default(),
        ))
    }

    pub fn language(mut self, language: Language) -> Self {
        self.0.language = language;
        self
    }

    pub fn identifier(mut self, identifier: Identifier) -> Self {
        self.0.identifier = identifier;
        self
    }

    pub fn creator(mut self, creator: S) -> Self {
        self.0.creator = Some(creator);
        self
    }

    pub fn contributor(mut self, contributor: S) -> Self {
        self.0.contributor = Some(contributor);
        self
    }

    pub fn publisher(mut self, publisher: S) -> Self {
        self.0.publisher = Some(publisher);
        self
    }

    pub fn date(mut self, date: S) -> Self {
        self.0.date = Some(date);
        self
    }

    pub fn subject(mut self, subject: S) -> Self {
        self.0.subject = Some(subject);
        self
    }

    pub fn description(mut self, description: S) -> Self {
        self.0.description = Some(description);
        self
    }

    pub fn build(self) -> Metadata<S> {
        self.0
    }
}

#[derive(Debug, Clone, Default)]
pub enum Language {
    Arabic,
    Chinese,
    Croatian,
    Czech,
    Dutch,
    #[default]
    English,
    French,
    Greek,
    German,
    Hungarian,
    Italian,
    Japanese,
    Korean,
    Polish,
    Portuguese,
    Romanian,
    Russian,
    Slovak,
    Slovenian,
    Spanish,
    Swedish,
    Turkish,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Arabic => "ar",
            Language::Chinese => "zh",
            Language::Croatian => "hr",
            Language::Czech => "cs",
            Language::Dutch => "nl",
            Language::English => "en",
            Language::French => "fr",
            Language::Greek => "gl",
            Language::German => "de",
            Language::Hungarian => "hu",
            Language::Italian => "it",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Polish => "pl",
            Language::Portuguese => "pt",
            Language::Romanian => "ro",
            Language::Russian => "ru",
            Language::Slovak => "sk",
            Language::Slovenian => "sl",
            Language::Spanish => "es",
            Language::Swedish => "sv",
            Language::Turkish => "tr",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Identifier {
    UUID(String),
    ISBN(String),
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        match self {
            Self::UUID(value) | Self::ISBN(value) => value.as_str(),
        }
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Identifier::UUID(format!("urn:uuid:{}", Uuid::new_v4()))
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UUID(_) => write!(f, "UUID"),
            Self::ISBN(_) => write!(f, "ISBN"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn get_test_identifier() -> Identifier {
        Identifier::ISBN("978-3-16-148410-0".to_string())
    }

    #[test]
    fn test_metadata_builder_new() {
        let title = "The Rust Programming Language";
        let language = Language::English;
        let identifier = get_test_identifier();

        let metadata = MetadataBuilder::title(title)
            .language(language)
            .identifier(identifier)
            .build();

        assert_eq!(metadata.title, title);
        assert!(matches!(metadata.language, Language::English));
        assert!(matches!(metadata.identifier, Identifier::ISBN(_)));

        assert_eq!(metadata.creator, None);
        assert_eq!(metadata.publisher, None);
        assert_eq!(metadata.date, None);
        assert_eq!(metadata.subject, None);
        assert_eq!(metadata.description, None);
    }

    #[test]
    fn test_metadata_builder_full() {
        let title = "The Hitchhiker's Guide to the Galaxy";
        let language = Language::English;
        let identifier = get_test_identifier();

        let creator = "Douglas Adams";
        let publisher = "Pan Books";
        let date = "1979-10-12";
        let subject = "Science Fiction";
        let description = "A comic science fiction series created by Douglas Adams.";

        let metadata = MetadataBuilder::title(title)
            .language(language)
            .identifier(identifier)
            .creator(creator)
            .publisher(publisher)
            .date(date)
            .subject(subject)
            .description(description)
            .build();

        assert_eq!(metadata.creator, Some(creator));
        assert_eq!(metadata.contributor, None);
        assert_eq!(metadata.publisher, Some(publisher));
        assert_eq!(metadata.date, Some(date));
        assert_eq!(metadata.subject, Some(subject));
        assert_eq!(metadata.description, Some(description));
    }

    #[test]
    fn test_identifier_default_uuid() {
        let default_identifier = Identifier::default();

        match default_identifier {
            Identifier::UUID(uuid_string) => {
                assert!(uuid_string.starts_with("urn:uuid:"));

                let uuid_str = uuid_string.strip_prefix("urn:uuid:").unwrap();

                let parsed_uuid = Uuid::parse_str(uuid_str);

                assert!(parsed_uuid.is_ok());
            }
            _ => panic!("Default identifier was not a UUID"),
        }
    }
}
