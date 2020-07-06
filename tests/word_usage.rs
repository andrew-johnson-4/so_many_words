use so_many_words::en::grammar::WordUsage;

#[test]
fn test1() {
   assert_eq!(
      WordUsage::NONE.to_string(),
      "",
   );
}

#[test]
fn test2() {
   assert_eq!(
      WordUsage::VERB.to_string(),
      "verb",
   );
}

#[test]
fn test3() {
   assert_eq!(
      WordUsage::NOUN.to_string(),
      "noun",
   );
}

#[test]
fn test4() {
   assert_eq!(
      (WordUsage::VERB | WordUsage::NOUN).to_string(),
      "noun,verb",
   );
}

#[test]
fn test5() {
   assert_eq!(
      WordUsage::PUNCTUATION.to_string(),
      "punctuation",
   );
}

#[test]
fn test6() {
   assert_eq!(
      WordUsage::NUMERAL.to_string(),
      "numeral",
   );
}

#[test]
fn test7() {
   assert_eq!(
      WordUsage::ARTICLE.to_string(),
      "article",
   );
}

#[test]
fn test8() {
   assert_eq!(
      WordUsage::DETERMINER.to_string(),
      "determiner",
   );
}

#[test]
fn test9() {
   assert_eq!(
      WordUsage::ADJECTIVE.to_string(),
      "adjective",
   );
}

#[test]
fn test10() {
   assert_eq!(
      WordUsage::ADVERB.to_string(),
      "adverb",
   );
}

#[test]
fn test11() {
   assert_eq!(
      WordUsage::PRONOUN.to_string(),
      "pronoun",
   );
}

#[test]
fn test12() {
   assert_eq!(
      WordUsage::PREPOSITION.to_string(),
      "preposition",
   );
}

#[test]
fn test13() {
   assert_eq!(
      WordUsage::CONJUNCTION.to_string(),
      "conjunction",
   );
}

#[test]
fn test14() {
   assert_eq!(
      WordUsage::INTERJECTION.to_string(),
      "interjection",
   );
}
