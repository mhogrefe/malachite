// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::{
    ComparableGaussianRational, ComparableGaussianRationalRef, GaussianRational,
};
use core::fmt::{Display, Formatter, Result, Write};
use malachite_base::strings::latex::ToLatex;

// Writes the magnitude of an imaginary term. The imaginary unit goes in the numerator, so that 5/6
// times i is ``\\frac{5i}{6}`` rather than a fraction with an `i` hung off it, and a coefficient of
// 1 is elided. The sign is the caller's to write, so that it falls outside the fraction.
fn fmt_latex_unsigned_imaginary_term(q: &Rational, f: &mut Formatter) -> Result {
    let n = q.numerator_ref();
    let d = q.denominator_ref();
    if *d == 1u32 {
        if *n != 1u32 {
            Display::fmt(n, f)?;
        }
        return f.write_char('i');
    }
    f.write_str("\\frac{")?;
    if *n != 1u32 {
        Display::fmt(n, f)?;
    }
    f.write_char('i')?;
    f.write_str("}{")?;
    Display::fmt(d, f)?;
    f.write_str("}")
}

impl ToLatex for GaussianRational {
    /// Writes a [`GaussianRational`] as a LaTeX math-mode fragment.
    ///
    /// A value with a zero imaginary part is written as its real part alone; in particular, zero is
    /// `0`. An imaginary term puts the imaginary unit in the numerator, so 5/6 times i is
    /// ``\\frac{5i}{6}`` rather than a fraction with an `i` hung off it, and coefficients of 1 are
    /// elided. A purely imaginary value carries its own sign, and otherwise the real term is
    /// written first and the imaginary term follows with a joining sign.
    ///
    /// Every sign is written outside its fraction, as ``-\\frac{2}{3}`` rather than
    /// ``\\frac{-2}{3}``.
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
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(GaussianRational::default().to_latex_string(), "0");
    /// assert_eq!(GaussianRational::imaginary_from(1).to_latex_string(), "i");
    /// assert_eq!(
    ///     GaussianRational::imaginary_from(Rational::from_signeds(1, 2)).to_latex_string(),
    ///     r"\frac{i}{2}"
    /// );
    /// assert_eq!(
    ///     GaussianRational::imaginary_from(Rational::from_signeds(-5, 6)).to_latex_string(),
    ///     r"-\frac{5i}{6}"
    /// );
    ///
    /// let g = GaussianRational {
    ///     real: Rational::from_signeds(2, 3),
    ///     imaginary: Rational::from_signeds(-5, 6),
    /// };
    /// assert_eq!(g.to_latex_string(), r"\frac{2}{3}-\frac{5i}{6}");
    /// ```
    ///
    /// | value                         | fragment        | renders as      |
    /// |-------------------------------|-----------------|-----------------|
    /// | `GaussianRational::default()` | `0`             | $0$             |
    /// | `i`                           | `i`             | $i$             |
    /// | `i/2`                         | `\frac{i}{2}`   | $\frac{i}{2}$   |
    /// | `-5i/6`                       | `-\frac{5i}{6}` | $-\frac{5i}{6}$ |
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        if self.imaginary == 0u32 {
            return self.real.fmt_latex(f);
        }
        if self.real != 0u32 {
            self.real.fmt_latex(f)?;
            if self.imaginary > 0u32 {
                f.write_char('+')?;
            } else {
                f.write_char('-')?;
            }
        } else if self.imaginary < 0u32 {
            f.write_char('-')?;
        }
        fmt_latex_unsigned_imaginary_term(&self.imaginary, f)
    }
}

impl ToLatex for ComparableGaussianRational {
    /// Writes a [`ComparableGaussianRational`] as a LaTeX math-mode fragment.
    ///
    /// The fragment is the wrapped [`GaussianRational`]'s own: the wrapper exists to give an
    /// ordering, and does not change what the value is.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_latex` for [`GaussianRational`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::{ComparableGaussianRational, GaussianRational};
    ///
    /// let x = GaussianRational::from(Rational::from_signeds(2, 3));
    /// assert_eq!(
    ///     ComparableGaussianRational(x.clone()).to_latex_string(),
    ///     x.to_latex_string()
    /// );
    /// ```
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        self.0.fmt_latex(f)
    }
}

impl ToLatex for ComparableGaussianRationalRef<'_> {
    /// Writes a [`ComparableGaussianRationalRef`] as a LaTeX math-mode fragment.
    ///
    /// The fragment is the wrapped [`GaussianRational`]'s own: the wrapper exists to give an
    /// ordering, and does not change what the value is.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_latex` for [`GaussianRational`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::{ComparableGaussianRationalRef, GaussianRational};
    ///
    /// let x = GaussianRational::from(Rational::from_signeds(2, 3));
    /// assert_eq!(
    ///     ComparableGaussianRationalRef(&x).to_latex_string(),
    ///     x.to_latex_string()
    /// );
    /// ```
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        self.0.fmt_latex(f)
    }
}
