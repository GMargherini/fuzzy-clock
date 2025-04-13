mod clock;
use clap::Parser;
use clock::{Time, lang::Language};

#[derive(Parser, Debug)]
#[command(
    version,
    long_about = "Tells the time with a precision of ±2.5 minutes"
)]
pub struct Input {
    #[arg(value_enum, default_value_t=Language::En)]
    language: Language,
    /// Print over two lines centering text
    #[arg(short, long, default_value_t = false)]
    tabulate: bool,
}
impl Default for Input {
    fn default() -> Self {
        Self::parse()
    }
}

pub fn run(input: Input) {
    let t = Time::new(input.language, input.tabulate);
    println!("{}", t);
}

#[cfg(test)]
mod tests {
    use super::clock::{self, lang};
	use clap::ValueEnum;
    #[test]
    fn language_is_valid() {
        let lang = lang::Language::from_str("en", true);
        assert!(lang.is_ok());
        let lang = lang.unwrap();
        assert_eq!(lang, lang::Language::En);
    }

    #[test]
    fn language_is_not_valid() {
        let lang = lang::Language::from_str("zh", true);
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
