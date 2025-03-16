mod clock {
    use chrono::Timelike;
    use std::cmp::Ordering;
    use std::collections::HashMap;
    pub struct Time {
        h: u32,
        m: f32,
    }

    impl Time {
        pub fn new() -> Time {
            let now = chrono::offset::Local::now();
            Time {
                h: now.hour(),
                m: now.minute() as f32,
            }
        }

        pub fn get_time_string(&self, lan: &str, nl: bool) -> String {
            let (hours, mins) = Self::get_language(lan);
            let (h_ind, m_ind) = self.get_indexes();

            let hours = if h_ind == 0 && (self.h == 11 || self.h == 12) {
                hours[12]
            } else {
                hours[h_ind]
            };

            let mins = mins[m_ind];
            if m_ind == 0 {
                return hours.to_string();
            }

            match lan {
                "it" => format_text(hours, mins, nl),
                _ => format_text(mins, hours, nl),
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
            (h_ind, m_ind)
        }

        fn get_language(lan: &str) -> ([&str; 13], [&str; 12]) {
            let languages = HashMap::from([
                (
                    "en",
                    (
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
                ),
                (
                    "it",
                    (
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
                ),
            ]);
            languages[lan]
        }
    }

    fn format_text(first: &str, second: &str, new_line: bool) -> String {

        if !new_line {
            return format!("{} {}", first, second);
        }

        let len_first = first.len() as isize;
        let len_second = second.len() as isize;

        let diff = (isize::abs(len_first - len_second) / 2) as usize;
        let offset = " ".repeat(diff);

        match len_first.cmp(&len_second) {
            Ordering::Less => format!("{}\n{}", offset + first, second),
            Ordering::Greater => format!("{}\n{}", first, offset + second),
            Ordering::Equal => format!("{}\n{}", first, second),
        }
    }
}

use std:: env;
use clock::Time;
fn main() {
    let time = Time::new();
    let new_line = env::args().any(|x| x == "t");
    let available_languages = ["it", "en"];
    let lang = match env::args().find(|x| available_languages.contains(&(x.as_str()))) {
        Some(t) => t,
        None => "en".to_string(),
    };
    println!("{}", time.get_time_string(&lang, new_line));
}
