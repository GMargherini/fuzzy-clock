use fuzzy_clock::{self, Input};
use std::process;

fn main() {
    let input = Input::default();
    if let Err(s) = fuzzy_clock::run(input) {
        eprintln!("{s}");
        process::exit(1);
    }
}
