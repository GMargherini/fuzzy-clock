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
    println!(
"Usage:\tfuzzy_clock [LANGUAGE] [t]\n\tfuzzy_clock help
Arguments can be specified in any order\n
    help\t:\tprint this help message
    t\t\t:\toutput over two lines instead of one
\nlanguage:
    en\t\t:\tEnglish (default)
    fr\t\t:\tFrench
    it\t\t:\tItalian
    sv\t\t:\tSwedish"
    );
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
        let dictionary = clock::dict::Dictionary::new(&lang::Language::En);
        let (hour, mins) = dictionary.text(23, 59);
        assert_eq!(hour, "midnight");
        assert_eq!(mins, "");

        let (hour, mins) = dictionary.text(0, 0);
        assert_eq!(hour, "midnight");
        assert_eq!(mins, "");

        let (hour, mins) = dictionary.text(0, 1);
        assert_eq!(hour, "midnight");
        assert_eq!(mins, "");
    }

    #[test]
    fn noon() {
        let dictionary = clock::dict::Dictionary::new(&lang::Language::En);
        let (hour, mins) = dictionary.text(11, 59);
        assert_eq!(hour, "noon");
        assert_eq!(mins, "");

        let (hour, mins) = dictionary.text(12, 0);
        assert_eq!(hour, "noon");
        assert_eq!(mins, "");

        let (hour, mins) = dictionary.text(12, 1);
        assert_eq!(hour, "noon");
        assert_eq!(mins, "");
    }

    #[test]
    fn half() {
        let dictionary = clock::dict::Dictionary::new(&lang::Language::En);
        let (hour, mins) = dictionary.text(0, 29);
        assert_eq!(hour, "midnight");
        assert_eq!(mins, "half past");

        let (hour, mins) = dictionary.text(0, 30);
        assert_eq!(hour, "midnight");
        assert_eq!(mins, "half past");

        let (hour, mins) = dictionary.text(0, 31);
        assert_eq!(hour, "midnight");
        assert_eq!(mins, "half past");
    }

}