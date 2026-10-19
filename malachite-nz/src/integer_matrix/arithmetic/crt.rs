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
use crate::integer_matrix::IntegerMatrix;
use crate::natural::Natural;
use crate::natural_matrix::arithmetic::crt::{
    check_crt_matrix_args, crt_limb_element, crt_limb_inverse,
};
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{BalancedCrt, UnsignedAbs};
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Combines elements already checked to lie in `[-m1, m1)` with the elements of `r2`, returning the
// balanced representatives.
//
// This is _fmpz_CRT_ui_precomp from fmpz/CRT.c, FLINT 3.6.0, with sign = 1. FLINT compares the
// canonical solution against itself minus the product; comparing it with half the product is
// equivalent, and keeps the positive representative at exactly half.
fn balanced_crt_elements(
    xs: impl Iterator<Item = Integer>,
    m1: &Natural,
    r2: &UnsignedMatrix<Limb>,
    m2: Limb,
) -> Option<Vec<Integer>> {
    let inverse = crt_limb_inverse(m1, m2)?;
    let m2_natural = Natural::from(m2);
    let m = m1 * &m2_natural;
    let half = &m >> 1u32;
    Some(
        xs.zip(&r2.elements)
            .map(|(x, &r)| {
                // Lift x into [0, m1), as _fmpz_CRT_ui_precomp does for negative residues.
                let x = if x < 0u32 {
                    m1 - x.unsigned_abs()
                } else {
                    x.unsigned_abs()
                };
                let x = crt_limb_element(x, m1, r, m2, &m2_natural, inverse);
                if x > half {
                    -Integer::from(&m - x)
                } else {
                    Integer::from(x)
                }
            })
            .collect(),
    )
}

fn check_balanced(a: &IntegerMatrix, m1: &Natural) {
    for x in &a.elements {
        assert!(
            if *x < 0u32 {
                x.unsigned_abs_ref() <= m1
            } else {
                x.unsigned_abs_ref() < m1
            },
            "self must satisfy -m1 <= x < m1 for each element x, but x = {x} and m1 = {m1}"
        );
    }
}

fn balanced_crt_matrix_owned(
    a: IntegerMatrix,
    m1: &Natural,
    r2: &UnsignedMatrix<Limb>,
    m2: Limb,
) -> Option<IntegerMatrix> {
    let (height, width) = (a.height(), a.width());
    check_crt_matrix_args(height, width, m1, r2, m2);
    check_balanced(&a, m1);
    Some(IntegerMatrix::from_owned_elements(
        height,
        width,
        balanced_crt_elements(a.elements.into_iter(), m1, r2, m2)?,
    ))
}

fn balanced_crt_matrix_ref(
    a: &IntegerMatrix,
    m1: &Natural,
    r2: &UnsignedMatrix<Limb>,
    m2: Limb,
) -> Option<IntegerMatrix> {
    let (height, width) = (a.height(), a.width());
    check_crt_matrix_args(height, width, m1, r2, m2);
    check_balanced(a, m1);
    Some(IntegerMatrix::from_owned_elements(
        height,
        width,
        balanced_crt_elements(a.elements.iter().cloned(), m1, r2, m2)?,
    ))
}

impl BalancedCrt<Natural, UnsignedMatrix<Limb>, Limb> for IntegerMatrix {
    type Output = Self;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element,
    /// returning balanced representatives: finds the [`IntegerMatrix`] whose every element is the
    /// unique number $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is congruent to the element of
    /// `self` in the same position modulo `m1` and to the element of `r2` modulo `m2`. `self` is
    /// taken by value, `m1` by value, and `r2` by value.
    ///
    /// Returns `None` if the moduli are not coprime. Each element of `self` may be any
    /// representative in $[-m_1, m_1)$, negative representatives included, so balanced results may
    /// be combined again; the elements of `r2` must be reduced modulo `m2`. The inverse of `m1`
    /// modulo `m2` is computed once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $-m_1m_2/2 < X_{ij} \leq m_1m_2/2$,
    /// $X_{ij} \equiv A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if
    /// $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m1.significant_bits()`, and $k$ is the
    /// number of elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `r2` have different shapes, if `m1` or `m2` is 0, if any element of
    /// `self` is less than `-m1` or greater than or equal to `m1`, or if any element of `r2` is
    /// greater than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::natural::Natural;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = IntegerMatrix::from_str("[-2, 1; 0, -5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.balanced_crt(Natural::from(5u32), b, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "[-2, 6; 5, -5]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = IntegerMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!(a.balanced_crt(Natural::from(6u32), b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign = 1
    /// and a noninvertible modulus reported as `None` rather than thrown.
    #[inline]
    fn balanced_crt(self, m1: Natural, r2: UnsignedMatrix<Limb>, m2: Limb) -> Option<Self> {
        balanced_crt_matrix_owned(self, &m1, &r2, m2)
    }
}

impl BalancedCrt<Natural, &UnsignedMatrix<Limb>, Limb> for IntegerMatrix {
    type Output = Self;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element,
    /// returning balanced representatives: finds the [`IntegerMatrix`] whose every element is the
    /// unique number $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is congruent to the element of
    /// `self` in the same position modulo `m1` and to the element of `r2` modulo `m2`. `self` is
    /// taken by value, `m1` by value, and `r2` by reference.
    ///
    /// Returns `None` if the moduli are not coprime. Each element of `self` may be any
    /// representative in $[-m_1, m_1)$, negative representatives included, so balanced results may
    /// be combined again; the elements of `r2` must be reduced modulo `m2`. The inverse of `m1`
    /// modulo `m2` is computed once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $-m_1m_2/2 < X_{ij} \leq m_1m_2/2$,
    /// $X_{ij} \equiv A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if
    /// $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m1.significant_bits()`, and $k$ is the
    /// number of elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `r2` have different shapes, if `m1` or `m2` is 0, if any element of
    /// `self` is less than `-m1` or greater than or equal to `m1`, or if any element of `r2` is
    /// greater than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::natural::Natural;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = IntegerMatrix::from_str("[-2, 1; 0, -5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.balanced_crt(Natural::from(5u32), &b, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "[-2, 6; 5, -5]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = IntegerMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!(a.balanced_crt(Natural::from(6u32), &b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign = 1
    /// and a noninvertible modulus reported as `None` rather than thrown.
    #[inline]
    fn balanced_crt(self, m1: Natural, r2: &UnsignedMatrix<Limb>, m2: Limb) -> Option<Self> {
        balanced_crt_matrix_owned(self, &m1, r2, m2)
    }
}

impl BalancedCrt<&Natural, UnsignedMatrix<Limb>, Limb> for IntegerMatrix {
    type Output = Self;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element,
    /// returning balanced representatives: finds the [`IntegerMatrix`] whose every element is the
    /// unique number $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is congruent to the element of
    /// `self` in the same position modulo `m1` and to the element of `r2` modulo `m2`. `self` is
    /// taken by value, `m1` by reference, and `r2` by value.
    ///
    /// Returns `None` if the moduli are not coprime. Each element of `self` may be any
    /// representative in $[-m_1, m_1)$, negative representatives included, so balanced results may
    /// be combined again; the elements of `r2` must be reduced modulo `m2`. The inverse of `m1`
    /// modulo `m2` is computed once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $-m_1m_2/2 < X_{ij} \leq m_1m_2/2$,
    /// $X_{ij} \equiv A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if
    /// $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m1.significant_bits()`, and $k$ is the
    /// number of elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `r2` have different shapes, if `m1` or `m2` is 0, if any element of
    /// `self` is less than `-m1` or greater than or equal to `m1`, or if any element of `r2` is
    /// greater than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::natural::Natural;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = IntegerMatrix::from_str("[-2, 1; 0, -5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.balanced_crt(&Natural::from(5u32), b, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "[-2, 6; 5, -5]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = IntegerMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!(a.balanced_crt(&Natural::from(6u32), b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign = 1
    /// and a noninvertible modulus reported as `None` rather than thrown.
    #[inline]
    fn balanced_crt(self, m1: &Natural, r2: UnsignedMatrix<Limb>, m2: Limb) -> Option<Self> {
        balanced_crt_matrix_owned(self, m1, &r2, m2)
    }
}

impl BalancedCrt<&Natural, &UnsignedMatrix<Limb>, Limb> for IntegerMatrix {
    type Output = Self;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element,
    /// returning balanced representatives: finds the [`IntegerMatrix`] whose every element is the
    /// unique number $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is congruent to the element of
    /// `self` in the same position modulo `m1` and to the element of `r2` modulo `m2`. `self` is
    /// taken by value, `m1` by reference, and `r2` by reference.
    ///
    /// Returns `None` if the moduli are not coprime. Each element of `self` may be any
    /// representative in $[-m_1, m_1)$, negative representatives included, so balanced results may
    /// be combined again; the elements of `r2` must be reduced modulo `m2`. The inverse of `m1`
    /// modulo `m2` is computed once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $-m_1m_2/2 < X_{ij} \leq m_1m_2/2$,
    /// $X_{ij} \equiv A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if
    /// $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m1.significant_bits()`, and $k$ is the
    /// number of elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `r2` have different shapes, if `m1` or `m2` is 0, if any element of
    /// `self` is less than `-m1` or greater than or equal to `m1`, or if any element of `r2` is
    /// greater than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::natural::Natural;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = IntegerMatrix::from_str("[-2, 1; 0, -5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.balanced_crt(&Natural::from(5u32), &b, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "[-2, 6; 5, -5]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = IntegerMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!(a.balanced_crt(&Natural::from(6u32), &b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign = 1
    /// and a noninvertible modulus reported as `None` rather than thrown.
    #[inline]
    fn balanced_crt(self, m1: &Natural, r2: &UnsignedMatrix<Limb>, m2: Limb) -> Option<Self> {
        balanced_crt_matrix_owned(self, m1, r2, m2)
    }
}

impl BalancedCrt<Natural, UnsignedMatrix<Limb>, Limb> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element,
    /// returning balanced representatives: finds the [`IntegerMatrix`] whose every element is the
    /// unique number $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is congruent to the element of
    /// `self` in the same position modulo `m1` and to the element of `r2` modulo `m2`. `self` is
    /// taken by reference, `m1` by value, and `r2` by value.
    ///
    /// Returns `None` if the moduli are not coprime. Each element of `self` may be any
    /// representative in $[-m_1, m_1)$, negative representatives included, so balanced results may
    /// be combined again; the elements of `r2` must be reduced modulo `m2`. The inverse of `m1`
    /// modulo `m2` is computed once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $-m_1m_2/2 < X_{ij} \leq m_1m_2/2$,
    /// $X_{ij} \equiv A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if
    /// $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m1.significant_bits()`, and $k$ is the
    /// number of elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `r2` have different shapes, if `m1` or `m2` is 0, if any element of
    /// `self` is less than `-m1` or greater than or equal to `m1`, or if any element of `r2` is
    /// greater than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::natural::Natural;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = IntegerMatrix::from_str("[-2, 1; 0, -5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).balanced_crt(Natural::from(5u32), b, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "[-2, 6; 5, -5]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = IntegerMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!((&a).balanced_crt(Natural::from(6u32), b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign = 1
    /// and a noninvertible modulus reported as `None` rather than thrown.
    #[inline]
    fn balanced_crt(
        self,
        m1: Natural,
        r2: UnsignedMatrix<Limb>,
        m2: Limb,
    ) -> Option<IntegerMatrix> {
        balanced_crt_matrix_ref(self, &m1, &r2, m2)
    }
}

impl BalancedCrt<Natural, &UnsignedMatrix<Limb>, Limb> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element,
    /// returning balanced representatives: finds the [`IntegerMatrix`] whose every element is the
    /// unique number $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is congruent to the element of
    /// `self` in the same position modulo `m1` and to the element of `r2` modulo `m2`. `self` is
    /// taken by reference, `m1` by value, and `r2` by reference.
    ///
    /// Returns `None` if the moduli are not coprime. Each element of `self` may be any
    /// representative in $[-m_1, m_1)$, negative representatives included, so balanced results may
    /// be combined again; the elements of `r2` must be reduced modulo `m2`. The inverse of `m1`
    /// modulo `m2` is computed once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $-m_1m_2/2 < X_{ij} \leq m_1m_2/2$,
    /// $X_{ij} \equiv A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if
    /// $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m1.significant_bits()`, and $k$ is the
    /// number of elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `r2` have different shapes, if `m1` or `m2` is 0, if any element of
    /// `self` is less than `-m1` or greater than or equal to `m1`, or if any element of `r2` is
    /// greater than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::natural::Natural;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = IntegerMatrix::from_str("[-2, 1; 0, -5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).balanced_crt(Natural::from(5u32), &b, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "[-2, 6; 5, -5]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = IntegerMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!((&a).balanced_crt(Natural::from(6u32), &b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign = 1
    /// and a noninvertible modulus reported as `None` rather than thrown.
    #[inline]
    fn balanced_crt(
        self,
        m1: Natural,
        r2: &UnsignedMatrix<Limb>,
        m2: Limb,
    ) -> Option<IntegerMatrix> {
        balanced_crt_matrix_ref(self, &m1, r2, m2)
    }
}

impl BalancedCrt<&Natural, UnsignedMatrix<Limb>, Limb> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element,
    /// returning balanced representatives: finds the [`IntegerMatrix`] whose every element is the
    /// unique number $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is congruent to the element of
    /// `self` in the same position modulo `m1` and to the element of `r2` modulo `m2`. `self` is
    /// taken by reference, `m1` by reference, and `r2` by value.
    ///
    /// Returns `None` if the moduli are not coprime. Each element of `self` may be any
    /// representative in $[-m_1, m_1)$, negative representatives included, so balanced results may
    /// be combined again; the elements of `r2` must be reduced modulo `m2`. The inverse of `m1`
    /// modulo `m2` is computed once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $-m_1m_2/2 < X_{ij} \leq m_1m_2/2$,
    /// $X_{ij} \equiv A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if
    /// $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m1.significant_bits()`, and $k$ is the
    /// number of elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `r2` have different shapes, if `m1` or `m2` is 0, if any element of
    /// `self` is less than `-m1` or greater than or equal to `m1`, or if any element of `r2` is
    /// greater than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::natural::Natural;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = IntegerMatrix::from_str("[-2, 1; 0, -5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).balanced_crt(&Natural::from(5u32), b, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "[-2, 6; 5, -5]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = IntegerMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!((&a).balanced_crt(&Natural::from(6u32), b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign = 1
    /// and a noninvertible modulus reported as `None` rather than thrown.
    #[inline]
    fn balanced_crt(
        self,
        m1: &Natural,
        r2: UnsignedMatrix<Limb>,
        m2: Limb,
    ) -> Option<IntegerMatrix> {
        balanced_crt_matrix_ref(self, m1, &r2, m2)
    }
}

impl BalancedCrt<&Natural, &UnsignedMatrix<Limb>, Limb> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element,
    /// returning balanced representatives: finds the [`IntegerMatrix`] whose every element is the
    /// unique number $x$ with $-m_1m_2/2 < x \leq m_1m_2/2$ that is congruent to the element of
    /// `self` in the same position modulo `m1` and to the element of `r2` modulo `m2`. `self` is
    /// taken by reference, `m1` by reference, and `r2` by reference.
    ///
    /// Returns `None` if the moduli are not coprime. Each element of `self` may be any
    /// representative in $[-m_1, m_1)$, negative representatives included, so balanced results may
    /// be combined again; the elements of `r2` must be reduced modulo `m2`. The inverse of `m1`
    /// modulo `m2` is computed once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $-m_1m_2/2 < X_{ij} \leq m_1m_2/2$,
    /// $X_{ij} \equiv A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if
    /// $m_1$ and $m_2$ are coprime.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(n, k) = O(nk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `m1.significant_bits()`, and $k$ is the
    /// number of elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `r2` have different shapes, if `m1` or `m2` is 0, if any element of
    /// `self` is less than `-m1` or greater than or equal to `m1`, or if any element of `r2` is
    /// greater than or equal to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedCrt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::natural::Natural;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = IntegerMatrix::from_str("[-2, 1; 0, -5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).balanced_crt(&Natural::from(5u32), &b, 3)
    ///         .unwrap()
    ///         .to_string(),
    ///     "[-2, 6; 5, -5]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = IntegerMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!((&a).balanced_crt(&Natural::from(6u32), &b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign = 1
    /// and a noninvertible modulus reported as `None` rather than thrown.
    #[inline]
    fn balanced_crt(
        self,
        m1: &Natural,
        r2: &UnsignedMatrix<Limb>,
        m2: Limb,
    ) -> Option<IntegerMatrix> {
        balanced_crt_matrix_ref(self, m1, r2, m2)
    }
}
