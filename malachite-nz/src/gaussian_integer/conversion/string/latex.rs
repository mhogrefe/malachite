// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::{
    ComparableGaussianInteger, ComparableGaussianIntegerRef, GaussianInteger,
};
use core::fmt::{Display, Formatter, Result};
use malachite_base::strings::latex::ToLatex;

impl ToLatex for GaussianInteger {
    /// Writes a [`GaussianInteger`] as a LaTeX math-mode fragment.
    ///
    /// The fragment is what [`Display`] gives, which is already how a Gaussian integer is written
    /// in mathematics: a value with a zero imaginary part is its real part alone, a purely
    /// imaginary value is a coefficient directly followed by `i` with coefficients of 1 and -1
    /// elided, and otherwise the real term comes first and the imaginary term follows with a
    /// joining sign.
    ///
    /// The imaginary unit is written as a plain `i`, which LaTeX sets in italics, as most
    /// mathematical writing does. An upright one would need a spelling of its own, and would not
    /// match what [`Display`] gives.
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
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(GaussianInteger::default().to_latex_string(), "0");
    /// assert_eq!(GaussianInteger::from(2).to_latex_string(), "2");
    /// assert_eq!(GaussianInteger::from(-2).to_latex_string(), "-2");
    /// assert_eq!(GaussianInteger::imaginary_from(1).to_latex_string(), "i");
    /// assert_eq!(GaussianInteger::imaginary_from(-1).to_latex_string(), "-i");
    /// assert_eq!(GaussianInteger::imaginary_from(2).to_latex_string(), "2i");
    /// ```
    ///
    /// | value                                | fragment | renders as |
    /// |--------------------------------------|----------|------------|
    /// | `GaussianInteger::default()`         | `0`      | $0$        |
    /// | `GaussianInteger::from(-2)`          | `-2`     | $-2$       |
    /// | `GaussianInteger::imaginary_from(1)` | `i`      | $i$        |
    /// | `GaussianInteger::imaginary_from(2)` | `2i`     | $2i$       |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

impl ToLatex for ComparableGaussianInteger {
    /// Writes a [`ComparableGaussianInteger`] as a LaTeX math-mode fragment.
    ///
    /// The fragment is the wrapped [`GaussianInteger`]'s own: the wrapper exists to give an
    /// ordering, and does not change what the value is.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_latex` for [`GaussianInteger`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_nz::gaussian_integer::{ComparableGaussianInteger, GaussianInteger};
    ///
    /// let x = GaussianInteger::from(2);
    /// assert_eq!(
    ///     ComparableGaussianInteger(x.clone()).to_latex_string(),
    ///     x.to_latex_string()
    /// );
    /// ```
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        self.0.fmt_latex(f)
    }
}

impl ToLatex for ComparableGaussianIntegerRef<'_> {
    /// Writes a [`ComparableGaussianIntegerRef`] as a LaTeX math-mode fragment.
    ///
    /// The fragment is the wrapped [`GaussianInteger`]'s own: the wrapper exists to give an
    /// ordering, and does not change what the value is.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_latex` for [`GaussianInteger`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_nz::gaussian_integer::{ComparableGaussianIntegerRef, GaussianInteger};
    ///
    /// let x = GaussianInteger::from(2);
    /// assert_eq!(
    ///     ComparableGaussianIntegerRef(&x).to_latex_string(),
    ///     x.to_latex_string()
    /// );
    /// ```
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        self.0.fmt_latex(f)
    }
}
