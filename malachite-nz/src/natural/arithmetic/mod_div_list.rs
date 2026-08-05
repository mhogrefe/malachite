// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2020 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural::arithmetic::mod_div::gcdinv_helper;
use malachite_base::num::arithmetic::traits::{DivExact, DivMod, ModDivList};

// Computes the solutions `q` of `qc ≡ b mod m` as `(start, stride, length)`: the solutions are
// exactly `start + stride * i` for `0 <= i < length`, and `start` is the smallest. `b` and `c`
// must be reduced mod `m`. The small-modulus case is handled by the caller. Unlike a quotient
// from `ModDiv`, the result is canonical: it does not depend on the extended GCD's choice of
// cofactor.
//
// This is fmpz_divides_mod_list from fmpz/divides_mod_list.c, FLINT 3.6.0, where the inputs are
// reduced mod the modulus and the solutions are returned as an Option.
fn mod_div_list_helper(b: Natural, c: Natural, m: Natural) -> Option<(Natural, Natural, Natural)> {
    // Solve d = cx + my, where d = gcd(c, m). (FLINT reduces the divisor mod m here; the
    // precondition makes that a no-op.)
    let (d, x) = gcdinv_helper(c, m.clone());
    let (q, r) = b.div_mod(&d);
    if r != 0u32 {
        return None;
    }
    let stride = m.div_exact(&d);
    let start = x * q % &stride;
    Some((start, stride, d))
}

impl ModDivList<Self, Self> for Natural {
    type Output = Self;

    /// Finds all quotients of a [`Natural`] and another [`Natural`] modulo a third [`Natural`]
    /// $m$, returning `None` if no quotient exists. The inputs must be already reduced modulo
    /// $m$. All three [`Natural`]s are taken by value.
    ///
    /// A quotient exists if and only if $g = \gcd(y, m)$ divides $x$. In that case the quotients
    /// are exactly the numbers $\text{start} + \text{stride} \cdot i$ for
    /// $0 \leq i < \text{length}$, where $\text{start}$ is the smallest quotient, $\text{stride}
    /// = m/g$, and $\text{length} = g$. Unlike the quotient returned by
    /// [`ModDiv`](malachite_base::num::arithmetic::traits::ModDiv), the result is canonical.
    ///
    /// $f(x, y, m) = \operatorname{Some}((s, t, \ell))$, where $qy \equiv x \mod m$ if and only
    /// if $q = s + ti$ for some $0 \leq i < \ell$, if such $q$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModDivList;
    /// use malachite_nz::natural::Natural;
    ///
    /// // The quotients of 6 and 4 mod 10 are 4 and 9: 4 + 5 * i for 0 <= i < 2.
    /// assert_eq!(
    ///     Natural::from(6u32).mod_div_list(Natural::from(4u32), Natural::from(10u32)),
    ///     Some((
    ///         Natural::from(4u32),
    ///         Natural::from(5u32),
    ///         Natural::from(2u32)
    ///     ))
    /// );
    /// assert_eq!(
    ///     Natural::from(2u32).mod_div_list(Natural::from(5u32), Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_div_list(self, other: Self, m: Self) -> Option<(Self, Self, Self)> {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        match (self, other, m) {
            (Self(Small(b)), Self(Small(c)), Self(Small(m))) => b
                .mod_div_list(c, m)
                .map(|(s, t, l)| (Self::from(s), Self::from(t), Self::from(l))),
            (b, c, m) => mod_div_list_helper(b, c, m),
        }
    }
}

impl<'a> ModDivList<Self, &'a Self> for Natural {
    type Output = Self;

    /// Finds all quotients of a [`Natural`] and another [`Natural`] modulo a third [`Natural`]
    /// $m$, returning `None` if no quotient exists. The inputs must be already reduced modulo
    /// $m$. The first two [`Natural`]s are taken by value and the third by reference.
    ///
    /// A quotient exists if and only if $g = \gcd(y, m)$ divides $x$. In that case the quotients
    /// are exactly the numbers $\text{start} + \text{stride} \cdot i$ for
    /// $0 \leq i < \text{length}$, where $\text{start}$ is the smallest quotient, $\text{stride}
    /// = m/g$, and $\text{length} = g$. Unlike the quotient returned by
    /// [`ModDiv`](malachite_base::num::arithmetic::traits::ModDiv), the result is canonical.
    ///
    /// $f(x, y, m) = \operatorname{Some}((s, t, \ell))$, where $qy \equiv x \mod m$ if and only
    /// if $q = s + ti$ for some $0 \leq i < \ell$, if such $q$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModDivList;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_div_list(Natural::from(4u32), &Natural::from(10u32)),
    ///     Some((
    ///         Natural::from(4u32),
    ///         Natural::from(5u32),
    ///         Natural::from(2u32)
    ///     ))
    /// );
    /// ```
    fn mod_div_list(self, other: Self, m: &'a Self) -> Option<(Self, Self, Self)> {
        assert!(self < *m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            other < *m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m) {
            (Self(Small(b)), Self(Small(c)), Self(Small(m))) => b
                .mod_div_list(c, *m)
                .map(|(s, t, l)| (Self::from(s), Self::from(t), Self::from(l))),
            (b, c, m) => mod_div_list_helper(b, c, m.clone()),
        }
    }
}

impl<'a> ModDivList<&'a Self, Self> for Natural {
    type Output = Self;

    /// Finds all quotients of a [`Natural`] and another [`Natural`] modulo a third [`Natural`]
    /// $m$, returning `None` if no quotient exists. The inputs must be already reduced modulo
    /// $m$. The first and third [`Natural`]s are taken by value and the second by reference.
    ///
    /// A quotient exists if and only if $g = \gcd(y, m)$ divides $x$. In that case the quotients
    /// are exactly the numbers $\text{start} + \text{stride} \cdot i$ for
    /// $0 \leq i < \text{length}$, where $\text{start}$ is the smallest quotient, $\text{stride}
    /// = m/g$, and $\text{length} = g$. Unlike the quotient returned by
    /// [`ModDiv`](malachite_base::num::arithmetic::traits::ModDiv), the result is canonical.
    ///
    /// $f(x, y, m) = \operatorname{Some}((s, t, \ell))$, where $qy \equiv x \mod m$ if and only
    /// if $q = s + ti$ for some $0 \leq i < \ell$, if such $q$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModDivList;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_div_list(&Natural::from(4u32), Natural::from(10u32)),
    ///     Some((
    ///         Natural::from(4u32),
    ///         Natural::from(5u32),
    ///         Natural::from(2u32)
    ///     ))
    /// );
    /// ```
    fn mod_div_list(self, other: &'a Self, m: Self) -> Option<(Self, Self, Self)> {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m) {
            (Self(Small(b)), Self(Small(c)), Self(Small(m))) => b
                .mod_div_list(*c, m)
                .map(|(s, t, l)| (Self::from(s), Self::from(t), Self::from(l))),
            (b, c, m) => mod_div_list_helper(b, c.clone(), m),
        }
    }
}

impl<'a, 'b> ModDivList<&'a Self, &'b Self> for Natural {
    type Output = Self;

    /// Finds all quotients of a [`Natural`] and another [`Natural`] modulo a third [`Natural`]
    /// $m$, returning `None` if no quotient exists. The inputs must be already reduced modulo
    /// $m$. The first [`Natural`] is taken by value and the second and third by reference.
    ///
    /// A quotient exists if and only if $g = \gcd(y, m)$ divides $x$. In that case the quotients
    /// are exactly the numbers $\text{start} + \text{stride} \cdot i$ for
    /// $0 \leq i < \text{length}$, where $\text{start}$ is the smallest quotient, $\text{stride}
    /// = m/g$, and $\text{length} = g$. Unlike the quotient returned by
    /// [`ModDiv`](malachite_base::num::arithmetic::traits::ModDiv), the result is canonical.
    ///
    /// $f(x, y, m) = \operatorname{Some}((s, t, \ell))$, where $qy \equiv x \mod m$ if and only
    /// if $q = s + ti$ for some $0 \leq i < \ell$, if such $q$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModDivList;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_div_list(&Natural::from(4u32), &Natural::from(10u32)),
    ///     Some((
    ///         Natural::from(4u32),
    ///         Natural::from(5u32),
    ///         Natural::from(2u32)
    ///     ))
    /// );
    /// ```
    fn mod_div_list(self, other: &'a Self, m: &'b Self) -> Option<(Self, Self, Self)> {
        assert!(self < *m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < *m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m) {
            (Self(Small(b)), Self(Small(c)), Self(Small(m))) => b
                .mod_div_list(*c, *m)
                .map(|(s, t, l)| (Self::from(s), Self::from(t), Self::from(l))),
            (b, c, m) => mod_div_list_helper(b, c.clone(), m.clone()),
        }
    }
}

impl ModDivList<Natural, Natural> for &Natural {
    type Output = Natural;

    /// Finds all quotients of a [`Natural`] and another [`Natural`] modulo a third [`Natural`]
    /// $m$, returning `None` if no quotient exists. The inputs must be already reduced modulo
    /// $m$. The first [`Natural`] is taken by reference and the second and third by value.
    ///
    /// A quotient exists if and only if $g = \gcd(y, m)$ divides $x$. In that case the quotients
    /// are exactly the numbers $\text{start} + \text{stride} \cdot i$ for
    /// $0 \leq i < \text{length}$, where $\text{start}$ is the smallest quotient, $\text{stride}
    /// = m/g$, and $\text{length} = g$. Unlike the quotient returned by
    /// [`ModDiv`](malachite_base::num::arithmetic::traits::ModDiv), the result is canonical.
    ///
    /// $f(x, y, m) = \operatorname{Some}((s, t, \ell))$, where $qy \equiv x \mod m$ if and only
    /// if $q = s + ti$ for some $0 \leq i < \ell$, if such $q$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModDivList;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_div_list(Natural::from(4u32), Natural::from(10u32)),
    ///     Some((
    ///         Natural::from(4u32),
    ///         Natural::from(5u32),
    ///         Natural::from(2u32)
    ///     ))
    /// );
    /// ```
    fn mod_div_list(self, other: Natural, m: Natural) -> Option<(Natural, Natural, Natural)> {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        match (self, other, m) {
            (Natural(Small(b)), Natural(Small(c)), Natural(Small(m))) => b
                .mod_div_list(c, m)
                .map(|(s, t, l)| (Natural::from(s), Natural::from(t), Natural::from(l))),
            (b, c, m) => mod_div_list_helper(b.clone(), c, m),
        }
    }
}

impl ModDivList<Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Finds all quotients of a [`Natural`] and another [`Natural`] modulo a third [`Natural`]
    /// $m$, returning `None` if no quotient exists. The inputs must be already reduced modulo
    /// $m$. The first and third [`Natural`]s are taken by reference and the second by value.
    ///
    /// A quotient exists if and only if $g = \gcd(y, m)$ divides $x$. In that case the quotients
    /// are exactly the numbers $\text{start} + \text{stride} \cdot i$ for
    /// $0 \leq i < \text{length}$, where $\text{start}$ is the smallest quotient, $\text{stride}
    /// = m/g$, and $\text{length} = g$. Unlike the quotient returned by
    /// [`ModDiv`](malachite_base::num::arithmetic::traits::ModDiv), the result is canonical.
    ///
    /// $f(x, y, m) = \operatorname{Some}((s, t, \ell))$, where $qy \equiv x \mod m$ if and only
    /// if $q = s + ti$ for some $0 \leq i < \ell$, if such $q$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModDivList;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_div_list(Natural::from(4u32), &Natural::from(10u32)),
    ///     Some((
    ///         Natural::from(4u32),
    ///         Natural::from(5u32),
    ///         Natural::from(2u32)
    ///     ))
    /// );
    /// ```
    fn mod_div_list(self, other: Natural, m: &Natural) -> Option<(Natural, Natural, Natural)> {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            other < *m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m) {
            (Natural(Small(b)), Natural(Small(c)), Natural(Small(m))) => b
                .mod_div_list(c, *m)
                .map(|(s, t, l)| (Natural::from(s), Natural::from(t), Natural::from(l))),
            (b, c, m) => mod_div_list_helper(b.clone(), c, m.clone()),
        }
    }
}

impl ModDivList<&Natural, Natural> for &Natural {
    type Output = Natural;

    /// Finds all quotients of a [`Natural`] and another [`Natural`] modulo a third [`Natural`]
    /// $m$, returning `None` if no quotient exists. The inputs must be already reduced modulo
    /// $m$. The first two [`Natural`]s are taken by reference and the third by value.
    ///
    /// A quotient exists if and only if $g = \gcd(y, m)$ divides $x$. In that case the quotients
    /// are exactly the numbers $\text{start} + \text{stride} \cdot i$ for
    /// $0 \leq i < \text{length}$, where $\text{start}$ is the smallest quotient, $\text{stride}
    /// = m/g$, and $\text{length} = g$. Unlike the quotient returned by
    /// [`ModDiv`](malachite_base::num::arithmetic::traits::ModDiv), the result is canonical.
    ///
    /// $f(x, y, m) = \operatorname{Some}((s, t, \ell))$, where $qy \equiv x \mod m$ if and only
    /// if $q = s + ti$ for some $0 \leq i < \ell$, if such $q$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModDivList;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_div_list(&Natural::from(4u32), Natural::from(10u32)),
    ///     Some((
    ///         Natural::from(4u32),
    ///         Natural::from(5u32),
    ///         Natural::from(2u32)
    ///     ))
    /// );
    /// ```
    fn mod_div_list(self, other: &Natural, m: Natural) -> Option<(Natural, Natural, Natural)> {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m) {
            (Natural(Small(b)), Natural(Small(c)), Natural(Small(m))) => b
                .mod_div_list(*c, m)
                .map(|(s, t, l)| (Natural::from(s), Natural::from(t), Natural::from(l))),
            (b, c, m) => mod_div_list_helper(b.clone(), c.clone(), m),
        }
    }
}

impl ModDivList<&Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Finds all quotients of a [`Natural`] and another [`Natural`] modulo a third [`Natural`]
    /// $m$, returning `None` if no quotient exists. The inputs must be already reduced modulo
    /// $m$. All three [`Natural`]s are taken by reference.
    ///
    /// A quotient exists if and only if $g = \gcd(y, m)$ divides $x$. In that case the quotients
    /// are exactly the numbers $\text{start} + \text{stride} \cdot i$ for
    /// $0 \leq i < \text{length}$, where $\text{start}$ is the smallest quotient, $\text{stride}
    /// = m/g$, and $\text{length} = g$. Unlike the quotient returned by
    /// [`ModDiv`](malachite_base::num::arithmetic::traits::ModDiv), the result is canonical.
    ///
    /// $f(x, y, m) = \operatorname{Some}((s, t, \ell))$, where $qy \equiv x \mod m$ if and only
    /// if $q = s + ti$ for some $0 \leq i < \ell$, if such $q$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModDivList;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_div_list(&Natural::from(4u32), &Natural::from(10u32)),
    ///     Some((
    ///         Natural::from(4u32),
    ///         Natural::from(5u32),
    ///         Natural::from(2u32)
    ///     ))
    /// );
    /// ```
    fn mod_div_list(self, other: &Natural, m: &Natural) -> Option<(Natural, Natural, Natural)> {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        match (self, other, m) {
            (Natural(Small(b)), Natural(Small(c)), Natural(Small(m))) => b
                .mod_div_list(*c, *m)
                .map(|(s, t, l)| (Natural::from(s), Natural::from(t), Natural::from(l))),
            (b, c, m) => mod_div_list_helper(b.clone(), c.clone(), m.clone()),
        }
    }
}
