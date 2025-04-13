use clap::ValueEnum;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Copy, Clone, PartialOrd, ValueEnum)]
pub enum Language {
    /// English
    En,
    /// French
    Fr,
    /// Italian
    It,
    /// Swedish
    Sv,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lang = match self {
            Language::En => "en",
            Language::Fr => "fr",
            Language::It => "it",
            Language::Sv => "sv",
        };
        write!(f, "{}", lang)
    }
}
