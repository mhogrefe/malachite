// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec::Vec;
use core::str::FromStr;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
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

// Splits a polynomial into its terms.
//
// A `+` separates two terms and is dropped; a `-` separates them too, but stays, since it belongs
// to the coefficient that follows it. A sign at the very front is part of the first term rather
// than a separator, which is how a polynomial that begins with a negative term is read.
fn split_terms(s: &str) -> Vec<&str> {
    let mut terms = Vec::new();
    let mut start = 0;
    for (i, c) in s.char_indices() {
        if i != 0 && (c == '+' || c == '-') {
            terms.push(&s[start..i]);
            start = if c == '+' { i + 1 } else { i };
        }
    }
    terms.push(&s[start..]);
    terms
}

// Reads one term, giving its coefficient and the exponent of the variable in it.
//
// A term is a coefficient, or a coefficient and a variable part with a `*` between them, or a
// variable part alone, whose coefficient is 1, or a variable part behind a `-`, whose coefficient
// is -1. Which of those it is can be told from its first character or two: a name holds no reserved
// character, and a coefficient is a run of digits behind an optional sign.
//
// A coefficient of 0 is refused. A term that is zero contributes nothing, and is not written at
// all; the one polynomial with nothing to write is the zero polynomial, which is written `0`, and
// that is read before this is ever reached.
fn parse_term<S: VarScheme + ?Sized>(var: Var<'_, S>, term: &str) -> Option<(Rational, usize)> {
    if term.starts_with(|c: char| !char_is_reserved(c)) {
        return Some((Rational::ONE, parse_monic(var, term)?));
    }
    // `-x` is the variable part with a coefficient of -1, not a coefficient of `-` times anything.
    if let Some(rest) = term.strip_prefix('-')
        && rest.starts_with(|c: char| !char_is_reserved(c))
    {
        return Some((Rational::NEGATIVE_ONE, parse_monic(var, rest)?));
    }
    let (coefficient, exponent) = match term.split_once('*') {
        Some((coefficient, monic)) => (coefficient, parse_monic(var, monic)?),
        None => (term, 0),
    };
    let coefficient = Rational::from_str(coefficient).ok()?;
    if coefficient == 0u32 {
        None
    } else {
        Some((coefficient, exponent))
    }
}

impl RationalPolynomial {
    /// Converts a string to an [`RationalPolynomial`], with its variable named by any
    /// [`VarScheme`].
    ///
    /// The syntax is the one [`FromStr`] reads, which that implementation describes; the only
    /// difference is that the variable is whichever one is handed in rather than `x`.
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
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::list::ListVars;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_string_with(GreekVars.var(0), "α^2+3*α+2").unwrap();
    /// assert_eq!(p.to_string(), "x^2+3*x+2");
    ///
    /// let vars = ListVars::new(["t"]);
    /// assert_eq!(
    ///     RationalPolynomial::from_string_with(vars.var(0), "t^2+1")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+1"
    /// );
    ///
    /// // The variable must be the one that was asked for.
    /// assert!(RationalPolynomial::from_string_with(GreekVars.var(0), "β^2").is_none());
    /// ```
    pub fn from_string_with<S: VarScheme + ?Sized>(var: Var<'_, S>, s: &str) -> Option<Self> {
        // The zero polynomial has no terms, so it is the one string that the loop below could not
        // read.
        if s == "0" {
            return Some(Self::ZERO);
        }
        // `Display` never writes a leading `+`, so this does not read one.
        if s.starts_with('+') {
            return None;
        }
        let mut coefficients: Vec<Rational> = Vec::new();
        for term in split_terms(s) {
            let (coefficient, exponent) = parse_term(var, term)?;
            if exponent >= coefficients.len() {
                coefficients.resize(exponent + 1, Rational::ZERO);
            } else if coefficients[exponent] != 0u32 {
                // Two terms of the same degree. Which of them to believe is not for this to decide.
                return None;
            }
            coefficients[exponent] = coefficient;
        }
        Some(Self::from_coefficients_asc(coefficients))
    }
}

impl FromStr for RationalPolynomial {
    type Err = ();

    /// Converts a string to an [`RationalPolynomial`].
    ///
    /// The variable is called `x`. [`from_string_with`](RationalPolynomial::from_string_with) is
    /// the way to call it something else.
    ///
    /// This reads back everything [`Display`](core::fmt::Display) writes, and more besides: the
    /// terms may come in any order, an exponent may be written `^1`, and a coefficient may have
    /// leading zeros. A term may be negative, in which case the `-` that separates it from the term
    /// before it is the same `-` that its coefficient carries; a term at the front keeps its sign
    /// and has nothing to be separated from. What it will not accept is a term whose coefficient is
    /// zero, two terms of the same degree, a leading `+`, a variable other than the one asked for,
    /// or anything with a space in it. The zero polynomial is `0`, and is the only string in which
    /// a zero coefficient may be written.
    ///
    /// If the string does not represent an [`RationalPolynomial`], an `Err` is returned.
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
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+3*x+2"
    /// );
    /// assert_eq!(RationalPolynomial::from_str("0").unwrap().to_string(), "0");
    /// assert_eq!(RationalPolynomial::from_str("5").unwrap().to_string(), "5");
    /// assert_eq!(RationalPolynomial::from_str("x").unwrap().to_string(), "x");
    ///
    /// // A term may be negative, and a leading `-` belongs to the first term rather than
    /// // separating it from anything.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("-5").unwrap().to_string(),
    ///     "-5"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("-x").unwrap().to_string(),
    ///     "-x"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x^2-2*x+5")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2-2*x+5"
    /// );
    ///
    /// // The terms may come in any order.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("2+3*x+x^2")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+3*x+2"
    /// );
    ///
    /// assert!(RationalPolynomial::from_str("").is_err());
    /// assert!(RationalPolynomial::from_str("y").is_err());
    /// assert!(RationalPolynomial::from_str("x^2 + 1").is_err());
    /// assert!(RationalPolynomial::from_str("0*x").is_err());
    /// assert!(RationalPolynomial::from_str("x+x").is_err());
    /// // A `+` is a separator, so a leading one is neither written nor read, and two signs in a
    /// // row leave a term with nothing in it.
    /// assert!(RationalPolynomial::from_str("+x").is_err());
    /// assert!(RationalPolynomial::from_str("x--1").is_err());
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        Self::from_string_with(Var::new(&XyzVars, 0), s).ok_or(())
    }
}
