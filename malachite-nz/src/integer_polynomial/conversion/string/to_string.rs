// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use alloc::string::String;
use core::fmt::{Display, Formatter, Result, Write};
use malachite_base::strings::latex::ToLatex;
use malachite_base::strings::typst::ToTypst;
use malachite_base::vars::xyz::XyzVars;
use malachite_base::vars::{Var, VarScheme};

// The languages a polynomial can be written in.
//
// They differ in only three places: how the variable is spelled, whether a coefficient and a
// variable need something between them, and how an exponent is attached. Everything else — which
// terms there are, what order they come in, and which parts of a term are left off — is the same,
// so one function writes all three.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Language {
    Plain,
    Latex,
    Typst,
}

impl IntegerPolynomial {
    // Writes a `IntegerPolynomial` whose variable is named by `var`, in one of three languages.
    //
    // The destination is anything that can be written to rather than a `Formatter`, since a
    // `Formatter` cannot be made outside a `fmt` method, and the `to_*_string_with` functions need
    // somewhere to write that is not one.
    pub(crate) fn write_with_var<W: Write, S: VarScheme + ?Sized>(
        &self,
        var: Var<'_, S>,
        language: Language,
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
            // A term joins the one before it with a `+`, unless it is negative, in which case the
            // `-` that its coefficient already carries is the join.
            if first {
                first = false;
            } else if *coefficient > 0u32 {
                w.write_char('+')?;
            }
            if exponent == 0 {
                write!(w, "{coefficient}")?;
                continue;
            }
            // A coefficient of 1 is left off, as it is when a polynomial is written by hand, and a
            // coefficient of -1 leaves only its sign behind.
            if *coefficient == -1i32 {
                w.write_char('-')?;
            } else if *coefficient != 1u32 {
                write!(w, "{coefficient}")?;
                // Typeset, a coefficient sits right up against its variable, which is unambiguous
                // because the one is a number and the other is not. The plain form is read back
                // rather than typeset, and its `*` is what tells the two apart.
                if language == Language::Plain {
                    w.write_char('*')?;
                }
            }
            match language {
                Language::Plain => write!(w, "{var}")?,
                Language::Latex => write!(w, "{}", var.to_latex())?,
                Language::Typst => write!(w, "{}", var.to_typst())?,
            }
            if exponent != 1 {
                // An exponent of one digit holds together on its own; a longer one has to be
                // bracketed, or only its first digit would be raised. The plain form has no scripts
                // to begin with, so it writes the digits and nothing else.
                match language {
                    Language::Plain => write!(w, "^{exponent}")?,
                    _ if exponent < 10 => write!(w, "^{exponent}")?,
                    Language::Latex => write!(w, "^{{{exponent}}}")?,
                    Language::Typst => write!(w, "^({exponent})")?,
                }
            }
        }
        Ok(())
    }

    /// Converts an [`IntegerPolynomial`] to a [`String`], naming its variable with any
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.to_string_with(GreekVars.var(0)), "α^2+3*α+2");
    /// assert_eq!(p.to_string_with(IndexedVars.var(7)), "x₇^2+3*x₇+2");
    ///
    /// let vars = ListVars::new(["t"]);
    /// assert_eq!(p.to_string_with(vars.var(0)), "t^2+3*t+2");
    /// ```
    pub fn to_string_with<S: VarScheme + ?Sized>(&self, var: Var<'_, S>) -> String {
        let mut s = String::new();
        // Writing to a `String` cannot fail, so the result is the string itself.
        self.write_with_var(var, Language::Plain, &mut s).unwrap();
        s
    }
}

impl Display for IntegerPolynomial {
    /// Converts an [`IntegerPolynomial`] to a [`String`].
    ///
    /// The variable is called `x`. [`to_string_with`](IntegerPolynomial::to_string_with) is the way
    /// to call it something else.
    ///
    /// The terms are written in order of decreasing degree and joined with `+`. A term is its
    /// coefficient, then `*`, then the variable, then `^` and the exponent; but a coefficient of 1
    /// is left off along with its `*`, an exponent of 1 is left off along with its `^`, and the
    /// constant term is its coefficient alone. The zero polynomial, which has no terms at all, is
    /// `0`.
    ///
    /// A negative term joins the one before it with the `-` its coefficient already carries, rather
    /// than with a `+`, and a coefficient of -1 leaves only that sign behind: the polynomial with
    /// coefficients 5, -2, 1 is `x^2-2*x+5`, and the one with 0, 1, -1 is `-x^2+x`.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+3*x+2"
    /// );
    /// assert_eq!(IntegerPolynomial::from_str("0").unwrap().to_string(), "0");
    /// assert_eq!(IntegerPolynomial::from_str("5").unwrap().to_string(), "5");
    /// assert_eq!(IntegerPolynomial::from_str("x").unwrap().to_string(), "x");
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("2*x^3").unwrap().to_string(),
    ///     "2*x^3"
    /// );
    ///
    /// // The terms come out in decreasing degree, whatever order they went in, and an exponent
    /// // of 1 is left off.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("2+3*x+x^2")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+3*x+2"
    /// );
    /// assert_eq!(IntegerPolynomial::from_str("x^1").unwrap().to_string(), "x");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_with_var(Var::new(&XyzVars, 0), Language::Plain, f)
    }
}
