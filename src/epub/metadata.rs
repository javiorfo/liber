use std::fmt::Display;

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Core structure holding all necessary descriptive information about a resource (e.g., a book).
///
/// Use the [`MetadataBuilder`] to create instances of this struct.
#[derive(Debug, Clone)]
pub struct Metadata {
    /// The primary title of the resource.
    pub title: String,
    /// The primary language of the resource's content.
    pub language: Language,
    /// A unique identifier for the resource.
    pub identifier: Identifier,
    /// The primary person or entity responsible for the content's creation.
    pub creator: Option<String>,
    /// A secondary person or entity who has made contributions (e.g., translator, editor).
    pub contributor: Option<String>,
    /// The entity responsible for making the resource available.
    pub publisher: Option<String>,
    /// The date of the resource's publication or creation. Defaults to the current UTC time when created via `new()`.
    pub date: Option<DateTime<Utc>>,
    /// Keywords or phrases describing the content of the resource.
    pub subject: Option<String>,
    /// A short summary or description of the resource's content.
    pub description: Option<String>,
}

impl Metadata {
    /// Creates a new `Metadata` instance with mandatory fields and default values for optional fields.
    ///
    /// The `date` field is set to the current UTC time.
    fn new<S: Into<String>>(title: S, language: Language, identifier: Identifier) -> Self {
        Self {
            title: title.into(),
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

    /// Generates the XML representation for the **title** element.
    pub(crate) fn title_as_metadata_xml(&self) -> String {
        format!("<dc:title>{}</dc:title>", self.title)
    }

    /// Generates the XML representation for the **creator** element, including the `opf:role="aut"` attribute.
    ///
    /// Returns `None` if the creator is not set.
    pub(crate) fn creator_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            r#"<dc:creator opf:role="aut">{}</dc:creator>"#,
            self.creator.as_ref()?
        ))
    }

    /// Generates the XML representation for the **contributor** element, including the `opf:role="trl"` attribute.
    ///
    /// Returns `None` if the contributor is not set.
    pub(crate) fn contributor_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            r#"<dc:contributor opf:role="trl">{}</dc:contributor>"#,
            self.contributor.as_ref()?
        ))
    }

    /// Generates the XML representation for the **publisher** element.
    ///
    /// Returns `None` if the publisher is not set.
    pub(crate) fn publisher_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            "<dc:publisher>{}</dc:publisher>",
            self.publisher.as_ref()?
        ))
    }

    /// Generates the XML representation for the **date** element, formatted as YYYY-MM-DD.
    ///
    /// Returns `None` if the date is not set.
    pub(crate) fn date_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            r#"<dc:date opf:event="publication">{}</dc:date>"#,
            self.date?.format("%Y-%m-%d")
        ))
    }

    /// Generates the XML representation for the **subject** element.
    ///
    /// Returns `None` if the subject is not set.
    pub(crate) fn subject_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            "<dc:subject>{}</dc:subject>",
            self.subject.as_ref()?
        ))
    }

    /// Generates the XML representation for the **description** element.
    ///
    /// Returns `None` if the description is not set.
    pub(crate) fn description_as_metadata_xml(&self) -> Option<String> {
        Some(format!(
            "<dc:description>{}</dc:description>",
            self.description.as_ref()?
        ))
    }
}

/// A builder for easily constructing [`Metadata`] structs.
///
/// This uses a **fluent interface** to set optional fields before finalizing the structure with `build()`.
#[derive(Debug)]
pub struct MetadataBuilder(Metadata);

impl MetadataBuilder {
    /// Starts the builder by setting the **mandatory title** and initializing with default language and identifier (a new UUID).
    #[must_use]
    pub fn title<S: Into<String>>(title: S) -> Self {
        Self(Metadata::new(
            title,
            Language::default(),
            Identifier::default(),
        ))
    }

    /// Sets the primary **language** of the resource.
    pub fn language(mut self, language: Language) -> Self {
        self.0.language = language;
        self
    }

    /// Sets the unique **identifier** for the resource (e.g., UUID or ISBN).
    pub fn identifier(mut self, identifier: Identifier) -> Self {
        self.0.identifier = identifier;
        self
    }

    /// Sets the **creator** of the resource.
    pub fn creator<S: Into<String>>(mut self, creator: S) -> Self {
        self.0.creator = Some(creator.into());
        self
    }

    /// Sets the **contributor** of the resource.
    pub fn contributor<S: Into<String>>(mut self, contributor: S) -> Self {
        self.0.contributor = Some(contributor.into());
        self
    }

    /// Sets the **publisher** of the resource.
    pub fn publisher<S: Into<String>>(mut self, publisher: S) -> Self {
        self.0.publisher = Some(publisher.into());
        self
    }

    /// Sets the publication **date** using a specific `DateTime<Utc>`.
    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.0.date = Some(date);
        self
    }

    /// Sets the **subject** (keywords/tags) for the resource.
    pub fn subject<S: Into<String>>(mut self, subject: S) -> Self {
        self.0.subject = Some(subject.into());
        self
    }

    /// Sets the **description** (summary) for the resource.
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.0.description = Some(description.into());
        self
    }

    /// Consumes the builder and returns the final [`Metadata`] instance.
    pub fn build(self) -> Metadata {
        self.0
    }
}

/// Represents the primary language of the resource content, using its corresponding **ISO 639-1** code.
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
    /// Generates the XML representation for the **language** element.
    ///
    /// The language code (e.g., `en`, `fr`) is used as the content.
    pub fn as_metadata_xml(&self) -> String {
        format!("<dc:language>{}</dc:language>", self.as_ref())
    }
}

/// Helper implementation to get the two-letter ISO 639-1 code for the language.
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

/// Represents a unique identifier for the resource, typically a UUID or ISBN.
#[derive(Debug, Clone)]
pub enum Identifier {
    /// A standard **UUID** (Universally Unique Identifier).
    UUID(String),
    /// An **ISBN** (International Standard Book Number).
    ISBN(String),
}

impl Identifier {
    /// Generates the XML representation for the **identifier** element.
    ///
    /// The scheme (`UUID` or `ISBN`) and the URN value are included.
    pub(crate) fn as_metadata_xml(&self) -> String {
        format!(
            r#"<dc:identifier id="BookId" opf:scheme="{}">{}</dc:identifier>"#,
            self,
            std::string::String::from(self)
        )
    }

    /// Generates the XML representation for the **TOC (Table of Contents)** metadata, typically used for DTB UID.
    pub(crate) fn as_toc_xml(&self) -> String {
        format!(
            r#"<meta name="dtb:uid" content="{}"/>"#,
            std::string::String::from(self)
        )
    }
}

/// Converts the identifier into its URN (Uniform Resource Name) format, e.g., `urn:uuid:...` or `urn:isbn:...`.
impl From<&Identifier> for String {
    fn from(value: &Identifier) -> Self {
        match value {
            Identifier::UUID(value) => format!("urn:uuid:{}", value),
            Identifier::ISBN(value) => format!("urn:isbn:{}", value),
        }
    }
}

impl Default for Identifier {
    /// Generates a new random **UUID** as the default identifier.
    fn default() -> Self {
        Identifier::UUID(Uuid::new_v4().to_string())
    }
}

/// Displays the identifier scheme (`UUID` or `ISBN`).
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

        assert_eq!(metadata.creator, Some(creator.to_string()));
        assert_eq!(metadata.contributor, None);
        assert_eq!(metadata.publisher, Some(publisher.to_string()));
        assert!(metadata.date.is_some());
        assert_eq!(metadata.subject, Some(subject.to_string()));
        assert_eq!(metadata.description, Some(description.to_string()));
    }

    #[test]
    fn test_identifier_default_uuid() {
        let default_identifier = Identifier::default();

        match default_identifier {
            Identifier::UUID(value) => {
                assert!(Uuid::parse_str(&value).is_ok());
            }
            _ => panic!("Default identifier was not a UUID"),
        }
    }
}
