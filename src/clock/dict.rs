use super::Language;
pub struct Dictionary<'a> {
    hours: [&'a str; 13],
    mins: [&'a str; 12],
}

impl<'a> Dictionary<'a> {
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

    pub fn text(&self, hour: usize, min: usize) -> (&str, &str) {
        (self.hours[hour], self.mins[min])
    }
}