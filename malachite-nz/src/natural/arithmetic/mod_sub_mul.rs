// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{
    ModMul, ModMulAssign, ModSub, ModSubAssign, ModSubMul, ModSubMulAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;

fn assert_reduced(x: &Natural, y: &Natural, z: &Natural, m: &Natural) {
    assert!(x < m, "self must be reduced mod m, but {x} >= {m}");
    assert!(y < m, "y must be reduced mod m, but {y} >= {m}");
    assert!(z < m, "z must be reduced mod m, but {z} >= {m}");
}

// When `m` fits in a `Limb`, so does every reduced argument.
fn mod_sub_mul_limb(x: &Natural, y: &Natural, z: &Natural, m: Limb) -> Natural {
    Natural::from(Limb::exact_from(x).mod_sub_mul(Limb::exact_from(y), Limb::exact_from(z), m))
}

impl ModSubMul<Self, Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. All four [`Natural`]s are taken by
    /// value.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_sub_mul(Natural::TWO, Natural::from(5u32), Natural::from(7u32)),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_sub_mul(
    ///             Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "100000000000000005700"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, y: Self, z: Self, m: Self) -> Self {
        self.mod_sub_mul_assign(y, z, m);
        self
    }
}

impl<'c> ModSubMul<Self, Self, &'c Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by value, the third by value, and the fourth by reference.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_sub_mul(
    ///         Natural::TWO,
    ///         Natural::from(5u32),
    ///         &Natural::from(7u32)
    ///     ),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_sub_mul(
    ///             Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "100000000000000005700"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, y: Self, z: Self, m: &'c Self) -> Self {
        self.mod_sub_mul_assign(y, z, m);
        self
    }
}

impl<'b> ModSubMul<Self, &'b Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by value, the third by reference, and the fourth by value.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_sub_mul(
    ///         Natural::TWO,
    ///         &Natural::from(5u32),
    ///         Natural::from(7u32)
    ///     ),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_sub_mul(
    ///             Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "100000000000000005700"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, y: Self, z: &'b Self, m: Self) -> Self {
        self.mod_sub_mul_assign(y, z, m);
        self
    }
}

impl<'b, 'c> ModSubMul<Self, &'b Self, &'c Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by value, the third by reference, and the fourth by reference.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_sub_mul(
    ///         Natural::TWO,
    ///         &Natural::from(5u32),
    ///         &Natural::from(7u32)
    ///     ),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_sub_mul(
    ///             Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "100000000000000005700"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, y: Self, z: &'b Self, m: &'c Self) -> Self {
        self.mod_sub_mul_assign(y, z, m);
        self
    }
}

impl<'a> ModSubMul<&'a Self, Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by reference, the third by value, and the fourth by value.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_sub_mul(
    ///         &Natural::TWO,
    ///         Natural::from(5u32),
    ///         Natural::from(7u32)
    ///     ),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_sub_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "100000000000000005700"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, y: &'a Self, z: Self, m: Self) -> Self {
        self.mod_sub_mul_assign(y, z, m);
        self
    }
}

impl<'a, 'c> ModSubMul<&'a Self, Self, &'c Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by reference, the third by value, and the fourth by reference.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_sub_mul(
    ///         &Natural::TWO,
    ///         Natural::from(5u32),
    ///         &Natural::from(7u32)
    ///     ),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_sub_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "100000000000000005700"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, y: &'a Self, z: Self, m: &'c Self) -> Self {
        self.mod_sub_mul_assign(y, z, m);
        self
    }
}

impl<'a, 'b> ModSubMul<&'a Self, &'b Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by reference, the third by reference, and the fourth by value.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_sub_mul(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         Natural::from(7u32)
    ///     ),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_sub_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "100000000000000005700"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, y: &'a Self, z: &'b Self, m: Self) -> Self {
        self.mod_sub_mul_assign(y, z, m);
        self
    }
}

impl<'a, 'b, 'c> ModSubMul<&'a Self, &'b Self, &'c Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by reference, the third by reference, and the fourth by reference.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_sub_mul(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         &Natural::from(7u32)
    ///     ),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_sub_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "100000000000000005700"
    /// );
    /// ```
    #[inline]
    fn mod_sub_mul(mut self, y: &'a Self, z: &'b Self, m: &'c Self) -> Self {
        self.mod_sub_mul_assign(y, z, m);
        self
    }
}

impl ModSubMul<&Natural, &Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. All four [`Natural`]s are taken by
    /// reference.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_sub_mul(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         &Natural::from(7u32)
    ///     ),
    ///     3
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(20))
    ///         .mod_sub_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "100000000000000005700"
    /// );
    /// ```
    fn mod_sub_mul(self, y: &Natural, z: &Natural, m: &Natural) -> Natural {
        assert_reduced(self, y, z, m);
        if let Natural(Small(m)) = *m {
            mod_sub_mul_limb(self, y, z, m)
        } else {
            self.mod_sub(y.mod_mul(z, m), m)
        }
    }
}

impl ModSubMulAssign<Self, Self, Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo a fourth [`Natural`]
    /// $m$, in place. All three inputs must be already reduced modulo $m$. All three [`Natural`]s
    /// on the right-hand side are taken by value.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(6u32);
    /// x.mod_sub_mul_assign(Natural::TWO, Natural::from(5u32), Natural::from(7u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_sub_mul_assign(
    ///     Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "100000000000000005700");
    /// ```
    fn mod_sub_mul_assign(&mut self, mut y: Self, z: Self, m: Self) {
        assert_reduced(self, &y, &z, &m);
        if let Self(Small(m)) = m {
            *self = mod_sub_mul_limb(self, &y, &z, m);
        } else {
            y.mod_mul_assign(z, &m);
            self.mod_sub_assign(y, &m);
        }
    }
}

impl<'c> ModSubMulAssign<Self, Self, &'c Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo a fourth [`Natural`]
    /// $m$, in place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by value, by value, and by reference, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(6u32);
    /// x.mod_sub_mul_assign(Natural::TWO, Natural::from(5u32), &Natural::from(7u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_sub_mul_assign(
    ///     Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "100000000000000005700");
    /// ```
    fn mod_sub_mul_assign(&mut self, mut y: Self, z: Self, m: &'c Self) {
        assert_reduced(self, &y, &z, m);
        if let Self(Small(m)) = *m {
            *self = mod_sub_mul_limb(self, &y, &z, m);
        } else {
            y.mod_mul_assign(z, m);
            self.mod_sub_assign(y, m);
        }
    }
}

impl<'b> ModSubMulAssign<Self, &'b Self, Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo a fourth [`Natural`]
    /// $m$, in place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by value, by reference, and by value, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(6u32);
    /// x.mod_sub_mul_assign(Natural::TWO, &Natural::from(5u32), Natural::from(7u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_sub_mul_assign(
    ///     Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "100000000000000005700");
    /// ```
    fn mod_sub_mul_assign(&mut self, mut y: Self, z: &'b Self, m: Self) {
        assert_reduced(self, &y, z, &m);
        if let Self(Small(m)) = m {
            *self = mod_sub_mul_limb(self, &y, z, m);
        } else {
            y.mod_mul_assign(z, &m);
            self.mod_sub_assign(y, &m);
        }
    }
}

impl<'b, 'c> ModSubMulAssign<Self, &'b Self, &'c Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo a fourth [`Natural`]
    /// $m$, in place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by value, by reference, and by reference, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(6u32);
    /// x.mod_sub_mul_assign(Natural::TWO, &Natural::from(5u32), &Natural::from(7u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_sub_mul_assign(
    ///     Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "100000000000000005700");
    /// ```
    fn mod_sub_mul_assign(&mut self, mut y: Self, z: &'b Self, m: &'c Self) {
        assert_reduced(self, &y, z, m);
        if let Self(Small(m)) = *m {
            *self = mod_sub_mul_limb(self, &y, z, m);
        } else {
            y.mod_mul_assign(z, m);
            self.mod_sub_assign(y, m);
        }
    }
}

impl<'a> ModSubMulAssign<&'a Self, Self, Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo a fourth [`Natural`]
    /// $m$, in place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by reference, by value, and by value, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(6u32);
    /// x.mod_sub_mul_assign(&Natural::TWO, Natural::from(5u32), Natural::from(7u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_sub_mul_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "100000000000000005700");
    /// ```
    fn mod_sub_mul_assign(&mut self, y: &'a Self, mut z: Self, m: Self) {
        assert_reduced(self, y, &z, &m);
        if let Self(Small(m)) = m {
            *self = mod_sub_mul_limb(self, y, &z, m);
        } else {
            z.mod_mul_assign(y, &m);
            self.mod_sub_assign(z, &m);
        }
    }
}

impl<'a, 'c> ModSubMulAssign<&'a Self, Self, &'c Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo a fourth [`Natural`]
    /// $m$, in place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by reference, by value, and by reference, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(6u32);
    /// x.mod_sub_mul_assign(&Natural::TWO, Natural::from(5u32), &Natural::from(7u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_sub_mul_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "100000000000000005700");
    /// ```
    fn mod_sub_mul_assign(&mut self, y: &'a Self, mut z: Self, m: &'c Self) {
        assert_reduced(self, y, &z, m);
        if let Self(Small(m)) = *m {
            *self = mod_sub_mul_limb(self, y, &z, m);
        } else {
            z.mod_mul_assign(y, m);
            self.mod_sub_assign(z, m);
        }
    }
}

impl<'a, 'b> ModSubMulAssign<&'a Self, &'b Self, Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo a fourth [`Natural`]
    /// $m$, in place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by reference, by reference, and by value, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(6u32);
    /// x.mod_sub_mul_assign(&Natural::TWO, &Natural::from(5u32), Natural::from(7u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_sub_mul_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "100000000000000005700");
    /// ```
    fn mod_sub_mul_assign(&mut self, y: &'a Self, z: &'b Self, m: Self) {
        assert_reduced(self, y, z, &m);
        if let Self(Small(m)) = m {
            *self = mod_sub_mul_limb(self, y, z, m);
        } else {
            self.mod_sub_assign(y.mod_mul(z, &m), &m);
        }
    }
}

impl<'a, 'b, 'c> ModSubMulAssign<&'a Self, &'b Self, &'c Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo a fourth [`Natural`]
    /// $m$, in place. All three inputs must be already reduced modulo $m$. All three [`Natural`]s
    /// on the right-hand side are taken by reference.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x - yz \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModSubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(6u32);
    /// x.mod_sub_mul_assign(&Natural::TWO, &Natural::from(5u32), &Natural::from(7u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_sub_mul_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "100000000000000005700");
    /// ```
    fn mod_sub_mul_assign(&mut self, y: &'a Self, z: &'b Self, m: &'c Self) {
        assert_reduced(self, y, z, m);
        if let Self(Small(m)) = *m {
            *self = mod_sub_mul_limb(self, y, z, m);
        } else {
            self.mod_sub_assign(y.mod_mul(z, m), m);
        }
    }
}
