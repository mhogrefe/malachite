// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModInverse, ModIsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_solve::solve_nonsingular;
use alloc::vec;

// This is equivalent to `nmod_mat_inv` from `nmod_mat/inv.c`, FLINT 3.6.0.
fn mod_inverse_helper<T: PrimitiveUnsigned>(
    a: UnsignedMatrix<T>,
    m: T,
) -> Option<UnsignedMatrix<T>> {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert!(
        a.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    let n = a.height;
    if m == T::ONE {
        // Modulo 1, every matrix is the identity, and its own inverse.
        return Some(a);
    }
    let elements = match n {
        0 => Some(vec![]),
        1 => {
            let x = a.elements[0];
            if x == T::ZERO {
                None
            } else {
                x.mod_inverse(m).map(|x| vec![x])
            }
        }
        _ => {
            let mut identity = vec![T::ZERO; n * n];
            for i in 0..n {
                identity[i * (n + 1)] = T::ONE;
            }
            solve_nonsingular(a, &identity, n, m)
        }
    };
    elements.map(|elements| UnsignedMatrix {
        elements,
        height: n,
        width: n,
    })
}

impl<T: PrimitiveUnsigned> ModInverse<T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes the inverse of a square [`UnsignedMatrix`] modulo a prime $m$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The inverse of an $n \times
    /// n$ matrix $A$ is the matrix $B$ with $AB = BA = I$ modulo $m$, where $I$ is the identity
    /// matrix; it exists if and only if the determinant of $A$ is nonzero modulo $m$, and `None` is
    /// returned otherwise. Modulo 1, every matrix is the zero matrix, which is both the identity
    /// and its own inverse.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the height of the matrix.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_inverse(7).unwrap().to_string(), "[5, 1; 5, 3]");
    ///
    /// // A singular matrix
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!(a.mod_inverse(7), None);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(a.mod_inverse(7).unwrap().to_string(), "[]_{0,0}");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_inv` from `nmod_mat/inv.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square or an element is not reduced, and that modulo 1 it
    /// returns the zero matrix, where FLINT reports that a matrix of dimension at least 1 is not
    /// invertible.
    #[inline]
    fn mod_inverse(self, m: T) -> Option<Self> {
        mod_inverse_helper(self, m)
    }
}

impl<T: PrimitiveUnsigned> ModInverse<T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes the inverse of a square [`UnsignedMatrix`] modulo a prime $m$, taking it by
    /// reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The inverse of an $n \times
    /// n$ matrix $A$ is the matrix $B$ with $AB = BA = I$ modulo $m$, where $I$ is the identity
    /// matrix; it exists if and only if the determinant of $A$ is nonzero modulo $m$, and `None` is
    /// returned otherwise. Modulo 1, every matrix is the zero matrix, which is both the identity
    /// and its own inverse.
    ///
    /// The modulus must be prime. Elimination only ever divides by a pivot, so if $m$ is composite
    /// but every pivot is a unit modulo $m$, the result is still correct.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the height of the matrix.
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_inverse(7).unwrap().to_string(), "[5, 1; 5, 3]");
    ///
    /// // A singular matrix
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 2, 4]").unwrap();
    /// assert_eq!((&a).mod_inverse(7), None);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    /// assert_eq!((&a).mod_inverse(7).unwrap().to_string(), "[]_{0,0}");
    /// ```
    ///
    /// This is equivalent to `nmod_mat_inv` from `nmod_mat/inv.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square or an element is not reduced, and that modulo 1 it
    /// returns the zero matrix, where FLINT reports that a matrix of dimension at least 1 is not
    /// invertible.
    #[inline]
    fn mod_inverse(self, m: T) -> Option<UnsignedMatrix<T>> {
        mod_inverse_helper(self.clone(), m)
    }
}
