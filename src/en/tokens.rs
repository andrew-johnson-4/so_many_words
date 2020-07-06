use regex::Regex;

pub struct Tokenizer {
   re_word: Regex,
   re_numeral: Regex,
   re_nonpunct: Regex,
   re_whitespace: Regex,
}

impl Tokenizer {
   pub fn new() -> Tokenizer {
      Tokenizer {
         re_word: Regex::new("^\\w+").unwrap(),
         re_numeral: Regex::new("^[0123456789]+").unwrap(),
         re_nonpunct: Regex::new("(\\s|[0123456789]|\\w)").unwrap(),
         re_whitespace: Regex::new("^\\s+").unwrap(),
      }
   }
   pub fn is_word(&self, s: &str) -> bool {
      if let Some(m) = self.re_word.find(s) {
         m.start()==0 && m.end()==s.len()
      } else { false }
   }
   pub fn is_numeral(&self, s: &str) -> bool {
      if let Some(m) = self.re_numeral.find(s) {
         m.start()==0 && m.end()==s.len()
      } else { false }
   }
   pub fn is_punctuation(&self, s: &str) -> bool {
      !self.re_nonpunct.is_match(s)
   }
   pub fn tokenize(&self, s: &str) -> Vec<String> {
      let mut s = s.to_string();
      let mut ts = Vec::new();
      while s.len() > 0 {
         if let Some(m) = self.re_whitespace.find(&s) {
            s = s[m.end()..].to_string();
         } else if let Some(m) = self.re_numeral.find(&s) {
            ts.push(s[0..m.end()].to_string());
            s = s[m.end()..].to_string();
         } else if let Some(m) = self.re_word.find(&s) {
            ts.push(s[0..m.end()].to_string());
            s = s[m.end()..].to_string();
         } else {
            ts.push(s[0..1].to_string());
            s = s[1..].to_string();
         }
      }
      ts

   }
}
