// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use core::fmt::{Formatter, Result, Write};

// The ten Unicode subscript digits, in order, so that a digit's value is its position.
//
// They are contiguous in Unicode, but writing them out keeps the conversion from needing a fallible
// `char::from_u32`, and shows at a glance which characters are meant.
const SUBSCRIPT_DIGITS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

/// Writes a number as a run of Unicode subscript digits.
///
/// The digits are the Subscripts block's `₀` through `₉`, so that 10 is written `₁₀`. There
/// is no sign and no leading zero: zero is written `₀`, and nothing else begins with it.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `n.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::chars::scripts::fmt_subscript_digits;
/// use std::fmt::{Display, Formatter, Result};
///
/// struct Indexed(u64);
///
/// impl Display for Indexed {
///     fn fmt(&self, f: &mut Formatter) -> Result {
///         f.write_str("x")?;
///         fmt_subscript_digits(self.0, f)
///     }
/// }
///
/// assert_eq!(Indexed(0).to_string(), "x₀");
/// assert_eq!(Indexed(10).to_string(), "x₁₀");
/// ```
pub fn fmt_subscript_digits(n: u64, f: &mut Formatter) -> Result {
    // The digits come out least-significant first, so they are buffered rather than written as they
    // are found. Twenty is how many a `u64` can have.
    let mut digits = [0u8; 20];
    let mut len = 0;
    let mut n = n;
    loop {
        digits[len] = u8::wrapping_from(n % 10);
        len += 1;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    for &d in digits[..len].iter().rev() {
        f.write_char(SUBSCRIPT_DIGITS[usize::from(d)])?;
    }
    Ok(())
}

/// Reads a number written as a run of Unicode subscript digits.
///
/// This accepts exactly what [`fmt_subscript_digits`] writes, and nothing else: the empty string, a
/// string holding anything but a subscript digit, a string with a leading zero, and a number too
/// large for a [`u64`] are all rejected. Accepting only the one spelling of a number is what makes
/// the two functions inverse to each other.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
///
/// # Examples
/// ```
/// use malachite_base::chars::scripts::parse_subscript_digits;
///
/// assert_eq!(parse_subscript_digits("₀"), Some(0));
/// assert_eq!(parse_subscript_digits("₁₀"), Some(10));
/// assert_eq!(parse_subscript_digits(""), None);
/// assert_eq!(parse_subscript_digits("10"), None);
/// assert_eq!(parse_subscript_digits("₀₁"), None);
/// ```
pub fn parse_subscript_digits(s: &str) -> Option<u64> {
    let mut n: u64 = 0;
    let mut len = 0;
    for c in s.chars() {
        let d = SUBSCRIPT_DIGITS.iter().position(|&x| x == c)?;
        if len == 1 && n == 0 {
            // A leading zero, which nothing this reads back is written with.
            return None;
        }
        n = n.checked_mul(10)?.checked_add(u64::exact_from(d))?;
        len += 1;
    }
    if len == 0 { None } else { Some(n) }
}
