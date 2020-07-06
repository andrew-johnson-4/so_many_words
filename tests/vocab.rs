use so_many_words::en::grammar::WordUsage;
use so_many_words::en::vocabulary::Dictionary;

#[test]
fn vocab0() {
   let mut d = Dictionary::new();
   d.load("data/testy.txt");
   assert!(
      d.usage("abc123") == WordUsage::NONE
   );
}

#[test]
fn vocab1() {
   let mut d = Dictionary::new();
   d.load("data/testy.txt");
   assert!(
      d.usage("123").contains(WordUsage::NUMERAL | WordUsage::NOUN | WordUsage::ADJECTIVE)
   );
}

#[test]
fn vocab2() {
   let mut d = Dictionary::new();
   d.load("data/testy.txt");
   assert!(
      d.usage(",").contains(WordUsage::PUNCTUATION)
   );
}

#[test]
fn vocab3() {
   let mut d = Dictionary::new();
   d.load("data/testy.txt");
   assert!(
      d.usage("I").contains(WordUsage::PRONOUN)
   );
}

#[test]
fn vocab4() {
   let mut d = Dictionary::new();
   d.load("data/testy.txt");
   assert!(
      d.usage("climb").contains(WordUsage::VERB)
   );
}

#[test]
fn vocab5() {
   let mut d = Dictionary::new();
   d.load("data/testy.txt");
   assert!(
      d.usage("the").contains(WordUsage::DETERMINER)
   );
}

#[test]
fn vocab6() {
   let mut d = Dictionary::new();
   d.load("data/testy.txt");
   assert!(
      d.usage("fence").contains(WordUsage::NOUN)
   );
}

#[test]
fn vocab7() {
   let mut d = Dictionary::new();
   d.load("data/testy.txt");
   assert!(
      d.usage("quickly").contains(WordUsage::ADVERB)
   );
}
