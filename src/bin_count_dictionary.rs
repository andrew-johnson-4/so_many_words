use std::env;
use so_many_words::en::vocabulary::Dictionary;

fn main() {
   let mut dict = Dictionary::new();
   let args: Vec<String> = env::args().collect();
   for fp in args[1..].iter() {
      dict.load(fp);
   }
   println!("|dictionary| = {}", dict.len());
}
