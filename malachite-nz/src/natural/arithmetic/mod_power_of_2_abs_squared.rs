// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2AbsSquared, ModPowerOf2AbsSquaredAssign, ModPowerOf2Square, ModPowerOf2SquareAssign,
};

impl ModPowerOf2AbsSquared for Natural {
    type Output = Self;

    /// Computes the squared absolute value of a [`Natural`] modulo $2^k$, which for a [`Natural`]
    /// is its square modulo $2^k$. The [`Natural`] must be already reduced modulo $2^k$. The
    /// [`Natural`] is taken by value.
    ///
    /// $f(x, k) = y$, where $x, y < 2^k$ and $|x|^2 = x^2 \equiv y \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AbsSquared;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(5u32).mod_power_of_2_abs_squared(3), 1);
    /// ```
    #[inline]
    fn mod_power_of_2_abs_squared(self, pow: u64) -> Self {
        self.mod_power_of_2_square(pow)
    }
}

impl ModPowerOf2AbsSquared for &Natural {
    type Output = Natural;

    /// Computes the squared absolute value of a [`Natural`] modulo $2^k$, which for a [`Natural`]
    /// is its square modulo $2^k$. The [`Natural`] must be already reduced modulo $2^k$. The
    /// [`Natural`] is taken by reference.
    ///
    /// $f(x, k) = y$, where $x, y < 2^k$ and $|x|^2 = x^2 \equiv y \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AbsSquared;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(5u32)).mod_power_of_2_abs_squared(3), 1);
    /// ```
    #[inline]
    fn mod_power_of_2_abs_squared(self, pow: u64) -> Natural {
        self.mod_power_of_2_square(pow)
    }
}

impl ModPowerOf2AbsSquaredAssign for Natural {
    /// Replaces a [`Natural`] with its squared absolute value modulo $2^k$, which for a [`Natural`]
    /// is its square modulo $2^k$. The [`Natural`] must be already reduced modulo $2^k$.
    ///
    /// $x \gets y$, where $x, y < 2^k$ and $|x|^2 = x^2 \equiv y \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AbsSquaredAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(5u32);
    /// n.mod_power_of_2_abs_squared_assign(3);
    /// assert_eq!(n, 1);
    /// ```
    #[inline]
    fn mod_power_of_2_abs_squared_assign(&mut self, pow: u64) {
        self.mod_power_of_2_square_assign(pow);
    }
}
