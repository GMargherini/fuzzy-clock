use std::env;

use chrono::Timelike;

fn main() {
    let now = chrono::offset::Utc::now();
    let m = now.minute();
    let h = now.hour();
    let mut lang = "en".to_owned();
    let mut new_line = false;
    for argument in env::args(){
        if ["it", "en"].contains(&argument.as_str()){
            lang = argument.clone();
        }
        if argument == "t".to_owned(){
            new_line = true
        }
    }
    println!("{}",get_time(h, m, lang, new_line));
}

fn get_time(h:u32,m:u32,lan:String, nl:bool) -> String{
    let l = lan.as_str();
    let (hours, mins) = get_language(l);
    let mut h_ind = (h%12) as usize;
	let m_ind = ((m) as f32 /6.67) as usize;
    if m_ind > 4{
		h_ind = (h_ind)%12 + 1;
        if h == 23{
            h_ind = 0;
        }
	}
    let mut ht = hours[h_ind].to_owned();
    if h_ind == 0 && (h == 11 || h == 12){
        ht = hours[12].to_owned();
    }
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
        else if ml> hl{
            let diff = (ml - hl)/2;
            let new = " ".repeat(diff) + &ht;
            ht = new;
        }
        res = match l{
            "it" => if m_ind == 8 {format!("{mt}\n{ht}").to_owned()} else {format!("{ht}\n{mt}").to_owned()}
            "en"|_ => format!("{mt}\n{ht}").to_owned(),
        };
    }
    else{
        res = match l{
            "it" => if m_ind == 8 {format!("{mt} {ht}").to_owned()} else {format!("{ht} {mt}").to_owned()}
            "en"|_ => format!("{mt} {ht}").to_owned(),
        };
    }
    return res
}

fn get_language(lan:&str) -> ([&'static str;13],[&'static str;9]){
    let hours_en = ["midnight","one","two","three","four","five","six","seven","eight","nine", "ten", "eleven", "noon"];
    let mins_en = ["","ten past","quarter past","twenty past","half past","twenty to","quarter to","ten to","almost"];
    let hours_it = ["mezzanotte","l'una","le due", "le tre", "le quattro", "le cinque", "le sei", "le sette", "le otto", "le nove", "le dieci", "le undici", "mezzogiorno"];
	let mins_it = ["", "e dieci", "e un quarto", "e venti", " e mezza", "meno venti", "meno un quarto", "meno dieci", "quasi"];
    let (hours, mins) = match lan{
        "it" => (hours_it, mins_it),
        "en"|_ => (hours_en, mins_en),
    };
    return (hours, mins);
}
