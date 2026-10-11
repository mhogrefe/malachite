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
    ModAdd, ModAddAssign, ModAddMulShl, ModAddMulShlAssign, ModMul, ModMulAssign, ModShl,
    ModShlAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;

fn assert_reduced(x: &Natural, y: &Natural, z: &Natural, m: &Natural) {
    assert!(x < m, "self must be reduced mod m, but {x} >= {m}");
    assert!(y < m, "y must be reduced mod m, but {y} >= {m}");
    assert!(z < m, "z must be reduced mod m, but {z} >= {m}");
}

// When `m` fits in a `Limb`, so does every reduced argument.
fn mod_add_mul_shl_limb(x: &Natural, y: &Natural, z: &Natural, bits: u64, m: Limb) -> Natural {
    Natural::from(Limb::exact_from(x).mod_add_mul_shl(
        Limb::exact_from(y),
        Limb::exact_from(z),
        bits,
        m,
    ))
}

impl ModAddMulShl<Self, Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// a fourth [`Natural`] $m$. All three inputs must be already reduced modulo $m$. All four
    /// [`Natural`]s are taken by value.
    ///
    /// $f(x, y, z, b, m) = w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul_shl(
    ///         Natural::TWO,
    ///         Natural::from(5u32),
    ///         1,
    ///         Natural::from(7u32)
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul_shl(
    ///             Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             5,
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "99999999999999817600"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, y: Self, z: Self, bits: u64, m: Self) -> Self {
        self.mod_add_mul_shl_assign(y, z, bits, m);
        self
    }
}

impl<'c> ModAddMulShl<Self, Self, &'c Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// a fourth [`Natural`] $m$. All three inputs must be already reduced modulo $m$. The first
    /// [`Natural`] is taken by value, the second by value, the third by value, and the fourth by
    /// reference.
    ///
    /// $f(x, y, z, b, m) = w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul_shl(
    ///         Natural::TWO,
    ///         Natural::from(5u32),
    ///         1,
    ///         &Natural::from(7u32)
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul_shl(
    ///             Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             5,
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "99999999999999817600"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, y: Self, z: Self, bits: u64, m: &'c Self) -> Self {
        self.mod_add_mul_shl_assign(y, z, bits, m);
        self
    }
}

impl<'b> ModAddMulShl<Self, &'b Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// a fourth [`Natural`] $m$. All three inputs must be already reduced modulo $m$. The first
    /// [`Natural`] is taken by value, the second by value, the third by reference, and the fourth
    /// by value.
    ///
    /// $f(x, y, z, b, m) = w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul_shl(
    ///         Natural::TWO,
    ///         &Natural::from(5u32),
    ///         1,
    ///         Natural::from(7u32)
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul_shl(
    ///             Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             5,
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "99999999999999817600"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, y: Self, z: &'b Self, bits: u64, m: Self) -> Self {
        self.mod_add_mul_shl_assign(y, z, bits, m);
        self
    }
}

impl<'b, 'c> ModAddMulShl<Self, &'b Self, &'c Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// a fourth [`Natural`] $m$. All three inputs must be already reduced modulo $m$. The first
    /// [`Natural`] is taken by value, the second by value, the third by reference, and the fourth
    /// by reference.
    ///
    /// $f(x, y, z, b, m) = w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul_shl(
    ///         Natural::TWO,
    ///         &Natural::from(5u32),
    ///         1,
    ///         &Natural::from(7u32)
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul_shl(
    ///             Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             5,
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "99999999999999817600"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, y: Self, z: &'b Self, bits: u64, m: &'c Self) -> Self {
        self.mod_add_mul_shl_assign(y, z, bits, m);
        self
    }
}

impl<'a> ModAddMulShl<&'a Self, Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// a fourth [`Natural`] $m$. All three inputs must be already reduced modulo $m$. The first
    /// [`Natural`] is taken by value, the second by reference, the third by value, and the fourth
    /// by value.
    ///
    /// $f(x, y, z, b, m) = w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul_shl(
    ///         &Natural::TWO,
    ///         Natural::from(5u32),
    ///         1,
    ///         Natural::from(7u32)
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul_shl(
    ///             &Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             5,
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "99999999999999817600"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, y: &'a Self, z: Self, bits: u64, m: Self) -> Self {
        self.mod_add_mul_shl_assign(y, z, bits, m);
        self
    }
}

impl<'a, 'c> ModAddMulShl<&'a Self, Self, &'c Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// a fourth [`Natural`] $m$. All three inputs must be already reduced modulo $m$. The first
    /// [`Natural`] is taken by value, the second by reference, the third by value, and the fourth
    /// by reference.
    ///
    /// $f(x, y, z, b, m) = w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul_shl(
    ///         &Natural::TWO,
    ///         Natural::from(5u32),
    ///         1,
    ///         &Natural::from(7u32)
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul_shl(
    ///             &Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             5,
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "99999999999999817600"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, y: &'a Self, z: Self, bits: u64, m: &'c Self) -> Self {
        self.mod_add_mul_shl_assign(y, z, bits, m);
        self
    }
}

impl<'a, 'b> ModAddMulShl<&'a Self, &'b Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// a fourth [`Natural`] $m$. All three inputs must be already reduced modulo $m$. The first
    /// [`Natural`] is taken by value, the second by reference, the third by reference, and the
    /// fourth by value.
    ///
    /// $f(x, y, z, b, m) = w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul_shl(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         1,
    ///         Natural::from(7u32)
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul_shl(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             5,
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "99999999999999817600"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, y: &'a Self, z: &'b Self, bits: u64, m: Self) -> Self {
        self.mod_add_mul_shl_assign(y, z, bits, m);
        self
    }
}

impl<'a, 'b, 'c> ModAddMulShl<&'a Self, &'b Self, &'c Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// a fourth [`Natural`] $m$. All three inputs must be already reduced modulo $m$. The first
    /// [`Natural`] is taken by value, the second by reference, the third by reference, and the
    /// fourth by reference.
    ///
    /// $f(x, y, z, b, m) = w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul_shl(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         1,
    ///         &Natural::from(7u32)
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul_shl(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             5,
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "99999999999999817600"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul_shl(mut self, y: &'a Self, z: &'b Self, bits: u64, m: &'c Self) -> Self {
        self.mod_add_mul_shl_assign(y, z, bits, m);
        self
    }
}

impl ModAddMulShl<&Natural, &Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// a fourth [`Natural`] $m$. All three inputs must be already reduced modulo $m$. All four
    /// [`Natural`]s are taken by reference.
    ///
    /// $f(x, y, z, b, m) = w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_add_mul_shl(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         1,
    ///         &Natural::from(7u32)
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(20))
    ///         .mod_add_mul_shl(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             5,
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "99999999999999817600"
    /// );
    /// ```
    fn mod_add_mul_shl(self, y: &Natural, z: &Natural, bits: u64, m: &Natural) -> Natural {
        assert_reduced(self, y, z, m);
        if let Natural(Small(m)) = *m {
            mod_add_mul_shl_limb(self, y, z, bits, m)
        } else {
            self.mod_add(y.mod_mul(z, m).mod_shl(bits, m), m)
        }
    }
}

impl ModAddMulShlAssign<Self, Self, Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo a
    /// fourth [`Natural`] $m$, in place. All three inputs must be already reduced modulo $m$. All
    /// three [`Natural`]s on the right-hand side are taken by value.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_shl_assign(Natural::TWO, Natural::from(5u32), 1, Natural::from(7u32));
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_shl_assign(
    ///     Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     5,
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999817600");
    /// ```
    fn mod_add_mul_shl_assign(&mut self, mut y: Self, z: Self, bits: u64, m: Self) {
        assert_reduced(self, &y, &z, &m);
        if let Self(Small(m)) = m {
            *self = mod_add_mul_shl_limb(self, &y, &z, bits, m);
        } else {
            y.mod_mul_assign(z, &m);
            y.mod_shl_assign(bits, &m);
            self.mod_add_assign(y, &m);
        }
    }
}

impl<'c> ModAddMulShlAssign<Self, Self, &'c Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo a
    /// fourth [`Natural`] $m$, in place. All three inputs must be already reduced modulo $m$. The
    /// [`Natural`]s on the right-hand side are taken by value, by value, and by reference,
    /// respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_shl_assign(Natural::TWO, Natural::from(5u32), 1, &Natural::from(7u32));
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_shl_assign(
    ///     Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     5,
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999817600");
    /// ```
    fn mod_add_mul_shl_assign(&mut self, mut y: Self, z: Self, bits: u64, m: &'c Self) {
        assert_reduced(self, &y, &z, m);
        if let Self(Small(m)) = *m {
            *self = mod_add_mul_shl_limb(self, &y, &z, bits, m);
        } else {
            y.mod_mul_assign(z, m);
            y.mod_shl_assign(bits, m);
            self.mod_add_assign(y, m);
        }
    }
}

impl<'b> ModAddMulShlAssign<Self, &'b Self, Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo a
    /// fourth [`Natural`] $m$, in place. All three inputs must be already reduced modulo $m$. The
    /// [`Natural`]s on the right-hand side are taken by value, by reference, and by value,
    /// respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_shl_assign(Natural::TWO, &Natural::from(5u32), 1, Natural::from(7u32));
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_shl_assign(
    ///     Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     5,
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999817600");
    /// ```
    fn mod_add_mul_shl_assign(&mut self, mut y: Self, z: &'b Self, bits: u64, m: Self) {
        assert_reduced(self, &y, z, &m);
        if let Self(Small(m)) = m {
            *self = mod_add_mul_shl_limb(self, &y, z, bits, m);
        } else {
            y.mod_mul_assign(z, &m);
            y.mod_shl_assign(bits, &m);
            self.mod_add_assign(y, &m);
        }
    }
}

impl<'b, 'c> ModAddMulShlAssign<Self, &'b Self, &'c Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo a
    /// fourth [`Natural`] $m$, in place. All three inputs must be already reduced modulo $m$. The
    /// [`Natural`]s on the right-hand side are taken by value, by reference, and by reference,
    /// respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_shl_assign(Natural::TWO, &Natural::from(5u32), 1, &Natural::from(7u32));
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_shl_assign(
    ///     Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     5,
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999817600");
    /// ```
    fn mod_add_mul_shl_assign(&mut self, mut y: Self, z: &'b Self, bits: u64, m: &'c Self) {
        assert_reduced(self, &y, z, m);
        if let Self(Small(m)) = *m {
            *self = mod_add_mul_shl_limb(self, &y, z, bits, m);
        } else {
            y.mod_mul_assign(z, m);
            y.mod_shl_assign(bits, m);
            self.mod_add_assign(y, m);
        }
    }
}

impl<'a> ModAddMulShlAssign<&'a Self, Self, Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo a
    /// fourth [`Natural`] $m$, in place. All three inputs must be already reduced modulo $m$. The
    /// [`Natural`]s on the right-hand side are taken by reference, by value, and by value,
    /// respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_shl_assign(&Natural::TWO, Natural::from(5u32), 1, Natural::from(7u32));
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_shl_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     5,
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999817600");
    /// ```
    fn mod_add_mul_shl_assign(&mut self, y: &'a Self, mut z: Self, bits: u64, m: Self) {
        assert_reduced(self, y, &z, &m);
        if let Self(Small(m)) = m {
            *self = mod_add_mul_shl_limb(self, y, &z, bits, m);
        } else {
            z.mod_mul_assign(y, &m);
            z.mod_shl_assign(bits, &m);
            self.mod_add_assign(z, &m);
        }
    }
}

impl<'a, 'c> ModAddMulShlAssign<&'a Self, Self, &'c Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo a
    /// fourth [`Natural`] $m$, in place. All three inputs must be already reduced modulo $m$. The
    /// [`Natural`]s on the right-hand side are taken by reference, by value, and by reference,
    /// respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_shl_assign(&Natural::TWO, Natural::from(5u32), 1, &Natural::from(7u32));
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_shl_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     5,
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999817600");
    /// ```
    fn mod_add_mul_shl_assign(&mut self, y: &'a Self, mut z: Self, bits: u64, m: &'c Self) {
        assert_reduced(self, y, &z, m);
        if let Self(Small(m)) = *m {
            *self = mod_add_mul_shl_limb(self, y, &z, bits, m);
        } else {
            z.mod_mul_assign(y, m);
            z.mod_shl_assign(bits, m);
            self.mod_add_assign(z, m);
        }
    }
}

impl<'a, 'b> ModAddMulShlAssign<&'a Self, &'b Self, Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo a
    /// fourth [`Natural`] $m$, in place. All three inputs must be already reduced modulo $m$. The
    /// [`Natural`]s on the right-hand side are taken by reference, by reference, and by value,
    /// respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_shl_assign(&Natural::TWO, &Natural::from(5u32), 1, Natural::from(7u32));
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_shl_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     5,
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999817600");
    /// ```
    fn mod_add_mul_shl_assign(&mut self, y: &'a Self, z: &'b Self, bits: u64, m: Self) {
        assert_reduced(self, y, z, &m);
        if let Self(Small(m)) = m {
            *self = mod_add_mul_shl_limb(self, y, z, bits, m);
        } else {
            self.mod_add_assign(y.mod_mul(z, &m).mod_shl(bits, &m), &m);
        }
    }
}

impl<'a, 'b, 'c> ModAddMulShlAssign<&'a Self, &'b Self, &'c Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo a
    /// fourth [`Natural`] $m$, in place. All three inputs must be already reduced modulo $m$. All
    /// three [`Natural`]s on the right-hand side are taken by reference.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz2^b \equiv w \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n \log n \log\log n \log b)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m.significant_bits()`, and $b$ is
    /// `bits`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModAddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_shl_assign(&Natural::TWO, &Natural::from(5u32), 1, &Natural::from(7u32));
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_shl_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     5,
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999817600");
    /// ```
    fn mod_add_mul_shl_assign(&mut self, y: &'a Self, z: &'b Self, bits: u64, m: &'c Self) {
        assert_reduced(self, y, z, m);
        if let Self(Small(m)) = *m {
            *self = mod_add_mul_shl_limb(self, y, z, bits, m);
        } else {
            self.mod_add_assign(y.mod_mul(z, m).mod_shl(bits, m), m);
        }
    }
}
