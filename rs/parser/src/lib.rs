use pest;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "glicol.pest"]
pub struct GlicolParser;