mod clock;

use clock::Time;
use std::env;

fn main() {
    let new_line = env::args().any(|x| x == "t");
    let lang = match env::args().find(|x| x.len() == 2) {
        Some(l) => l,
        None => "en".to_string(),
    };
    let time = Time::new(&lang[..], new_line);
    println!("{}", time);
}
