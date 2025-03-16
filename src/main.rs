mod clock;

use clock::{Language, Time};
use std::env;
fn main() {
    let time = Time::new();
    let new_line = env::args().any(|x| x == "t");
    let lang = match env::args().find(|x| x.len() == 2) {
        Some(l) => match &l[..] {
            "it" => Language::It,
            "en" => Language::En,
            _ => panic!("language not available"),
        },
        None => Language::En,
    };
    println!("{}", time.get_time_string(&lang, new_line));
}
