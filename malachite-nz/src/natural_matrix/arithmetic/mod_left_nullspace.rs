// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::arithmetic::mod_lu_decomposition::assert_reduced;
use crate::natural_matrix::arithmetic::mod_nullspace::nullspace_in_place;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use crate::natural_matrix::{NaturalMatrix, transposed_elements};
use alloc::borrow::Cow;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::ModLeftNullspace;

// The left nullspace of A is the right nullspace of the transpose of A, with the basis vectors
// written as rows.
fn mod_left_nullspace_helper(a: Cow<'_, NaturalMatrix>, m: &Natural) -> NaturalMatrix {
    assert_reduced(&a, m);
    if let Natural(Small(small_m)) = *m {
        return NaturalMatrix::from(to_limb_matrix(&a).mod_left_nullspace(small_m));
    }
    let height = a.height;
    let mut at = a.into_owned();
    at.transpose_assign();
    let (elements, nullity) = nullspace_in_place(&mut at, m);
    NaturalMatrix {
        elements: transposed_elements(elements, height, nullity),
        height: nullity,
        width: height,
    }
}

impl ModLeftNullspace<Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes a basis of the left nullspace of a [`NaturalMatrix`] modulo a prime $m$, taking the
    /// matrix by value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The left nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $h$ with $xA = 0$
    /// modulo $m$; it has dimension $h - r$. The result is the $(h - r) \times h$ matrix whose rows
    /// are the basis vectors of the right nullspace of the transpose of $A$, as computed by
    /// [`ModNullspace`](malachite_base::num::arithmetic::traits::ModNullspace). Modulo 1, where
    /// every element is 0, the basis vectors are 0.
    ///
    /// When $m$ fits in a limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O((h + w)hb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModLeftNullspace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1; 2; 3]").unwrap();
    /// assert_eq!(
    ///     a.mod_left_nullspace(Natural::from(5u32)).to_string(),
    ///     "[3, 1, 0; 2, 0, 1]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_left_nullspace(m).to_string(),
    ///     "[170141183460469231731687303715884105725, 1]"
    /// );
    /// ```
    ///
    /// FLINT 3.6.0 has no `fmpz_mod_mat` function for the left nullspace; this is the counterpart
    /// of `nmod_mat_left_nullspace` from `nmod_mat/left_nullspace.c`.
    #[inline]
    fn mod_left_nullspace(self, m: Natural) -> Self {
        mod_left_nullspace_helper(Cow::Owned(self), &m)
    }
}

impl ModLeftNullspace<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes a basis of the left nullspace of a [`NaturalMatrix`] modulo a prime $m$, taking the
    /// matrix by value and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The left nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $h$ with $xA = 0$
    /// modulo $m$; it has dimension $h - r$. The result is the $(h - r) \times h$ matrix whose rows
    /// are the basis vectors of the right nullspace of the transpose of $A$, as computed by
    /// [`ModNullspace`](malachite_base::num::arithmetic::traits::ModNullspace). Modulo 1, where
    /// every element is 0, the basis vectors are 0.
    ///
    /// When $m$ fits in a limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O((h + w)hb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModLeftNullspace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1; 2; 3]").unwrap();
    /// assert_eq!(
    ///     a.mod_left_nullspace(&Natural::from(5u32)).to_string(),
    ///     "[3, 1, 0; 2, 0, 1]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_left_nullspace(&m).to_string(),
    ///     "[170141183460469231731687303715884105725, 1]"
    /// );
    /// ```
    ///
    /// FLINT 3.6.0 has no `fmpz_mod_mat` function for the left nullspace; this is the counterpart
    /// of `nmod_mat_left_nullspace` from `nmod_mat/left_nullspace.c`.
    #[inline]
    fn mod_left_nullspace(self, m: &Natural) -> Self {
        mod_left_nullspace_helper(Cow::Owned(self), m)
    }
}

impl ModLeftNullspace<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes a basis of the left nullspace of a [`NaturalMatrix`] modulo a prime $m$, taking the
    /// matrix by reference and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The left nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $h$ with $xA = 0$
    /// modulo $m$; it has dimension $h - r$. The result is the $(h - r) \times h$ matrix whose rows
    /// are the basis vectors of the right nullspace of the transpose of $A$, as computed by
    /// [`ModNullspace`](malachite_base::num::arithmetic::traits::ModNullspace). Modulo 1, where
    /// every element is 0, the basis vectors are 0.
    ///
    /// When $m$ fits in a limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O((h + w)hb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModLeftNullspace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1; 2; 3]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_left_nullspace(Natural::from(5u32)).to_string(),
    ///     "[3, 1, 0; 2, 0, 1]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_left_nullspace(m).to_string(),
    ///     "[170141183460469231731687303715884105725, 1]"
    /// );
    /// ```
    ///
    /// FLINT 3.6.0 has no `fmpz_mod_mat` function for the left nullspace; this is the counterpart
    /// of `nmod_mat_left_nullspace` from `nmod_mat/left_nullspace.c`.
    #[inline]
    fn mod_left_nullspace(self, m: Natural) -> NaturalMatrix {
        mod_left_nullspace_helper(Cow::Borrowed(self), &m)
    }
}

impl ModLeftNullspace<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes a basis of the left nullspace of a [`NaturalMatrix`] modulo a prime $m$, taking the
    /// matrix by reference and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The left nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $h$ with $xA = 0$
    /// modulo $m$; it has dimension $h - r$. The result is the $(h - r) \times h$ matrix whose rows
    /// are the basis vectors of the right nullspace of the transpose of $A$, as computed by
    /// [`ModNullspace`](malachite_base::num::arithmetic::traits::ModNullspace). Modulo 1, where
    /// every element is 0, the basis vectors are 0.
    ///
    /// When $m$ fits in a limb, the result is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw\min(h, w) b \log b \log \log b)$
    ///
    /// $M(h, w, b) = O((h + w)hb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any element of the matrix is greater than or equal to `m`, or if a
    /// pivot is not invertible modulo `m`, which can only happen if `m` is composite.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModLeftNullspace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1; 2; 3]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_left_nullspace(&Natural::from(5u32)).to_string(),
    ///     "[3, 1, 0; 2, 0, 1]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_left_nullspace(&m).to_string(),
    ///     "[170141183460469231731687303715884105725, 1]"
    /// );
    /// ```
    ///
    /// FLINT 3.6.0 has no `fmpz_mod_mat` function for the left nullspace; this is the counterpart
    /// of `nmod_mat_left_nullspace` from `nmod_mat/left_nullspace.c`.
    #[inline]
    fn mod_left_nullspace(self, m: &Natural) -> NaturalMatrix {
        mod_left_nullspace_helper(Cow::Borrowed(self), m)
    }
}
