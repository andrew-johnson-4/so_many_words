use std::env;
use so_many_words::util::read_lines;
use so_many_words::en::tokens::Tokenizer;
use std::collections::HashSet;

fn main() {
    let tokenizer = Tokenizer::new();
    let mut dict: HashSet<String> = HashSet::new();
    let args: Vec<String> = env::args().collect();
    for fp in args[1..].iter() {
       if let Ok(lines) = read_lines(fp) {
          for (ln,line) in lines.enumerate() {
             if ln>0 && (ln%1000)==0 { println!("passed line #{}", ln); }
             if let Ok(sent) = line {
                let ws = tokenizer.tokenize(&sent);
                for w in ws.iter() {
                   if tokenizer.is_word(w) {
                      dict.insert(w.to_string());
                   }
                }
             }
          }
       }
   }
   println!("{}", dict.len());
}
