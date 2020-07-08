use std::env;
use so_many_words::util::read_lines;
use std::fs::File;
use fst::MapBuilder;
use eudex::Hash;

fn main() {
   let lang = "eng".to_string();
   let args: Vec<String> = env::args().collect();
   let wtr = std::io::BufWriter::new(File::create(&format!("data/phoneme_{}.fst",lang)).expect("Unable to create file"));
   let mut build = MapBuilder::new(wtr).expect("Unable to create fst::MapBuilder");

   let mut i = 0;
   for fp in args[1..].iter() {
      if let Ok(lines) = read_lines(fp) {
         for line in lines {
            if let Ok(sent) = line {
               if let Some(word) = sent.split_whitespace().next() {
                  i += 1;
                  if (i%1000)==0 { println!("pass build line: {}", i); }
                  let phone: u64 = Hash::new(word).into();
                  build.insert(word, phone).unwrap();
               }
            }
         }
      }
   }
   build.finish().expect("Unable to finish build of fst::MapBuilder");
}
