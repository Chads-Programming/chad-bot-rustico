use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

pub fn check_english_lang(text: &str) -> bool {
    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&[
        lingua::Language::English,
        lingua::Language::Spanish,
    ])
    .build();
    let detected_language: Option<Language> = detector.detect_language_of(text);

    if let Some(lang) = detected_language {
        return lang == lingua::Language::English;
    }

    false
}
