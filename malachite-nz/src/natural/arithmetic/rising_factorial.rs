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

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::RisingFactorial;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

// Multiplies the `step` consecutive factors starting at `y`. The caller guarantees that the product
// fits in a limb.
//
// This is rfac from fmpz/rfac.c, FLINT 3.6.0.
fn word_rfac(y: Limb, step: u64) -> Limb {
    let mut c = y;
    for i in 1..step {
        c *= y + Limb::exact_from(i);
    }
    c
}

// Computes the partial rising factorial `(x + a) (x + a + 1) ... (x + b - 1)`.
//
// This is _fmpz_rfac_ui from fmpz/rfac.c, FLINT 3.6.0, where x is positive and b > a. FLINT's
// small-operand bound guarantees that its factors fit in a word; a `Small` limb here can occupy the
// full width, so the packed path additionally checks that the largest factor still fits, and falls
// through to splitting otherwise, terminating at single multi-limb factors.
crate_test_fn! {limbs_rising_factorial_in_range(x: &Natural, a: u64, b: u64) -> Natural {
    let len = b - a;
    if len == 1 {
        return x + Natural::from(a);
    }
    if let Natural(Small(y)) = x {
        let y = *y;
        if len < 60
            && let Some(d) = Limb::try_from(b - 1).ok()
            && let Some(top) = y.checked_add(d)
        {
            // Bound the size of the largest factor, and pack as many factors as fit in a limb.
            let bits = top.significant_bits();
            let (step, factors_per_limb) = if len * bits < Limb::WIDTH {
                // The entire result fits in a single limb.
                (len, len)
            } else {
                let factors_per_limb = Limb::WIDTH / bits;
                (min(len, factors_per_limb), factors_per_limb)
            };
            let mut r = Natural::from(word_rfac(y + Limb::exact_from(a), step));
            let mut a = a + step;
            while a < b {
                let step = min(b - a, factors_per_limb);
                r *= Natural::from(word_rfac(y + Limb::exact_from(a), step));
                a += step;
            }
            return r;
        }
    }
    let m = a + (len >> 1);
    limbs_rising_factorial_in_range(x, a, m) * limbs_rising_factorial_in_range(x, m, b)
}}

fn rising_factorial_helper(x: &Natural, n: u64) -> Natural {
    if n == 0 {
        Natural::ONE
    } else if n == 1 {
        x.clone()
    } else if *x == 0u32 {
        Natural::ZERO
    } else {
        limbs_rising_factorial_in_range(x, 0, n)
    }
}

impl RisingFactorial for Natural {
    type Output = Self;

    /// Computes the rising factorial of a [`Natural`]: the product of the `n` consecutive numbers
    /// starting at `self`, or 1 when `n` is 0. The [`Natural`] is taken by value.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).rising_factorial(4), 360u32);
    /// assert_eq!(Natural::from(10u32).rising_factorial(0), 1u32);
    /// ```
    ///
    /// This is fmpz_rfac_uiui from fmpz/rfac.c, FLINT 3.6.0, generalized to a base of any size,
    /// which is also the nonnegative case of fmpz_rfac_ui.
    #[inline]
    fn rising_factorial(self, n: u64) -> Self {
        rising_factorial_helper(&self, n)
    }
}

impl RisingFactorial for &Natural {
    type Output = Natural;

    /// Computes the rising factorial of a [`Natural`]: the product of the `n` consecutive numbers
    /// starting at `self`, or 1 when `n` is 0. The [`Natural`] is taken by reference.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).rising_factorial(4), 360u32);
    /// assert_eq!((&Natural::from(10u32)).rising_factorial(0), 1u32);
    /// ```
    ///
    /// This is fmpz_rfac_uiui from fmpz/rfac.c, FLINT 3.6.0, generalized to a base of any size,
    /// which is also the nonnegative case of fmpz_rfac_ui.
    #[inline]
    fn rising_factorial(self, n: u64) -> Natural {
        rising_factorial_helper(self, n)
    }
}
