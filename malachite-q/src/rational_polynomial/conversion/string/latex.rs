// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::conversion::string::to_string::Language;
use core::fmt::{Formatter, Result};
use malachite_base::strings::latex::ToLatex;
use malachite_base::vars::Var;
use malachite_base::vars::xyz::XyzVars;

impl ToLatex for RationalPolynomial {
    /// Writes an [`RationalPolynomial`] as a LaTeX math-mode fragment.
    ///
    /// The variable is called `x`.
    /// [`to_latex_string_with`](malachite_base::polynomial::Polynomial::to_latex_string_with) is
    /// the way to call it something else.
    ///
    /// The fragment is the polynomial as it would be written by hand: the terms in order of
    /// decreasing degree, joined with `+`, each one its coefficient followed by its variable and
    /// then a superscript. A coefficient of 1 is left off, and so is an exponent of 1; the constant
    /// term is its coefficient alone, and the zero polynomial, which has no terms, is `0`.
    ///
    /// A negative term joins the one before it with its own `-` rather than with a `+`, and a
    /// coefficient of -1 leaves only that sign behind. Nothing stands between a coefficient and its
    /// variable, since a number written against a variable can only be multiplying it.
    ///
    /// A superscript is braced only when the exponent has more than one digit, since a superscript
    /// of one character needs nothing to hold it together.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .to_latex_string(),
    ///     "x^2+3x+2"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("0").unwrap().to_latex_string(),
    ///     "0"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("5").unwrap().to_latex_string(),
    ///     "5"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x").unwrap().to_latex_string(),
    ///     "x"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("2*x^3")
    ///         .unwrap()
    ///         .to_latex_string(),
    ///     "2x^3"
    /// );
    ///
    /// // An exponent of more than one digit is braced.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x^12+x^2")
    ///         .unwrap()
    ///         .to_latex_string(),
    ///     "x^{12}+x^2"
    /// );
    /// ```
    ///
    /// The value column holds each polynomial as [`Display`](core::fmt::Display) writes it.
    ///
    /// | value       | fragment     | renders as   |
    /// |-------------|--------------|--------------|
    /// | `x^2+3*x+2` | `x^2+3x+2`   | $x^2+3x+2$   |
    /// | `0`         | `0`          | $0$          |
    /// | `5`         | `5`          | $5$          |
    /// | `x`         | `x`          | $x$          |
    /// | `2*x^3`     | `2x^3`       | $2x^3$       |
    /// | `x^12+x^2`  | `x^{12}+x^2` | $x^{12}+x^2$ |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        self.write_with_var(Var::new(&XyzVars, 0), Language::Latex, f)
    }
}
