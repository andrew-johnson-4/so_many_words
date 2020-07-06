use std::env;
use so_many_words::util::read_lines;
use so_many_words::en::tokens::tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    for fp in args[1..].iter() {
       if let Ok(lines) = read_lines(fp) {
          for line in lines {
             if let Ok(sent) = line {
                let ws = tokenize(&sent);
                for w in ws.iter() {
                   println!("{}", w);
                }
             }
          }
       }
   }
}
