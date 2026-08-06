// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{Parity, RisingFactorial};
use malachite_base::num::basic::traits::{One, Zero};

// This is fmpz_rfac_ui from fmpz/rfac.c, FLINT 3.6.0. A negative base either spans zero, giving
// zero, or contributes an all-negative factor sequence: its magnitude is the rising factorial of
// the negated top factor, and its sign is the parity of the number of factors.
fn rising_factorial_helper(x: &Integer, n: u64) -> Integer {
    if n == 0 {
        Integer::ONE
    } else if n == 1 {
        x.clone()
    } else if *x == 0u32 {
        Integer::ZERO
    } else if *x < 0u32 {
        let abs = x.unsigned_abs_ref();
        if *abs < n {
            Integer::ZERO
        } else {
            let magnitude = (abs - Natural::from(n - 1)).rising_factorial(n);
            Integer::from_sign_and_abs(n.even(), magnitude)
        }
    } else {
        Integer::from(x.unsigned_abs_ref().rising_factorial(n))
    }
}

impl RisingFactorial for Integer {
    type Output = Self;

    /// Computes the rising factorial of an [`Integer`]: the product of the `n` consecutive numbers
    /// starting at `self`, or 1 when `n` is 0. The [`Integer`] is taken by value.
    ///
    /// A negative base whose factor sequence reaches or crosses zero gives exactly zero; otherwise
    /// all factors are negative, and the sign of the product is determined by the parity of `n`.
    ///
    /// $$
    /// f(x, n) = x^{(n)} = x (x + 1) \cdots (x + n - 1).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m (\log m)^2 \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the number of significant bits of
    /// the result.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RisingFactorial;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(3).rising_factorial(4), 360);
    /// assert_eq!(Integer::from(-5).rising_factorial(3), -60);
    /// assert_eq!(Integer::from(-2).rising_factorial(5), 0);
    /// ```
    ///
    /// This is fmpz_rfac_ui from fmpz/rfac.c, FLINT 3.6.0.
    #[inline]
    fn rising_factorial(self, n: u64) -> Self {
        rising_factorial_helper(&self, n)
    }
}

impl RisingFactorial for &Integer {
    type Output = Integer;

    /// Computes the rising factorial of an [`Integer`]: the product of the `n` consecutive numbers
    /// starting at `self`, or 1 when `n` is 0. The [`Integer`] is taken by reference.
    ///
    /// A negative base whose factor sequence reaches or crosses zero gives exactly zero; otherwise
    /// all factors are negative, and the sign of the product is determined by the parity of `n`.
    ///
    /// $$
    /// f(x, n) = x^{(n)} = x (x + 1) \cdots (x + n - 1).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(m) = O(m (\log m)^2 \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $m$ is the number of significant bits of
    /// the result.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RisingFactorial;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(3)).rising_factorial(4), 360);
    /// assert_eq!((&Integer::from(-5)).rising_factorial(3), -60);
    /// assert_eq!((&Integer::from(-2)).rising_factorial(5), 0);
    /// ```
    ///
    /// This is fmpz_rfac_ui from fmpz/rfac.c, FLINT 3.6.0.
    #[inline]
    fn rising_factorial(self, n: u64) -> Integer {
        rising_factorial_helper(self, n)
    }
}
