use crate::lang::Lang;

pub struct Stemmer {
   stemmer: Option<rust_stemmers::Stemmer>,
}

impl Stemmer {
   pub fn new(lang: &Lang) -> Stemmer {
      let alg = if lang==&Lang::Arb { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Arabic)) }
           else if lang==&Lang::Dan { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Danish)) }
           else if lang==&Lang::Nld { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Dutch)) }
           else if lang==&Lang::Eng { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English)) }
           else if lang==&Lang::Fra { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::French)) }
           else if lang==&Lang::Deu { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::German)) }
           else if lang==&Lang::Ell { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Greek)) }
           else if lang==&Lang::Hun { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Hungarian)) }
           else if lang==&Lang::Ita { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Italian)) }
           else if lang==&Lang::Por { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Portuguese)) }
           else if lang==&Lang::Ron { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Romanian)) }
           else if lang==&Lang::Rus { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Russian)) }
           else if lang==&Lang::Spa { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Spanish)) }
           else if lang==&Lang::Swe { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Swedish)) }
           else if lang==&Lang::Tam { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Tamil)) }
           else if lang==&Lang::Tur { Some(rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::Turkish)) }
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
