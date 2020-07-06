use crate::en::grammar::WordUsage;
use crate::util::read_lines;
use radix_trie::Trie;

pub struct Dictionary {
   diction: Trie<String,WordUsage>
}
impl Dictionary {
   pub fn new() -> Dictionary {
      Dictionary {
         diction: Trie::new()
      }
   }
   pub fn insert(&mut self, key: String, usage: WordUsage) {
      if let Some(record) = self.diction.get_mut(&key) {
         *record |= usage;
      } else {
         self.diction.insert(key, usage);
      }
   }
   pub fn load(&mut self, path: &str) {
      if let Ok(lines) = read_lines(path) {
         for line in lines {
            if let Ok(def) = line {
               let ds = def.split_whitespace().collect::<Vec<&str>>();
               if ds.len()!=2 { continue; }
               let word = ds[0].to_string();
               let usage = ds[1].to_string();
               for u in usage.split(",") {
                  println!("{}: {}", word, u);
               }
            }
         }
      }
   }
   pub fn usage(&self, word: &str) -> WordUsage {
      WordUsage::NONE
   }
}
