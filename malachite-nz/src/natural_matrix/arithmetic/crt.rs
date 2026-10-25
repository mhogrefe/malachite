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

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{AddMul, Crt, ModInverse, ModMul, ModSub};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the shapes and moduli of the matrix CRT functions, and that the second matrix's elements
// are reduced; the caller checks the first matrix's elements.
pub(crate) fn check_crt_matrix_args(
    height: u64,
    width: u64,
    m1: &Natural,
    r2: &UnsignedMatrix<Limb>,
    m2: Limb,
) {
    assert!(
        r2.height() == height && r2.width() == width,
        "the matrices must have the same shape, but they are {height} by {width} and {} by {}",
        r2.height(),
        r2.width()
    );
    assert!(*m1 != 0u32 && m2 != 0, "the moduli must be positive");
    for &r in r2.elements_ref() {
        assert!(r < m2, "r2 must be reduced mod m2, but {r} >= {m2}");
    }
}

// The inverse of `m1` modulo `m2`, which combining with the word-sized modulus `m2` needs once per
// call rather than once per element, or `None` if the moduli are not coprime. When `m2` is 1, every
// residue modulo it is 0, and 0 serves.
pub(crate) fn crt_limb_inverse(m1: &Natural, m2: Limb) -> Option<Limb> {
    if m2 == 1 {
        return Some(0);
    }
    let c = Limb::exact_from(&(m1 % Natural::from(m2)));
    if c == 0 { None } else { c.mod_inverse(m2) }
}

// The unique `x` with `x ≡ r1 mod m1`, `x ≡ r2 mod m2`, and `0 <= x < m1 * m2`, given reduced
// residues, `m2` both as a `Limb` and as a `Natural`, and `inverse`, the inverse of `m1` modulo
// `m2`.
//
// This is _fmpz_CRT_ui_precomp from fmpz/CRT.c, FLINT 3.6.0, with sign = 0 and `r1` nonnegative.
pub(crate) fn crt_limb_element(
    r1: Natural,
    m1: &Natural,
    r2: Limb,
    m2: Limb,
    m2_natural: &Natural,
    inverse: Limb,
) -> Natural {
    let s = r2
        .mod_sub(Limb::exact_from(&(&r1 % m2_natural)), m2)
        .mod_mul(inverse, m2);
    r1.add_mul(m1, Natural::from(s))
}

// Combines elements already checked to be reduced modulo `m1` with the elements of `r2`.
fn crt_elements(
    xs: impl Iterator<Item = Natural>,
    m1: &Natural,
    r2: &UnsignedMatrix<Limb>,
    m2: Limb,
) -> Option<Vec<Natural>> {
    let inverse = crt_limb_inverse(m1, m2)?;
    let m2_natural = Natural::from(m2);
    Some(
        xs.zip(r2.elements_ref())
            .map(|(x, &r)| crt_limb_element(x, m1, r, m2, &m2_natural, inverse))
            .collect(),
    )
}

fn check_reduced(a: &NaturalMatrix, m1: &Natural) {
    for x in &a.elements {
        assert!(x < m1, "self must be reduced mod m1, but {x} >= {m1}");
    }
}

fn crt_matrix_owned(
    a: NaturalMatrix,
    m1: &Natural,
    r2: &UnsignedMatrix<Limb>,
    m2: Limb,
) -> Option<NaturalMatrix> {
    let (height, width) = (a.height(), a.width());
    check_crt_matrix_args(height, width, m1, r2, m2);
    check_reduced(&a, m1);
    Some(NaturalMatrix::from_owned_elements(
        height,
        width,
        crt_elements(a.elements.into_iter(), m1, r2, m2)?,
    ))
}

fn crt_matrix_ref(
    a: &NaturalMatrix,
    m1: &Natural,
    r2: &UnsignedMatrix<Limb>,
    m2: Limb,
) -> Option<NaturalMatrix> {
    let (height, width) = (a.height(), a.width());
    check_crt_matrix_args(height, width, m1, r2, m2);
    check_reduced(a, m1);
    Some(NaturalMatrix::from_owned_elements(
        height,
        width,
        crt_elements(a.elements.iter().cloned(), m1, r2, m2)?,
    ))
}

impl Crt<Natural, UnsignedMatrix<Limb>, Limb> for NaturalMatrix {
    type Output = Self;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element:
    /// finds the [`NaturalMatrix`] whose every element is the unique number below `m1 * m2` that is
    /// congruent to the element of `self` in the same position modulo `m1` and to the element of
    /// `r2` modulo `m2`. `self` is taken by value, `m1` by value, and `r2` by value.
    ///
    /// Returns `None` if the moduli are not coprime. The elements of `self` must be already reduced
    /// modulo `m1`, and those of `r2` modulo `m2`. The inverse of `m1` modulo `m2` is computed
    /// once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $0 \leq X_{ij} < m_1m_2$, $X_{ij} \equiv
    /// A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if $m_1$ and
    /// $m_2$ are coprime.
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
    /// `self` is greater than or equal to `m1`, or if any element of `r2` is greater than or equal
    /// to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.crt(Natural::from(5u32), b, 3).unwrap().to_string(),
    ///     "[7, 6; 5, 4]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!(a.crt(Natural::from(6u32), b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign =
    /// 0, the residues required to be reduced, and a noninvertible modulus reported as `None`
    /// rather than thrown.
    #[inline]
    fn crt(self, m1: Natural, r2: UnsignedMatrix<Limb>, m2: Limb) -> Option<Self> {
        crt_matrix_owned(self, &m1, &r2, m2)
    }
}

impl Crt<Natural, &UnsignedMatrix<Limb>, Limb> for NaturalMatrix {
    type Output = Self;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element:
    /// finds the [`NaturalMatrix`] whose every element is the unique number below `m1 * m2` that is
    /// congruent to the element of `self` in the same position modulo `m1` and to the element of
    /// `r2` modulo `m2`. `self` is taken by value, `m1` by value, and `r2` by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The elements of `self` must be already reduced
    /// modulo `m1`, and those of `r2` modulo `m2`. The inverse of `m1` modulo `m2` is computed
    /// once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $0 \leq X_{ij} < m_1m_2$, $X_{ij} \equiv
    /// A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if $m_1$ and
    /// $m_2$ are coprime.
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
    /// `self` is greater than or equal to `m1`, or if any element of `r2` is greater than or equal
    /// to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.crt(Natural::from(5u32), &b, 3).unwrap().to_string(),
    ///     "[7, 6; 5, 4]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!(a.crt(Natural::from(6u32), &b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign =
    /// 0, the residues required to be reduced, and a noninvertible modulus reported as `None`
    /// rather than thrown.
    #[inline]
    fn crt(self, m1: Natural, r2: &UnsignedMatrix<Limb>, m2: Limb) -> Option<Self> {
        crt_matrix_owned(self, &m1, r2, m2)
    }
}

impl Crt<&Natural, UnsignedMatrix<Limb>, Limb> for NaturalMatrix {
    type Output = Self;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element:
    /// finds the [`NaturalMatrix`] whose every element is the unique number below `m1 * m2` that is
    /// congruent to the element of `self` in the same position modulo `m1` and to the element of
    /// `r2` modulo `m2`. `self` is taken by value, `m1` by reference, and `r2` by value.
    ///
    /// Returns `None` if the moduli are not coprime. The elements of `self` must be already reduced
    /// modulo `m1`, and those of `r2` modulo `m2`. The inverse of `m1` modulo `m2` is computed
    /// once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $0 \leq X_{ij} < m_1m_2$, $X_{ij} \equiv
    /// A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if $m_1$ and
    /// $m_2$ are coprime.
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
    /// `self` is greater than or equal to `m1`, or if any element of `r2` is greater than or equal
    /// to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.crt(&Natural::from(5u32), b, 3).unwrap().to_string(),
    ///     "[7, 6; 5, 4]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!(a.crt(&Natural::from(6u32), b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign =
    /// 0, the residues required to be reduced, and a noninvertible modulus reported as `None`
    /// rather than thrown.
    #[inline]
    fn crt(self, m1: &Natural, r2: UnsignedMatrix<Limb>, m2: Limb) -> Option<Self> {
        crt_matrix_owned(self, m1, &r2, m2)
    }
}

impl Crt<&Natural, &UnsignedMatrix<Limb>, Limb> for NaturalMatrix {
    type Output = Self;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element:
    /// finds the [`NaturalMatrix`] whose every element is the unique number below `m1 * m2` that is
    /// congruent to the element of `self` in the same position modulo `m1` and to the element of
    /// `r2` modulo `m2`. `self` is taken by value, `m1` by reference, and `r2` by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The elements of `self` must be already reduced
    /// modulo `m1`, and those of `r2` modulo `m2`. The inverse of `m1` modulo `m2` is computed
    /// once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $0 \leq X_{ij} < m_1m_2$, $X_{ij} \equiv
    /// A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if $m_1$ and
    /// $m_2$ are coprime.
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
    /// `self` is greater than or equal to `m1`, or if any element of `r2` is greater than or equal
    /// to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.crt(&Natural::from(5u32), &b, 3).unwrap().to_string(),
    ///     "[7, 6; 5, 4]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!(a.crt(&Natural::from(6u32), &b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign =
    /// 0, the residues required to be reduced, and a noninvertible modulus reported as `None`
    /// rather than thrown.
    #[inline]
    fn crt(self, m1: &Natural, r2: &UnsignedMatrix<Limb>, m2: Limb) -> Option<Self> {
        crt_matrix_owned(self, m1, r2, m2)
    }
}

impl Crt<Natural, UnsignedMatrix<Limb>, Limb> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element:
    /// finds the [`NaturalMatrix`] whose every element is the unique number below `m1 * m2` that is
    /// congruent to the element of `self` in the same position modulo `m1` and to the element of
    /// `r2` modulo `m2`. `self` is taken by reference, `m1` by value, and `r2` by value.
    ///
    /// Returns `None` if the moduli are not coprime. The elements of `self` must be already reduced
    /// modulo `m1`, and those of `r2` modulo `m2`. The inverse of `m1` modulo `m2` is computed
    /// once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $0 \leq X_{ij} < m_1m_2$, $X_{ij} \equiv
    /// A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if $m_1$ and
    /// $m_2$ are coprime.
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
    /// `self` is greater than or equal to `m1`, or if any element of `r2` is greater than or equal
    /// to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).crt(Natural::from(5u32), b, 3).unwrap().to_string(),
    ///     "[7, 6; 5, 4]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!((&a).crt(Natural::from(6u32), b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign =
    /// 0, the residues required to be reduced, and a noninvertible modulus reported as `None`
    /// rather than thrown.
    #[inline]
    fn crt(self, m1: Natural, r2: UnsignedMatrix<Limb>, m2: Limb) -> Option<NaturalMatrix> {
        crt_matrix_ref(self, &m1, &r2, m2)
    }
}

impl Crt<Natural, &UnsignedMatrix<Limb>, Limb> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element:
    /// finds the [`NaturalMatrix`] whose every element is the unique number below `m1 * m2` that is
    /// congruent to the element of `self` in the same position modulo `m1` and to the element of
    /// `r2` modulo `m2`. `self` is taken by reference, `m1` by value, and `r2` by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The elements of `self` must be already reduced
    /// modulo `m1`, and those of `r2` modulo `m2`. The inverse of `m1` modulo `m2` is computed
    /// once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $0 \leq X_{ij} < m_1m_2$, $X_{ij} \equiv
    /// A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if $m_1$ and
    /// $m_2$ are coprime.
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
    /// `self` is greater than or equal to `m1`, or if any element of `r2` is greater than or equal
    /// to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).crt(Natural::from(5u32), &b, 3).unwrap().to_string(),
    ///     "[7, 6; 5, 4]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!((&a).crt(Natural::from(6u32), &b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign =
    /// 0, the residues required to be reduced, and a noninvertible modulus reported as `None`
    /// rather than thrown.
    #[inline]
    fn crt(self, m1: Natural, r2: &UnsignedMatrix<Limb>, m2: Limb) -> Option<NaturalMatrix> {
        crt_matrix_ref(self, &m1, r2, m2)
    }
}

impl Crt<&Natural, UnsignedMatrix<Limb>, Limb> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element:
    /// finds the [`NaturalMatrix`] whose every element is the unique number below `m1 * m2` that is
    /// congruent to the element of `self` in the same position modulo `m1` and to the element of
    /// `r2` modulo `m2`. `self` is taken by reference, `m1` by reference, and `r2` by value.
    ///
    /// Returns `None` if the moduli are not coprime. The elements of `self` must be already reduced
    /// modulo `m1`, and those of `r2` modulo `m2`. The inverse of `m1` modulo `m2` is computed
    /// once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $0 \leq X_{ij} < m_1m_2$, $X_{ij} \equiv
    /// A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if $m_1$ and
    /// $m_2$ are coprime.
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
    /// `self` is greater than or equal to `m1`, or if any element of `r2` is greater than or equal
    /// to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).crt(&Natural::from(5u32), b, 3).unwrap().to_string(),
    ///     "[7, 6; 5, 4]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!((&a).crt(&Natural::from(6u32), b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign =
    /// 0, the residues required to be reduced, and a noninvertible modulus reported as `None`
    /// rather than thrown.
    #[inline]
    fn crt(self, m1: &Natural, r2: UnsignedMatrix<Limb>, m2: Limb) -> Option<NaturalMatrix> {
        crt_matrix_ref(self, m1, &r2, m2)
    }
}

impl Crt<&Natural, &UnsignedMatrix<Limb>, Limb> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Combines two matrices of congruences by the Chinese remainder theorem, element by element:
    /// finds the [`NaturalMatrix`] whose every element is the unique number below `m1 * m2` that is
    /// congruent to the element of `self` in the same position modulo `m1` and to the element of
    /// `r2` modulo `m2`. `self` is taken by reference, `m1` by reference, and `r2` by reference.
    ///
    /// Returns `None` if the moduli are not coprime. The elements of `self` must be already reduced
    /// modulo `m1`, and those of `r2` modulo `m2`. The inverse of `m1` modulo `m2` is computed
    /// once, not once per element.
    ///
    /// $f(A, m_1, B, m_2) = \operatorname{Some}(X)$, where $0 \leq X_{ij} < m_1m_2$, $X_{ij} \equiv
    /// A_{ij} \mod m_1$, and $X_{ij} \equiv B_{ij} \mod m_2$ for all $i$ and $j$, if $m_1$ and
    /// $m_2$ are coprime.
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
    /// `self` is greater than or equal to `m1`, or if any element of `r2` is greater than or equal
    /// to `m2`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Crt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // In each position, the result is congruent to the element of the first matrix mod 5 and to
    /// // the element of the second matrix mod 3.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 4]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).crt(&Natural::from(5u32), &b, 3).unwrap().to_string(),
    ///     "[7, 6; 5, 4]"
    /// );
    ///
    /// // 6 and 3 are not coprime.
    /// let a = NaturalMatrix::from_str("[2, 1; 0, 5]").unwrap();
    /// let b = UnsignedMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert!((&a).crt(&Natural::from(6u32), &b, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_CRT_ui` from `fmpz_mat/CRT_ui.c`, FLINT 3.6.0, with sign =
    /// 0, the residues required to be reduced, and a noninvertible modulus reported as `None`
    /// rather than thrown.
    #[inline]
    fn crt(self, m1: &Natural, r2: &UnsignedMatrix<Limb>, m2: Limb) -> Option<NaturalMatrix> {
        crt_matrix_ref(self, m1, r2, m2)
    }
}
