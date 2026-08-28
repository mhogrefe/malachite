// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use core::fmt::{Display, Formatter, Result, Write};

// Writes the imaginary term of a nonzero imaginary part, without its sign: the absolute value of
// the numerator directly followed by 'i', then the denominator if it is not 1. Numerators of 1 are
// elided, so 1/2 is written "i/2" and 1 is written "i".
fn fmt_unsigned_imaginary_term(q: &Rational, f: &mut Formatter) -> Result {
    let n = q.numerator_ref();
    let d = q.denominator_ref();
    if *n != 1 {
        Display::fmt(n, f)?;
    }
    f.write_char('i')?;
    if *d != 1 {
        f.write_char('/')?;
        Display::fmt(d, f)?;
    }
    Ok(())
}

impl Display for GaussianRational {
    /// Converts a [`GaussianRational`] to a [`String`].
    ///
    /// A value with a zero imaginary part is written as its real part alone; in particular, zero is
    /// `"0"`. An imaginary term is written as the absolute value of its numerator, directly
    /// followed by `'i'`, then a `'/'` and the denominator if the denominator is not 1; numerators
    /// of 1 are elided. So 1/2 times i is written `"i/2"`, and 5/6 times i is `"5i/6"`. A purely
    /// imaginary value carries its own sign, and otherwise the real term is written first and the
    /// imaginary term follows with a joining sign, as in `"2/3-5i/6"`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ImaginaryFrom;
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(GaussianRational::default().to_string(), "0");
    /// assert_eq!(
    ///     GaussianRational::from(Rational::from_signeds(-2, 3)).to_string(),
    ///     "-2/3"
    /// );
    /// assert_eq!(GaussianRational::imaginary_from(1).to_string(), "i");
    /// assert_eq!(
    ///     GaussianRational::imaginary_from(Rational::from_signeds(1, 2)).to_string(),
    ///     "i/2"
    /// );
    /// assert_eq!(
    ///     GaussianRational::imaginary_from(Rational::from_signeds(-5, 6)).to_string(),
    ///     "-5i/6"
    /// );
    ///
    /// let g = GaussianRational {
    ///     real: Rational::from_signeds(2, 3),
    ///     imaginary: Rational::from_signeds(-5, 6),
    /// };
    /// assert_eq!(g.to_string(), "2/3-5i/6");
    /// let g = GaussianRational {
    ///     real: Rational::from(1),
    ///     imaginary: Rational::from_signeds(-1, 2),
    /// };
    /// assert_eq!(g.to_string(), "1-i/2");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.imaginary == 0 {
            return Display::fmt(&self.real, f);
        }
        if self.real != 0 {
            Display::fmt(&self.real, f)?;
            if self.imaginary > 0 {
                f.write_char('+')?;
            } else {
                f.write_char('-')?;
            }
        } else if self.imaginary < 0 {
            f.write_char('-')?;
        }
        fmt_unsigned_imaginary_term(&self.imaginary, f)
    }
}
