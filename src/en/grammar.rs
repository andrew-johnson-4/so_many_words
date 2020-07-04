use smart_default::*;

#[derive(SmartDefault)]
pub enum Plurality {
   #[default]
   Neuter,
   Singular,
   Plural,
}
