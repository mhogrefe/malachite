// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModSum};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_add::{add_assign_ref, add_assign_val};
use alloc::vec::Vec;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "every polynomial must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// Adds the polynomials' coefficients into one vector, reducing as it goes, and trims once at the
// end, since leading coefficients can cancel. Summands of different degrees are fine, and the sum
// of no polynomials is zero.
fn mod_sum_owned<T: PrimitiveUnsigned, I: Iterator<Item = UnsignedPolynomial<T>>>(
    xs: I,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    let mut sum: Vec<T> = Vec::new();
    for p in xs {
        assert_reduced(&p, m);
        add_assign_val(&mut sum, p.coefficients, m);
    }
    let mut sum = UnsignedPolynomial { coefficients: sum };
    sum.trim();
    sum
}

fn mod_sum_ref<'a, T: PrimitiveUnsigned + 'a, I: Iterator<Item = &'a UnsignedPolynomial<T>>>(
    xs: I,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    let mut sum: Vec<T> = Vec::new();
    for p in xs {
        assert_reduced(p, m);
        add_assign_ref(&mut sum, &p.coefficients, m);
    }
    let mut sum = UnsignedPolynomial { coefficients: sum };
    sum.trim();
    sum
}

impl<T: PrimitiveUnsigned> ModSum<Self, T> for UnsignedPolynomial<T> {
    /// Adds up all the [`UnsignedPolynomial`]s in an iterator modulo another number $m$, taking the
    /// polynomials by value. The coefficients must already be reduced.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let ps = ["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"]
    ///     .map(|s| UnsignedPolynomial::<u8>::from_str(s).unwrap());
    /// assert_eq!(
    ///     UnsignedPolynomial::mod_sum(ps.into_iter(), 9).to_string(),
    ///     "x^3+8*x^2+6*x+8"
    /// );
    /// ```
    #[inline]
    fn mod_sum<I: Iterator<Item = Self>>(xs: I, m: T) -> Self {
        mod_sum_owned(xs, m)
    }
}

impl<'a, T: PrimitiveUnsigned> ModSum<&'a Self, T> for UnsignedPolynomial<T> {
    /// Adds up all the [`UnsignedPolynomial`]s in an iterator modulo another number $m$, taking the
    /// polynomials by reference. The coefficients must already be reduced.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let ps = ["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"]
    ///     .map(|s| UnsignedPolynomial::<u8>::from_str(s).unwrap());
    /// assert_eq!(
    ///     UnsignedPolynomial::mod_sum(ps.iter(), 9).to_string(),
    ///     "x^3+8*x^2+6*x+8"
    /// );
    /// ```
    #[inline]
    fn mod_sum<I: Iterator<Item = &'a Self>>(xs: I, m: T) -> Self {
        mod_sum_ref(xs, m)
    }
}
