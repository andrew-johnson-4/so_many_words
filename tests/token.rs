use so_many_words::en::tokens::{is_numeral,is_punctuation};


#[test]
fn token1() {
   assert!(
      is_numeral("1")
   );
}

#[test]
fn token2() {
   assert!(
      is_numeral("0123456789")
   );
}

#[test]
fn token3() {
   assert!(
      !is_numeral("0a")
   );
}

#[test]
fn token4() {
   assert!(
      !is_punctuation("0a")
   );
}

#[test]
fn token5() {
   assert!(
      is_punctuation(",")
   );
}

#[test]
fn token6() {
   assert!(
      is_punctuation("{")
   );
}
