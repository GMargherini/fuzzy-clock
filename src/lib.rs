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
    match Time::build(input.lang(), input.newline()) {
        Ok(t) => {
            println!("{}", t);
            Ok(())
        },
        Err(s) => {
            Err(s)
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::clock::{self, lang};

    #[test]
    fn language_is_valid() {
        let lang = lang::Language::build("en");
        assert!(lang.is_ok());
        let lang = lang.unwrap();
        assert_eq!(lang, lang::Language::En);
    }

    #[test]
    fn language_is_not_valid() {
        let lang = lang::Language::build("zh");
        assert!(lang.is_err());
    }

    #[test]
    fn midnight() {
        let indexes = clock::Indexes::new(23, 59);
        assert_eq!(indexes.hour(), 0);
        assert_eq!(indexes.min(), 0);

        let indexes = clock::Indexes::new(0, 0);
        assert_eq!(indexes.hour(), 0);
        assert_eq!(indexes.min(), 0);

        let indexes = clock::Indexes::new(0, 1);
        assert_eq!(indexes.hour(), 0);
        assert_eq!(indexes.min(), 0);
    }

    #[test]
    fn noon() {
        let indexes = clock::Indexes::new(11, 59);
        assert_eq!(indexes.hour(), 12);
        assert_eq!(indexes.min(), 0);

        let indexes = clock::Indexes::new(12, 0);
        assert_eq!(indexes.hour(), 12);
        assert_eq!(indexes.min(), 0);

        let indexes = clock::Indexes::new(12, 1);
        assert_eq!(indexes.hour(), 12);
        assert_eq!(indexes.min(), 0);
    }

    #[test]
    fn half() {
        let indexes = clock::Indexes::new(1, 29);
        assert_eq!(indexes.hour(), 1);
        assert_eq!(indexes.min(), 6);

        let indexes = clock::Indexes::new(1, 30);
        assert_eq!(indexes.hour(), 1);
        assert_eq!(indexes.min(), 6);

        let indexes = clock::Indexes::new(1, 31);
        assert_eq!(indexes.hour(), 1);
        assert_eq!(indexes.min(), 6);
    }

}