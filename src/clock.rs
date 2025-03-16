use chrono::Timelike;
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};

pub struct Time {
    h: u32,
    m: f32,
    lang: Language,
    newline: bool,
}

pub enum Language {
    It,
    En,
}

impl Time {
    pub fn new(lang: &str, newline: bool) -> Time {
        let now = chrono::offset::Local::now();
        Time {
            h: now.hour(),
            m: now.minute() as f32,
            lang: Language::new(lang),
            newline,
        }
    }

    fn get_time_string(&self) -> String {
        let (hours, mins) = self.get_dictionary();
        let (h_ind, m_ind) = self.get_indexes();

        let (hours, mins) = (hours[h_ind], mins[m_ind]);

        match self.lang {
            Language::It => self.format_text(hours, mins),
            Language::En => self.format_text(mins, hours),
        }
    }

    fn get_indexes(&self) -> (usize, usize) {
        let m_ind = (((self.m + 2.5) / 5.0) as usize) % 12;
        let h_ind = if self.m > 30.0 {
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

    fn get_dictionary(&self) -> ([&str; 13], [&str; 12]) {
        match self.lang {
            Language::En => (
                [
                    "midnight", "one", "two", "three", "four", "five", "six", "seven", "eight",
                    "nine", "ten", "eleven", "noon",
                ],
                [
                    "",
                    "five past",
                    "ten past",
                    "quarter past",
                    "twenty past",
                    "twenty-five past",
                    "half past",
                    "twenty-five to",
                    "twenty to",
                    "quarter to",
                    "ten to",
                    "five to",
                ],
            ),
            Language::It => (
                [
                    "mezzanotte",
                    "l'una",
                    "le due",
                    "le tre",
                    "le quattro",
                    "le cinque",
                    "le sei",
                    "le sette",
                    "le otto",
                    "le nove",
                    "le dieci",
                    "le undici",
                    "mezzogiorno",
                ],
                [
                    "",
                    "e cinque",
                    "e dieci",
                    "e un quarto",
                    "e venti",
                    "e venticinque",
                    "e mezza",
                    "meno venticinque",
                    "meno venti",
                    "meno un quarto",
                    "meno dieci",
                    "meno cinque",
                ],
            ),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.get_time_string())
    }
}

impl Language {
    fn new(lang: &str) -> Language {
        match lang {
            "it" => Language::It,
            "en" => Language::En,
            l => {
                panic!("language \"{l}\" not available");
            }
        }
    }
}
