// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::fmt::{Display, Formatter, Result, Write};
use malachite_base::strings::typst::ToTypst;

// Writes a [`Rational`]'s magnitude: a fraction if its denominator is not 1, and the numerator
// alone if it is. The sign is the caller's to write, so that it falls outside the fraction.
fn fmt_typst_unsigned(q: &Rational, f: &mut Formatter) -> Result {
    let n = q.numerator_ref();
    let d = q.denominator_ref();
    if *d == 1u32 {
        return Display::fmt(n, f);
    }
    f.write_str("frac(")?;
    Display::fmt(n, f)?;
    f.write_str(", ")?;
    Display::fmt(d, f)?;
    f.write_str(")")
}

impl ToTypst for Rational {
    /// Writes a [`Rational`] as a Typst math-mode fragment.
    ///
    /// A value whose denominator is 1 is written as its numerator alone, so an integer is an
    /// ordinary number and zero is `0`. Otherwise it is written as a fraction.
    ///
    /// A negative value's sign is written outside the fraction, as ``-frac(2, 3)`` rather than
    /// ``frac(-2, 3)``: the sign belongs to the value, not to its numerator, and a minus sign set
    /// on the fraction's own line is what a reader expects.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from(0).to_typst_string(), "0");
    /// assert_eq!(Rational::from(123).to_typst_string(), "123");
    /// assert_eq!(Rational::from(-123).to_typst_string(), "-123");
    /// assert_eq!(
    ///     Rational::from_str("22/7").unwrap().to_typst_string(),
    ///     "frac(22, 7)"
    /// );
    /// assert_eq!(
    ///     Rational::from_str("-2/3").unwrap().to_typst_string(),
    ///     "-frac(2, 3)"
    /// );
    /// ```
    ///
    /// | value                        | fragment      |
    /// |------------------------------|---------------|
    /// | `Rational::from(123)`        | `123`         |
    /// | `Rational::from_str("22/7")` | `frac(22, 7)` |
    /// | `Rational::from_str("-2/3")` | `-frac(2, 3)` |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        if !self.sign {
            f.write_char('-')?;
        }
        fmt_typst_unsigned(self, f)
    }
}
