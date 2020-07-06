use so_many_words::en::tokens::Tokenizer;

#[test]
fn token1() {
   let tokenizer = Tokenizer::new();
   assert!(
      tokenizer.is_numeral("1")
   );
}

#[test]
fn token2() {
   let tokenizer = Tokenizer::new();
   assert!(
      tokenizer.is_numeral("0123456789")
   );
}

#[test]
fn token3() {
   let tokenizer = Tokenizer::new();
   assert!(
      !tokenizer.is_numeral("0a")
   );
}

#[test]
fn token4() {
   let tokenizer = Tokenizer::new();
   assert!(
      !tokenizer.is_punctuation("0a")
   );
}

#[test]
fn token5() {
   let tokenizer = Tokenizer::new();
   assert!(
      tokenizer.is_punctuation(",")
   );
}

#[test]
fn token6() {
   let tokenizer = Tokenizer::new();
   assert!(
      tokenizer.is_punctuation("{")
   );
}

#[test]
fn token7() {
   let tokenizer = Tokenizer::new();
   assert!(
      tokenizer.is_word("albeit")
   );
}

#[test]
fn token8() {
   let tokenizer = Tokenizer::new();
   assert!(
      tokenizer.is_word("gr8")
   );
}

#[test]
fn token9() {
   let tokenizer = Tokenizer::new();
   let v: Vec<String> = Vec::new();
   assert_eq!(
      tokenizer.tokenize("   "),
      v
   )
}

#[test]
fn token10() {
   let tokenizer = Tokenizer::new();
   assert_eq!(
      tokenizer.tokenize("abcd8 999   :,"),
      vec!["abcd8".to_string(), "999".to_string(), ":".to_string(), ",".to_string()]
   )
}

