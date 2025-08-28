use std::{fmt};

use crate::Rule;

#[derive(Debug, Clone)]

pub enum Error {
    ParseError {message: Box<str>},
    LogicError {message: Box<str>},
    TypeError {message: Box<str>},
    RuntimeError {message: Box<str>}
}

impl Error {
    pub fn from_pest_error(err: pest::error::Error<Rule>) -> Error {
        let message = err.variant.message();
        let position_message = match err.line_col {
            pest::error::LineColLocation::Pos((line, col)) => format!("at position {}:{}", line, col),
            pest::error::LineColLocation::Span((l1, c1), (l2, c2)) => format!("at {}:{} - {}:{}", l1, c1, l2, c2),
        };
        Error::ParseError { message: format!("Parsing error: {} {}", message, position_message).into() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Error::ParseError {message} => {
                write!(f, "Got error during parsing: {}", message)
            },
            Error::LogicError{message} => {
                write!(f, "Got logical error: {}", message)
            },
            Error::TypeError{message} => {
                write!(f, "Got type checker error: {}", message)
            }
            Error::RuntimeError{message} => {
                write!(f, "Got runtime error: {}", message)
            }
        }
        
    }
}