// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    ModAbsSquared, ModAbsSquaredAssign, ModSquare, ModSquareAssign,
};

impl ModAbsSquared<Self> for Natural {
    type Output = Self;

    /// Computes the squared absolute value of a [`Natural`] modulo another [`Natural`] $m$, which
    /// for a [`Natural`] is its square modulo $m$. The [`Natural`] must be already reduced modulo
    /// $m$. The [`Natural`] is taken by value and the modulus by value.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $|x|^2 = x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAbsSquared;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(100u32).mod_abs_squared(Natural::from(497u32)),
    ///     60
    /// );
    /// ```
    #[inline]
    fn mod_abs_squared(self, m: Self) -> Self {
        self.mod_square(m)
    }
}

impl ModAbsSquared<&Self> for Natural {
    type Output = Self;

    /// Computes the squared absolute value of a [`Natural`] modulo another [`Natural`] $m$, which
    /// for a [`Natural`] is its square modulo $m$. The [`Natural`] must be already reduced modulo
    /// $m$. The [`Natural`] is taken by value and the modulus by reference.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $|x|^2 = x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAbsSquared;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(100u32).mod_abs_squared(&Natural::from(497u32)),
    ///     60
    /// );
    /// ```
    #[inline]
    fn mod_abs_squared(self, m: &Self) -> Self {
        self.mod_square(m)
    }
}

impl ModAbsSquared<Natural> for &Natural {
    type Output = Natural;

    /// Computes the squared absolute value of a [`Natural`] modulo another [`Natural`] $m$, which
    /// for a [`Natural`] is its square modulo $m$. The [`Natural`] must be already reduced modulo
    /// $m$. The [`Natural`] is taken by reference and the modulus by value.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $|x|^2 = x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAbsSquared;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_abs_squared(Natural::from(497u32)),
    ///     60
    /// );
    /// ```
    #[inline]
    fn mod_abs_squared(self, m: Natural) -> Natural {
        self.mod_square(m)
    }
}

impl ModAbsSquared<&Natural> for &Natural {
    type Output = Natural;

    /// Computes the squared absolute value of a [`Natural`] modulo another [`Natural`] $m$, which
    /// for a [`Natural`] is its square modulo $m$. The [`Natural`] must be already reduced modulo
    /// $m$. The [`Natural`] is taken by reference and the modulus by reference.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $|x|^2 = x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAbsSquared;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_abs_squared(&Natural::from(497u32)),
    ///     60
    /// );
    /// ```
    #[inline]
    fn mod_abs_squared(self, m: &Natural) -> Natural {
        self.mod_square(m)
    }
}

impl ModAbsSquaredAssign<Self> for Natural {
    /// Replaces a [`Natural`] with its squared absolute value modulo another [`Natural`] $m$, which
    /// for a [`Natural`] is its square modulo $m$. The [`Natural`] must be already reduced modulo
    /// $m$. The modulus is taken by value.
    ///
    /// $x \gets y$, where $x, y < m$ and $|x|^2 = x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAbsSquaredAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(100u32);
    /// n.mod_abs_squared_assign(Natural::from(497u32));
    /// assert_eq!(n, 60);
    /// ```
    #[inline]
    fn mod_abs_squared_assign(&mut self, m: Self) {
        self.mod_square_assign(m);
    }
}

impl ModAbsSquaredAssign<&Self> for Natural {
    /// Replaces a [`Natural`] with its squared absolute value modulo another [`Natural`] $m$, which
    /// for a [`Natural`] is its square modulo $m$. The [`Natural`] must be already reduced modulo
    /// $m$. The modulus is taken by reference.
    ///
    /// $x \gets y$, where $x, y < m$ and $|x|^2 = x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAbsSquaredAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(100u32);
    /// n.mod_abs_squared_assign(&Natural::from(497u32));
    /// assert_eq!(n, 60);
    /// ```
    #[inline]
    fn mod_abs_squared_assign(&mut self, m: &Self) {
        self.mod_square_assign(m);
    }
}
