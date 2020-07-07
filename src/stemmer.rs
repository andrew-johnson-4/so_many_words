use crate::lang::Lang;

pub struct Stemmer {
   stemmer: Option<rust_stemmers::Stemmer>,
}

impl Stemmer {
   pub fn new(lang: &Lang) -> Stemmer {
      let alg = if lang==&Lang::Arabic { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Arabic)) }
           else if lang==&Lang::Danish { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Danish)) }
           else if lang==&Lang::Dutch { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Dutch)) }
           else if lang==&Lang::English { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English)) }
           else if lang==&Lang::French { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::French)) }
           else if lang==&Lang::German { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::German)) }
           else if lang==&Lang::Greek { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Greek)) }
           else if lang==&Lang::Hungarian { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Hungarian)) }
           else if lang==&Lang::Italian { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Italian)) }
           else if lang==&Lang::Norwegian { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Norwegian)) }
           else if lang==&Lang::Portuguese { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Portuguese)) }
           else if lang==&Lang::Romanian { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Romanian)) }
           else if lang==&Lang::Russian { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Russian)) }
           else if lang==&Lang::Spanish { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Spanish)) }
           else if lang==&Lang::Swedish { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Swedish)) }
           else if lang==&Lang::Tamil { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Tamil)) }
           else if lang==&Lang::Turkish { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Turkish)) }
           else { None };
      Stemmer {
         stemmer: alg
      }
   }
   pub fn stem(&self, w: &str) -> String {
      let w = w.to_lowercase();
      if let Some(ref stemmer) = self.stemmer {
         stemmer.stem(&w).to_string()
      } else { w }
   }
}
