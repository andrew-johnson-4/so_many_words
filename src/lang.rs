
#[derive(PartialEq,Eq)]
pub enum Lang {
   Arabic,
   Danish,
   Dutch,
   English,
   French,
   German,
   Greek,
   Hungarian,
   Italian,
   Norwegian,
   Portuguese,
   Romanian,
   Russian,
   Spanish,
   Swedish,
   Tamil,
   Turkish,
   None,
}

impl From<String> for Lang {
   fn from(lang: String) -> Self {
      let lang = lang.to_lowercase();
      match lang.as_str() {
         "arabic" => Lang::Arabic,
         "danish" => Lang::Danish,
         "dutch" => Lang::Dutch,
         "english" => Lang::English,
         "french" => Lang::French,
         "german" => Lang::German,
         "greek" => Lang::Greek,
         "hungarian" => Lang::Hungarian,
         "italian" => Lang::Italian,
         "norwegian" => Lang::Norwegian,
         "portuguese" => Lang::Portuguese,
         "romanian" => Lang::Romanian,
         "russian" => Lang::Russian,
         "spanish" => Lang::Spanish,
         "swedish" => Lang::Swedish,
         "tamil" => Lang::Tamil,
         "turkish" => Lang::Turkish,
         _ => Lang::None,
      }
   }
}
