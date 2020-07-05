use regex::Regex;

pub fn tokenize(s: &str) -> Vec<String> {
   let mut s = s.to_string();
   let whitespace = Regex::new("^\\s+").unwrap();
   let numeral = Regex::new("^\\d+").unwrap();
   let word = Regex::new("^\\w+").unwrap();
   let mut ts = Vec::new();
   while s.len() > 0 {
      if let Some(m) = whitespace.find(&s) {
         s = s[m.end()..].to_string();
      } else if let Some(m) = numeral.find(&s) {
         ts.push(s[0..m.end()].to_string());
         s = s[m.end()..].to_string();
      } else if let Some(m) = word.find(&s) {
         ts.push(s[0..m.end()].to_string());
         s = s[m.end()..].to_string();
      } else {
         ts.push(s[0..1].to_string());
         s = s[1..].to_string();
      }
   }
   ts
}

pub fn is_word(s: &str) -> bool {
   let word = Regex::new("^\\w+").unwrap();
   if let Some(m) = word.find(s) {
      m.start()==0 && m.end()==s.len()
   } else { false }
}

pub fn is_numeral(s: &str) -> bool {
   let numeral = Regex::new("^[0123456789]+").unwrap();
   if let Some(m) = numeral.find(s) {
      m.start()==0 && m.end()==s.len()
   } else { false }
}

pub fn is_punctuation(s: &str) -> bool {
   let nonpunct = Regex::new("(\\s|[0123456789]|\\w)").unwrap();
   !nonpunct.is_match(s)
}
