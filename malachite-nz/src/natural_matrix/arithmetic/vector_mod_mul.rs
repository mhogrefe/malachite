// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::vector_mul::vector_matrix_mul;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModAssign, ModIsReduced, ModMul};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

// The elements of a matrix whose elements all fit in a limb, as limbs.
fn to_limb_matrix(a: &NaturalMatrix) -> UnsignedMatrix<Limb> {
    UnsignedMatrix::from_owned_elements(
        a.height(),
        a.width(),
        a.elements.iter().map(Limb::wrapping_from).collect(),
    )
}

// The elements of a vector whose elements all fit in a limb, as limbs.
fn to_limb_vector(v: &NaturalVector) -> UnsignedVector<Limb> {
    UnsignedVector::from_owned_elements(v.elements.iter().map(Limb::wrapping_from).collect())
}

// Checks the arguments of a vector-matrix product modulo `m`: `m` is positive, the vector's
// dimension is the height of the matrix, and the elements of both are reduced.
fn assert_arguments(v: &NaturalVector, a: &NaturalMatrix, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert_eq!(
        v.elements.len(),
        a.height,
        "the vector's dimension must equal the height of the matrix"
    );
    assert!(
        v.mod_is_reduced(m),
        "the vector must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        a.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// The product `v * a` modulo `m`, after checking the arguments. When `m` fits in a limb, so does
// every element, and the product is computed by the `UnsignedVector` implementation, which
// accumulates each column exactly in one to three limbs and reduces once; otherwise the exact
// product is computed and each of its elements reduced once.
fn vector_mod_mul(v: &NaturalVector, a: &NaturalMatrix, m: &Natural) -> NaturalVector {
    assert_arguments(v, a, m);
    if let Natural(Small(small_m)) = *m {
        NaturalVector::from(to_limb_vector(v).mod_mul(to_limb_matrix(a), small_m))
    } else {
        let mut elements = vector_matrix_mul(&v.elements, &a.elements, a.width);
        for x in &mut elements {
            x.mod_assign(m);
        }
        NaturalVector { elements }
    }
}

impl ModMul<NaturalMatrix, Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo a [`Natural`] $m$,
    /// taking the vector by value, the matrix by value, and the modulus by value.
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
    /// When $m$ fits in a limb, the products are summed exactly, in one, two, or three limbs, and
    /// each sum is reduced once; otherwise the exact product is computed and each of its elements
    /// reduced once.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((v).mod_mul(a, Natural::from(7u32)).to_string(), "(2, 5, 1)");
    /// ```
    #[inline]
    fn mod_mul(self, other: NaturalMatrix, m: Natural) -> Self {
        vector_mod_mul(&self, &other, &m)
    }
}

impl ModMul<NaturalMatrix, &Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo a [`Natural`] $m$,
    /// taking the vector by value, the matrix by value, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(
    ///     (v).mod_mul(a, &Natural::from(7u32)).to_string(),
    ///     "(2, 5, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: NaturalMatrix, m: &Natural) -> Self {
        vector_mod_mul(&self, &other, m)
    }
}

impl ModMul<&NaturalMatrix, Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo a [`Natural`] $m$,
    /// taking the vector by value, the matrix by reference, and the modulus by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(
    ///     (v).mod_mul(&a, Natural::from(7u32)).to_string(),
    ///     "(2, 5, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: &NaturalMatrix, m: Natural) -> Self {
        vector_mod_mul(&self, other, &m)
    }
}

impl ModMul<&NaturalMatrix, &Natural> for NaturalVector {
    type Output = Self;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo a [`Natural`] $m$,
    /// taking the vector by value, the matrix by reference, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(
    ///     (v).mod_mul(&a, &Natural::from(7u32)).to_string(),
    ///     "(2, 5, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: &NaturalMatrix, m: &Natural) -> Self {
        vector_mod_mul(&self, other, m)
    }
}

impl ModMul<NaturalMatrix, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo a [`Natural`] $m$,
    /// taking the vector by reference, the matrix by value, and the modulus by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(
    ///     (&v).mod_mul(a, Natural::from(7u32)).to_string(),
    ///     "(2, 5, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: NaturalMatrix, m: Natural) -> NaturalVector {
        vector_mod_mul(self, &other, &m)
    }
}

impl ModMul<NaturalMatrix, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo a [`Natural`] $m$,
    /// taking the vector by reference, the matrix by value, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(
    ///     (&v).mod_mul(a, &Natural::from(7u32)).to_string(),
    ///     "(2, 5, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: NaturalMatrix, m: &Natural) -> NaturalVector {
        vector_mod_mul(self, &other, m)
    }
}

impl ModMul<&NaturalMatrix, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo a [`Natural`] $m$,
    /// taking the vector by reference, the matrix by reference, and the modulus by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(
    ///     (&v).mod_mul(&a, Natural::from(7u32)).to_string(),
    ///     "(2, 5, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: &NaturalMatrix, m: Natural) -> NaturalVector {
        vector_mod_mul(self, other, &m)
    }
}

impl ModMul<&NaturalMatrix, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right modulo a [`Natural`] $m$,
    /// taking the vector by reference, the matrix by reference, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the dimension of `self` is not the height of `other`, or if any
    /// element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(
    ///     (&v).mod_mul(&a, &Natural::from(7u32)).to_string(),
    ///     "(2, 5, 1)"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: &NaturalMatrix, m: &Natural) -> NaturalVector {
        vector_mod_mul(self, other, m)
    }
}
