use std::env;

use chrono::Timelike;

struct Time {
    h:u32, 
    m:f32,
}

impl Time {
    fn get_time_string(&self, lan:&str, nl:bool) -> String{
        let (hours, mins) = Time::get_language(lan);
        let m_ind = (((self.m+2.5)) /5.0) as usize;
        let h_ind = if m_ind > 6 {
            if self.h == 23{
                0
            } else {
                ((self.h%12) as usize)%12 + 1
            }
        } else {
            (self.h%12) as usize
        };

        let mut ht = if h_ind == 0 && (self.h == 11 || self.h == 12){
            hours[12].to_owned()
        } else {
            hours[h_ind].to_owned()
        };

        let mut mt = mins[m_ind].to_owned();
        if m_ind == 0{
            let res = format!("{mt}{ht}").to_owned();
            return res
        }

        let res;
        if nl{
            let hl = ht.len();
            let ml = mt.len();
            if hl > ml{
                let diff = (hl - ml)/2;
                let new = " ".repeat(diff) + &mt;
                mt = new;
            }
            else if ml> hl {
                let diff = (ml - hl)/2;
                let new = " ".repeat(diff) + &ht;
                ht = new;
            }
            res = match lan {
                "it" => {format!("{ht}\n{mt}").to_owned()}
                "en"|_ => format!("{mt}\n{ht}").to_owned(),
            };
        }
        else{
            res = match lan {
                "it" => {format!("{ht} {mt}").to_owned()}
                "en"|_ => format!("{mt} {ht}").to_owned(),
            };
        }
        return res
    }

    fn get_language(lan:&str) -> ([&'static str;13],[&'static str;12]){
        let hours_en = ["midnight","one","two","three","four","five","six","seven","eight","nine", "ten", "eleven", "noon"];
        let mins_en = ["", "five past", "ten past", "quarter past", "twenty past", "twenty-five past", "half past", "twenty-five to", "twenty to", "quarter to", "ten to", "five to"];
        let hours_it = ["mezzanotte","l'una","le due", "le tre", "le quattro", "le cinque", "le sei", "le sette", "le otto", "le nove", "le dieci", "le undici", "mezzogiorno"];
        let mins_it = ["", "e cinque", "e dieci", "e un quarto", "e venti", "e venticinque", "e mezza", "meno venticinque", "meno venti", "meno un quarto", "meno dieci", "meno cinque"];
        let (hours, mins) = match lan{
            "it" => (hours_it, mins_it),
            "en"|_ => (hours_en, mins_en),
        };
        return (hours, mins);
    }
}

fn main() {
    let now = chrono::offset::Local::now();
    let time = Time{h: now.hour(), m: now.minute() as f32};
    let new_line = match env::args().find(|x| x == "t"){
        Some(_) => true,
        None => false
    };
    let lang = match env::args().find(|x| x == "it" || x == "en"){
        Some(t) => t,
        None => "en".to_string()
    };
    println!("{}",time.get_time_string(&lang, new_line));
}

