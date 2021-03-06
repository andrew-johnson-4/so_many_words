use bitflags::bitflags;

bitflags! {
   pub struct WordUsage: u64 {
      const NONE                       = 0;

      const SINGULAR                   = 0b00000000_00000000_00000000_00000001;
      const PLURAL                     = 0b00000000_00000000_00000000_00000010;
      //2 Pluralities

      const LINKING                    = 0b00000000_00000000_00000000_00000100;
      const INTRANSITIVE               = 0b00000000_00000000_00000000_00001000;
      const MONOTRANSITIVE             = 0b00000000_00000000_00000000_00010000;
      const DITRANSITIVE               = 0b00000000_00000000_00000000_00100000;
      //4 Transitivities

      const PRESENT_SIMPLE             = 0b00000000_00000000_00000001_00000000;
      const PRESENT_CONTINUOUS         = 0b00000000_00000000_00000010_00000000;
      const PRESENT_PERFECT            = 0b00000000_00000000_00000100_00000000;
      const PRESENT_PERFECT_CONTINUOUS = 0b00000000_00000000_00001000_00000000;
      const PAST_SIMPLE                = 0b00000000_00000000_00010000_00000000;
      const PAST_CONTINUOUS            = 0b00000000_00000000_00100000_00000000;
      const PAST_PERFECT               = 0b00000000_00000000_01000000_00000000;
      const PAST_PERFECT_CONTINUOUS    = 0b00000000_00000000_10000000_00000000;
      const FUTURE_SIMPLE              = 0b00000000_00000001_00000000_00000000;
      const FUTURE_PERFECT             = 0b00000000_00000010_00000000_00000000;
      const FUTURE_CONTINUOUS          = 0b00000000_00000100_00000000_00000000;
      const FUTURE_PERFECT_CONTINUOUS  = 0b00000000_00001000_00000000_00000000;
      //12 Verb Tenses

      const PUNCTUATION                = 0b00000000_00010000_00000000_00000000;
      const NUMERAL                    = 0b00000000_00100000_00000000_00000000;
      const ARTICLE                    = 0b00000000_01000000_00000000_00000000;
      const DETERMINER                 = 0b00000000_10000000_00000000_00000000;
      const NOUN                       = 0b00000001_00000000_00000000_00000000;
      const VERB                       = 0b00000010_00000000_00000000_00000000;
      const ADJECTIVE                  = 0b00000100_00000000_00000000_00000000;
      const ADVERB                     = 0b00001000_00000000_00000000_00000000;
      const PRONOUN                    = 0b00010000_00000000_00000000_00000000;
      const PREPOSITION                = 0b00100000_00000000_00000000_00000000;
      const CONJUNCTION                = 0b01000000_00000000_00000000_00000000;
      const INTERJECTION               = 0b10000000_00000000_00000000_00000000;
      //12 Parts of Speech
   }
}
impl WordUsage {
   pub fn raise(&mut self, flag: &str) {
      let flag = flag.to_uppercase();
      if flag=="NONE" {}
      else if flag=="PUNCTUATION" { *self |= WordUsage::PUNCTUATION }
      else if flag=="NUMERAL" { *self |= WordUsage::NUMERAL }
      else if flag=="ARTICLE" { *self |= WordUsage::ARTICLE }
      else if flag=="DETERMINER" { *self |= WordUsage::DETERMINER }
      else if flag=="NOUN" { *self |= WordUsage::NOUN }
      else if flag=="VERB" { *self |= WordUsage::VERB }
      else if flag=="ADJECTIVE" { *self |= WordUsage::ADJECTIVE }
      else if flag=="ADVERB" { *self |= WordUsage::ADVERB }
      else if flag=="PRONOUN" { *self |= WordUsage::PRONOUN }
      else if flag=="PREPOSITION" { *self |= WordUsage::PREPOSITION }
      else if flag=="CONJUNCTION" { *self |= WordUsage::CONJUNCTION }
      else if flag=="INTERJECTION" { *self |= WordUsage::INTERJECTION }
      else if flag=="SINGULAR" { *self |= WordUsage::SINGULAR }
      else if flag=="PLURAL" { *self |= WordUsage::PLURAL }
      else if flag=="LINKING" { *self |= WordUsage::LINKING }
      else if flag=="INTRANSITIVE" { *self |= WordUsage::INTRANSITIVE }
      else if flag=="MONOTRANSITIVE" { *self |= WordUsage::MONOTRANSITIVE }
      else if flag=="DITRANSITIVE" { *self |= WordUsage::DITRANSITIVE }
      else if flag=="PRESENT_SIMPLE" { *self |= WordUsage::PRESENT_SIMPLE }
      else if flag=="PRESENT_CONTINUOUS" { *self |= WordUsage::PRESENT_CONTINUOUS }
      else if flag=="PRESENT_PERFECT" { *self |= WordUsage::PRESENT_PERFECT }
      else if flag=="PRESENT_PERFECT_CONTINUOUS" { *self |= WordUsage::PRESENT_PERFECT_CONTINUOUS }
      else if flag=="PAST_SIMPLE" { *self |= WordUsage::PAST_SIMPLE }
      else if flag=="PAST_CONTINUOUS" { *self |= WordUsage::PAST_CONTINUOUS }
      else if flag=="PAST_PERFECT" { *self |= WordUsage::PAST_PERFECT }
      else if flag=="PAST_PERFECT_CONTINUOUS" { *self |= WordUsage::PAST_PERFECT_CONTINUOUS }
      else if flag=="FUTURE_SIMPLE" { *self |= WordUsage::FUTURE_SIMPLE }
      else if flag=="FUTURE_CONTINUOUS" { *self |= WordUsage::FUTURE_CONTINUOUS }
      else if flag=="FUTURE_PERFECT" { *self |= WordUsage::FUTURE_PERFECT }
      else if flag=="FUTURE_PERFECT_CONTINUOUS" { *self |= WordUsage::FUTURE_PERFECT_CONTINUOUS }
      else { panic!("unknown WordUsage flag: {}", flag) }
   }
}

impl std::fmt::Display for WordUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       let out = format!("{:?}", self).replace(" | ",",").to_lowercase().replace("none","");
       write!(f, "{}", out)
    }
}

pub struct GrammarVertex {
   pub is_terminal: bool,
   pub word: WordUsage,
}
impl GrammarVertex {
   pub fn new(word: WordUsage, is_terminal: bool) -> GrammarVertex {
      GrammarVertex {
         is_terminal: is_terminal,
         word: word,
      }
   }
}

pub struct RegularLanguage {
   pub nodes: Vec<GrammarVertex>,
   pub adjacency_list: Vec<(usize,usize)>,
}
impl RegularLanguage {
   pub fn new(is_terminal: bool) -> RegularLanguage {
      RegularLanguage {
         nodes: vec![
            GrammarVertex::new(WordUsage::NONE, is_terminal)
         ],
         adjacency_list: Vec::new(),
      }
   }
   pub fn put(&mut self, n: GrammarVertex) -> usize {
      let i = self.nodes.len();
      self.nodes.push(n);
      i
   }
   pub fn v(&mut self, word: WordUsage, is_terminal: bool) -> usize {
      self.put(GrammarVertex::new(word, is_terminal))
   }
   pub fn e(&mut self, a: usize, b: usize) {
      self.adjacency_list.push((a, b));
   }
}

pub struct ParseLine {
   pub passed: Vec<WordUsage>,
   pub at_node: usize,
}
pub struct ParseLines {
   pub lines: Vec<ParseLine>,
}

pub struct RigidLanguage;
impl RigidLanguage {
   pub fn new() -> RegularLanguage {
      let mut l                = RegularLanguage::new(false);
      let rigid_subject        = l.v(WordUsage::NOUN, false);
      let rigid_verb           = l.v(WordUsage::VERB, false);
      let rigid_direct_object  = l.v(WordUsage::NOUN, false);
      let rigid_adverb         = l.v(WordUsage::ADVERB, true);
      l.e(0, rigid_subject);
      l.e(rigid_subject, rigid_verb);
      l.e(rigid_verb, rigid_direct_object);
      l.e(rigid_direct_object, rigid_adverb);
      l
   }
}
