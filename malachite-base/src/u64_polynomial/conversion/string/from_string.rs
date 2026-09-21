// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::u64_polynomial::U64Polynomial;
use crate::vars::xyz::XyzVars;
use crate::vars::{Var, VarScheme, char_is_reserved};
use alloc::vec::Vec;
use core::str::FromStr;

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
fn parse_term<S: VarScheme + ?Sized>(var: Var<'_, S>, term: &str) -> Option<(u64, usize)> {
    if term.starts_with(|c: char| !char_is_reserved(c)) {
        return Some((1, parse_monic(var, term)?));
    }
    let (coefficient, exponent) = match term.split_once('*') {
        Some((coefficient, monic)) => (coefficient, parse_monic(var, monic)?),
        None => (term, 0),
    };
    let coefficient = u64::from_str(coefficient).ok()?;
    if coefficient == 0 {
        None
    } else {
        Some((coefficient, exponent))
    }
}

impl U64Polynomial {
    /// Converts a string to a [`U64Polynomial`], with its variable named by any [`VarScheme`].
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::list::ListVars;
    ///
    /// let p = U64Polynomial::from_string_with(GreekVars.var(0), "α^2+3*α+2").unwrap();
    /// assert_eq!(p.to_string(), "x^2+3*x+2");
    ///
    /// let vars = ListVars::new(["t"]);
    /// assert_eq!(
    ///     U64Polynomial::from_string_with(vars.var(0), "t^2+1")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+1"
    /// );
    ///
    /// // The variable must be the one that was asked for.
    /// assert!(U64Polynomial::from_string_with(GreekVars.var(0), "β^2").is_none());
    /// ```
    pub fn from_string_with<S: VarScheme + ?Sized>(var: Var<'_, S>, s: &str) -> Option<Self> {
        // The zero polynomial has no terms, so it is the one string that the loop below could not
        // read.
        if s == "0" {
            return Some(Self::ZERO);
        }
        let mut coefficients: Vec<u64> = Vec::new();
        for term in s.split('+') {
            let (coefficient, exponent) = parse_term(var, term)?;
            if exponent >= coefficients.len() {
                coefficients.resize(exponent + 1, 0);
            } else if coefficients[exponent] != 0 {
                // Two terms of the same degree. Which of them to believe is not for this to decide.
                return None;
            }
            coefficients[exponent] = coefficient;
        }
        Some(Self::from_coefficients_asc(coefficients))
    }
}

impl FromStr for U64Polynomial {
    type Err = ();

    /// Converts a string to a [`U64Polynomial`].
    ///
    /// The variable is called `x`. [`from_string_with`](U64Polynomial::from_string_with) is the way
    /// to call it something else.
    ///
    /// This reads back everything [`Display`](core::fmt::Display) writes, and more besides: the
    /// terms may come in any order, an exponent may be written `^1`, and a coefficient may have
    /// leading zeros. What it will not accept is a term whose coefficient is zero, two terms of the
    /// same degree, a variable other than the one asked for, or anything with a space in it. The
    /// zero polynomial is `0`, and is the only string in which a zero coefficient may be written.
    ///
    /// If the string does not represent a [`U64Polynomial`], an `Err` is returned.
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// assert_eq!(
    ///     U64Polynomial::from_str("x^2+3*x+2").unwrap().to_string(),
    ///     "x^2+3*x+2"
    /// );
    /// assert_eq!(U64Polynomial::from_str("0").unwrap().to_string(), "0");
    /// assert_eq!(U64Polynomial::from_str("5").unwrap().to_string(), "5");
    /// assert_eq!(U64Polynomial::from_str("x").unwrap().to_string(), "x");
    ///
    /// // The terms may come in any order.
    /// assert_eq!(
    ///     U64Polynomial::from_str("2+3*x+x^2").unwrap().to_string(),
    ///     "x^2+3*x+2"
    /// );
    ///
    /// assert!(U64Polynomial::from_str("").is_err());
    /// assert!(U64Polynomial::from_str("y").is_err());
    /// assert!(U64Polynomial::from_str("x^2 + 1").is_err());
    /// assert!(U64Polynomial::from_str("0*x").is_err());
    /// assert!(U64Polynomial::from_str("x+x").is_err());
    /// assert!(U64Polynomial::from_str("-x").is_err());
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        Self::from_string_with(Var::new(&XyzVars, 0), s).ok_or(())
    }
}
