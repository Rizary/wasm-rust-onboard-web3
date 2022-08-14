use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Language(pub LanguageCode, pub LanguageName);

#[derive(Debug, Clone)]
pub enum LanguageCode {
    Ar,
    Id,
    En,
}

impl LanguageCode {
    pub fn from_code(url: &str) -> Self {
        match url {
            "ar" => Self::Ar,
            "id" => Self::Id,
            "en" => Self::En,
            _ => Self::En,
        }
    }
}

impl From<LanguageCode> for String {
    fn from(lang_code: LanguageCode) -> Self {
        (&lang_code).into()
    }
}

impl From<&LanguageCode> for String {
    fn from(lang_code: &LanguageCode) -> Self {
        match lang_code {
            LanguageCode::Ar => "ar".to_string(),
            LanguageCode::Id => "id".to_string(),
            LanguageCode::En => "en".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LanguageName {
    Arabic,
    Bahasa,
    English,
}

impl LanguageName {
    pub fn from_name(url: &str) -> Self {
        match url {
            "Arabic" => Self::Arabic,
            "Bahasa" => Self::Bahasa,
            "English" => Self::English,
            _ => Self::English,
        }
    }
}

impl From<LanguageName> for String {
    fn from(lang_name: LanguageName) -> Self {
        (&lang_name).into()
    }
}

impl From<&LanguageName> for String {
    fn from(lang_name: &LanguageName) -> Self {
        match lang_name {
            LanguageName::Arabic => "Arabic".to_string(),
            LanguageName::Bahasa => "Bahasa".to_string(),
            LanguageName::English => "English".to_string(),
        }
    }
}