// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModMul};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::UnsignedVector;
use crate::unsigned_vector::arithmetic::mod_dot::{ModData, accumulate};
use alloc::vec;

// Checks the arguments of a vector-matrix product modulo `m`: `m` is positive, the vector's
// dimension is the height of the matrix, and the elements of both are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, a: &UnsignedMatrix<T>, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert_eq!(
        v.elements.len(),
        a.height,
        "the vector's dimension must equal the height of the matrix"
    );
    assert!(
        v.mod_is_reduced(&m),
        "the vector must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        a.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// The product `v * a` modulo `m`, after checking the arguments. Each row of the matrix, scaled by
// the matching element of the vector, is added to one exact accumulator per column, so the matrix
// is read in storage order; as for dot products, the accumulators use one, two, or three words,
// depending on how large the sums can get, and each is reduced once. With three words, the
// accumulators are reduced after every `max_terms` rows, so they cannot overflow.
fn vector_mod_mul<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedVector<T> {
    assert_arguments(v, a, m);
    let width = a.width;
    if width == 0 || a.height == 0 {
        return UnsignedVector {
            elements: vec![T::ZERO; width],
        };
    }
    let d = ModData::new(m, a.height);
    let rows = v.elements.iter().zip(a.elements.chunks_exact(width));
    let elements = match d.words {
        1 => {
            let mut out = vec![T::ZERO; width];
            for (&x, row) in rows {
                if x != T::ZERO {
                    for (o, &y) in out.iter_mut().zip(row) {
                        o.wrapping_add_assign(x.wrapping_mul(y));
                    }
                }
            }
            for o in &mut out {
                *o %= m;
            }
            out
        }
        2 => {
            let mut out = vec![(T::ZERO, T::ZERO); width];
            for (&x, row) in rows {
                if x != T::ZERO {
                    for ((hi, lo), &y) in out.iter_mut().zip(row) {
                        let (p_hi, p_lo) = T::x_mul_y_to_zz(x, y);
                        (*hi, *lo) = T::xx_add_yy_to_zz(*hi, *lo, p_hi, p_lo);
                    }
                }
            }
            out.into_iter().map(|(hi, lo)| d.reduce_2(hi, lo)).collect()
        }
        _ => {
            let mut out = vec![(T::ZERO, T::ZERO, T::ZERO); width];
            for (i, (xs, block)) in v
                .elements
                .chunks(d.max_terms)
                .zip(a.elements.chunks(d.max_terms.saturating_mul(width)))
                .enumerate()
            {
                if i != 0 {
                    for acc in &mut out {
                        *acc = (T::ZERO, T::ZERO, d.reduce(acc.0, acc.1, acc.2));
                    }
                }
                for (&x, row) in xs.iter().zip(block.chunks_exact(width)) {
                    if x != T::ZERO {
                        for (acc, &y) in out.iter_mut().zip(row) {
                            accumulate(acc, x, y);
                        }
                    }
                }
            }
            out.into_iter()
                .map(|(x2, x1, x0)| d.reduce(x2, x1, x0))
                .collect()
        }
    };
    UnsignedVector { elements }
}

impl<T: PrimitiveUnsigned> ModMul<UnsignedMatrix<T>, T> for UnsignedVector<T> {
    type Output = Self;

    /// Multiplies an [`UnsignedVector`] by an [`UnsignedMatrix`] on the right modulo a number $m$,
    /// taking the vector by value and the matrix by value.
    ///
    /// The elements of both must already be reduced modulo $m$. Element $j$ of the result is the
    /// dot product of the vector with column $j$ of the matrix, modulo $m$:
    ///
    /// $$
    /// f(v, A, m)_j = \sum_{i=0}^{n-1} v_iA_{ij} \bmod m,
    /// $$
    ///
    /// where $n$ is the height of $A$, which must be the dimension of $v$.
    ///
    /// The products are summed exactly, in one, two, or three words depending on how large the sums
    /// can get, and each sum is reduced once.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((v).mod_mul(a, 7).to_string(), "(2, 5, 1)");
    /// ```
    #[inline]
    fn mod_mul(self, other: UnsignedMatrix<T>, m: T) -> Self {
        vector_mod_mul(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<&UnsignedMatrix<T>, T> for UnsignedVector<T> {
    type Output = Self;

    /// Multiplies an [`UnsignedVector`] by an [`UnsignedMatrix`] on the right modulo a number $m$,
    /// taking the vector by value and the matrix by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((v).mod_mul(&a, 7).to_string(), "(2, 5, 1)");
    /// ```
    #[inline]
    fn mod_mul(self, other: &UnsignedMatrix<T>, m: T) -> Self {
        vector_mod_mul(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<UnsignedMatrix<T>, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedVector`] by an [`UnsignedMatrix`] on the right modulo a number $m$,
    /// taking the vector by reference and the matrix by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((&v).mod_mul(a, 7).to_string(), "(2, 5, 1)");
    /// ```
    #[inline]
    fn mod_mul(self, other: UnsignedMatrix<T>, m: T) -> UnsignedVector<T> {
        vector_mod_mul(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<&UnsignedMatrix<T>, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies an [`UnsignedVector`] by an [`UnsignedMatrix`] on the right modulo a number $m$,
    /// taking the vector by reference and the matrix by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((&v).mod_mul(&a, 7).to_string(), "(2, 5, 1)");
    /// ```
    #[inline]
    fn mod_mul(self, other: &UnsignedMatrix<T>, m: T) -> UnsignedVector<T> {
        vector_mod_mul(self, other, m)
    }
}
