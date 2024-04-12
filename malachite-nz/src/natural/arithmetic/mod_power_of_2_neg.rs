// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Neg, ModPowerOf2NegAssign, NegModPowerOf2, NegModPowerOf2Assign,
};
use malachite_base::num::logic::traits::SignificantBits;

impl ModPowerOf2Neg for Natural {
    type Output = Natural;

    /// Negates a [`Natural`] modulo $2^k$. The input must be already reduced modulo $2^k$. The
    /// [`Natural`] is taken by value.
    ///
    /// $f(x, k) = y$, where $x, y < 2^k$ and $-x \equiv y \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_2_neg(5), 0);
    /// assert_eq!(Natural::ZERO.mod_power_of_2_neg(100), 0);
    /// assert_eq!(Natural::from(100u32).mod_power_of_2_neg(8), 156);
    /// assert_eq!(
    ///     Natural::from(100u32).mod_power_of_2_neg(100).to_string(),
    ///     "1267650600228229401496703205276"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_neg(mut self, pow: u64) -> Natural {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        self.neg_mod_power_of_2_assign(pow);
        self
    }
}

impl<'a> ModPowerOf2Neg for &'a Natural {
    type Output = Natural;

    /// Negates a [`Natural`] modulo $2^k$. The input must be already reduced modulo $2^k$. The
    /// [`Natural`] is taken by reference.
    ///
    /// $f(x, k) = y$, where $x, y < 2^k$ and $-x \equiv y \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).mod_power_of_2_neg(5), 0);
    /// assert_eq!((&Natural::ZERO).mod_power_of_2_neg(100), 0);
    /// assert_eq!((&Natural::from(100u32)).mod_power_of_2_neg(8), 156);
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_power_of_2_neg(100).to_string(),
    ///     "1267650600228229401496703205276"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_neg(self, pow: u64) -> Natural {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        self.neg_mod_power_of_2(pow)
    }
}

impl ModPowerOf2NegAssign for Natural {
    /// Negates a [`Natural`] modulo $2^k$, in place. The input must be already reduced modulo
    /// $2^k$.
    ///
    /// $x \gets y$, where $x, y < 2^p$ and $-x \equiv y \mod 2^p$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_power_of_2_neg_assign(5);
    /// assert_eq!(n, 0);
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_power_of_2_neg_assign(100);
    /// assert_eq!(n, 0);
    ///
    /// let mut n = Natural::from(100u32);
    /// n.mod_power_of_2_neg_assign(8);
    /// assert_eq!(n, 156);
    ///
    /// let mut n = Natural::from(100u32);
    /// n.mod_power_of_2_neg_assign(100);
    /// assert_eq!(n.to_string(), "1267650600228229401496703205276");
    /// ```
    #[inline]
    fn mod_power_of_2_neg_assign(&mut self, pow: u64) {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        self.neg_mod_power_of_2_assign(pow);
    }
}
