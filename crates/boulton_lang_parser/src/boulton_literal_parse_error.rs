use thiserror::Error;

use crate::BoultonLangTokenKind;

use super::peekable_lexer::LowLevelParseError;

pub(crate) type ParseResult<T> = Result<T, BoultonLiteralParseError>;

/// Errors tha make semantic sense when referring to parsing a Boulton literal
#[derive(Error, Debug)]
pub enum BoultonLiteralParseError {
    #[error("{error}")]
    ParseError { error: LowLevelParseError },

    #[error("Expected scalar, type, interface, union, enum, input object, schema or directive, found \"{found_text}\"")]
    TopLevelSchemaDeclarationExpected { found_text: String },

    #[error("Unable to parse constant value")]
    UnableToParseConstantValue,

    #[error("Invalid integer value. Received {text}")]
    InvalidIntValue { text: String },

    #[error("Invalid float value. Received {text}")]
    InvalidFloatValue { text: String },

    #[error("Expected a type (e.g. String, [String], or String!)")]
    ExpectedTypeAnnotation,

    #[error("Leftover tokens. Next token: {token}")]
    LeftoverTokens { token: BoultonLangTokenKind },
}

impl From<LowLevelParseError> for BoultonLiteralParseError {
    fn from(error: LowLevelParseError) -> Self {
        BoultonLiteralParseError::ParseError { error }
    }
}
