// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::Polynomial;
use malachite_base::vars::xyz::XyzVars;
use malachite_base::vars::{Var, VarScheme, char_is_reserved};

// Reads the variable-and-exponent part of a term, such as `x` or `x^2`, giving the exponent.
//
// The name must be the one `var` goes by; a name some other variable of the same scheme goes by is
// no more acceptable than a name from another scheme altogether, since a polynomial in one variable
// has nowhere to put a second. An exponent of 0 is refused, as it is what a term with no variable
// would have, and that term is written as its coefficient alone.
fn parse_monic<S: VarScheme + ?Sized>(var: Var<'_, S>, monic: &str) -> Option<usize> {
    let (name, exponent) = match monic.split_once('^') {
        Some((name, exponent)) => {
            let exponent = usize::from_str(exponent).ok()?;
            if exponent == 0 {
                return None;
            }
            (name, exponent)
        }
        None => (monic, 1),
    };
    if var.scheme().parse_var(name) == Some(var.index()) {
        Some(exponent)
    } else {
        None
    }
}

// Reads one term, giving its coefficient and the exponent of the variable in it.
//
// A term is a coefficient, or a coefficient and a variable part with a `*` between them, or a
// variable part alone, whose coefficient is 1. Which of the three it is can be told from its first
// character: a name holds no reserved character, and a coefficient is all digits, so a term that
// begins with an unreserved character is one that begins with a name.
//
// A coefficient of 0 is refused. A term that is zero contributes nothing, and is not written at
// all; the one polynomial with nothing to write is the zero polynomial, which is written `0`, and
// that is read before this is ever reached.
fn parse_term<S: VarScheme + ?Sized>(var: Var<'_, S>, term: &str) -> Option<(Natural, usize)> {
    if term.starts_with(|c: char| !char_is_reserved(c)) {
        return Some((Natural::ONE, parse_monic(var, term)?));
    }
    let (coefficient, exponent) = match term.split_once('*') {
        Some((coefficient, monic)) => (coefficient, parse_monic(var, monic)?),
        None => (term, 0),
    };
    let coefficient = Natural::from_str(coefficient).ok()?;
    if coefficient == 0u32 {
        None
    } else {
        Some((coefficient, exponent))
    }
}

// The implementation of `NaturalPolynomial::from_string_with`.
pub(crate) fn from_string_with<S: VarScheme + ?Sized>(
    var: Var<'_, S>,
    s: &str,
) -> Option<NaturalPolynomial> {
    // The zero polynomial has no terms, so it is the one string that the loop below could not read.
    if s == "0" {
        return Some(NaturalPolynomial::ZERO);
    }
    let mut coefficients: Vec<Natural> = Vec::new();
    for term in s.split('+') {
        let (coefficient, exponent) = parse_term(var, term)?;
        if exponent >= coefficients.len() {
            coefficients.resize(exponent + 1, Natural::ZERO);
        } else if coefficients[exponent] != 0u32 {
            // Two terms of the same degree. Which of them to believe is not for this to decide.
            return None;
        }
        coefficients[exponent] = coefficient;
    }
    Some(NaturalPolynomial::from_coefficients_asc(coefficients))
}

impl FromStr for NaturalPolynomial {
    type Err = ();

    /// Converts a string to a [`NaturalPolynomial`].
    ///
    /// The variable is called `x`. [`from_string_with`](NaturalPolynomial::from_string_with) is the
    /// way to call it something else.
    ///
    /// This reads back everything [`Display`](core::fmt::Display) writes, and more besides: the
    /// terms may come in any order, an exponent may be written `^1`, and a coefficient may have
    /// leading zeros. What it will not accept is a term whose coefficient is zero, two terms of the
    /// same degree, a variable other than the one asked for, or anything with a space in it. The
    /// zero polynomial is `0`, and is the only string in which a zero coefficient may be written.
    ///
    /// If the string does not represent a [`NaturalPolynomial`], an `Err` is returned.
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
    ///
    /// // The terms may come in any order.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("2+3*x+x^2")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+3*x+2"
    /// );
    ///
    /// assert!(NaturalPolynomial::from_str("").is_err());
    /// assert!(NaturalPolynomial::from_str("y").is_err());
    /// assert!(NaturalPolynomial::from_str("x^2 + 1").is_err());
    /// assert!(NaturalPolynomial::from_str("0*x").is_err());
    /// assert!(NaturalPolynomial::from_str("x+x").is_err());
    /// assert!(NaturalPolynomial::from_str("-x").is_err());
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        Self::from_string_with(Var::new(&XyzVars, 0), s).ok_or(())
    }
}
