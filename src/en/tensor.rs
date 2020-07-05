use crate::en::grammar::WordUsage;

pub struct WordUsageTensor {
   pub tensor: [f64; 32]
}

impl From<WordUsage> for WordUsageTensor {
   fn from(wu: WordUsage) -> Self {
      let bits = wu.bits();
      WordUsageTensor {
         tensor: [
            if (bits & 0b00000000_00000000_00000000_00000001)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00000000_00000010)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00000000_00000100)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00000000_00001000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00000000_00010000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00000000_00100000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00000000_01000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00000000_10000000)>0 { 1.0 } else { 0.0 },

            if (bits & 0b00000000_00000000_00000001_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00000010_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00000100_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00001000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00010000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_00100000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_01000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000000_10000000_00000000)>0 { 1.0 } else { 0.0 },

            if (bits & 0b00000000_00000001_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000010_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00000100_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00001000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00010000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_00100000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_01000000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000000_10000000_00000000_00000000)>0 { 1.0 } else { 0.0 },

            if (bits & 0b00000001_00000000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000010_00000000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00000100_00000000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00001000_00000000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00010000_00000000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b00100000_00000000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b01000000_00000000_00000000_00000000)>0 { 1.0 } else { 0.0 },
            if (bits & 0b10000000_00000000_00000000_00000000)>0 { 1.0 } else { 0.0 },
         ]
      }
   }
}

impl std::ops::Add for WordUsageTensor {
   type Output = Self;

   fn add(self, other: Self) -> Self {
      let mut tensor: [f64; 32] = [0.0; 32];
      for i in 0..32 {
         tensor[i] += self.tensor[i];
         tensor[i] += other.tensor[i];
      }
      WordUsageTensor {
         tensor: tensor
      }
   }
}

impl WordUsageTensor {
   pub fn multiply(&mut self, c: f64) {
      for i in 0..32 {
         self.tensor[i] *= c;
      }
   }
}

/*
use radix_trie::Trie;

pub struct DictionNetwork {
   diction: Trie<String,WordUsageTensor>
}
impl DictionNetwork {
   pub fn new() -> Dictionary {
      Dictionary {
         diction: Trie::new()
      }
   }
   pub fn add(&mut self, key: String, usage: WordUsageTensor) {
      if let Some(record) = self.diction.get_mut(&key) {
         *record += usage;
      } else {
         self.diction.insert(key, usage);
      }
   }
}
*/
