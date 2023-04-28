use std::error::Error;
use std::fmt;

#[macro_use]
mod macros;
// mod bigint;
mod biguint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBigIntError {
    kind: BigIntErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BigIntErrorKind {
    Empty,
    InvalidDigit,
}

impl ParseBigIntError {
    fn __description(&self) -> &str {
        use crate::BigIntErrorKind::*;
        match self.kind {
            Empty => "cannot parse integer from empty string",
            InvalidDigit => "invalid digit found in string",
        }
    }

    fn empty() -> Self {
        ParseBigIntError {
            kind: BigIntErrorKind::Empty,
        }
    }

    fn invalid() -> Self {
        ParseBigIntError {
            kind: BigIntErrorKind::InvalidDigit,
        }
    }
}

impl fmt::Display for ParseBigIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}

impl Error for ParseBigIntError {
    fn description(&self) -> &str {
        self.__description()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromBigIntError<T> {
    original: T,
}

impl<T> TryFromBigIntError<T> {
    fn new(original: T) -> Self {
        TryFromBigIntError { original }
    }

    fn __description(&self) -> &str {
        "out of range conversion regarding big integer attempted"
    }

    pub fn into_original(self) -> T {
        self.original
    }
}

impl<T> std::error::Error for TryFromBigIntError<T>
where
    T: fmt::Debug,
{
    fn description(&self) -> &str {
        self.__description()
    }
}

impl<T> fmt::Display for TryFromBigIntError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}
