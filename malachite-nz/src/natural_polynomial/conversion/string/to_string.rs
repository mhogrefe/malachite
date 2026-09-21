// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use alloc::string::String;
use core::fmt::{Display, Formatter, Result, Write};
use malachite_base::vars::xyz::XyzVars;
use malachite_base::vars::{Var, VarScheme};

impl NaturalPolynomial {
    // Writes a `NaturalPolynomial` whose variable is named by `var`.
    //
    // The destination is anything that can be written to rather than a `Formatter`, since a
    // `Formatter` cannot be made outside a `fmt` method, and `to_string_with` needs somewhere to
    // write that is not one.
    fn write_with_var<W: Write, S: VarScheme + ?Sized>(
        &self,
        var: Var<'_, S>,
        w: &mut W,
    ) -> Result {
        let coefficients = self.coefficients_asc();
        if coefficients.is_empty() {
            return w.write_str("0");
        }
        let mut first = true;
        for (exponent, coefficient) in coefficients.iter().enumerate().rev() {
            if *coefficient == 0u32 {
                continue;
            }
            if first {
                first = false;
            } else {
                w.write_char('+')?;
            }
            if exponent == 0 {
                write!(w, "{coefficient}")?;
                continue;
            }
            // A coefficient of 1 is left off, as it is when a polynomial is written by hand.
            if *coefficient != 1u32 {
                write!(w, "{coefficient}*")?;
            }
            write!(w, "{var}")?;
            if exponent != 1 {
                write!(w, "^{exponent}")?;
            }
        }
        Ok(())
    }

    /// Converts a [`NaturalPolynomial`] to a [`String`], naming its variable with any
    /// [`VarScheme`].
    ///
    /// The syntax is the one [`Display`] writes, which that implementation describes; the only
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
    /// use malachite_base::vars::list::ListVars;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.to_string_with(GreekVars.var(0)), "α^2+3*α+2");
    /// assert_eq!(p.to_string_with(IndexedVars.var(7)), "x₇^2+3*x₇+2");
    ///
    /// let vars = ListVars::new(["t"]);
    /// assert_eq!(p.to_string_with(vars.var(0)), "t^2+3*t+2");
    /// ```
    pub fn to_string_with<S: VarScheme + ?Sized>(&self, var: Var<'_, S>) -> String {
        let mut s = String::new();
        // Writing to a `String` cannot fail, so the result is the string itself.
        self.write_with_var(var, &mut s).unwrap();
        s
    }
}

impl Display for NaturalPolynomial {
    /// Converts a [`NaturalPolynomial`] to a [`String`].
    ///
    /// The variable is called `x`. [`to_string_with`](NaturalPolynomial::to_string_with) is the way
    /// to call it something else.
    ///
    /// The terms are written in order of decreasing degree and joined with `+`. A term is its
    /// coefficient, then `*`, then the variable, then `^` and the exponent; but a coefficient of 1
    /// is left off along with its `*`, an exponent of 1 is left off along with its `^`, and the
    /// constant term is its coefficient alone. The zero polynomial, which has no terms at all, is
    /// `0`.
    ///
    /// The syntax is the one [Azurite](https://github.com/mhogrefe/azurite) writes polynomials in,
    /// and holds no characters that [`char_is_reserved`](malachite_base::vars::char_is_reserved)
    /// allows in a variable's name, so a polynomial can be read back whatever its variable is
    /// called.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+3*x+2"
    /// );
    /// assert_eq!(NaturalPolynomial::from_str("0").unwrap().to_string(), "0");
    /// assert_eq!(NaturalPolynomial::from_str("5").unwrap().to_string(), "5");
    /// assert_eq!(NaturalPolynomial::from_str("x").unwrap().to_string(), "x");
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("2*x^3").unwrap().to_string(),
    ///     "2*x^3"
    /// );
    ///
    /// // The terms come out in decreasing degree, whatever order they went in, and an exponent
    /// // of 1 is left off.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("2+3*x+x^2")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+3*x+2"
    /// );
    /// assert_eq!(NaturalPolynomial::from_str("x^1").unwrap().to_string(), "x");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_with_var(Var::new(&XyzVars, 0), f)
    }
}
