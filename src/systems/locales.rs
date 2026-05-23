use fluent_templates::Loader;
use serde::{Serialize, Deserialize};
use strum_macros::Display;


fluent_templates::static_loader!{
    static LOCALES = {
        locales: "locales",
        fallback_language: "en",
    };
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum Language {
    #[strum(serialize = "en-language")] En,
    #[strum(serialize = "ru-language")] Ru,
}

pub fn translate(key: &str, lang: &Language) -> String {
    let leng_id = match lang {
        Language::En => "en".parse().unwrap(),
        Language::Ru => "ru".parse().unwrap(),
    };

    LOCALES.try_lookup(&leng_id, key).unwrap_or_else(|| key.to_string())
}
