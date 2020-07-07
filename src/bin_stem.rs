use std::env;
use so_many_words::util::read_lines;
use so_many_words::en::tokens::Tokenizer;
use so_many_words::stemmer::Stemmer;
use so_many_words::lang::Lang;

fn main() {
    let mut lang = "eng".to_string();
    let tokenizer = Tokenizer::new();
    let args: Vec<String> = env::args().collect();
    for fp in args[1..].iter() {
       if vec!["arabic","danish","dutch","english","french","german",
          "greek","hungarian","italian","norwegian","portuguese","romanian",
          "russian","spanish","swedish","tamil","turkish"].contains(&fp.as_str()) {
          lang = fp.to_string();
       }
    }
    let lang = Lang::from_code(lang.clone()).expect(&format!("Lang code expects ISO 639-3 formatted string, found {}", lang));
    let stemmer = Stemmer::new(&lang);
    for fp in args[1..].iter() {
       if fp.starts_with("--") { continue; }
       if let Ok(lines) = read_lines(fp) {
          for line in lines {
             if let Ok(sent) = line {
                let ws = tokenizer.tokenize(&sent);
                for w in ws.iter() {
                   let sw = stemmer.stem(w);
                   println!("{}", sw);
                }
             }
          }
       }
   }
}
