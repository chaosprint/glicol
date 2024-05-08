use glicol_parser::Rule;
use pest::error::Error;
pub use pest::error::ErrorVariant;

#[derive(Debug)]
pub enum EngineError {
    ParsingError(Box<pest::error::Error<glicol_parser::Rule>>),
    NonExistReference(String),
    NonExistSample(String),
}

impl From<Box<Error<Rule>>> for EngineError {
    fn from(err: Box<Error<Rule>>) -> EngineError {
        EngineError::ParsingError(err)
    }
}

pub fn get_error_info(e: Error<Rule>) -> (Vec<Rule>, Vec<Rule>) {
    match e.variant {
        ErrorVariant::ParsingError {
            positives,
            negatives,
        } => (positives, negatives),
        _ => unimplemented!(),
    }
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParsingError(err) => writeln!(f, "Parsing error: {err}"),
            EngineError::NonExistSample(v) => writeln!(f, "There is no sample named {v}s"),
            EngineError::NonExistReference(v) => writeln!(f, "There is no reference named {v}"),
        }
    }
}
