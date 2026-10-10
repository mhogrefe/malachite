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
    ModAdd, ModAddAssign, ModAddMul, ModAddMulAssign, ModMul, ModMulAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;

fn assert_reduced(x: &Natural, y: &Natural, z: &Natural, m: &Natural) {
    assert!(x < m, "self must be reduced mod m, but {x} >= {m}");
    assert!(y < m, "y must be reduced mod m, but {y} >= {m}");
    assert!(z < m, "z must be reduced mod m, but {z} >= {m}");
}

// When `m` fits in a `Limb`, so does every reduced argument.
fn mod_add_mul_limb(x: &Natural, y: &Natural, z: &Natural, m: Limb) -> Natural {
    Natural::from(Limb::exact_from(x).mod_add_mul(Limb::exact_from(y), Limb::exact_from(z), m))
}

impl ModAddMul<Self, Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. All four [`Natural`]s are taken by
    /// value.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul(Natural::TWO, Natural::from(5u32), Natural::from(7u32)),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul(
    ///             Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "99999999999999994300"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul(mut self, y: Self, z: Self, m: Self) -> Self {
        self.mod_add_mul_assign(y, z, m);
        self
    }
}

impl<'c> ModAddMul<Self, Self, &'c Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by value, the third by value, and the fourth by reference.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul(
    ///         Natural::TWO,
    ///         Natural::from(5u32),
    ///         &Natural::from(7u32)
    ///     ),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul(
    ///             Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "99999999999999994300"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul(mut self, y: Self, z: Self, m: &'c Self) -> Self {
        self.mod_add_mul_assign(y, z, m);
        self
    }
}

impl<'b> ModAddMul<Self, &'b Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by value, the third by reference, and the fourth by value.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul(
    ///         Natural::TWO,
    ///         &Natural::from(5u32),
    ///         Natural::from(7u32)
    ///     ),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul(
    ///             Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "99999999999999994300"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul(mut self, y: Self, z: &'b Self, m: Self) -> Self {
        self.mod_add_mul_assign(y, z, m);
        self
    }
}

impl<'b, 'c> ModAddMul<Self, &'b Self, &'c Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by value, the third by reference, and the fourth by reference.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul(
    ///         Natural::TWO,
    ///         &Natural::from(5u32),
    ///         &Natural::from(7u32)
    ///     ),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul(
    ///             Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "99999999999999994300"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul(mut self, y: Self, z: &'b Self, m: &'c Self) -> Self {
        self.mod_add_mul_assign(y, z, m);
        self
    }
}

impl<'a> ModAddMul<&'a Self, Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by reference, the third by value, and the fourth by value.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul(
    ///         &Natural::TWO,
    ///         Natural::from(5u32),
    ///         Natural::from(7u32)
    ///     ),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "99999999999999994300"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul(mut self, y: &'a Self, z: Self, m: Self) -> Self {
        self.mod_add_mul_assign(y, z, m);
        self
    }
}

impl<'a, 'c> ModAddMul<&'a Self, Self, &'c Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by reference, the third by value, and the fourth by reference.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul(
    ///         &Natural::TWO,
    ///         Natural::from(5u32),
    ///         &Natural::from(7u32)
    ///     ),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "99999999999999994300"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul(mut self, y: &'a Self, z: Self, m: &'c Self) -> Self {
        self.mod_add_mul_assign(y, z, m);
        self
    }
}

impl<'a, 'b> ModAddMul<&'a Self, &'b Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by reference, the third by reference, and the fourth by value.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         Natural::from(7u32)
    ///     ),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             Natural::from(10u32).pow(30) + Natural::from(57u32)
    ///         )
    ///         .to_string(),
    ///     "99999999999999994300"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul(mut self, y: &'a Self, z: &'b Self, m: Self) -> Self {
        self.mod_add_mul_assign(y, z, m);
        self
    }
}

impl<'a, 'b, 'c> ModAddMul<&'a Self, &'b Self, &'c Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. The first [`Natural`] is taken by
    /// value, the second by reference, the third by reference, and the fourth by reference.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_add_mul(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         &Natural::from(7u32)
    ///     ),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_add_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "99999999999999994300"
    /// );
    /// ```
    #[inline]
    fn mod_add_mul(mut self, y: &'a Self, z: &'b Self, m: &'c Self) -> Self {
        self.mod_add_mul_assign(y, z, m);
        self
    }
}

impl ModAddMul<&Natural, &Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, modulo a fourth [`Natural`]
    /// $m$. All three inputs must be already reduced modulo $m$. All four [`Natural`]s are taken by
    /// reference.
    ///
    /// $f(x, y, z, m) = w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_add_mul(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         &Natural::from(7u32)
    ///     ),
    ///     6
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(20))
    ///         .mod_add_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             &(Natural::from(10u32).pow(30) + Natural::from(57u32))
    ///         )
    ///         .to_string(),
    ///     "99999999999999994300"
    /// );
    /// ```
    fn mod_add_mul(self, y: &Natural, z: &Natural, m: &Natural) -> Natural {
        assert_reduced(self, y, z, m);
        if let Natural(Small(m)) = *m {
            mod_add_mul_limb(self, y, z, m)
        } else {
            self.mod_add(y.mod_mul(z, m), m)
        }
    }
}

impl ModAddMulAssign<Self, Self, Self> for Natural {
    /// Adds the product of two [`Natural`]s to a [`Natural`] modulo a fourth [`Natural`] $m$, in
    /// place. All three inputs must be already reduced modulo $m$. All three [`Natural`]s on the
    /// right-hand side are taken by value.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_assign(Natural::TWO, Natural::from(5u32), Natural::from(7u32));
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_assign(
    ///     Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999994300");
    /// ```
    fn mod_add_mul_assign(&mut self, mut y: Self, z: Self, m: Self) {
        assert_reduced(self, &y, &z, &m);
        if let Self(Small(m)) = m {
            *self = mod_add_mul_limb(self, &y, &z, m);
        } else {
            y.mod_mul_assign(z, &m);
            self.mod_add_assign(y, &m);
        }
    }
}

impl<'c> ModAddMulAssign<Self, Self, &'c Self> for Natural {
    /// Adds the product of two [`Natural`]s to a [`Natural`] modulo a fourth [`Natural`] $m$, in
    /// place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by value, by value, and by reference, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_assign(Natural::TWO, Natural::from(5u32), &Natural::from(7u32));
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_assign(
    ///     Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999994300");
    /// ```
    fn mod_add_mul_assign(&mut self, mut y: Self, z: Self, m: &'c Self) {
        assert_reduced(self, &y, &z, m);
        if let Self(Small(m)) = *m {
            *self = mod_add_mul_limb(self, &y, &z, m);
        } else {
            y.mod_mul_assign(z, m);
            self.mod_add_assign(y, m);
        }
    }
}

impl<'b> ModAddMulAssign<Self, &'b Self, Self> for Natural {
    /// Adds the product of two [`Natural`]s to a [`Natural`] modulo a fourth [`Natural`] $m$, in
    /// place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by value, by reference, and by value, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_assign(Natural::TWO, &Natural::from(5u32), Natural::from(7u32));
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_assign(
    ///     Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999994300");
    /// ```
    fn mod_add_mul_assign(&mut self, mut y: Self, z: &'b Self, m: Self) {
        assert_reduced(self, &y, z, &m);
        if let Self(Small(m)) = m {
            *self = mod_add_mul_limb(self, &y, z, m);
        } else {
            y.mod_mul_assign(z, &m);
            self.mod_add_assign(y, &m);
        }
    }
}

impl<'b, 'c> ModAddMulAssign<Self, &'b Self, &'c Self> for Natural {
    /// Adds the product of two [`Natural`]s to a [`Natural`] modulo a fourth [`Natural`] $m$, in
    /// place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by value, by reference, and by reference, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_assign(Natural::TWO, &Natural::from(5u32), &Natural::from(7u32));
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_assign(
    ///     Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999994300");
    /// ```
    fn mod_add_mul_assign(&mut self, mut y: Self, z: &'b Self, m: &'c Self) {
        assert_reduced(self, &y, z, m);
        if let Self(Small(m)) = *m {
            *self = mod_add_mul_limb(self, &y, z, m);
        } else {
            y.mod_mul_assign(z, m);
            self.mod_add_assign(y, m);
        }
    }
}

impl<'a> ModAddMulAssign<&'a Self, Self, Self> for Natural {
    /// Adds the product of two [`Natural`]s to a [`Natural`] modulo a fourth [`Natural`] $m$, in
    /// place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by reference, by value, and by value, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_assign(&Natural::TWO, Natural::from(5u32), Natural::from(7u32));
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999994300");
    /// ```
    fn mod_add_mul_assign(&mut self, y: &'a Self, mut z: Self, m: Self) {
        assert_reduced(self, y, &z, &m);
        if let Self(Small(m)) = m {
            *self = mod_add_mul_limb(self, y, &z, m);
        } else {
            z.mod_mul_assign(y, &m);
            self.mod_add_assign(z, &m);
        }
    }
}

impl<'a, 'c> ModAddMulAssign<&'a Self, Self, &'c Self> for Natural {
    /// Adds the product of two [`Natural`]s to a [`Natural`] modulo a fourth [`Natural`] $m$, in
    /// place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by reference, by value, and by reference, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_assign(&Natural::TWO, Natural::from(5u32), &Natural::from(7u32));
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999994300");
    /// ```
    fn mod_add_mul_assign(&mut self, y: &'a Self, mut z: Self, m: &'c Self) {
        assert_reduced(self, y, &z, m);
        if let Self(Small(m)) = *m {
            *self = mod_add_mul_limb(self, y, &z, m);
        } else {
            z.mod_mul_assign(y, m);
            self.mod_add_assign(z, m);
        }
    }
}

impl<'a, 'b> ModAddMulAssign<&'a Self, &'b Self, Self> for Natural {
    /// Adds the product of two [`Natural`]s to a [`Natural`] modulo a fourth [`Natural`] $m$, in
    /// place. All three inputs must be already reduced modulo $m$. The [`Natural`]s on the
    /// right-hand side are taken by reference, by reference, and by value, respectively.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_assign(&Natural::TWO, &Natural::from(5u32), Natural::from(7u32));
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     Natural::from(10u32).pow(30) + Natural::from(57u32),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999994300");
    /// ```
    fn mod_add_mul_assign(&mut self, y: &'a Self, z: &'b Self, m: Self) {
        assert_reduced(self, y, z, &m);
        if let Self(Small(m)) = m {
            *self = mod_add_mul_limb(self, y, z, m);
        } else {
            self.mod_add_assign(y.mod_mul(z, &m), &m);
        }
    }
}

impl<'a, 'b, 'c> ModAddMulAssign<&'a Self, &'b Self, &'c Self> for Natural {
    /// Adds the product of two [`Natural`]s to a [`Natural`] modulo a fourth [`Natural`] $m$, in
    /// place. All three inputs must be already reduced modulo $m$. All three [`Natural`]s on the
    /// right-hand side are taken by reference.
    ///
    /// $x \gets w$, where $x, y, z, w < m$ and $x + yz \equiv w \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::{ModAddMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_add_mul_assign(&Natural::TWO, &Natural::from(5u32), &Natural::from(7u32));
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_add_mul_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     &(Natural::from(10u32).pow(30) + Natural::from(57u32)),
    /// );
    /// assert_eq!(x.to_string(), "99999999999999994300");
    /// ```
    fn mod_add_mul_assign(&mut self, y: &'a Self, z: &'b Self, m: &'c Self) {
        assert_reduced(self, y, z, m);
        if let Self(Small(m)) = *m {
            *self = mod_add_mul_limb(self, y, z, m);
        } else {
            self.mod_add_assign(y.mod_mul(z, m), m);
        }
    }
}
