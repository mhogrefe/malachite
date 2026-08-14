// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2014 William Hart
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{AddMul, Crt, ModInverse, ModMul, ModSub};

// Computes the unique `x` with `x ≡ r1 mod m1`, `x ≡ r2 mod m2`, and `0 <= x < m1 * m2`, or
// `None` if the moduli are not coprime. The residues must be reduced mod their moduli. The all-word
// case is handled by the caller.
//
// This is fmpz_CRT from fmpz/CRT.c, FLINT 3.6.0, where the inputs are nonnegative and reduced and a
// noninvertible modulus is reported as an Option rather than thrown.
fn crt_helper(r1: Natural, m1: Natural, r2: Natural, m2: Natural) -> Option<Natural> {
    let c = &m1 % &m2;
    if c == 0u32 {
        // m2 divides m1, so the moduli are coprime only if m2 is 1, and then the second congruence
        // is vacuous.
        return if m2 == 1u32 { Some(r1) } else { None };
    }
    let inv = (&c).mod_inverse(&m2)?;
    let s = r2.mod_sub(&r1 % &m2, &m2).mod_mul(inv, m2);
    Some(r1.add_mul(m1, s))
}

impl Crt<Self, Self, Self> for Natural {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. All four
    /// [`Natural`]s are taken by value.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     Natural::from(2u32).crt(
    ///         Natural::from(3u32),
    ///         Natural::from(3u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).crt(
    ///         Natural::from(4u32),
    ///         Natural::from(3u32),
    ///         Natural::from(6u32)
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: Self, r2: Self, m2: Self) -> Option<Self> {
        assert!(self < m1, "self must be reduced mod m1, but {self} >= {m1}");
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Self(Small(r1)), Self(Small(m1)), Self(Small(r2)), Self(Small(m2)))
                if m1.checked_mul(m2).is_some() =>
            {
                r1.crt(m1, r2, m2).map(Self::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1, m1, r2, m2),
        }
    }
}

impl Crt<Self, Self, &Self> for Natural {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The
    /// first, second, and third [`Natural`]s are taken by value and the fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     Natural::from(2u32).crt(
    ///         Natural::from(3u32),
    ///         Natural::from(3u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).crt(
    ///         Natural::from(4u32),
    ///         Natural::from(3u32),
    ///         &Natural::from(6u32)
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: Self, r2: Self, m2: &Self) -> Option<Self> {
        assert!(self < m1, "self must be reduced mod m1, but {self} >= {m1}");
        assert!(r2 < *m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Self(Small(r1)), Self(Small(m1)), Self(Small(r2)), Self(Small(m2)))
                if m1.checked_mul(*m2).is_some() =>
            {
                r1.crt(m1, r2, *m2).map(Self::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1, m1, r2, m2.clone()),
        }
    }
}

impl Crt<Self, &Self, Self> for Natural {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The
    /// first, second, and fourth [`Natural`]s are taken by value and the third by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     Natural::from(2u32).crt(
    ///         Natural::from(3u32),
    ///         &Natural::from(3u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).crt(
    ///         Natural::from(4u32),
    ///         &Natural::from(3u32),
    ///         Natural::from(6u32)
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: Self, r2: &Self, m2: Self) -> Option<Self> {
        assert!(self < m1, "self must be reduced mod m1, but {self} >= {m1}");
        assert!(*r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Self(Small(r1)), Self(Small(m1)), Self(Small(r2)), Self(Small(m2)))
                if m1.checked_mul(m2).is_some() =>
            {
                r1.crt(m1, *r2, m2).map(Self::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1, m1, r2.clone(), m2),
        }
    }
}

impl Crt<Self, &Self, &Self> for Natural {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The first
    /// and second [`Natural`]s are taken by value and the third and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     Natural::from(2u32).crt(
    ///         Natural::from(3u32),
    ///         &Natural::from(3u32),
    ///         &Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).crt(
    ///         Natural::from(4u32),
    ///         &Natural::from(3u32),
    ///         &Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: Self, r2: &Self, m2: &Self) -> Option<Self> {
        assert!(self < m1, "self must be reduced mod m1, but {self} >= {m1}");
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Self(Small(r1)), Self(Small(m1)), Self(Small(r2)), Self(Small(m2)))
                if m1.checked_mul(*m2).is_some() =>
            {
                r1.crt(m1, *r2, *m2).map(Self::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1, m1, r2.clone(), m2.clone()),
        }
    }
}

impl Crt<&Self, Self, Self> for Natural {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The
    /// first, third, and fourth [`Natural`]s are taken by value and the second by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     Natural::from(2u32).crt(
    ///         &Natural::from(3u32),
    ///         Natural::from(3u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).crt(
    ///         &Natural::from(4u32),
    ///         Natural::from(3u32),
    ///         Natural::from(6u32)
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: &Self, r2: Self, m2: Self) -> Option<Self> {
        assert!(
            self < *m1,
            "self must be reduced mod m1, but {self} >= {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Self(Small(r1)), Self(Small(m1)), Self(Small(r2)), Self(Small(m2)))
                if m1.checked_mul(m2).is_some() =>
            {
                r1.crt(*m1, r2, m2).map(Self::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1, m1.clone(), r2, m2),
        }
    }
}

impl Crt<&Self, Self, &Self> for Natural {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The first
    /// and third [`Natural`]s are taken by value and the second and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     Natural::from(2u32).crt(
    ///         &Natural::from(3u32),
    ///         Natural::from(3u32),
    ///         &Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).crt(
    ///         &Natural::from(4u32),
    ///         Natural::from(3u32),
    ///         &Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: &Self, r2: Self, m2: &Self) -> Option<Self> {
        assert!(
            self < *m1,
            "self must be reduced mod m1, but {self} >= {m1}"
        );
        assert!(r2 < *m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Self(Small(r1)), Self(Small(m1)), Self(Small(r2)), Self(Small(m2)))
                if m1.checked_mul(*m2).is_some() =>
            {
                r1.crt(*m1, r2, *m2).map(Self::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1, m1.clone(), r2, m2.clone()),
        }
    }
}

impl Crt<&Self, &Self, Self> for Natural {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The first
    /// and fourth [`Natural`]s are taken by value and the second and third by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     Natural::from(2u32).crt(
    ///         &Natural::from(3u32),
    ///         &Natural::from(3u32),
    ///         Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).crt(
    ///         &Natural::from(4u32),
    ///         &Natural::from(3u32),
    ///         Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: &Self, r2: &Self, m2: Self) -> Option<Self> {
        assert!(
            self < *m1,
            "self must be reduced mod m1, but {self} >= {m1}"
        );
        assert!(*r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Self(Small(r1)), Self(Small(m1)), Self(Small(r2)), Self(Small(m2)))
                if m1.checked_mul(m2).is_some() =>
            {
                r1.crt(*m1, *r2, m2).map(Self::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1, m1.clone(), r2.clone(), m2),
        }
    }
}

impl Crt<&Self, &Self, &Self> for Natural {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The first
    /// [`Natural`] is taken by value and the second, third, and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     Natural::from(2u32).crt(
    ///         &Natural::from(3u32),
    ///         &Natural::from(3u32),
    ///         &Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).crt(
    ///         &Natural::from(4u32),
    ///         &Natural::from(3u32),
    ///         &Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: &Self, r2: &Self, m2: &Self) -> Option<Self> {
        assert!(
            self < *m1,
            "self must be reduced mod m1, but {self} >= {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Self(Small(r1)), Self(Small(m1)), Self(Small(r2)), Self(Small(m2)))
                if m1.checked_mul(*m2).is_some() =>
            {
                r1.crt(*m1, *r2, *m2).map(Self::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1, m1.clone(), r2.clone(), m2.clone()),
        }
    }
}

impl Crt<Natural, Natural, Natural> for &Natural {
    type Output = Natural;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The
    /// second, third, and fourth [`Natural`]s are taken by value and the first by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     (&Natural::from(2u32)).crt(
    ///         Natural::from(3u32),
    ///         Natural::from(3u32),
    ///         Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1u32)).crt(
    ///         Natural::from(4u32),
    ///         Natural::from(3u32),
    ///         Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: Natural, r2: Natural, m2: Natural) -> Option<Natural> {
        assert!(
            *self < m1,
            "self must be reduced mod m1, but {self} >= {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Natural(Small(r1)), Natural(Small(m1)), Natural(Small(r2)), Natural(Small(m2)))
                if m1.checked_mul(m2).is_some() =>
            {
                r1.crt(m1, r2, m2).map(Natural::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1.clone(), m1, r2, m2),
        }
    }
}

impl Crt<Natural, Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The
    /// second and third [`Natural`]s are taken by value and the first and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     (&Natural::from(2u32)).crt(
    ///         Natural::from(3u32),
    ///         Natural::from(3u32),
    ///         &Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1u32)).crt(
    ///         Natural::from(4u32),
    ///         Natural::from(3u32),
    ///         &Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: Natural, r2: Natural, m2: &Natural) -> Option<Natural> {
        assert!(
            *self < m1,
            "self must be reduced mod m1, but {self} >= {m1}"
        );
        assert!(r2 < *m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Natural(Small(r1)), Natural(Small(m1)), Natural(Small(r2)), Natural(Small(m2)))
                if m1.checked_mul(*m2).is_some() =>
            {
                r1.crt(m1, r2, *m2).map(Natural::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1.clone(), m1, r2, m2.clone()),
        }
    }
}

impl Crt<Natural, &Natural, Natural> for &Natural {
    type Output = Natural;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The
    /// second and fourth [`Natural`]s are taken by value and the first and third by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     (&Natural::from(2u32)).crt(
    ///         Natural::from(3u32),
    ///         &Natural::from(3u32),
    ///         Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1u32)).crt(
    ///         Natural::from(4u32),
    ///         &Natural::from(3u32),
    ///         Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: Natural, r2: &Natural, m2: Natural) -> Option<Natural> {
        assert!(
            *self < m1,
            "self must be reduced mod m1, but {self} >= {m1}"
        );
        assert!(*r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Natural(Small(r1)), Natural(Small(m1)), Natural(Small(r2)), Natural(Small(m2)))
                if m1.checked_mul(m2).is_some() =>
            {
                r1.crt(m1, *r2, m2).map(Natural::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1.clone(), m1, r2.clone(), m2),
        }
    }
}

impl Crt<Natural, &Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The
    /// second [`Natural`] is taken by value and the first, third, and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     (&Natural::from(2u32)).crt(
    ///         Natural::from(3u32),
    ///         &Natural::from(3u32),
    ///         &Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1u32)).crt(
    ///         Natural::from(4u32),
    ///         &Natural::from(3u32),
    ///         &Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: Natural, r2: &Natural, m2: &Natural) -> Option<Natural> {
        assert!(
            *self < m1,
            "self must be reduced mod m1, but {self} >= {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Natural(Small(r1)), Natural(Small(m1)), Natural(Small(r2)), Natural(Small(m2)))
                if m1.checked_mul(*m2).is_some() =>
            {
                r1.crt(m1, *r2, *m2).map(Natural::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1.clone(), m1, r2.clone(), m2.clone()),
        }
    }
}

impl Crt<&Natural, Natural, Natural> for &Natural {
    type Output = Natural;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The third
    /// and fourth [`Natural`]s are taken by value and the first and second by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     (&Natural::from(2u32)).crt(
    ///         &Natural::from(3u32),
    ///         Natural::from(3u32),
    ///         Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1u32)).crt(
    ///         &Natural::from(4u32),
    ///         Natural::from(3u32),
    ///         Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: &Natural, r2: Natural, m2: Natural) -> Option<Natural> {
        assert!(self < m1, "self must be reduced mod m1, but {self} >= {m1}");
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Natural(Small(r1)), Natural(Small(m1)), Natural(Small(r2)), Natural(Small(m2)))
                if m1.checked_mul(m2).is_some() =>
            {
                r1.crt(*m1, r2, m2).map(Natural::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1.clone(), m1.clone(), r2, m2),
        }
    }
}

impl Crt<&Natural, Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The third
    /// [`Natural`] is taken by value and the first, second, and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     (&Natural::from(2u32)).crt(
    ///         &Natural::from(3u32),
    ///         Natural::from(3u32),
    ///         &Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1u32)).crt(
    ///         &Natural::from(4u32),
    ///         Natural::from(3u32),
    ///         &Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: &Natural, r2: Natural, m2: &Natural) -> Option<Natural> {
        assert!(self < m1, "self must be reduced mod m1, but {self} >= {m1}");
        assert!(r2 < *m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Natural(Small(r1)), Natural(Small(m1)), Natural(Small(r2)), Natural(Small(m2)))
                if m1.checked_mul(*m2).is_some() =>
            {
                r1.crt(*m1, r2, *m2).map(Natural::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1.clone(), m1.clone(), r2, m2.clone()),
        }
    }
}

impl Crt<&Natural, &Natural, Natural> for &Natural {
    type Output = Natural;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. The
    /// fourth [`Natural`] is taken by value and the first, second, and third by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     (&Natural::from(2u32)).crt(
    ///         &Natural::from(3u32),
    ///         &Natural::from(3u32),
    ///         Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1u32)).crt(
    ///         &Natural::from(4u32),
    ///         &Natural::from(3u32),
    ///         Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: &Natural, r2: &Natural, m2: Natural) -> Option<Natural> {
        assert!(self < m1, "self must be reduced mod m1, but {self} >= {m1}");
        assert!(*r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Natural(Small(r1)), Natural(Small(m1)), Natural(Small(r2)), Natural(Small(m2)))
                if m1.checked_mul(m2).is_some() =>
            {
                r1.crt(*m1, *r2, m2).map(Natural::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1.clone(), m1.clone(), r2.clone(), m2),
        }
    }
}

impl Crt<&Natural, &Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Combines two congruences by the Chinese remainder theorem: finds the unique [`Natural`]
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`, reduced modulo `m1 * m2`. All four
    /// [`Natural`]s are taken by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The residues must be already reduced modulo
    /// their moduli.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $x < m_1m_2$, $x \equiv r_1 \mod
    /// m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(m1.significant_bits(),
    /// m2.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m1` or if `r2` is greater than or equal to
    /// `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5.
    /// assert_eq!(
    ///     (&Natural::from(2u32)).crt(
    ///         &Natural::from(3u32),
    ///         &Natural::from(3u32),
    ///         &Natural::from(5u32),
    ///     ),
    ///     Some(Natural::from(8u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1u32)).crt(
    ///         &Natural::from(4u32),
    ///         &Natural::from(3u32),
    ///         &Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    fn crt(self, m1: &Natural, r2: &Natural, m2: &Natural) -> Option<Natural> {
        assert!(self < m1, "self must be reduced mod m1, but {self} >= {m1}");
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        match (self, m1, r2, m2) {
            (Natural(Small(r1)), Natural(Small(m1)), Natural(Small(r2)), Natural(Small(m2)))
                if m1.checked_mul(*m2).is_some() =>
            {
                r1.crt(*m1, *r2, *m2).map(Natural::from)
            }
            (r1, m1, r2, m2) => crt_helper(r1.clone(), m1.clone(), r2.clone(), m2.clone()),
        }
    }
}
