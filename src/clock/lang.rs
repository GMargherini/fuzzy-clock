pub enum Language {
    It,
    En,
    Fr,
    Sv,
}

impl Language {
    pub fn build(lang: &str) -> Result<Language, String> {
        let lang = lang.to_lowercase();
        match &lang[..] {
            "it" => Ok(Language::It),
            "en" => Ok(Language::En),
            "fr" => Ok(Language::Fr),
            "sv" => Ok(Language::Sv),
            l => Err(format!("Language \"{l}\" not available"))
        }
    }
}
