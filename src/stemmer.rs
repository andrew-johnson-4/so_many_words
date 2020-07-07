
pub struct Stemmer {
   stemmer: rust_stemmers::Stemmer,
}

impl Stemmer {
   pub fn new(lang: &str) -> Stemmer {
      let lang = lang.to_lowercase();
      let alg = if lang=="arabic" { rust_stemmers::Algorithm::Arabic }
           else if lang=="danish" { rust_stemmers::Algorithm::Danish }
           else if lang=="dutch" { rust_stemmers::Algorithm::Dutch }
           else if lang=="english" { rust_stemmers::Algorithm::English }
           else if lang=="french" { rust_stemmers::Algorithm::French }
           else if lang=="german" { rust_stemmers::Algorithm::German }
           else if lang=="greek" { rust_stemmers::Algorithm::Greek }
           else if lang=="hungarian" { rust_stemmers::Algorithm::Hungarian }
           else if lang=="italian" { rust_stemmers::Algorithm::Italian }
           else if lang=="norwegian" { rust_stemmers::Algorithm::Norwegian }
           else if lang=="portuguese" { rust_stemmers::Algorithm::Portuguese }
           else if lang=="romanian" { rust_stemmers::Algorithm::Romanian }
           else if lang=="russian" { rust_stemmers::Algorithm::Russian }
           else if lang=="spanish" { rust_stemmers::Algorithm::Spanish }
           else if lang=="swedish" { rust_stemmers::Algorithm::Swedish }
           else if lang=="tamil" { rust_stemmers::Algorithm::Tamil }
           else if lang=="turkish" { rust_stemmers::Algorithm::Turkish }
           else { panic!("unrecognized language: {}", lang) };
      Stemmer {
         stemmer: rust_stemmers::Stemmer::create(alg)
      }
   }
   pub fn stem(&self, w: &str) -> String {
      let w = w.to_lowercase();
      self.stemmer.stem(&w).to_string()
   }
}
