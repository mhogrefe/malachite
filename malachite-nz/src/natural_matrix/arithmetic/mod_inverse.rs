// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
//      Copyright © 2019 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_solve::solve_nonsingular;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use alloc::borrow::Cow;
use alloc::vec;
use malachite_base::num::arithmetic::traits::{ModInverse, ModIsReduced};
use malachite_base::num::basic::traits::{One, Zero};

// This is equivalent to `fmpz_mod_mat_inv` from `fmpz_mod_mat/inv.c`, FLINT 3.6.0.
fn mod_inverse_helper(a: Cow<'_, NaturalMatrix>, m: &Natural) -> Option<NaturalMatrix> {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert!(
        a.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    if let Natural(Small(small_m)) = *m {
        return to_limb_matrix(&a)
            .mod_inverse(small_m)
            .map(NaturalMatrix::from);
    }
    let n = a.height;
    let elements = match n {
        0 => Some(vec![]),
        1 => {
            let x = &a.elements[0];
            if *x == 0u32 {
                None
            } else {
                x.mod_inverse(m).map(|x| vec![x])
            }
        }
        _ => {
            let mut identity = vec![Natural::ZERO; n * n];
            for i in 0..n {
                identity[i * (n + 1)] = Natural::ONE;
            }
            solve_nonsingular(a.into_owned(), Cow::Owned(identity), n, m)
        }
    };
    elements.map(|elements| NaturalMatrix {
        elements,
        height: n,
        width: n,
    })
}

impl ModInverse<Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the inverse of a square [`NaturalMatrix`] modulo a prime $m$, taking the matrix by
    /// value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The inverse of an $n \times
    /// n$ matrix $A$ is the matrix $B$ with $AB = BA = I$ modulo $m$, where $I$ is the identity
    /// matrix; it exists if and only if the determinant of $A$ is a unit modulo $m$, and `None` is
    /// returned otherwise. Modulo 1, every matrix is the zero matrix, which is both the identity
    /// and its own inverse.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the inverse is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^3 b \log b \log \log b)$
    ///
    /// $M(n, b) = O(n^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, if any element of the matrix is greater
    /// than or equal to `m`, or if a pivot is not invertible modulo `m`, which can only happen if
    /// `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_inverse(Natural::from(7u32)).unwrap().to_string(),
    ///     "[5, 1; 5, 3]"
    /// );
    ///
    /// // A singular matrix
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(a.mod_inverse(Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_inv` from `fmpz_mod_mat/inv.c`, FLINT 3.6.0, except that
    /// it panics if the matrix is not square, an element is not reduced, or a pivot is not
    /// invertible, where FLINT reports that the matrix is not invertible, and that modulo 1 it
    /// returns the zero matrix.
    #[inline]
    fn mod_inverse(self, m: Natural) -> Option<Self> {
        mod_inverse_helper(Cow::Owned(self), &m)
    }
}

impl ModInverse<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the inverse of a square [`NaturalMatrix`] modulo a prime $m$, taking the matrix by
    /// value and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The inverse of an $n \times
    /// n$ matrix $A$ is the matrix $B$ with $AB = BA = I$ modulo $m$, where $I$ is the identity
    /// matrix; it exists if and only if the determinant of $A$ is a unit modulo $m$, and `None` is
    /// returned otherwise. Modulo 1, every matrix is the zero matrix, which is both the identity
    /// and its own inverse.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the inverse is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^3 b \log b \log \log b)$
    ///
    /// $M(n, b) = O(n^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, if any element of the matrix is greater
    /// than or equal to `m`, or if a pivot is not invertible modulo `m`, which can only happen if
    /// `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_inverse(&Natural::from(7u32)).unwrap().to_string(),
    ///     "[5, 1; 5, 3]"
    /// );
    ///
    /// // A singular matrix
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(a.mod_inverse(&Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_inv` from `fmpz_mod_mat/inv.c`, FLINT 3.6.0, except that
    /// it panics if the matrix is not square, an element is not reduced, or a pivot is not
    /// invertible, where FLINT reports that the matrix is not invertible, and that modulo 1 it
    /// returns the zero matrix.
    #[inline]
    fn mod_inverse(self, m: &Natural) -> Option<Self> {
        mod_inverse_helper(Cow::Owned(self), m)
    }
}

impl ModInverse<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the inverse of a square [`NaturalMatrix`] modulo a prime $m$, taking the matrix by
    /// reference and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The inverse of an $n \times
    /// n$ matrix $A$ is the matrix $B$ with $AB = BA = I$ modulo $m$, where $I$ is the identity
    /// matrix; it exists if and only if the determinant of $A$ is a unit modulo $m$, and `None` is
    /// returned otherwise. Modulo 1, every matrix is the zero matrix, which is both the identity
    /// and its own inverse.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the inverse is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^3 b \log b \log \log b)$
    ///
    /// $M(n, b) = O(n^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, if any element of the matrix is greater
    /// than or equal to `m`, or if a pivot is not invertible modulo `m`, which can only happen if
    /// `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_inverse(Natural::from(7u32)).unwrap().to_string(),
    ///     "[5, 1; 5, 3]"
    /// );
    ///
    /// // A singular matrix
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!((&a).mod_inverse(Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_inv` from `fmpz_mod_mat/inv.c`, FLINT 3.6.0, except that
    /// it panics if the matrix is not square, an element is not reduced, or a pivot is not
    /// invertible, where FLINT reports that the matrix is not invertible, and that modulo 1 it
    /// returns the zero matrix.
    #[inline]
    fn mod_inverse(self, m: Natural) -> Option<NaturalMatrix> {
        mod_inverse_helper(Cow::Borrowed(self), &m)
    }
}

impl ModInverse<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the inverse of a square [`NaturalMatrix`] modulo a prime $m$, taking the matrix by
    /// reference and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The inverse of an $n \times
    /// n$ matrix $A$ is the matrix $B$ with $AB = BA = I$ modulo $m$, where $I$ is the identity
    /// matrix; it exists if and only if the determinant of $A$ is a unit modulo $m$, and `None` is
    /// returned otherwise. Modulo 1, every matrix is the zero matrix, which is both the identity
    /// and its own inverse.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct. When $m$ fits in a limb,
    /// the inverse is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^3 b \log b \log \log b)$
    ///
    /// $M(n, b) = O(n^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, if any element of the matrix is greater
    /// than or equal to `m`, or if a pivot is not invertible modulo `m`, which can only happen if
    /// `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_inverse(&Natural::from(7u32)).unwrap().to_string(),
    ///     "[5, 1; 5, 3]"
    /// );
    ///
    /// // A singular matrix
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!((&a).mod_inverse(&Natural::from(7u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_inv` from `fmpz_mod_mat/inv.c`, FLINT 3.6.0, except that
    /// it panics if the matrix is not square, an element is not reduced, or a pivot is not
    /// invertible, where FLINT reports that the matrix is not invertible, and that modulo 1 it
    /// returns the zero matrix.
    #[inline]
    fn mod_inverse(self, m: &Natural) -> Option<NaturalMatrix> {
        mod_inverse_helper(Cow::Borrowed(self), m)
    }
}
