use uuid::Uuid;

#[derive(Debug)]
pub struct Metadata {
    title: String,
    language: Language,
    identifier: Identifier,
    creator: Option<String>,
    publisher: Option<String>,
    date: Option<String>,
    subject: Option<String>,
    description: Option<String>,
}

impl Metadata {
    fn new(title: String, language: Language, identifier: Identifier) -> Self {
        Self {
            title,
            language,
            identifier,
            creator: None,
            publisher: None,
            date: None,
            subject: None,
            description: None,
        }
    }
}

#[derive(Debug)]
pub struct MetadataBuilder(Metadata);

impl MetadataBuilder {
    #[must_use]
    pub fn new(title: String, language: Language, identifier: Identifier) -> Self {
        Self(Metadata::new(title, language, identifier))
    }

    pub fn creator(mut self, creator: String) -> Self {
        self.0.creator = Some(creator);
        self
    }

    pub fn publisher(mut self, publisher: String) -> Self {
        self.0.publisher = Some(publisher);
        self
    }

    pub fn date(mut self, date: String) -> Self {
        self.0.date = Some(date);
        self
    }

    pub fn subject(mut self, subject: String) -> Self {
        self.0.subject = Some(subject);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.0.description = Some(description);
        self
    }

    pub fn build(self) -> Metadata {
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

impl From<Language> for &str {
    fn from(value: Language) -> Self {
        match value {
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

impl Default for Identifier {
    fn default() -> Self {
        Identifier::UUID(Uuid::new_v4().to_string())
    }
}
