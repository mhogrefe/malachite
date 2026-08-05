// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural::arithmetic::mod_div::gcdinv_helper;
use malachite_base::num::arithmetic::traits::ModInverse;
use malachite_base::num::basic::traits::One;

// The modular inverse is unique, so filtering the cofactor from `gcdinv_helper` on the GCD being 1
// produces the same value as any other algorithm.
fn mod_inverse_helper(x: Natural, m: Natural) -> Option<Natural> {
    let (g, s) = gcdinv_helper(x, m);
    if g == 1u32 { Some(s) } else { None }
}

impl ModInverse for Natural {
    type Output = Self;

    /// Computes the multiplicative inverse of a [`Natural`] modulo another [`Natural`] $m$. The
    /// input must be already reduced modulo $m$. Both [`Natural`]s are taken by value.
    ///
    /// Returns `None` if $x$ and $m$ are not coprime.
    ///
    /// $f(x, m) = y$, where $x, y < m$, $\gcd(x, y) = 1$, and $xy \equiv 1 \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// m.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is 0 or if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_inverse(Natural::from(10u32)),
    ///     Some(Natural::from(7u32))
    /// );
    /// assert_eq!(Natural::from(4u32).mod_inverse(Natural::from(10u32)), None);
    /// ```
    fn mod_inverse(self, m: Self) -> Option<Self> {
        assert_ne!(self, 0u32);
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        match (self, m) {
            (x @ Self::ONE, _) => Some(x),
            (Self(Small(x)), Self(Small(y))) => x.mod_inverse(y).map(Self::from),
            (a, b) => mod_inverse_helper(a, b),
        }
    }
}

impl<'a> ModInverse<&'a Self> for Natural {
    type Output = Self;

    /// Computes the multiplicative inverse of a [`Natural`] modulo another [`Natural`] $m$. The
    /// input must be already reduced modulo $m$. The first [`Natural`] is taken by value and the
    /// second by reference.
    ///
    /// Returns `None` if $x$ and $m$ are not coprime.
    ///
    /// $f(x, m) = y$, where $x, y < m$, $\gcd(x, y) = 1$, and $xy \equiv 1 \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// m.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is 0 or if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_inverse(&Natural::from(10u32)),
    ///     Some(Natural::from(7u32))
    /// );
    /// assert_eq!(Natural::from(4u32).mod_inverse(&Natural::from(10u32)), None);
    /// ```
    fn mod_inverse(self, m: &'a Self) -> Option<Self> {
        assert_ne!(self, 0u32);
        assert!(self < *m, "self must be reduced mod m, but {self} >= {m}");
        match (self, m) {
            (x @ Self::ONE, _) => Some(x),
            (Self(Small(x)), Self(Small(y))) => x.mod_inverse(*y).map(Self::from),
            (a, b) => mod_inverse_helper(a, b.clone()),
        }
    }
}

impl ModInverse<Natural> for &Natural {
    type Output = Natural;

    /// Computes the multiplicative inverse of a [`Natural`] modulo another [`Natural`] $m$. The
    /// input must be already reduced modulo $m$. The first [`Natural`]s is taken by reference and
    /// the second by value.
    ///
    /// Returns `None` if $x$ and $m$ are not coprime.
    ///
    /// $f(x, m) = y$, where $x, y < m$, $\gcd(x, y) = 1$, and $xy \equiv 1 \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// m.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is 0 or if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_inverse(Natural::from(10u32)),
    ///     Some(Natural::from(7u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_inverse(Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_inverse(self, m: Natural) -> Option<Natural> {
        assert_ne!(*self, 0u32);
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        match (self, m) {
            (&Natural::ONE, _) => Some(Natural::ONE),
            (Natural(Small(x)), Natural(Small(y))) => x.mod_inverse(y).map(Natural::from),
            (a, b) => mod_inverse_helper(a.clone(), b),
        }
    }
}

impl ModInverse<&Natural> for &Natural {
    type Output = Natural;

    /// Computes the multiplicative inverse of a [`Natural`] modulo another [`Natural`] $m$. The
    /// input must be already reduced modulo $m$. Both [`Natural`]s are taken by reference.
    ///
    /// Returns `None` if $x$ and $m$ are not coprime.
    ///
    /// $f(x, m) = y$, where $x, y < m$, $\gcd(x, y) = 1$, and $xy \equiv 1 \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// m.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is 0 or if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_inverse(&Natural::from(10u32)),
    ///     Some(Natural::from(7u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_inverse(&Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_inverse(self, m: &Natural) -> Option<Natural> {
        assert_ne!(*self, 0u32);
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        match (self, m) {
            (&Natural::ONE, _) => Some(Natural::ONE),
            (Natural(Small(x)), Natural(Small(y))) => x.mod_inverse(*y).map(Natural::from),
            (a, b) => mod_inverse_helper(a.clone(), b.clone()),
        }
    }
}
