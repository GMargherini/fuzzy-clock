mod clock;

use clock::Time;
use std::env;

fn main() {
    let newline = env::args().any(|x| x == "t");
    let lang = match env::args().find(|x| x.len() == 2) {
        Some(l) => l,
        None => "en".to_string(),
    };
    match Time::new(&lang[..], newline) {
        Ok(t) => println!("{}", t),
        Err(s) => eprintln!("{s}")
    };
}
