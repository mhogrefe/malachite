// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{CheckedRoot, UnsignedAbs};
use malachite_base::num::basic::traits::NegativeOne;
use malachite_base::num::factorization::traits::{ExpressAsPower, IsPower, Primes};
use malachite_base::num::logic::traits::SignificantBits;

// A negative value is a perfect power exactly when its absolute value is a perfect $p$th power for
// some odd prime $p$: if $x = a^b$ with $b > 1$ and $x < 0$ then $b$ is odd, so $b$ has an odd
// prime factor $p$ and $x = (a^{b/p})^p$; conversely $|x| = c^p$ with $p$ odd gives $x = (-c)^p$.
// Only exponents up to the bit length can work, since the smallest $p$th power above 1 is $2^p$.
fn negative_power_root(abs: &Natural, exp: u64) -> Option<Natural> {
    abs.checked_root(exp)
}

fn odd_prime_exponents(abs: &Natural) -> impl Iterator<Item = u64> {
    u64::primes_less_than_or_equal_to(&abs.significant_bits()).skip(1)
}

impl IsPower for Integer {
    /// Determines whether an [`Integer`] is a perfect power.
    ///
    /// A perfect power is any number of the form $a^x$ where $x > 1$, with $a$ and $x$ both
    /// integers. In particular, 0 and 1 are considered perfect powers.
    ///
    /// A negative [`Integer`] can only be an odd perfect power, since an even power is
    /// non-negative. For instance $-8 = (-2)^3$ is a perfect power but $-16$ is not, and $-1$ is,
    /// being $(-1)^3$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::num::factorization::traits::IsPower;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.is_power(), true);
    /// assert_eq!(Integer::ONE.is_power(), true);
    /// assert_eq!(Integer::from(8).is_power(), true);
    /// assert_eq!(Integer::from(6).is_power(), false);
    ///
    /// assert_eq!(Integer::NEGATIVE_ONE.is_power(), true);
    /// assert_eq!(Integer::from(-8).is_power(), true);
    /// assert_eq!(Integer::from(-16).is_power(), false);
    /// ```
    fn is_power(&self) -> bool {
        if *self >= 0u32 {
            return self.unsigned_abs_ref().is_power();
        }
        let abs = self.unsigned_abs();
        // -1 is (-1)^3, but its bit length admits no exponent below
        abs == 1u32 || odd_prime_exponents(&abs).any(|p| negative_power_root(&abs, p).is_some())
    }
}

impl ExpressAsPower for Integer {
    /// Expresses an [`Integer`] as a perfect power if possible.
    ///
    /// Returns `Some((root, exponent))` where `root ^ exponent = self` and `exponent > 1`, or
    /// `None` if the number cannot be expressed as a perfect power.
    ///
    /// The exponent returned for a negative [`Integer`] is always odd, since an even power is
    /// non-negative.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::num::factorization::traits::ExpressAsPower;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(8).express_as_power(), Some((Integer::TWO, 3)));
    /// assert_eq!(Integer::from(6).express_as_power(), None);
    ///
    /// assert_eq!(
    ///     Integer::from(-8).express_as_power(),
    ///     Some((Integer::from(-2), 3))
    /// );
    /// assert_eq!(Integer::from(-16).express_as_power(), None);
    /// ```
    fn express_as_power(&self) -> Option<(Self, u64)> {
        if *self >= 0u32 {
            return self
                .unsigned_abs_ref()
                .express_as_power()
                .map(|(root, exp)| (Self::from(root), exp));
        }
        let abs = self.unsigned_abs();
        if abs == 1u32 {
            // -1 = (-1)^3
            return Some((Self::NEGATIVE_ONE, 3));
        }
        odd_prime_exponents(&abs)
            .find_map(|p| negative_power_root(&abs, p).map(|root| (-Self::from(root), p)))
    }
}
