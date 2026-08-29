// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use core::str::FromStr;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};

// An imaginary term standing alone: an optional sign, then an optional coefficient in `Integer`
// syntax. A missing coefficient means 1, so "i" and "-i" work, but so do "1i" and "0i": requiring
// producers to elide degenerate coefficients would force a special case on them, so the parser
// accepts both spellings. A single leading '+' is accepted, exactly as `Integer`'s parser accepts
// "+1".
fn parse_lone_imaginary_coefficient(s: &str) -> Result<Integer, ()> {
    match s {
        "" | "+" => Ok(Integer::ONE),
        "-" => Ok(Integer::NEGATIVE_ONE),
        _ => Integer::from_str(s),
    }
}

// An imaginary term joined to a preceding real term, starting with the joining sign; the sign alone
// means a coefficient of 1 or -1.
fn parse_joined_imaginary_coefficient(s: &str) -> Result<Integer, ()> {
    match s {
        "+" => Ok(Integer::ONE),
        "-" => Ok(Integer::NEGATIVE_ONE),
        _ => {
            if let Some(digits) = s.strip_prefix('+') {
                Integer::from_str(digits)
            } else {
                Integer::from_str(s)
            }
        }
    }
}

impl FromStr for GaussianInteger {
    type Err = ();

    /// Converts a string to a [`GaussianInteger`].
    ///
    /// If the string does not represent a valid [`GaussianInteger`], an `Err` is returned. The
    /// grammar is strict about structure: the real term must precede the imaginary term, the
    /// imaginary term must end in `'i'`, and no whitespace is allowed. It is permissive about
    /// coefficients, much as `Rational`'s parser accepts fractions that are not in lowest terms:
    /// `"1i"`, `"0i"`, `"2+0i"`, and `"0+1i"` are all accepted, although
    /// [`Display`](core::fmt::Display) never produces them. Each component follows [`Integer`]'s
    /// syntax, so leading zeros are allowed, and so is a single leading `'-'` or `'+'` on the
    /// leading term.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(GaussianInteger::from_str("0").unwrap().to_string(), "0");
    /// assert_eq!(GaussianInteger::from_str("-2").unwrap().to_string(), "-2");
    /// assert_eq!(GaussianInteger::from_str("i").unwrap().to_string(), "i");
    /// assert_eq!(GaussianInteger::from_str("-i").unwrap().to_string(), "-i");
    /// assert_eq!(
    ///     GaussianInteger::from_str("2-3i").unwrap().to_string(),
    ///     "2-3i"
    /// );
    /// assert_eq!(GaussianInteger::from_str("1i").unwrap().to_string(), "i");
    /// assert_eq!(GaussianInteger::from_str("0i").unwrap().to_string(), "0");
    /// assert_eq!(GaussianInteger::from_str("2+0i").unwrap().to_string(), "2");
    /// assert_eq!(
    ///     GaussianInteger::from_str("+2+1i").unwrap().to_string(),
    ///     "2+i"
    /// );
    ///
    /// assert!(GaussianInteger::from_str("").is_err());
    /// assert!(GaussianInteger::from_str("i+1").is_err());
    /// assert!(GaussianInteger::from_str("1 + i").is_err());
    /// assert!(GaussianInteger::from_str("2+-3i").is_err());
    /// assert!(GaussianInteger::from_str("2ii").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, ()> {
        let Some(r) = s.strip_suffix('i') else {
            // No imaginary term: the whole string is the real part.
            return Ok(Self {
                real: Integer::from_str(s)?,
                imaginary: Integer::ZERO,
            });
        };
        // The sign joining the real and imaginary terms is the last '+' or '-', except at the start
        // of the string, where a '-' is the imaginary term's own sign. Any other sign characters
        // are left inside a component, whose parse then fails.
        match r.rfind(['+', '-']).filter(|&k| k != 0) {
            None => Ok(Self {
                real: Integer::ZERO,
                imaginary: parse_lone_imaginary_coefficient(r)?,
            }),
            Some(k) => Ok(Self {
                real: Integer::from_str(&r[..k])?,
                imaginary: parse_joined_imaginary_coefficient(&r[k..])?,
            }),
        }
    }
}
