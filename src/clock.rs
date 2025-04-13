pub mod dict;
pub mod lang;

use chrono::Timelike;
use clap::ValueEnum;
use dict::Dictionary;
use lang::Language;
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};

pub struct Time {
    h: u32,
    m: u32,
    lang: Language,
    newline: bool,
}

impl Time {
    pub fn build(lang: &str, newline: bool) -> Result<Time, String> {
        let now = chrono::offset::Local::now();
        Ok(Time {
            h: now.hour(),
            m: now.minute(),
            lang: Language::from_str(lang, true)?,
            newline,
        })
    }

    fn get_time_string(&self) -> Result<String, String> {
        let dict = Dictionary::new(&self.lang);

        let (hours, mins) = dict.text(self.h, self.m);

        match self.lang {
            Language::It | Language::Fr => Ok(self.format_text(hours, mins)),
            Language::En | Language::Sv => Ok(self.format_text(mins, hours)),
        }
    }

    fn format_text(&self, first: &str, second: &str) -> String {
        match (first, second) {
            ("", x) | (x, "") => return x.to_string(),
            _ => (),
        };
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
        write!(f, "{}", self.get_time_string().unwrap())
    }
}
