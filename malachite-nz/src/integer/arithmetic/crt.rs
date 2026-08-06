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

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{BalancedCrt, BalancedMod, Crt, UnsignedAbs};

// Computes the balanced Chinese-remainder combination: the unique `x` in `(-m1 * m2 / 2, m1 * m2 /
// 2]` with `x ≡ r1 mod m1` and `x ≡ r2 mod m2`, or `None` if the moduli are not coprime. `r1`
// may be any representative in `[-m1, m1)`; `r2` must be reduced mod `m2`.
//
// This is fmpz_CRT from fmpz/CRT.c, FLINT 3.6.0, with sign = 1. FLINT compares the canonical
// solution against itself minus the product directly; reducing with `balanced_mod` is equivalent,
// since the canonical solution is already in `[0, m1 * m2)`.
fn balanced_crt_helper(r1: Integer, m1: Natural, r2: Natural, m2: Natural) -> Option<Integer> {
    // Lift r1 into [0, m1), as _fmpz_CRT does for negative residues.
    let r1n = if r1 < 0u32 {
        &m1 - r1.unsigned_abs()
    } else {
        r1.unsigned_abs()
    };
    let m = &m1 * &m2;
    let x = r1n.crt(m1, r2, m2)?;
    Some(Integer::from(x).balanced_mod(Integer::from(m)))
}

impl BalancedCrt<Natural, Natural, Natural> for Integer {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. All four arguments are taken by
    /// value.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     Integer::from(-1).balanced_crt(
    ///     Natural::from(3u32),
    ///     Natural::from(3u32),
    ///     Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     Integer::from(1).balanced_crt(
    ///     Natural::from(4u32),
    ///     Natural::from(3u32),
    ///     Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: Natural, r2: Natural, m2: Natural) -> Option<Self> {
        assert!(
            if self < 0u32 {
                *self.unsigned_abs_ref() <= m1
            } else {
                *self.unsigned_abs_ref() < m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self, m1, r2, m2)
    }
}

impl BalancedCrt<Natural, Natural, &Natural> for Integer {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The first, second, and third
    /// arguments are taken by value and the fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     Integer::from(-1).balanced_crt(
    ///     Natural::from(3u32),
    ///     Natural::from(3u32),
    ///     &Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     Integer::from(1).balanced_crt(
    ///     Natural::from(4u32),
    ///     Natural::from(3u32),
    ///     &Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: Natural, r2: Natural, m2: &Natural) -> Option<Self> {
        assert!(
            if self < 0u32 {
                *self.unsigned_abs_ref() <= m1
            } else {
                *self.unsigned_abs_ref() < m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < *m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self, m1, r2, m2.clone())
    }
}

impl BalancedCrt<Natural, &Natural, Natural> for Integer {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The first, second, and fourth
    /// arguments are taken by value and the third by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     Integer::from(-1).balanced_crt(
    ///     Natural::from(3u32),
    ///     &Natural::from(3u32),
    ///     Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     Integer::from(1).balanced_crt(
    ///     Natural::from(4u32),
    ///     &Natural::from(3u32),
    ///     Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: Natural, r2: &Natural, m2: Natural) -> Option<Self> {
        assert!(
            if self < 0u32 {
                *self.unsigned_abs_ref() <= m1
            } else {
                *self.unsigned_abs_ref() < m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(*r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self, m1, r2.clone(), m2)
    }
}

impl BalancedCrt<Natural, &Natural, &Natural> for Integer {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The first and second arguments are
    /// taken by value and the third and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     Integer::from(-1).balanced_crt(
    ///     Natural::from(3u32),
    ///     &Natural::from(3u32),
    ///     &Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     Integer::from(1).balanced_crt(
    ///     Natural::from(4u32),
    ///     &Natural::from(3u32),
    ///     &Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: Natural, r2: &Natural, m2: &Natural) -> Option<Self> {
        assert!(
            if self < 0u32 {
                *self.unsigned_abs_ref() <= m1
            } else {
                *self.unsigned_abs_ref() < m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self, m1, r2.clone(), m2.clone())
    }
}

impl BalancedCrt<&Natural, Natural, Natural> for Integer {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The first, third, and fourth
    /// arguments are taken by value and the second by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     Integer::from(-1).balanced_crt(
    ///     &Natural::from(3u32),
    ///     Natural::from(3u32),
    ///     Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     Integer::from(1).balanced_crt(
    ///     &Natural::from(4u32),
    ///     Natural::from(3u32),
    ///     Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: &Natural, r2: Natural, m2: Natural) -> Option<Self> {
        assert!(
            if self < 0u32 {
                *self.unsigned_abs_ref() <= *m1
            } else {
                *self.unsigned_abs_ref() < *m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self, m1.clone(), r2, m2)
    }
}

impl BalancedCrt<&Natural, Natural, &Natural> for Integer {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The first and third arguments are
    /// taken by value and the second and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     Integer::from(-1).balanced_crt(
    ///     &Natural::from(3u32),
    ///     Natural::from(3u32),
    ///     &Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     Integer::from(1).balanced_crt(
    ///     &Natural::from(4u32),
    ///     Natural::from(3u32),
    ///     &Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: &Natural, r2: Natural, m2: &Natural) -> Option<Self> {
        assert!(
            if self < 0u32 {
                *self.unsigned_abs_ref() <= *m1
            } else {
                *self.unsigned_abs_ref() < *m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < *m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self, m1.clone(), r2, m2.clone())
    }
}

impl BalancedCrt<&Natural, &Natural, Natural> for Integer {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The first and fourth arguments are
    /// taken by value and the second and third by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     Integer::from(-1).balanced_crt(
    ///     &Natural::from(3u32),
    ///     &Natural::from(3u32),
    ///     Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     Integer::from(1).balanced_crt(
    ///     &Natural::from(4u32),
    ///     &Natural::from(3u32),
    ///     Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: &Natural, r2: &Natural, m2: Natural) -> Option<Self> {
        assert!(
            if self < 0u32 {
                *self.unsigned_abs_ref() <= *m1
            } else {
                *self.unsigned_abs_ref() < *m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(*r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self, m1.clone(), r2.clone(), m2)
    }
}

impl BalancedCrt<&Natural, &Natural, &Natural> for Integer {
    type Output = Self;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The first argument is taken by
    /// value and the second, third, and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     Integer::from(-1).balanced_crt(
    ///     &Natural::from(3u32),
    ///     &Natural::from(3u32),
    ///     &Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     Integer::from(1).balanced_crt(
    ///     &Natural::from(4u32),
    ///     &Natural::from(3u32),
    ///     &Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: &Natural, r2: &Natural, m2: &Natural) -> Option<Self> {
        assert!(
            if self < 0u32 {
                *self.unsigned_abs_ref() <= *m1
            } else {
                *self.unsigned_abs_ref() < *m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self, m1.clone(), r2.clone(), m2.clone())
    }
}

impl BalancedCrt<Natural, Natural, Natural> for &Integer {
    type Output = Integer;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The second, third, and fourth
    /// arguments are taken by value and the first by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     (&Integer::from(-1)).balanced_crt(
    ///     Natural::from(3u32),
    ///     Natural::from(3u32),
    ///     Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1)).balanced_crt(
    ///     Natural::from(4u32),
    ///     Natural::from(3u32),
    ///     Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: Natural, r2: Natural, m2: Natural) -> Option<Integer> {
        assert!(
            if *self < 0u32 {
                *self.unsigned_abs_ref() <= m1
            } else {
                *self.unsigned_abs_ref() < m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self.clone(), m1, r2, m2)
    }
}

impl BalancedCrt<Natural, Natural, &Natural> for &Integer {
    type Output = Integer;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The second and third arguments are
    /// taken by value and the first and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     (&Integer::from(-1)).balanced_crt(
    ///     Natural::from(3u32),
    ///     Natural::from(3u32),
    ///     &Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1)).balanced_crt(
    ///     Natural::from(4u32),
    ///     Natural::from(3u32),
    ///     &Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: Natural, r2: Natural, m2: &Natural) -> Option<Integer> {
        assert!(
            if *self < 0u32 {
                *self.unsigned_abs_ref() <= m1
            } else {
                *self.unsigned_abs_ref() < m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < *m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self.clone(), m1, r2, m2.clone())
    }
}

impl BalancedCrt<Natural, &Natural, Natural> for &Integer {
    type Output = Integer;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The second and fourth arguments are
    /// taken by value and the first and third by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     (&Integer::from(-1)).balanced_crt(
    ///     Natural::from(3u32),
    ///     &Natural::from(3u32),
    ///     Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1)).balanced_crt(
    ///     Natural::from(4u32),
    ///     &Natural::from(3u32),
    ///     Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: Natural, r2: &Natural, m2: Natural) -> Option<Integer> {
        assert!(
            if *self < 0u32 {
                *self.unsigned_abs_ref() <= m1
            } else {
                *self.unsigned_abs_ref() < m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(*r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self.clone(), m1, r2.clone(), m2)
    }
}

impl BalancedCrt<Natural, &Natural, &Natural> for &Integer {
    type Output = Integer;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The second argument is taken by
    /// value and the first, third, and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     (&Integer::from(-1)).balanced_crt(
    ///     Natural::from(3u32),
    ///     &Natural::from(3u32),
    ///     &Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1)).balanced_crt(
    ///     Natural::from(4u32),
    ///     &Natural::from(3u32),
    ///     &Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: Natural, r2: &Natural, m2: &Natural) -> Option<Integer> {
        assert!(
            if *self < 0u32 {
                *self.unsigned_abs_ref() <= m1
            } else {
                *self.unsigned_abs_ref() < m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self.clone(), m1, r2.clone(), m2.clone())
    }
}

impl BalancedCrt<&Natural, Natural, Natural> for &Integer {
    type Output = Integer;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The third and fourth arguments are
    /// taken by value and the first and second by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     (&Integer::from(-1)).balanced_crt(
    ///     &Natural::from(3u32),
    ///     Natural::from(3u32),
    ///     Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1)).balanced_crt(
    ///     &Natural::from(4u32),
    ///     Natural::from(3u32),
    ///     Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: &Natural, r2: Natural, m2: Natural) -> Option<Integer> {
        assert!(
            if *self < 0u32 {
                *self.unsigned_abs_ref() <= *m1
            } else {
                *self.unsigned_abs_ref() < *m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self.clone(), m1.clone(), r2, m2)
    }
}

impl BalancedCrt<&Natural, Natural, &Natural> for &Integer {
    type Output = Integer;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The third argument is taken by
    /// value and the first, second, and fourth by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     (&Integer::from(-1)).balanced_crt(
    ///     &Natural::from(3u32),
    ///     Natural::from(3u32),
    ///     &Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1)).balanced_crt(
    ///     &Natural::from(4u32),
    ///     Natural::from(3u32),
    ///     &Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: &Natural, r2: Natural, m2: &Natural) -> Option<Integer> {
        assert!(
            if *self < 0u32 {
                *self.unsigned_abs_ref() <= *m1
            } else {
                *self.unsigned_abs_ref() < *m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < *m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self.clone(), m1.clone(), r2, m2.clone())
    }
}

impl BalancedCrt<&Natural, &Natural, Natural> for &Integer {
    type Output = Integer;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. The fourth argument is taken by
    /// value and the first, second, and third by reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     (&Integer::from(-1)).balanced_crt(
    ///     &Natural::from(3u32),
    ///     &Natural::from(3u32),
    ///     Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1)).balanced_crt(
    ///     &Natural::from(4u32),
    ///     &Natural::from(3u32),
    ///     Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: &Natural, r2: &Natural, m2: Natural) -> Option<Integer> {
        assert!(
            if *self < 0u32 {
                *self.unsigned_abs_ref() <= *m1
            } else {
                *self.unsigned_abs_ref() < *m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(*r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self.clone(), m1.clone(), r2.clone(), m2)
    }
}

impl BalancedCrt<&Natural, &Natural, &Natural> for &Integer {
    type Output = Integer;

    /// Combines two congruences by the Chinese remainder theorem, returning the balanced
    /// representative: the unique [`Integer`] $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is
    /// congruent to `self` modulo `m1` and to `r2` modulo `m2`. All four arguments are taken by
    /// reference.
    ///
    /// Returns `None` if the moduli are not coprime. `self` may be any representative in $[-m_1,
    /// m_1)$, negative representatives included, so balanced results may be chained; `r2` must be
    /// reduced modulo `m2`.
    ///
    /// $f(r_1, m_1, r_2, m_2) = \operatorname{Some}(x)$, where $-m_1m_2/2 < x \leq m_1m_2/2$, $x
    /// \equiv r_1 \mod m_1$, and $x \equiv r_2 \mod m_2$, if $m_1$ and $m_2$ are coprime.
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
    /// Panics if `self` is less than `-m1` or greater than or equal to `m1`, or if `r2` is greater
    /// than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     (&Integer::from(-1)).balanced_crt(
    ///     &Natural::from(3u32),
    ///     &Natural::from(3u32),
    ///     &Natural::from(5u32),
    /// ),
    ///     Some(Integer::from(-7))
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1)).balanced_crt(
    ///     &Natural::from(4u32),
    ///     &Natural::from(3u32),
    ///     &Natural::from(6u32),
    /// ),
    ///     None
    /// );
    /// ```
    fn balanced_crt(self, m1: &Natural, r2: &Natural, m2: &Natural) -> Option<Integer> {
        assert!(
            if *self < 0u32 {
                *self.unsigned_abs_ref() <= *m1
            } else {
                *self.unsigned_abs_ref() < *m1
            },
            "self must satisfy -m1 <= self < m1, but self = {self} and m1 = {m1}"
        );
        assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
        balanced_crt_helper(self.clone(), m1.clone(), r2.clone(), m2.clone())
    }
}
