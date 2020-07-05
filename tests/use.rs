use so_many_words::en::grammar;

#[test]
fn use1() {
   let _ = grammar::WordUsage::SINGULAR;
}

#[test]
fn use2() {
   assert_eq!(
      grammar::WordUsage::SINGULAR.contains(grammar::WordUsage::SINGULAR),
      true
   );
}

#[test]
fn use3() {
   assert_eq!(
      grammar::WordUsage::SINGULAR.contains(grammar::WordUsage::PLURAL),
      false
   );
}
