// Copyright © 2026 Steve Shi
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

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
    const fn __description(&self) -> &str {
        use BigIntErrorKind::*;
        match self.kind {
            Empty => "cannot parse integer from empty string",
            InvalidDigit => "invalid digit found in string",
        }
    }

    pub(crate) const fn empty() -> Self {
        Self {
            kind: BigIntErrorKind::Empty,
        }
    }

    pub(crate) const fn invalid() -> Self {
        Self {
            kind: BigIntErrorKind::InvalidDigit,
        }
    }
}

impl core::fmt::Display for ParseBigIntError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.__description().fmt(f)
    }
}

impl core::error::Error for ParseBigIntError {
    fn description(&self) -> &str {
        self.__description()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromBigIntError<T> {
    original: T,
}

impl<T> TryFromBigIntError<T> {
    pub(crate) const fn new(original: T) -> Self {
        Self { original }
    }

    #[allow(clippy::unused_self)]
    const fn __description(&self) -> &str {
        "out of range conversion regarding big integer attempted"
    }

    pub fn into_original(self) -> T {
        self.original
    }
}

impl<T> core::error::Error for TryFromBigIntError<T>
where
    T: core::fmt::Debug,
{
    fn description(&self) -> &str {
        self.__description()
    }
}

impl<T> core::fmt::Display for TryFromBigIntError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.__description().fmt(f)
    }
}
