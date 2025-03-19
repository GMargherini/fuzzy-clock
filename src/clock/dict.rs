use std::u32;

use super::Language;
pub struct Dictionary<'a> {
    hours: [&'a str; 13],
    mins: [&'a str; 12],
}

impl Dictionary<'_> {
    fn get_mins_index(&self, mins: u32) -> usize {
        let buckets: Vec<_> = (3..=53)
            .step_by(5)
            .map(|i| i..=i+4 )
            .collect();
        
        match buckets.iter().enumerate().find(|(_, b)| b.contains(&mins)) {
            Some((i, _)) => i+1,
            None => 0
        }
    }

    fn get_hours_index(&self, hours: u32, mins: u32) -> usize {
        let h_ind = (hours % 12) as usize;
        match mins {
            33..=59 => match hours {
                11 => 12,
                23 => 0,
                _ => h_ind + 1
            },
            _ => if hours == 12 {12} else {h_ind}
        }
    }


    pub fn new(lang: &Language) -> Dictionary {
        let (hours, mins) = match lang {
            Language::En => (
                [
                    "midnight", "one", "two", "three", "four", "five", "six",
                    "seven", "eight", "nine", "ten", "eleven", "noon"
                ],
                [
                    "", "five past", "ten past", "quarter past", "twenty past",
                    "twenty-five past", "half past", "twenty-five to",
                    "twenty to", "quarter to", "ten to", "five to"
                ],
            ),
            Language::It => (
                [
                    "mezzanotte", "l'una", "le due", "le tre", "le quattro",
                    "le cinque", "le sei", "le sette", "le otto", "le nove",
                    "le dieci", "le undici", "mezzogiorno"
                ],
                [
                    "", "e cinque", "e dieci", "e un quarto", "e venti",
                    "e venticinque", "e mezza", "meno venticinque", "meno venti",
                    "meno un quarto", "meno dieci", "meno cinque"
                ],
            ),
            Language::Fr => (
                [
                    "minuit", "une heure", "deux heures", "troix heures", 
                    "quatre heures", "cinq heures", "six heures", "sept heures",
                    "huit heures", "neuf heures", "dix heures", "onze heures", "midi"
                ],
                [
                    "", "cinq", "dix", "et quart", "vingt", "vingt-cinq", "et demie",
                    "moins vingt-cinq", "moins vingt", "moins le quart", "moins dix",
                    "moins cinq"
                ]
            ),
            Language::Sv => (
                [
                    "midnatt", "ett", "två", "tre", "fyra", "fem", "sex", "sju",
                    "åtta", "nio", "tio", "elva", "middag"
                ],
                [
                    "", "fem över", "tio över", "kvart över", "tjugo över",
                    "tjugofem över", "halv", "tjugofem i", "tjugo i", 
                    "kvart i", "tio i", "fem i"
                ],
            ),
        };
        Dictionary {
            hours,
            mins,
        }
    }

    pub fn text(&self, hour: u32, min: u32) -> (&str, &str) {
        let mins = self.mins[self.get_mins_index(min)];
        let hours = self.hours[self.get_hours_index(hour, min)];
        (hours, mins)
    }
}