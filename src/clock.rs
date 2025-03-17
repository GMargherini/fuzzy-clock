pub mod dict;
pub mod lang;

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

pub struct Indexes {
    hour: usize,
    min: usize
}

impl Time {
    pub fn build(lang: &str, newline: bool) -> Result<Time, String> {
        let now = chrono::offset::Local::now();
        Ok(Time {
            h: now.hour(),
            m: now.minute(),
            lang: Language::build(lang)?,
            newline,
        })
    }

    fn get_time_string(&self) -> String {
        let dict = Dictionary::new(&self.lang);
        let indexes = Indexes::new(self.h, self.m);

        let (hours, mins) = dict.text(indexes.hour(), indexes.min());

        match self.lang {
            Language::It | Language::Fr => self.format_text(hours, mins),
            Language::En | Language::Sv => self.format_text(mins, hours),
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
        write!(f, "{}", self.get_time_string())
    }
}

impl Indexes {
    pub fn new(h: u32, m: u32) -> Indexes {
        let m_ind = (((m as f64 + 2.5) / 5.0) as usize) % 12;

        let h_ind = match m.cmp(&32) {
            Ordering::Less | Ordering::Equal => {
                if h == 12{
                    12
                } else {
                    (h % 12) as usize
                }
            },
            Ordering::Greater => {
                if h == 23 {
                    0
                } else {
                    ((h % 13) + 1) as usize
                }
            },
        };

        Indexes {
            hour: h_ind,
            min: m_ind
        }
    }

    pub fn hour(&self) -> usize {
        self.hour
    }

    pub fn min(&self) -> usize {
        self.min
    }
}