use crate::en::grammar::WordUsage;
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
}
