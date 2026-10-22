// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011, 2022 Fredrik Johansson
//
//      Copyright © 2013 Mike Hansen
//
//      Copyright © 2021 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_lu_decomposition::assert_reduced;
use crate::natural_matrix::arithmetic::mod_reduced_row_echelon_form::rref_in_place;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModNeg, ModNullspace};
use malachite_base::num::basic::traits::{One, Zero};

// Returns the elements of a `width` by `nullity` matrix whose columns form a basis of the right
// nullspace of `a` modulo the prime `m`, which does not fit in a limb, and the nullity. `a` is
// consumed as scratch space.
//
// This is equivalent to `gr_mat_nullspace_no_resize` and `gr_mat_nullspace_from_rref` from
// `gr_mat/nullspace.c`, FLINT 3.6.0, which `fmpz_mod_mat_nullspace` calls, returning only the first
// `nullity` columns of FLINT's `width` by `width` result.
pub(crate) fn nullspace_in_place(a: &mut NaturalMatrix, m: &Natural) -> (Vec<Natural>, usize) {
    let width = a.width;
    let (rank, pivots_nonpivots) = rref_in_place(a, m);
    let nullity = width - rank;
    let mut x = vec![Natural::ZERO; width * nullity];
    if rank == 0 {
        for i in 0..nullity {
            x[i * (nullity + 1)] = Natural::ONE;
        }
    } else if nullity != 0 {
        let xs = &mut a.elements;
        let (pivots, nonpivots) = pivots_nonpivots.split_at(rank);
        for (i, &c) in nonpivots.iter().enumerate() {
            for (j, &p) in pivots.iter().enumerate() {
                x[p * nullity + i] = core::mem::take(&mut xs[j * width + c]).mod_neg(m);
            }
            x[c * nullity + i] = Natural::ONE;
        }
    }
    (x, nullity)
}

fn mod_nullspace_helper(a: Cow<'_, NaturalMatrix>, m: &Natural) -> NaturalMatrix {
    assert_reduced(&a, m);
    if let Natural(Small(small_m)) = *m {
        return NaturalMatrix::from(to_limb_matrix(&a).mod_nullspace(small_m));
    }
    let mut a = a.into_owned();
    let width = a.width;
    let (elements, nullity) = nullspace_in_place(&mut a, m);
    NaturalMatrix {
        elements,
        height: width,
        width: nullity,
    }
}

impl ModNullspace<Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes a basis of the right nullspace of a [`NaturalMatrix`] modulo a prime $m$, taking
    /// the matrix by value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The right nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $w$ with $Ax = 0$
    /// modulo $m$; it has dimension $w - r$. The result is the $w \times (w - r)$ matrix whose
    /// columns are the basis vectors obtained from the reduced row echelon form $R$ of $A$: for
    /// each column $c$ of $R$ that does not contain a pivot, the vector with 1 in position $c$, the
    /// negation of $R_{i,c}$ in the position of the pivot of row $i$ of $R$, and 0 elsewhere.
    /// Modulo 1, where every element is 0, the basis vectors are 0.
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
    /// $M(h, w, b) = O((h + w)wb)$
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
    /// use malachite_base::num::arithmetic::traits::ModNullspace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_nullspace(Natural::from(7u32)).to_string(),
    ///     "[6; 6; 1]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_nullspace(m).to_string(),
    ///     "[170141183460469231731687303715884105725; 1]"
    /// );
    ///
    /// // A matrix of full column rank has a trivial nullspace.
    /// let a = NaturalMatrix::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!(a.mod_nullspace(Natural::from(5u32)).to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_nullspace` from `fmpz_mod_mat/nullspace.c`, FLINT 3.6.0,
    /// except that only the $w - r$ basis vectors are returned rather than a $w \times w$ matrix
    /// and the nullity, and that it panics if an element is not reduced or a pivot is not
    /// invertible, where FLINT aborts.
    #[inline]
    fn mod_nullspace(self, m: Natural) -> Self {
        mod_nullspace_helper(Cow::Owned(self), &m)
    }
}

impl ModNullspace<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes a basis of the right nullspace of a [`NaturalMatrix`] modulo a prime $m$, taking
    /// the matrix by value and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The right nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $w$ with $Ax = 0$
    /// modulo $m$; it has dimension $w - r$. The result is the $w \times (w - r)$ matrix whose
    /// columns are the basis vectors obtained from the reduced row echelon form $R$ of $A$: for
    /// each column $c$ of $R$ that does not contain a pivot, the vector with 1 in position $c$, the
    /// negation of $R_{i,c}$ in the position of the pivot of row $i$ of $R$, and 0 elsewhere.
    /// Modulo 1, where every element is 0, the basis vectors are 0.
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
    /// $M(h, w, b) = O((h + w)wb)$
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
    /// use malachite_base::num::arithmetic::traits::ModNullspace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_nullspace(&Natural::from(7u32)).to_string(),
    ///     "[6; 6; 1]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_nullspace(&m).to_string(),
    ///     "[170141183460469231731687303715884105725; 1]"
    /// );
    ///
    /// // A matrix of full column rank has a trivial nullspace.
    /// let a = NaturalMatrix::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!(
    ///     a.mod_nullspace(&Natural::from(5u32)).to_string(),
    ///     "[]_{2,0}"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_nullspace` from `fmpz_mod_mat/nullspace.c`, FLINT 3.6.0,
    /// except that only the $w - r$ basis vectors are returned rather than a $w \times w$ matrix
    /// and the nullity, and that it panics if an element is not reduced or a pivot is not
    /// invertible, where FLINT aborts.
    #[inline]
    fn mod_nullspace(self, m: &Natural) -> Self {
        mod_nullspace_helper(Cow::Owned(self), m)
    }
}

impl ModNullspace<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes a basis of the right nullspace of a [`NaturalMatrix`] modulo a prime $m$, taking
    /// the matrix by reference and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The right nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $w$ with $Ax = 0$
    /// modulo $m$; it has dimension $w - r$. The result is the $w \times (w - r)$ matrix whose
    /// columns are the basis vectors obtained from the reduced row echelon form $R$ of $A$: for
    /// each column $c$ of $R$ that does not contain a pivot, the vector with 1 in position $c$, the
    /// negation of $R_{i,c}$ in the position of the pivot of row $i$ of $R$, and 0 elsewhere.
    /// Modulo 1, where every element is 0, the basis vectors are 0.
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
    /// $M(h, w, b) = O((h + w)wb)$
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
    /// use malachite_base::num::arithmetic::traits::ModNullspace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_nullspace(Natural::from(7u32)).to_string(),
    ///     "[6; 6; 1]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_nullspace(m).to_string(),
    ///     "[170141183460469231731687303715884105725; 1]"
    /// );
    ///
    /// // A matrix of full column rank has a trivial nullspace.
    /// let a = NaturalMatrix::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_nullspace(Natural::from(5u32)).to_string(),
    ///     "[]_{2,0}"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_nullspace` from `fmpz_mod_mat/nullspace.c`, FLINT 3.6.0,
    /// except that only the $w - r$ basis vectors are returned rather than a $w \times w$ matrix
    /// and the nullity, and that it panics if an element is not reduced or a pivot is not
    /// invertible, where FLINT aborts.
    #[inline]
    fn mod_nullspace(self, m: Natural) -> NaturalMatrix {
        mod_nullspace_helper(Cow::Borrowed(self), &m)
    }
}

impl ModNullspace<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes a basis of the right nullspace of a [`NaturalMatrix`] modulo a prime $m$, taking
    /// the matrix by reference and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The right nullspace of an $h
    /// \times w$ matrix $A$ of rank $r$ is the space of vectors $x$ of length $w$ with $Ax = 0$
    /// modulo $m$; it has dimension $w - r$. The result is the $w \times (w - r)$ matrix whose
    /// columns are the basis vectors obtained from the reduced row echelon form $R$ of $A$: for
    /// each column $c$ of $R$ that does not contain a pivot, the vector with 1 in position $c$, the
    /// negation of $R_{i,c}$ in the position of the pivot of row $i$ of $R$, and 0 elsewhere.
    /// Modulo 1, where every element is 0, the basis vectors are 0.
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
    /// $M(h, w, b) = O((h + w)wb)$
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
    /// use malachite_base::num::arithmetic::traits::ModNullspace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 2, 4, 6; 1, 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_nullspace(&Natural::from(7u32)).to_string(),
    ///     "[6; 6; 1]"
    /// );
    ///
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_nullspace(&m).to_string(),
    ///     "[170141183460469231731687303715884105725; 1]"
    /// );
    ///
    /// // A matrix of full column rank has a trivial nullspace.
    /// let a = NaturalMatrix::from_str("[2, 1; 4, 3]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_nullspace(&Natural::from(5u32)).to_string(),
    ///     "[]_{2,0}"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_nullspace` from `fmpz_mod_mat/nullspace.c`, FLINT 3.6.0,
    /// except that only the $w - r$ basis vectors are returned rather than a $w \times w$ matrix
    /// and the nullity, and that it panics if an element is not reduced or a pivot is not
    /// invertible, where FLINT aborts.
    #[inline]
    fn mod_nullspace(self, m: &Natural) -> NaturalMatrix {
        mod_nullspace_helper(Cow::Borrowed(self), m)
    }
}
