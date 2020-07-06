use crate::en::grammar::WordUsage;
use crate::util::read_lines;
use crate::en::tokens::Tokenizer;
use radix_trie::Trie;

pub struct Dictionary {
   tokenizer: Tokenizer,
   diction: Trie<String,WordUsage>
}
impl Dictionary {
   pub fn new() -> Dictionary {
      Dictionary {
         tokenizer: Tokenizer::new(),
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
               let mut wu = WordUsage::NONE;
               for u in usage.split(",") {
                  wu.raise(u);
               }
               self.insert(word, wu);
            }
         }
      }
   }
   pub fn usage(&self, word: &str) -> WordUsage {
      if self.tokenizer.is_numeral(word) {
         WordUsage::NUMERAL | WordUsage::NOUN | WordUsage::ADJECTIVE |
         (if word=="1" { WordUsage::SINGULAR } else { WordUsage::PLURAL })
      } else if self.tokenizer.is_punctuation(word) {
         WordUsage::PUNCTUATION
      } else if let Some(usage) = self.diction.get(word) {
         usage.clone()
      } else {
         WordUsage::NONE
      }
   }
}
