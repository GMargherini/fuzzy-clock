pub mod dict;
pub mod lang;

use chrono::Timelike;
use dict::Dictionary;
use lang::Language;
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};

/// Represents the local time
pub struct Time {
    h: u32,
    m: u32,
    lang: Language,
    newline: bool,
}

impl Time {
    /// builds a `Time` given a language and if it needs to print over two lines
    pub fn new(lang: Language, newline: bool) -> Time {
        let now = chrono::offset::Local::now();
        Time {
            h: now.hour(),
            m: now.minute(),
            lang,
            newline,
        }
    }

    /// returns the time in text form
    fn get_time_string(&self) -> String {
        let dict = Dictionary::new(&self.lang);

        let (hours, mins) = dict.text(self.h, self.m);

        match self.lang {
            Language::It | Language::Fr => self.format_text(hours, mins),
            Language::En | Language::Sv => self.format_text(mins, hours),
        }
    }

    /// formats the given text
    fn format_text(&self, first: &str, second: &str) -> String {
        if let ("", x) | (x, "") = (first, second) {
            return x.to_string();
        }
        if !self.newline {
            return format!("{} {}", first, second);
        }

        let len_first = first.len() as isize;
        let len_second = second.len() as isize;

        let diff = (isize::abs(len_first - len_second) / 2) as usize;
        let offset = " ".repeat(diff);

        match len_first.cmp(&len_second) {
            Ordering::Less => format!("{offset}{first}\n{second}"),
            Ordering::Greater => format!("{first}\n{offset}{second}"),
            Ordering::Equal => format!("{first}\n{second}"),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.get_time_string())
    }
}
