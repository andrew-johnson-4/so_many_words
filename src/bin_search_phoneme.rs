use std::fs::File;
use fst::Map;
use memmap::Mmap;

fn main() {
   let lang = "eng";
   let load_file = format!("data/phoneme_{}.fst", lang);
   let mmap = unsafe {
      Mmap::map(
         &File::open(&load_file).expect(&format!("No file data file: {}", load_file))
      ).expect("unable to mmap file")
   };
   let _map = Map::new(mmap).expect("Could not create fst::Map from file");
}
