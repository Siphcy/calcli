use std::fmt;

#[derive(Debug)]
pub enum EvalError {
    EmptyInput,
    DefError(DefError),
    ParseError(String),
}

#[derive(Debug)]
pub enum DefError {
    InvalidDefinitionSyntax(String),
    InvalidDefinitionName(String),
    InvalidDefinitionIteration(String),
    MismatchedBatch(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::EmptyInput => write!(f, "Empty input"),
            EvalError::DefError(e) => write!(f, "{}", e),
            EvalError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl fmt::Display for DefError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DefError::InvalidDefinitionSyntax(msg) => write!(f, "Invalid let syntax: {}", msg),
            DefError::InvalidDefinitionName(msg) => write!(f, "Invalid variable name: {}", msg),
            DefError::InvalidDefinitionIteration(msg) => write!(f, "Invalid variable iteration: {}", msg),
            DefError::MismatchedBatch(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for EvalError {}
impl std::error::Error for DefError {}

impl From<DefError> for EvalError {
    fn from(e: DefError) -> Self {
        EvalError::DefError(e)
    }
}
