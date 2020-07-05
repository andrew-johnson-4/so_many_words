use so_many_words::en::tokens::{is_numeral,is_punctuation,is_word,tokenize};


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

#[test]
fn token7() {
   assert!(
      is_word("albeit")
   );
}

#[test]
fn token8() {
   assert!(
      is_word("gr8")
   );
}

#[test]
fn token9() {
   let v: Vec<String> = Vec::new();
   assert_eq!(
      tokenize("   "),
      v
   )
}

#[test]
fn token10() {
   assert_eq!(
      tokenize("abcd8 999   :,"),
      vec!["abcd8".to_string(), "999".to_string(), ":".to_string(), ",".to_string()]
   )
}

