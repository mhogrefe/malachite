// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Sebastian Pancratz
//
//      Copyright © 2019 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural::arithmetic::gcd::extended_gcd::limbs_extended_gcd;
use crate::natural::arithmetic::sub::limbs_sub_same_length_in_place_right;
use malachite_base::num::arithmetic::traits::{DivMod, ModDiv, ModMul};
use malachite_base::num::basic::traits::Zero;

// Computes `(gcd(x, m), s)`, where `s < m` and `sx ≡ gcd(x, m) mod m`. `x` must be nonzero and
// reduced mod `m`, and `m` must have more than one significant limb.
//
// This is fmpz_gcdinv from fmpz/gcdinv.c, FLINT 3.6.0, where f is nonzero and h is large.
//
// The two buffers stay separate allocations: both end up as owned `Natural`s, and merging them
// would force one to be copied out of the parent.
#[cfg_attr(dylint_lib = "malachite_lints", allow(adjacent_vec_allocations))]
pub(crate) fn gcdinv_helper(x: Natural, m: Natural) -> (Natural, Natural) {
    let mut xs = x.into_limbs_asc();
    let mut ys = m.to_limbs_asc();
    let len = ys.len();
    xs.resize(len, 0);
    let mut gs = vec![0; len];
    let mut ss = vec![0; len + 1];
    let (g_len, ss_sign) = limbs_extended_gcd(&mut gs, &mut ss, &mut xs, &mut ys);
    gs.truncate(g_len);
    if !ss_sign {
        assert_eq!(ss.pop(), Some(0));
        limbs_sub_same_length_in_place_right(&m.into_limbs_asc(), &mut ss);
    }
    (
        Natural::from_owned_limbs_asc(gs),
        Natural::from_owned_limbs_asc(ss),
    )
}

// Computes a quotient of `b` and `c` modulo `m`: a `q` such that `qc ≡ b mod m`. `b` and `c` must
// be reduced mod `m`. The small-modulus case is handled by the caller.
//
// This is fmpz_mod_divides from fmpz_mod/divides.c, FLINT 3.6.0, where the quotient is returned as
// an Option.
fn mod_div_helper(b: Natural, c: Natural, m: Natural) -> Option<Natural> {
    if c == 0u32 {
        return if b == 0u32 { Some(Natural::ZERO) } else { None };
    }
    if b == 0u32 {
        return Some(Natural::ZERO);
    }
    // b and c are both nonzero now. Solve g = cx + my, where g = gcd(c, m).
    let (g, s) = gcdinv_helper(c, m.clone());
    let (q, r) = b.div_mod(g);
    if r == 0u32 {
        Some(q.mod_mul(s, m))
    } else {
        None
    }
}

impl ModDiv<Self, Self> for Natural {
    type Output = Self;

    /// Divides a [`Natural`] by another [`Natural`] modulo a third [`Natural`] $m$, returning
    /// `None` if no quotient exists. The inputs must be already reduced modulo $m$. All three
    /// [`Natural`]s are taken by value.
    ///
    /// A quotient exists if and only if $\gcd(y, m)$ divides $x$. If $y$ is not invertible modulo
    /// $m$, the quotient is not unique; all quotients differ by multiples of $m/\gcd(y, m)$, and
    /// this function returns one of them.
    ///
    /// $f(x, y, m) = \operatorname{Some}(q)$, where $x, y, q < m$ and $qy \equiv x \mod m$, if such
    /// a $q$ exists.
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
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_div(Natural::from(4u32), Natural::from(10u32)),
    ///     Some(Natural::from(4u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(2u32).mod_div(Natural::from(5u32), Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_div(self, other: Self, m: Self) -> Option<Self> {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        match (self, other, m) {
            (Self(Small(b)), Self(Small(c)), Self(Small(m))) => b.mod_div(c, m).map(Self::from),
            (b, c, m) => mod_div_helper(b, c, m),
        }
    }
}

impl<'a> ModDiv<Self, &'a Self> for Natural {
    type Output = Self;

    /// Divides a [`Natural`] by another [`Natural`] modulo a third [`Natural`] $m$, returning
    /// `None` if no quotient exists. The inputs must be already reduced modulo $m$. The first two
    /// [`Natural`]s are taken by value and the third by reference.
    ///
    /// A quotient exists if and only if $\gcd(y, m)$ divides $x$. If $y$ is not invertible modulo
    /// $m$, the quotient is not unique; all quotients differ by multiples of $m/\gcd(y, m)$, and
    /// this function returns one of them.
    ///
    /// $f(x, y, m) = \operatorname{Some}(q)$, where $x, y, q < m$ and $qy \equiv x \mod m$, if such
    /// a $q$ exists.
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
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_div(Natural::from(4u32), &Natural::from(10u32)),
    ///     Some(Natural::from(4u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(2u32).mod_div(Natural::from(5u32), &Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_div(self, other: Self, m: &'a Self) -> Option<Self> {
        assert!(self < *m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            other < *m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m) {
            (Self(Small(b)), Self(Small(c)), Self(Small(m))) => b.mod_div(c, *m).map(Self::from),
            (b, c, m) => mod_div_helper(b, c, m.clone()),
        }
    }
}

impl<'a> ModDiv<&'a Self, Self> for Natural {
    type Output = Self;

    /// Divides a [`Natural`] by another [`Natural`] modulo a third [`Natural`] $m$, returning
    /// `None` if no quotient exists. The inputs must be already reduced modulo $m$. The first and
    /// third [`Natural`]s are taken by value and the second by reference.
    ///
    /// A quotient exists if and only if $\gcd(y, m)$ divides $x$. If $y$ is not invertible modulo
    /// $m$, the quotient is not unique; all quotients differ by multiples of $m/\gcd(y, m)$, and
    /// this function returns one of them.
    ///
    /// $f(x, y, m) = \operatorname{Some}(q)$, where $x, y, q < m$ and $qy \equiv x \mod m$, if such
    /// a $q$ exists.
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
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_div(&Natural::from(4u32), Natural::from(10u32)),
    ///     Some(Natural::from(4u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(2u32).mod_div(&Natural::from(5u32), Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_div(self, other: &'a Self, m: Self) -> Option<Self> {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m) {
            (Self(Small(b)), Self(Small(c)), Self(Small(m))) => b.mod_div(*c, m).map(Self::from),
            (b, c, m) => mod_div_helper(b, c.clone(), m),
        }
    }
}

impl<'a, 'b> ModDiv<&'a Self, &'b Self> for Natural {
    type Output = Self;

    /// Divides a [`Natural`] by another [`Natural`] modulo a third [`Natural`] $m$, returning
    /// `None` if no quotient exists. The inputs must be already reduced modulo $m$. The first
    /// [`Natural`] is taken by value and the second and third by reference.
    ///
    /// A quotient exists if and only if $\gcd(y, m)$ divides $x$. If $y$ is not invertible modulo
    /// $m$, the quotient is not unique; all quotients differ by multiples of $m/\gcd(y, m)$, and
    /// this function returns one of them.
    ///
    /// $f(x, y, m) = \operatorname{Some}(q)$, where $x, y, q < m$ and $qy \equiv x \mod m$, if such
    /// a $q$ exists.
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
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(6u32).mod_div(&Natural::from(4u32), &Natural::from(10u32)),
    ///     Some(Natural::from(4u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(2u32).mod_div(&Natural::from(5u32), &Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_div(self, other: &'a Self, m: &'b Self) -> Option<Self> {
        assert!(self < *m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < *m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m) {
            (Self(Small(b)), Self(Small(c)), Self(Small(m))) => b.mod_div(*c, *m).map(Self::from),
            (b, c, m) => mod_div_helper(b, c.clone(), m.clone()),
        }
    }
}

impl ModDiv<Natural, Natural> for &Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`] modulo a third [`Natural`] $m$, returning
    /// `None` if no quotient exists. The inputs must be already reduced modulo $m$. The first
    /// [`Natural`] is taken by reference and the second and third by value.
    ///
    /// A quotient exists if and only if $\gcd(y, m)$ divides $x$. If $y$ is not invertible modulo
    /// $m$, the quotient is not unique; all quotients differ by multiples of $m/\gcd(y, m)$, and
    /// this function returns one of them.
    ///
    /// $f(x, y, m) = \operatorname{Some}(q)$, where $x, y, q < m$ and $qy \equiv x \mod m$, if such
    /// a $q$ exists.
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
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_div(Natural::from(4u32), Natural::from(10u32)),
    ///     Some(Natural::from(4u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(2u32)).mod_div(Natural::from(5u32), Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_div(self, other: Natural, m: Natural) -> Option<Natural> {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        match (self, other, m) {
            (Natural(Small(b)), Natural(Small(c)), Natural(Small(m))) => {
                b.mod_div(c, m).map(Natural::from)
            }
            (b, c, m) => mod_div_helper(b.clone(), c, m),
        }
    }
}

impl ModDiv<Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`] modulo a third [`Natural`] $m$, returning
    /// `None` if no quotient exists. The inputs must be already reduced modulo $m$. The first and
    /// third [`Natural`]s are taken by reference and the second by value.
    ///
    /// A quotient exists if and only if $\gcd(y, m)$ divides $x$. If $y$ is not invertible modulo
    /// $m$, the quotient is not unique; all quotients differ by multiples of $m/\gcd(y, m)$, and
    /// this function returns one of them.
    ///
    /// $f(x, y, m) = \operatorname{Some}(q)$, where $x, y, q < m$ and $qy \equiv x \mod m$, if such
    /// a $q$ exists.
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
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_div(Natural::from(4u32), &Natural::from(10u32)),
    ///     Some(Natural::from(4u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(2u32)).mod_div(Natural::from(5u32), &Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_div(self, other: Natural, m: &Natural) -> Option<Natural> {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            other < *m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m) {
            (Natural(Small(b)), Natural(Small(c)), Natural(Small(m))) => {
                b.mod_div(c, *m).map(Natural::from)
            }
            (b, c, m) => mod_div_helper(b.clone(), c, m.clone()),
        }
    }
}

impl ModDiv<&Natural, Natural> for &Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`] modulo a third [`Natural`] $m$, returning
    /// `None` if no quotient exists. The inputs must be already reduced modulo $m$. The first two
    /// [`Natural`]s are taken by reference and the third by value.
    ///
    /// A quotient exists if and only if $\gcd(y, m)$ divides $x$. If $y$ is not invertible modulo
    /// $m$, the quotient is not unique; all quotients differ by multiples of $m/\gcd(y, m)$, and
    /// this function returns one of them.
    ///
    /// $f(x, y, m) = \operatorname{Some}(q)$, where $x, y, q < m$ and $qy \equiv x \mod m$, if such
    /// a $q$ exists.
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
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_div(&Natural::from(4u32), Natural::from(10u32)),
    ///     Some(Natural::from(4u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(2u32)).mod_div(&Natural::from(5u32), Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_div(self, other: &Natural, m: Natural) -> Option<Natural> {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        match (self, other, m) {
            (Natural(Small(b)), Natural(Small(c)), Natural(Small(m))) => {
                b.mod_div(*c, m).map(Natural::from)
            }
            (b, c, m) => mod_div_helper(b.clone(), c.clone(), m),
        }
    }
}

impl ModDiv<&Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`] modulo a third [`Natural`] $m$, returning
    /// `None` if no quotient exists. The inputs must be already reduced modulo $m$. All three
    /// [`Natural`]s are taken by reference.
    ///
    /// A quotient exists if and only if $\gcd(y, m)$ divides $x$. If $y$ is not invertible modulo
    /// $m$, the quotient is not unique; all quotients differ by multiples of $m/\gcd(y, m)$, and
    /// this function returns one of them.
    ///
    /// $f(x, y, m) = \operatorname{Some}(q)$, where $x, y, q < m$ and $qy \equiv x \mod m$, if such
    /// a $q$ exists.
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
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(6u32)).mod_div(&Natural::from(4u32), &Natural::from(10u32)),
    ///     Some(Natural::from(4u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(2u32)).mod_div(&Natural::from(5u32), &Natural::from(10u32)),
    ///     None
    /// );
    /// ```
    fn mod_div(self, other: &Natural, m: &Natural) -> Option<Natural> {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        match (self, other, m) {
            (Natural(Small(b)), Natural(Small(c)), Natural(Small(m))) => {
                b.mod_div(*c, *m).map(Natural::from)
            }
            (b, c, m) => mod_div_helper(b.clone(), c.clone(), m.clone()),
        }
    }
}
