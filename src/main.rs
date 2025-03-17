use std::{env, process};
use fuzzy_clock::{self, Input};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = 
        Input::build(args)
        .unwrap_or_else(|err| if err == "help" {
            fuzzy_clock::print_help();
            process::exit(0);
        } else {
            eprintln!("{err}");
            fuzzy_clock::print_help();
            process::exit(1);
        });

    if let Err(s) = fuzzy_clock::run(input) {
        eprintln!("{s}");
        fuzzy_clock::print_help();
        process::exit(1);
    }
}
