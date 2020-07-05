use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use so_many_words::en::tensor::DictionLayer;
use so_many_words::en::tokens::tokenize;

fn main() {
   let _nn = DictionLayer::new();

   if let Ok(lines) = read_lines("data/gutenberg_sentences.txt") {
      for line in lines {
         if let Ok(sent) = line {
            let ws = tokenize(&sent);
            println!("{}", ws.join(" "));
         }
      }
   }

   //softmax vocabulary words from seed
   //  data/wiktionary_en.txt
   //over
   //  data/wiktionary_sentences_en.txt
   //onto
   //  data/wordusage_en.txt
}
