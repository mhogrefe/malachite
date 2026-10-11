// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Sum};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_add::{add_assign_ref, add_assign_val};
use alloc::vec::Vec;

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "every polynomial must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// Adds the polynomials' coefficients into one vector, reducing as it goes, and trims once at the
// end, since leading coefficients can cancel. Summands of different degrees are fine, and the sum
// of no polynomials is zero.
fn mod_power_of_2_sum_owned<T: PrimitiveUnsigned, I: Iterator<Item = UnsignedPolynomial<T>>>(
    xs: I,
    pow: u64,
) -> UnsignedPolynomial<T> {
    assert!(pow <= T::WIDTH);
    let mut sum: Vec<T> = Vec::new();
    for p in xs {
        assert_reduced(&p, pow);
        add_assign_val(&mut sum, p.coefficients, pow);
    }
    let mut sum = UnsignedPolynomial { coefficients: sum };
    sum.trim();
    sum
}

fn mod_power_of_2_sum_ref<
    'a,
    T: PrimitiveUnsigned + 'a,
    I: Iterator<Item = &'a UnsignedPolynomial<T>>,
>(
    xs: I,
    pow: u64,
) -> UnsignedPolynomial<T> {
    assert!(pow <= T::WIDTH);
    let mut sum: Vec<T> = Vec::new();
    for p in xs {
        assert_reduced(p, pow);
        add_assign_ref(&mut sum, &p.coefficients, pow);
    }
    let mut sum = UnsignedPolynomial { coefficients: sum };
    sum.trim();
    sum
}

impl<T: PrimitiveUnsigned> ModPowerOf2Sum for UnsignedPolynomial<T> {
    /// Adds up all the [`UnsignedPolynomial`]s in an iterator modulo $2^k$, taking the polynomials
    /// by value. The coefficients must already be reduced.
    ///
    /// The summands may have different degrees, and the sum of no polynomials is zero.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $2^k$ and $\sum_
    /// {i=0}^{n-1} p_i \equiv q \mod 2^k$, coefficient by coefficient.
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
    /// Panics if `pow` is greater than `T::WIDTH` or if any coefficient is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sum;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let ps = ["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"]
    ///     .map(|s| UnsignedPolynomial::<u8>::from_str(s).unwrap());
    /// assert_eq!(
    ///     UnsignedPolynomial::mod_power_of_2_sum(ps.into_iter(), 3).to_string(),
    ///     "x^3+7*x"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sum<I: Iterator<Item = Self>>(xs: I, pow: u64) -> Self {
        mod_power_of_2_sum_owned(xs, pow)
    }
}

impl<'a, T: PrimitiveUnsigned> ModPowerOf2Sum<&'a Self> for UnsignedPolynomial<T> {
    /// Adds up all the [`UnsignedPolynomial`]s in an iterator modulo $2^k$, taking the polynomials
    /// by reference. The coefficients must already be reduced.
    ///
    /// The summands may have different degrees, and the sum of no polynomials is zero.
    ///
    /// $f((p_i)_ {i=0}^{n-1}) = q$, where every coefficient of $q$ is less than $2^k$ and $\sum_
    /// {i=0}^{n-1} p_i \equiv q \mod 2^k$, coefficient by coefficient.
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
    /// Panics if `pow` is greater than `T::WIDTH` or if any coefficient is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sum;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let ps = ["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"]
    ///     .map(|s| UnsignedPolynomial::<u8>::from_str(s).unwrap());
    /// assert_eq!(
    ///     UnsignedPolynomial::mod_power_of_2_sum(ps.iter(), 3).to_string(),
    ///     "x^3+7*x"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sum<I: Iterator<Item = &'a Self>>(xs: I, pow: u64) -> Self {
        mod_power_of_2_sum_ref(xs, pow)
    }
}
