// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use alloc::string::String;
use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;

// An imaginary term, with its sign if it has one: an optional sign, an optional numerator, an 'i',
// and an optional denominator, as in "-5i/6". The 'i' must directly follow the numerator, so "2/3i"
// is invalid. A missing numerator means 1, so "i/2" works, but so does "1i/2": requiring producers
// to elide degenerate numerators would force a special case on them. The term is rewritten with the
// 'i' deleted (and the elided numerator restored) and parsed as a `Rational`, which brings along
// `Rational`'s permissiveness: "2i/4" means i/2, and "+i" is accepted exactly where `Rational`'s
// parser accepts a leading '+'.
fn parse_imaginary_term(s: &str) -> Result<Rational, ()> {
    let j = s.find('i').ok_or(())?;
    let numerator = &s[..j];
    let rest = &s[j + 1..];
    if numerator.contains('/') || !rest.is_empty() && !rest.starts_with('/') {
        return Err(());
    }
    let mut rational_string = String::with_capacity(s.len());
    rational_string.push_str(numerator);
    if matches!(numerator, "" | "+" | "-") {
        rational_string.push('1');
    }
    rational_string.push_str(rest);
    Rational::from_str(&rational_string)
}

impl FromStr for GaussianRational {
    type Err = ();

    /// Converts a string to a [`GaussianRational`].
    ///
    /// If the string does not represent a valid [`GaussianRational`], an `Err` is returned. The
    /// grammar is strict about structure: the real term must precede the imaginary term, the `'i'`
    /// must directly follow the imaginary numerator (so `"2/3i"` is invalid), and no whitespace is
    /// allowed. It is permissive about coefficients: `"1i/2"`, `"0i"`, and fractions not in lowest
    /// terms like `"2i/4"` are all accepted, although [`Display`](core::fmt::Display) never
    /// produces them. Each component follows [`Rational`]'s syntax, including its optional leading
    /// `'+'`.
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(GaussianRational::from_str("0").unwrap().to_string(), "0");
    /// assert_eq!(
    ///     GaussianRational::from_str("-2/3").unwrap().to_string(),
    ///     "-2/3"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("i/2").unwrap().to_string(),
    ///     "i/2"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("-5i/6").unwrap().to_string(),
    ///     "-5i/6"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("2/3-5i/6").unwrap().to_string(),
    ///     "2/3-5i/6"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("2i/4").unwrap().to_string(),
    ///     "i/2"
    /// );
    /// assert_eq!(GaussianRational::from_str("1i").unwrap().to_string(), "i");
    /// assert_eq!(
    ///     GaussianRational::from_str("2/3+0i").unwrap().to_string(),
    ///     "2/3"
    /// );
    ///
    /// assert!(GaussianRational::from_str("").is_err());
    /// assert!(GaussianRational::from_str("2/3i").is_err());
    /// assert!(GaussianRational::from_str("i+1").is_err());
    /// assert!(GaussianRational::from_str("5i/0").is_err());
    /// assert!(GaussianRational::from_str("1 + i").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, ()> {
        let Some(i_index) = s.find('i') else {
            // No imaginary term: the whole string is the real part.
            return Ok(Self {
                real: Rational::from_str(s)?,
                imaginary: Rational::ZERO,
            });
        };
        // The sign joining the real and imaginary terms is the last '+' or '-' before the 'i',
        // except at the start of the string, where it is the imaginary term's own sign. Signs
        // within a component (a denominator's '+') come with `Rational`'s syntax and are left in
        // place; any other stray sign is left inside a component, whose parse then fails.
        match s[..i_index].rfind(['+', '-']).filter(|&k| k != 0) {
            None => Ok(Self {
                real: Rational::ZERO,
                imaginary: parse_imaginary_term(s)?,
            }),
            Some(k) => Ok(Self {
                real: Rational::from_str(&s[..k])?,
                imaginary: parse_imaginary_term(&s[k..])?,
            }),
        }
    }
}
