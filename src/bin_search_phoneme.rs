use std::env;
use std::fs::File;
use fst::{Map,IntoStreamer,Streamer};
use fst::automaton::Levenshtein;
use memmap::Mmap;
use eudex::Hash;

fn main() {
   let lang = "eng";
   let load_file = format!("data/phoneme_{}.fst", lang);
   let mmap = unsafe {
      Mmap::map(
         &File::open(&load_file).expect(&format!("No file data file: {}", load_file))
      ).expect("unable to mmap file")
   };
   let map = Map::new(mmap).expect("Could not create fst::Map from file");

   let args: Vec<String> = env::args().collect();
   for term in args[1..].iter() {
      let th = Hash::new(term);
      let lev = Levenshtein::new(term, 3).unwrap();
      let mut stream = map.search(&lev).into_stream();

      println!("{}", term);
      let mut results = Vec::new();
      while let Some((key,_)) = stream.next() {
         let key = String::from_utf8_lossy(key).to_string();
         let kh = Hash::new(&key);
         let dh = (th - kh).dist();
         results.push((dh, key));
      }

      results.sort();
      for (_,term) in results[..10].iter() {
         println!("\t{}", term);
      }
   }
}
