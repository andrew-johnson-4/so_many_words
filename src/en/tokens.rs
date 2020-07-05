use regex::Regex;

pub fn tokenize(s: &str) -> Vec<String> {
   let whitespace = Regex::new("\\s+").unwrap();
   let numeral = Regex::new("\\d+").unwrap();
   let word = Regex::new("\\w+").unwrap();
   vec![]
}

pub fn is_numeral(s: &str) -> bool {
   let numeral = Regex::new("[0123456789]+").unwrap();
   if let Some(m) = numeral.find(s) {
      m.start()==0 && m.end()==s.len()
   } else { false }
}

pub fn is_punctuation(s: &str) -> bool {
   let nonpunct = Regex::new("(\\s|[0123456789]|\\w)").unwrap();
   !nonpunct.is_match(s)
}
