use regex::Regex;

pub fn tokenize(s: &str) -> Vec<String> {
   let num = Regex::new("[0-9]+").unwrap();
   //let word = Regex::new("").unwrap();
   vec![]
}

pub fn is_numeral(s: &str) -> bool {
   false
}

pub fn is_punctuation(s: &str) -> bool {
   false
}
