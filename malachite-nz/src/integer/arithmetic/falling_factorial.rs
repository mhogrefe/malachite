// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{FallingFactorial, RisingFactorial};
use malachite_base::num::basic::traits::One;

impl FallingFactorial for Integer {
    type Output = Self;

    /// Computes the falling factorial of an [`Integer`]: the product of the `n` consecutive numbers
    /// counting down from `self`, or 1 when `n` is 0. The [`Integer`] is taken by value.
    ///
    /// $$
    /// f(x, n) = x^{\underline{n}} = x (x - 1) \cdots (x - n + 1).
    /// $$
    ///
    /// The falling factorial is the rising factorial of $x - n + 1$, which is how it is computed.
    /// When the factors span 0, the result is 0, and when they are all negative, its sign is that
    /// of $(-1)^n$.
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
    /// use malachite_base::num::arithmetic::traits::FallingFactorial;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(6).falling_factorial(4), 360);
    /// // The factors span 0.
    /// assert_eq!(Integer::from(3).falling_factorial(5), 0);
    /// // The factors are all negative: (-2)(-3)(-4) = -24.
    /// assert_eq!(Integer::from(-2).falling_factorial(3), -24);
    /// assert_eq!(Integer::from(-10).falling_factorial(0), 1);
    /// ```
    ///
    /// FLINT has no `fmpz` falling factorial; this is `gr_falling_ui` from `gr_special/bin.c`,
    /// FLINT 3.6.0, in the ring of integers, which also reduces to the rising factorial.
    fn falling_factorial(mut self, n: u64) -> Self {
        if n == 0 {
            Self::ONE
        } else {
            self -= Self::from(n - 1);
            self.rising_factorial(n)
        }
    }
}

impl FallingFactorial for &Integer {
    type Output = Integer;

    /// Computes the falling factorial of an [`Integer`]: the product of the `n` consecutive numbers
    /// counting down from `self`, or 1 when `n` is 0. The [`Integer`] is taken by reference.
    ///
    /// $$
    /// f(x, n) = x^{\underline{n}} = x (x - 1) \cdots (x - n + 1).
    /// $$
    ///
    /// The falling factorial is the rising factorial of $x - n + 1$, which is how it is computed.
    /// When the factors span 0, the result is 0, and when they are all negative, its sign is that
    /// of $(-1)^n$.
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
    /// use malachite_base::num::arithmetic::traits::FallingFactorial;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(6)).falling_factorial(4), 360);
    /// // The factors span 0.
    /// assert_eq!((&Integer::from(3)).falling_factorial(5), 0);
    /// // The factors are all negative: (-2)(-3)(-4) = -24.
    /// assert_eq!((&Integer::from(-2)).falling_factorial(3), -24);
    /// assert_eq!((&Integer::from(-10)).falling_factorial(0), 1);
    /// ```
    ///
    /// FLINT has no `fmpz` falling factorial; this is `gr_falling_ui` from `gr_special/bin.c`,
    /// FLINT 3.6.0, in the ring of integers, which also reduces to the rising factorial.
    fn falling_factorial(self, n: u64) -> Integer {
        if n == 0 {
            Integer::ONE
        } else {
            (self - Integer::from(n - 1)).rising_factorial(n)
        }
    }
}
