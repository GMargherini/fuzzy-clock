mod clock;
use clock::Time;

pub struct Input {
    lang: String,
    newline: bool
}

impl Input {
    pub fn build(args: Vec<String>) -> Result<Input, String> {
        match args.len() {
            1 => Ok(Input {lang: "en".to_string(), newline: false}),
            2 => Self::parse(&args[1]),
            3 => Self::parse_two(&args[1], &args[2]),
            _ => Err(format!("Wrong number of arguments: {}", args.len()-1))
        }
    }

    fn parse(arg: &str) -> Result<Input, String> {
        match arg {
            "t" => Ok(Input {lang: "en".to_string(), newline: true}),
            "help" => {
                Err("help".to_string())
            },
            s => Ok(Input {lang: s.to_string(), newline: false}),
        }
    }

    fn parse_two(arg1: &str, arg2: &str) -> Result<Input, String> {
        match (arg1, arg2) {
            (s, "t") | ("t", s) => Ok(Input {lang: s.to_string(), newline: true}),
            (_, _) => Err(format!("Unknown arguments \"{}\" \"{}\"", arg1, arg2)) 
        }
    }
    pub fn lang(&self) -> &str{
        &self.lang
    }
    pub fn newline(&self) -> bool{
        self.newline
    }
}

pub fn print_help() {
    println!("Usage:\tfuzzy_clock [language] [t]\n\tfuzzy_clock [help]");
    println!("Arguments can be specified in any order");
    println!("\n\thelp\t:\tprint this help message");
    println!("\tt\t:\toutput over two lines instead of one");
    println!("\nlanguage:
        en\t:\tEnglish (default)
        fr\t:\tFrench
        it\t:\tItalian
        sv\t:\tSwedish");
}

pub fn run(input: Input) -> Result<(), String> {
    match Time::build(input) {
        Ok(t) => {
            println!("{}", t);
            Ok(())
        },
        Err(s) => {
            Err(s)
        }
    }
    
}