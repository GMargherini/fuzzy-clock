mod dict;
mod lang;

use dict::Dictionary;
use lang::Language;
use chrono::Timelike;
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
pub struct Time {
    h: u32,
    m: u32,
    lang: Language,
    newline: bool,
}

impl Time {
    pub fn new(lang: &str, newline: bool) -> Result<Time, String> {
        let now = chrono::offset::Local::now();
        Ok(Time {
            h: now.hour(),
            m: now.minute(),
            lang: Language::new(lang)?,
            newline,
        })
    }

    fn get_time_string(&self) -> String {
        let dict = Dictionary::new(&self.lang);
        let (h_ind, m_ind) = self.get_indexes();

        let (hours, mins) = dict.get_text(h_ind, m_ind);

        match self.lang {
            Language::It | Language::Fr => self.format_text(hours, mins),
            Language::En | Language::Sv => self.format_text(mins, hours),
        }
    }

    fn get_indexes(&self) -> (usize, usize) {
        let m_ind = (((self.m as f64 + 2.5) / 5.0) as usize) % 12;
        let h_ind = if self.m as f64 > 30.0 {
            if self.h == 23 {
                0
            } else {
                ((self.h % 12) as usize) % 12 + 1
            }
        } else {
            (self.h % 12) as usize
        };

        let h_ind = if h_ind == 0 && (self.h == 11 || self.h == 12) {
            12
        } else {
            h_ind
        };
        (h_ind, m_ind)
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
        write!(f, "{}", self.get_time_string())
    }
}