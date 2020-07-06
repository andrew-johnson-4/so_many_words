use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use so_many_words::en::tokens::tokenize;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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
