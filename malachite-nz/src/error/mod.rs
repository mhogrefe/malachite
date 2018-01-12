use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
/// An error which can be returned when parsing an `Integer`.
pub struct ParseIntegerError {
    pub kind: ParseErrorKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseErrorKind {
    InvalidDigit,
    NoDigits,
}

//TODO test
impl Error for ParseIntegerError {
    fn description(&self) -> &str {
        use self::ParseErrorKind::*;
        match self.kind {
            InvalidDigit => "invalid digit found in string",
            NoDigits => "string has no digits",
        }
    }
}

//TODO test
impl Display for ParseIntegerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
