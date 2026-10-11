// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_add::{add_assign_ref, add_assign_val};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModSum};

fn assert_reduced(p: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "every polynomial must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Adds the polynomials' coefficients into one vector, reducing as it goes, and trims once at the
// end, since leading coefficients can cancel. Summands of different degrees are fine, and the sum
// of no polynomials is zero.
fn mod_sum_owned<I: Iterator<Item = NaturalPolynomial>>(xs: I, m: &Natural) -> NaturalPolynomial {
    assert_ne!(*m, 0u32, "m cannot be zero");
    let mut sum: Vec<Natural> = Vec::new();
    for p in xs {
        assert_reduced(&p, m);
        add_assign_val(&mut sum, p.coefficients, m);
    }
    let mut sum = NaturalPolynomial { coefficients: sum };
    sum.trim();
    sum
}

fn mod_sum_ref<'a, I: Iterator<Item = &'a NaturalPolynomial>>(
    xs: I,
    m: &Natural,
) -> NaturalPolynomial {
    assert_ne!(*m, 0u32, "m cannot be zero");
    let mut sum: Vec<Natural> = Vec::new();
    for p in xs {
        assert_reduced(p, m);
        add_assign_ref(&mut sum, &p.coefficients, m);
    }
    let mut sum = NaturalPolynomial { coefficients: sum };
    sum.trim();
    sum
}

impl ModSum<Self, Natural> for NaturalPolynomial {
    /// Adds up all the [`NaturalPolynomial`]s in an iterator modulo another number $m$, taking the
    /// polynomials by value and the modulus by value. The coefficients must already be reduced.
    ///
    /// The summands may have different degrees, and the sum of no polynomials is zero.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $m$ and $\sum_
    /// {i=0}^{n-1} p_i \equiv q \mod m$, coefficient by coefficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of coefficients of all
    /// the polynomials, and $m$ is the greatest number of coefficients of any of them.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any coefficient is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let ps = ["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"]
    ///     .map(|s| NaturalPolynomial::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalPolynomial::mod_sum(ps.into_iter(), Natural::from(9u32)).to_string(),
    ///     "x^3+8*x^2+6*x+8"
    /// );
    /// ```
    #[inline]
    fn mod_sum<I: Iterator<Item = Self>>(xs: I, m: Natural) -> Self {
        mod_sum_owned(xs, &m)
    }
}

impl<'b> ModSum<Self, &'b Natural> for NaturalPolynomial {
    /// Adds up all the [`NaturalPolynomial`]s in an iterator modulo another number $m$, taking the
    /// polynomials by value and the modulus by reference. The coefficients must already be reduced.
    ///
    /// The summands may have different degrees, and the sum of no polynomials is zero.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $m$ and $\sum_
    /// {i=0}^{n-1} p_i \equiv q \mod m$, coefficient by coefficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of coefficients of all
    /// the polynomials, and $m$ is the greatest number of coefficients of any of them.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any coefficient is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let ps = ["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"]
    ///     .map(|s| NaturalPolynomial::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalPolynomial::mod_sum(ps.into_iter(), &Natural::from(9u32)).to_string(),
    ///     "x^3+8*x^2+6*x+8"
    /// );
    /// ```
    #[inline]
    fn mod_sum<I: Iterator<Item = Self>>(xs: I, m: &'b Natural) -> Self {
        mod_sum_owned(xs, m)
    }
}

impl<'a> ModSum<&'a Self, Natural> for NaturalPolynomial {
    /// Adds up all the [`NaturalPolynomial`]s in an iterator modulo another number $m$, taking the
    /// polynomials by reference and the modulus by value. The coefficients must already be reduced.
    ///
    /// The summands may have different degrees, and the sum of no polynomials is zero.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $m$ and $\sum_
    /// {i=0}^{n-1} p_i \equiv q \mod m$, coefficient by coefficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of coefficients of all
    /// the polynomials, and $m$ is the greatest number of coefficients of any of them.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any coefficient is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let ps = ["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"]
    ///     .map(|s| NaturalPolynomial::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalPolynomial::mod_sum(ps.iter(), Natural::from(9u32)).to_string(),
    ///     "x^3+8*x^2+6*x+8"
    /// );
    /// ```
    #[inline]
    fn mod_sum<I: Iterator<Item = &'a Self>>(xs: I, m: Natural) -> Self {
        mod_sum_ref(xs, &m)
    }
}

impl<'a, 'b> ModSum<&'a Self, &'b Natural> for NaturalPolynomial {
    /// Adds up all the [`NaturalPolynomial`]s in an iterator modulo another number $m$, taking the
    /// polynomials by reference and the modulus by reference. The coefficients must already be
    /// reduced.
    ///
    /// The summands may have different degrees, and the sum of no polynomials is zero.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $m$ and $\sum_
    /// {i=0}^{n-1} p_i \equiv q \mod m$, coefficient by coefficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of coefficients of all
    /// the polynomials, and $m$ is the greatest number of coefficients of any of them.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any coefficient is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let ps = ["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"]
    ///     .map(|s| NaturalPolynomial::from_str(s).unwrap());
    /// assert_eq!(
    ///     NaturalPolynomial::mod_sum(ps.iter(), &Natural::from(9u32)).to_string(),
    ///     "x^3+8*x^2+6*x+8"
    /// );
    /// ```
    #[inline]
    fn mod_sum<I: Iterator<Item = &'a Self>>(xs: I, m: &'b Natural) -> Self {
        mod_sum_ref(xs, m)
    }
}
