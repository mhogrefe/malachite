// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{FallingFactorial, RisingFactorial};
use malachite_base::num::basic::traits::{One, Zero};

impl FallingFactorial for Natural {
    type Output = Self;

    /// Computes the falling factorial of a [`Natural`]: the product of the `n` consecutive numbers
    /// counting down from `self`, or 1 when `n` is 0. The [`Natural`] is taken by value.
    ///
    /// $$
    /// f(x, n) = x^{\underline{n}} = x (x - 1) \cdots (x - n + 1).
    /// $$
    ///
    /// When `n` exceeds `self`, one of the factors is 0, and so is the result. Otherwise the
    /// falling factorial is the rising factorial of $x - n + 1$, which is how it is computed.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(6u32).falling_factorial(4), 360u32);
    /// assert_eq!(Natural::from(6u32).falling_factorial(6), 720u32);
    /// // A factor is 0.
    /// assert_eq!(Natural::from(3u32).falling_factorial(4), 0u32);
    /// assert_eq!(Natural::from(10u32).falling_factorial(0), 1u32);
    /// ```
    ///
    /// FLINT has no `fmpz` falling factorial; this is `gr_falling_ui` from `gr_special/bin.c`,
    /// FLINT 3.6.0, in the ring of integers, which also reduces to the rising factorial.
    fn falling_factorial(mut self, n: u64) -> Self {
        if n == 0 {
            Self::ONE
        } else if self < n {
            Self::ZERO
        } else {
            self -= Self::from(n - 1);
            self.rising_factorial(n)
        }
    }
}

impl FallingFactorial for &Natural {
    type Output = Natural;

    /// Computes the falling factorial of a [`Natural`]: the product of the `n` consecutive numbers
    /// counting down from `self`, or 1 when `n` is 0. The [`Natural`] is taken by reference.
    ///
    /// $$
    /// f(x, n) = x^{\underline{n}} = x (x - 1) \cdots (x - n + 1).
    /// $$
    ///
    /// When `n` exceeds `self`, one of the factors is 0, and so is the result. Otherwise the
    /// falling factorial is the rising factorial of $x - n + 1$, which is how it is computed.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(6u32)).falling_factorial(4), 360u32);
    /// assert_eq!((&Natural::from(6u32)).falling_factorial(6), 720u32);
    /// // A factor is 0.
    /// assert_eq!((&Natural::from(3u32)).falling_factorial(4), 0u32);
    /// assert_eq!((&Natural::from(10u32)).falling_factorial(0), 1u32);
    /// ```
    ///
    /// FLINT has no `fmpz` falling factorial; this is `gr_falling_ui` from `gr_special/bin.c`,
    /// FLINT 3.6.0, in the ring of integers, which also reduces to the rising factorial.
    fn falling_factorial(self, n: u64) -> Natural {
        if n == 0 {
            Natural::ONE
        } else if *self < n {
            Natural::ZERO
        } else {
            (self - Natural::from(n - 1)).rising_factorial(n)
        }
    }
}
