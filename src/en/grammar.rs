use bitflags::bitflags;

bitflags! {
   pub struct WordUsage: u64 {
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

pub enum EBNF {
   Symbol(WordUsage),
   Sequence(Vec<EBNF>),
   Choice(Vec<EBNF>),
}

pub struct RigidLanguage;
impl RigidLanguage {
   pub fn new() -> EBNF {
      let rigid_subject      : EBNF = EBNF::Symbol(WordUsage::NOUN);
      let rigid_verb         : EBNF = EBNF::Symbol(WordUsage::VERB);
      let rigid_direct_object: EBNF = EBNF::Symbol(WordUsage::NOUN);
      let rigid_adverb       : EBNF = EBNF::Symbol(WordUsage::ADVERB);
      let rigid_sentence     : EBNF = EBNF::Sequence(vec![rigid_subject, rigid_verb, rigid_direct_object, rigid_adverb]);
      rigid_sentence
   }
}
