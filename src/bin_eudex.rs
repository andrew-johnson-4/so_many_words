use std::env;
use so_many_words::util::read_lines;
use so_many_words::en::tokens::Tokenizer;
use eudex::Hash;

fn main() {
    let tokenizer = Tokenizer::new();
    let args: Vec<String> = env::args().collect();
    for fp in args[1..].iter() {
       if let Ok(lines) = read_lines(fp) {
          for line in lines {
             if let Ok(sent) = line {
                let ws = tokenizer.tokenize(&sent);
                for w in ws.iter() {
                   let hw: u64 = Hash::new(w).into();
                   println!("{}", hw);
                }
             }
          }
       }
   }
}
