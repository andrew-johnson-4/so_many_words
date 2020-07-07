
pub struct Stemmer {
   stemmer: rust_stemmers::Stemmer,
}

impl Stemmer {
   pub fn new() -> Stemmer {
      Stemmer {
         stemmer: rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Arabic)
      }
   }
   pub fn stem(&self, w: &str) -> String {
      let w = w.to_lowercase();
      self.stemmer.stem(&w).to_string()
   }
}
