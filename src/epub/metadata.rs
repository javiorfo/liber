use std::fmt::Display;

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Metadata<'a> {
    pub title: String,
    pub language: Language,
    pub identifier: Identifier,
    pub creator: Option<&'a str>,
    pub contributor: Option<&'a str>,
    pub publisher: Option<&'a str>,
    pub date: Option<DateTime<Utc>>,
    pub subject: Option<&'a str>,
    pub description: Option<&'a str>,
}

impl<'a> Metadata<'a> {
    fn new<S: ToString>(title: S, language: Language, identifier: Identifier) -> Self {
        Self {
            title: title.to_string(),
            language,
            identifier,
            creator: None,
            contributor: None,
            publisher: None,
            date: Some(Utc::now()),
            subject: None,
            description: None,
        }
    }

    pub(crate) fn title_as_metadata_xml(&self) -> String {
        format!("<dc:title>{}</dc:title>", self.title)
    }

    pub(crate) fn creator_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            r#"<dc:creator opf:role="aut">{}</dc:creator>"#,
            self.creator?
        ))
    }

    pub(crate) fn contributor_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            r#"<dc:contributor opf:role="trl">{}</dc:contributor>"#,
            self.contributor?
        ))
    }

    pub(crate) fn publisher_as_metadata_xml(&self) -> Option<String> {
        Some(format!("<dc:publisher>{}</dc:publisher>", self.publisher?))
    }

    pub(crate) fn date_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            r#"<dc:date opf:event="publication">{}</dc:date>"#,
            self.date?.format("%Y-%m-%d")
        ))
    }

    pub(crate) fn subject_as_metadata_xml(&self) -> Option<String> {
        Some(format!("<dc:subject>{}</dc:subject>", self.subject?))
    }

    pub(crate) fn description_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            "<dc:description>{}</dc:description>",
            self.description?
        ))
    }
}

#[derive(Debug)]
pub struct MetadataBuilder<'a>(Metadata<'a>);

impl<'a> MetadataBuilder<'a> {
    #[must_use]
    pub fn title(title: &'a str) -> Self {
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

    pub fn creator(mut self, creator: &'a str) -> Self {
        self.0.creator = Some(creator);
        self
    }

    pub fn contributor(mut self, contributor: &'a str) -> Self {
        self.0.contributor = Some(contributor);
        self
    }

    pub fn publisher(mut self, publisher: &'a str) -> Self {
        self.0.publisher = Some(publisher);
        self
    }

    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.0.date = Some(date);
        self
    }

    pub fn subject(mut self, subject: &'a str) -> Self {
        self.0.subject = Some(subject);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.0.description = Some(description);
        self
    }

    pub fn build(self) -> Metadata<'a> {
        self.0
    }
}

#[derive(Debug, Clone, Default)]
pub enum Language {
    Arabic,
    Bulgarian,
    Chinese,
    Croatian,
    Czech,
    Danish,
    Dutch,
    #[default]
    English,
    Estonian,
    Finnish,
    French,
    Greek,
    German,
    Hebrew,
    Hungarian,
    Icelandic,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Korean,
    Latvian,
    Lithuanian,
    Macedonian,
    Malay,
    Maltese,
    Norwegian,
    Persian,
    Polish,
    Portuguese,
    Romanian,
    Russian,
    Serbian,
    Slovak,
    Slovenian,
    Spanish,
    Swahili,
    Swedish,
    Tagalog,
    Thai,
    Turkish,
    Ukrainian,
    Urdu,
    Vietnamese,
    Welsh,
    Yiddish,
}

impl Language {
    pub fn as_metadata_xml(&self) -> String {
        format!("<dc:language>{}</dc:language>", self.as_ref())
    }
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Arabic => "ar",
            Language::Bulgarian => "bg",
            Language::Chinese => "zh",
            Language::Croatian => "hr",
            Language::Czech => "cs",
            Language::Danish => "da",
            Language::Dutch => "nl",
            Language::English => "en",
            Language::Estonian => "et",
            Language::Finnish => "fi",
            Language::French => "fr",
            Language::Greek => "el",
            Language::German => "de",
            Language::Hebrew => "he",
            Language::Hungarian => "hu",
            Language::Icelandic => "is",
            Language::Indonesian => "id",
            Language::Irish => "ga",
            Language::Italian => "it",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Latvian => "lv",
            Language::Lithuanian => "lt",
            Language::Macedonian => "mk",
            Language::Malay => "ms",
            Language::Maltese => "mt",
            Language::Norwegian => "no",
            Language::Persian => "fa",
            Language::Polish => "pl",
            Language::Portuguese => "pt",
            Language::Romanian => "ro",
            Language::Russian => "ru",
            Language::Serbian => "sr",
            Language::Slovak => "sk",
            Language::Slovenian => "sl",
            Language::Spanish => "es",
            Language::Swahili => "sw",
            Language::Swedish => "sv",
            Language::Tagalog => "tl",
            Language::Thai => "th",
            Language::Turkish => "tr",
            Language::Ukrainian => "uk",
            Language::Urdu => "ur",
            Language::Vietnamese => "vi",
            Language::Welsh => "cy",
            Language::Yiddish => "yi",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Identifier {
    UUID(String),
    ISBN(String),
}

impl Identifier {
    pub(crate) fn as_metadata_xml(&self) -> String {
        format!(
            r#"<dc:identifier id="BookId" opf:scheme="{}">{}</dc:identifier>"#,
            self,
            self.as_ref()
        )
    }

    pub(crate) fn as_toc_xml(&self) -> String {
        format!(r#"<meta name="dtb:uid" content="{}"/>"#, self.as_ref())
    }
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
        assert!(metadata.date.is_some());
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
        let subject = "Science Fiction";
        let description = "A comic science fiction series created by Douglas Adams.";

        let metadata = MetadataBuilder::title(title)
            .language(language)
            .identifier(identifier)
            .creator(creator)
            .publisher(publisher)
            .subject(subject)
            .description(description)
            .build();

        assert_eq!(metadata.creator, Some(creator));
        assert_eq!(metadata.contributor, None);
        assert_eq!(metadata.publisher, Some(publisher));
        assert!(metadata.date.is_some());
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
