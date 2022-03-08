use glicol_parser::Rule;
use pest::error::Error;
pub use pest::error::ErrorVariant;

#[derive(Debug)]
pub enum EngineError {
    ParsingError( pest::error::Error<glicol_parser::Rule>),
    NonExistReference(&'static str),
    NonExsitSample(&'static str),
}

impl std::convert::From<Error<Rule>> for EngineError {
    fn from(err: Error<Rule>) -> EngineError {
        EngineError::ParsingError(err)
    }
}

pub fn get_error_info(e: Error<Rule>) -> (Vec<Rule>,Vec<Rule>) {
    match e.variant {
        ErrorVariant::ParsingError { positives, negatives} => {
            return (positives, negatives)              
        },
        _ => {
            unimplemented!();
        }
    }
}