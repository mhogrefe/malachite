// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::arithmetic::mod_mul::ModMulData;
use malachite_base::num::arithmetic::traits::{
    ModMulPrecomputed, ModPow, ModPowAssign, ModSquare, ModSquareAssign, ModSquarePrecomputed,
    ModSquarePrecomputedAssign,
};
use malachite_base::num::basic::traits::Two;

impl ModSquare<Self> for Natural {
    type Output = Self;

    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. Both [`Natural`]s are taken by value.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(2u32).mod_square(Natural::from(10u32)), 4);
    /// assert_eq!(Natural::from(100u32).mod_square(Natural::from(497u32)), 60);
    /// ```
    #[inline]
    fn mod_square(self, m: Self) -> Self {
        (&self).mod_pow(&Self::TWO, &m)
    }
}

impl ModSquare<&Self> for Natural {
    type Output = Self;

    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. The first [`Natural`] is taken by value and the second by reference.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(2u32).mod_square(&Natural::from(10u32)), 4);
    /// assert_eq!(Natural::from(100u32).mod_square(&Natural::from(497u32)), 60);
    /// ```
    #[inline]
    fn mod_square(self, m: &Self) -> Self {
        (&self).mod_pow(&Self::TWO, m)
    }
}

impl ModSquare<Natural> for &Natural {
    type Output = Natural;

    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. The first [`Natural`] is taken by reference and the second by value.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(2u32)).mod_square(Natural::from(10u32)), 4);
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_square(Natural::from(497u32)),
    ///     60
    /// );
    /// ```
    #[inline]
    fn mod_square(self, m: Natural) -> Natural {
        self.mod_pow(&Natural::TWO, &m)
    }
}

impl ModSquare<&Natural> for &Natural {
    type Output = Natural;

    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. Both [`Natural`]s are taken by reference.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(2u32)).mod_square(&Natural::from(10u32)), 4);
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_square(&Natural::from(497u32)),
    ///     60
    /// );
    /// ```
    #[inline]
    fn mod_square(self, m: &Natural) -> Natural {
        self.mod_pow(&Natural::TWO, m)
    }
}

impl ModSquareAssign<Self> for Natural {
    /// Squares a [`Natural`] modulo another [`Natural`] $m$, in place. The input must be already
    /// reduced modulo $m$. The [`Natural`] on the right-hand side is taken by value.
    ///
    /// $x \gets y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(2u32);
    /// x.mod_square_assign(Natural::from(10u32));
    /// assert_eq!(x, 4);
    ///
    /// let mut x = Natural::from(100u32);
    /// x.mod_square_assign(Natural::from(497u32));
    /// assert_eq!(x, 60);
    /// ```
    #[inline]
    fn mod_square_assign(&mut self, m: Self) {
        self.mod_pow_assign(&Self::TWO, &m);
    }
}

impl ModSquareAssign<&Self> for Natural {
    /// Squares a [`Natural`] modulo another [`Natural`] $m$, in place. The input must be already
    /// reduced modulo $m$. The [`Natural`] on the right-hand side is taken by reference.
    ///
    /// $x \gets y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(2u32);
    /// x.mod_square_assign(&Natural::from(10u32));
    /// assert_eq!(x, 4);
    ///
    /// let mut x = Natural::from(100u32);
    /// x.mod_square_assign(&Natural::from(497u32));
    /// assert_eq!(x, 60);
    /// ```
    #[inline]
    fn mod_square_assign(&mut self, m: &Self) {
        self.mod_pow_assign(&Self::TWO, m);
    }
}

impl ModSquarePrecomputed<Natural, Self> for Natural {
    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. Both [`Natural`]s are taken by value.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// squarings with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_pow_data`](malachite_base::num::arithmetic::traits::ModPowPrecomputed).
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowPrecomputed, ModSquarePrecomputed};
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(497u32));
    /// assert_eq!(
    ///     Natural::from(100u32).mod_square_precomputed(Natural::from(497u32), &data),
    ///     60
    /// );
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     Natural::from(2u32).mod_square_precomputed(Natural::from(10u32), &data),
    ///     4
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mul` from `fmpz_mod/mul.c`, FLINT 3.6.0, where `b == c`.
    #[inline]
    fn mod_square_precomputed(self, m: Self, data: &ModMulData) -> Natural {
        (&self).mod_mul_precomputed(&self, &m, data)
    }
}

impl<'a> ModSquarePrecomputed<Natural, &'a Self> for Natural {
    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. The first [`Natural`] is taken by value and the second by reference.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// squarings with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_pow_data`](malachite_base::num::arithmetic::traits::ModPowPrecomputed).
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowPrecomputed, ModSquarePrecomputed};
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(497u32));
    /// assert_eq!(
    ///     Natural::from(100u32).mod_square_precomputed(&Natural::from(497u32), &data),
    ///     60
    /// );
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     Natural::from(2u32).mod_square_precomputed(&Natural::from(10u32), &data),
    ///     4
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mul` from `fmpz_mod/mul.c`, FLINT 3.6.0, where `b == c`.
    #[inline]
    fn mod_square_precomputed(self, m: &'a Self, data: &ModMulData) -> Natural {
        (&self).mod_mul_precomputed(&self, m, data)
    }
}

impl ModSquarePrecomputed<Natural, Natural> for &Natural {
    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. The first [`Natural`] is taken by reference and the second by value.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// squarings with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_pow_data`](malachite_base::num::arithmetic::traits::ModPowPrecomputed).
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowPrecomputed, ModSquarePrecomputed};
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(497u32));
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_square_precomputed(Natural::from(497u32), &data),
    ///     60
    /// );
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     (&Natural::from(2u32)).mod_square_precomputed(Natural::from(10u32), &data),
    ///     4
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mul` from `fmpz_mod/mul.c`, FLINT 3.6.0, where `b == c`.
    #[inline]
    fn mod_square_precomputed(self, m: Natural, data: &ModMulData) -> Natural {
        self.mod_mul_precomputed(self, &m, data)
    }
}

impl ModSquarePrecomputed<Natural, &Natural> for &Natural {
    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. Both [`Natural`]s are taken by reference.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// squarings with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_pow_data`](malachite_base::num::arithmetic::traits::ModPowPrecomputed).
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowPrecomputed, ModSquarePrecomputed};
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(497u32));
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_square_precomputed(&Natural::from(497u32), &data),
    ///     60
    /// );
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(10u32));
    /// assert_eq!(
    ///     (&Natural::from(2u32)).mod_square_precomputed(&Natural::from(10u32), &data),
    ///     4
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mul` from `fmpz_mod/mul.c`, FLINT 3.6.0, where `b == c`.
    #[inline]
    fn mod_square_precomputed(self, m: &Natural, data: &ModMulData) -> Natural {
        self.mod_mul_precomputed(self, m, data)
    }
}

impl ModSquarePrecomputedAssign<Natural, Self> for Natural {
    /// Squares a [`Natural`] modulo another [`Natural`] $m$, in place. The input must be already
    /// reduced modulo $m$. The [`Natural`] on the right-hand side is taken by value.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// squarings with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_pow_data`](malachite_base::num::arithmetic::traits::ModPowPrecomputed).
    ///
    /// $x \\gets y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{
    ///     ModPowPrecomputed, ModSquarePrecomputedAssign,
    /// };
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(497u32));
    /// let mut x = Natural::from(100u32);
    /// x.mod_square_precomputed_assign(Natural::from(497u32), &data);
    /// assert_eq!(x, 60);
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(10u32));
    /// let mut x = Natural::from(2u32);
    /// x.mod_square_precomputed_assign(Natural::from(10u32), &data);
    /// assert_eq!(x, 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mul` from `fmpz_mod/mul.c`, FLINT 3.6.0, where `a == b ==
    /// c`.
    #[inline]
    fn mod_square_precomputed_assign(&mut self, m: Self, data: &ModMulData) {
        *self = (&*self).mod_mul_precomputed(&*self, &m, data);
    }
}

impl<'a> ModSquarePrecomputedAssign<Natural, &'a Self> for Natural {
    /// Squares a [`Natural`] modulo another [`Natural`] $m$, in place. The input must be already
    /// reduced modulo $m$. The [`Natural`] on the right-hand side is taken by reference.
    ///
    /// Some precomputed data is provided; this speeds up computations involving several modular
    /// squarings with the same modulus. The precomputed data should be obtained using
    /// [`precompute_mod_pow_data`](malachite_base::num::arithmetic::traits::ModPowPrecomputed).
    ///
    /// $x \\gets y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{
    ///     ModPowPrecomputed, ModSquarePrecomputedAssign,
    /// };
    /// use malachite_nz::natural::Natural;
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(497u32));
    /// let mut x = Natural::from(100u32);
    /// x.mod_square_precomputed_assign(&Natural::from(497u32), &data);
    /// assert_eq!(x, 60);
    ///
    /// let data = ModPowPrecomputed::<Natural>::precompute_mod_pow_data(&Natural::from(10u32));
    /// let mut x = Natural::from(2u32);
    /// x.mod_square_precomputed_assign(&Natural::from(10u32), &data);
    /// assert_eq!(x, 4);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mul` from `fmpz_mod/mul.c`, FLINT 3.6.0, where `a == b ==
    /// c`.
    #[inline]
    fn mod_square_precomputed_assign(&mut self, m: &'a Self, data: &ModMulData) {
        *self = (&*self).mod_mul_precomputed(&*self, m, data);
    }
}
