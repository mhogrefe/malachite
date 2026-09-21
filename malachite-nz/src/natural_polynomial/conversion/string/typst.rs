// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::conversion::string::to_string::Language;
use alloc::string::String;
use core::fmt::{Formatter, Result};
use malachite_base::strings::typst::ToTypst;
use malachite_base::vars::xyz::XyzVars;
use malachite_base::vars::{Var, VarScheme};

impl NaturalPolynomial {
    /// Converts a [`NaturalPolynomial`] to a Typst math-mode fragment, naming its variable with any
    /// [`VarScheme`].
    ///
    /// The fragment is the one [`ToTypst`] writes, which that implementation describes; the only
    /// difference is that the variable is whichever one is handed in rather than `x`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the bits of the
    /// coefficients.
    ///
    /// # Panics
    /// Panics if `var`'s index is not less than its scheme's [`capacity`](VarScheme::capacity).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::indexed::IndexedVars;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.to_typst_string_with(GreekVars.var(0)), "α^2+3α+2");
    /// assert_eq!(p.to_typst_string_with(IndexedVars.var(7)), "x_7^2+3x_7+2");
    /// ```
    ///
    /// The polynomial is `x^2+3*x+2` in each row; only its variable differs.
    ///
    /// | variable | fragment       |
    /// |----------|----------------|
    /// | `α`      | `α^2+3α+2`     |
    /// | `x₇`     | `x_7^2+3x_7+2` |
    pub fn to_typst_string_with<S: VarScheme + ?Sized>(&self, var: Var<'_, S>) -> String {
        let mut s = String::new();
        // Writing to a `String` cannot fail, so the result is the string itself.
        self.write_with_var(var, Language::Typst, &mut s).unwrap();
        s
    }
}

impl ToTypst for NaturalPolynomial {
    /// Writes a [`NaturalPolynomial`] as a Typst math-mode fragment.
    ///
    /// The variable is called `x`.
    /// [`to_typst_string_with`](NaturalPolynomial::to_typst_string_with) is the way to call it
    /// something else.
    ///
    /// The fragment is the polynomial as it would be written by hand: the terms in order of
    /// decreasing degree, joined with `+`, each one its coefficient followed by its variable and
    /// then a superscript. A coefficient of 1 is left off, and so is an exponent of 1; the constant
    /// term is its coefficient alone, and the zero polynomial, which has no terms, is `0`. Nothing
    /// stands between a coefficient and its variable, since a number written against a variable can
    /// only be multiplying it, and a digit ends the run of letters that Typst would otherwise read
    /// as one name.
    ///
    /// A superscript is parenthesized only when the exponent has more than one digit, since a
    /// superscript of one character needs nothing to hold it together.
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
    /// use malachite_base::strings::typst::ToTypst;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .to_typst_string(),
    ///     "x^2+3x+2"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("0").unwrap().to_typst_string(),
    ///     "0"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("5").unwrap().to_typst_string(),
    ///     "5"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x").unwrap().to_typst_string(),
    ///     "x"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("2*x^3")
    ///         .unwrap()
    ///         .to_typst_string(),
    ///     "2x^3"
    /// );
    ///
    /// // An exponent of more than one digit is parenthesized.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^12+x^2")
    ///         .unwrap()
    ///         .to_typst_string(),
    ///     "x^(12)+x^2"
    /// );
    /// ```
    ///
    /// The value column holds each polynomial as [`Display`](core::fmt::Display) writes it.
    ///
    /// | value       | fragment     |
    /// |-------------|--------------|
    /// | `x^2+3*x+2` | `x^2+3x+2`   |
    /// | `0`         | `0`          |
    /// | `5`         | `5`          |
    /// | `x`         | `x`          |
    /// | `2*x^3`     | `2x^3`       |
    /// | `x^12+x^2`  | `x^(12)+x^2` |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        self.write_with_var(Var::new(&XyzVars, 0), Language::Typst, f)
    }
}
