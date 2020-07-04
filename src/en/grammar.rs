use smart_default::*;

#[derive(SmartDefault)]
pub enum Plurality {
   #[default]
   Neuter,
   Singular,
   Plural,
}

#[derive(SmartDefault)]
pub enum VerbTense {
   #[default]
   PresentSimple,
   PresentContinuous,
   PresentPerfect,
   PresentPerfectContinuous,
   PastSimple,
   PastContinuous,
   PastPerfect,
   PastPerfectContinuous,
   FutureSimple,
   FuturePerfect,
   FutureContinuous,
   FuturePerfectContinuous,
}

#[derive(SmartDefault)]
pub enum PartOfSpeech {
   #[default]
   Noun,
   Verb,
   Adjective,
   Adverb,
   Pronoun,
   Preposition,
   Conjunction,
   Interjection,
   Numeral,
   Article,
   Determiner,
}
