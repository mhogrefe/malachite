// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::platform::Limb;
use core::borrow::Borrow;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Sum};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

fn assert_reduced(x: &Natural, pow: u64) {
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
}

// When `pow` is at most `Limb::WIDTH`, every reduced summand fits in a `Limb`, and the `Limb`
// implementation sums them with wrapping additions. Otherwise the summands are added exactly and
// the sum is reduced once at the end: the running sum exceeds `pow` bits by only about the
// logarithm of the number of summands, so this is cheaper than reducing after every addition.
fn mod_power_of_2_sum_helper<B: Borrow<Natural>, I: Iterator<Item = B>>(
    xs: I,
    pow: u64,
) -> Natural {
    if pow <= Limb::WIDTH {
        Natural::from(Limb::mod_power_of_2_sum(
            xs.map(|x| {
                let x = x.borrow();
                assert_reduced(x, pow);
                Limb::exact_from(x)
            }),
            pow,
        ))
    } else {
        let mut sum = Natural::ZERO;
        for x in xs {
            let x = x.borrow();
            assert_reduced(x, pow);
            sum += x;
        }
        sum.mod_power_of_2(pow)
    }
}

impl ModPowerOf2Sum for Natural {
    /// Adds up all the [`Natural`]s in an iterator, modulo $2^k$. Every [`Natural`] must be already
    /// reduced modulo $2^k$.
    ///
    /// $f((x_i)_ {i=0}^{n-1}, k) = y$, where $y < 2^k$ and $\sum_ {i=0}^{n-1} x_i \equiv y \mod
    /// 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any [`Natural`] is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sum;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::mod_power_of_2_sum(
    ///         vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().into_iter(),
    ///         4
    ///     ),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::mod_power_of_2_sum(
    ///         vec_from_str::<Natural>("[18446744073709551615, 18446744073709551615, 2]")
    ///             .unwrap()
    ///             .into_iter(),
    ///         65
    ///     ),
    ///     Natural::from_str("0").unwrap()
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sum<I: Iterator<Item = Self>>(xs: I, pow: u64) -> Self {
        mod_power_of_2_sum_helper(xs, pow)
    }
}

impl<'a> ModPowerOf2Sum<&'a Self> for Natural {
    /// Adds up all the [`Natural`]s in an iterator of [`Natural`] references, modulo $2^k$. Every
    /// [`Natural`] must be already reduced modulo $2^k$.
    ///
    /// $f((x_i)_ {i=0}^{n-1}, k) = y$, where $y < 2^k$ and $\sum_ {i=0}^{n-1} x_i \equiv y \mod
    /// 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any [`Natural`] is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sum;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::mod_power_of_2_sum(vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().iter(), 4),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::mod_power_of_2_sum(
    ///         vec_from_str::<Natural>("[18446744073709551615, 18446744073709551615, 2]")
    ///             .unwrap()
    ///             .iter(),
    ///         65
    ///     ),
    ///     Natural::from_str("0").unwrap()
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sum<I: Iterator<Item = &'a Self>>(xs: I, pow: u64) -> Self {
        mod_power_of_2_sum_helper(xs, pow)
    }
}
