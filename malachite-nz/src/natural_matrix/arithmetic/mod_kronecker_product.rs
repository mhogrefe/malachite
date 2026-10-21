// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModKroneckerProduct, ModKroneckerProductAssign, ModMul,
};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the arguments of a Kronecker product modulo `m`: `m` is positive, and the elements of both
// matrices are reduced.
fn assert_arguments(a: &NaturalMatrix, b: &NaturalMatrix, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert!(
        a.mod_is_reduced(m),
        "the first matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        b.mod_is_reduced(m),
        "the second matrix must be reduced mod m, but {b} has an element >= {m}"
    );
}

// The elements of a matrix whose elements all fit in a limb, as limbs.
fn to_limb_matrix(a: &NaturalMatrix) -> UnsignedMatrix<Limb> {
    UnsignedMatrix::from_owned_elements(
        a.height(),
        a.width(),
        a.elements.iter().map(Limb::wrapping_from).collect(),
    )
}

// The Kronecker product of `a` and `b` modulo `m`, after checking the arguments. When `m` fits in a
// limb, so does every element, and the product is that of the matrices as `UnsignedMatrix`es of
// limbs; otherwise each element is computed with `Natural`'s `ModMul`, one row at a time, in
// storage order.
fn mod_kronecker_product(a: &NaturalMatrix, b: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    assert_arguments(a, b, m);
    if let Natural(Small(small_m)) = *m {
        return NaturalMatrix::from(
            to_limb_matrix(a).mod_kronecker_product(to_limb_matrix(b), small_m),
        );
    }
    let mut elements = Vec::with_capacity(a.elements.len() * b.elements.len());
    if a.width != 0 && b.width != 0 {
        for a_row in a.elements.chunks_exact(a.width) {
            for b_row in b.elements.chunks_exact(b.width) {
                for x in a_row {
                    for y in b_row {
                        elements.push(x.mod_mul(y, m));
                    }
                }
            }
        }
    }
    NaturalMatrix {
        elements,
        height: a.height * b.height,
        width: a.width * b.width,
    }
}

impl ModKroneckerProduct<Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $m$, taking the first by
    /// value, the second by value, and the modulus by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. The Kronecker product $A
    /// \otimes B$ of an $h_A \times w_A$ matrix $A$ and a $p \times q$ matrix $B$ is the $h_Ap
    /// \times w_Aq$ block matrix whose block in row $i$ and column $j$ is $A_{ij}B$; this is that
    /// product with every element reduced modulo $m$:
    ///
    /// $$
    /// (A \otimes B \bmod m)_{ip + r, jq + s} = A_{ij}B_{rs} \bmod m.
    /// $$
    ///
    /// If either matrix has no elements, neither does the product, though its shape is still $h_Ap
    /// \times w_Aq$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(b, Natural::from(9u32)).to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[254, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(b, Natural::from(255u32))
    ///         .to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[10000000000000000001, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007; 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(b, m).to_string(),
    ///     "[80000000000000000007, 30000000000000000021; 50000000000000000005, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: Self, m: Natural) -> Self {
        mod_kronecker_product(&self, &other, &m)
    }
}

impl ModKroneckerProduct<Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $m$, taking the first by
    /// value, the second by value, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(b, &Natural::from(9u32)).to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[254, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(b, &Natural::from(255u32))
    ///         .to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[10000000000000000001, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007; 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(b, &m).to_string(),
    ///     "[80000000000000000007, 30000000000000000021; 50000000000000000005, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: Self, m: &Natural) -> Self {
        mod_kronecker_product(&self, &other, m)
    }
}

impl ModKroneckerProduct<&Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $m$, taking the first by
    /// value, the second by reference, and the modulus by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(&b, Natural::from(9u32)).to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[254, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(&b, Natural::from(255u32))
    ///         .to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[10000000000000000001, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007; 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(&b, m).to_string(),
    ///     "[80000000000000000007, 30000000000000000021; 50000000000000000005, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: &Self, m: Natural) -> Self {
        mod_kronecker_product(&self, other, &m)
    }
}

impl ModKroneckerProduct<&Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $m$, taking the first by
    /// value, the second by reference, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(&b, &Natural::from(9u32))
    ///         .to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[254, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(&b, &Natural::from(255u32))
    ///         .to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[10000000000000000001, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007; 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_kronecker_product(&b, &m).to_string(),
    ///     "[80000000000000000007, 30000000000000000021; 50000000000000000005, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: &Self, m: &Natural) -> Self {
        mod_kronecker_product(&self, other, m)
    }
}

impl ModKroneckerProduct<NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $m$, taking the first by
    /// reference, the second by value, and the modulus by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(b, Natural::from(9u32))
    ///         .to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[254, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(b, Natural::from(255u32))
    ///         .to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[10000000000000000001, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007; 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(b, m).to_string(),
    ///     "[80000000000000000007, 30000000000000000021; 50000000000000000005, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: NaturalMatrix, m: Natural) -> NaturalMatrix {
        mod_kronecker_product(self, &other, &m)
    }
}

impl ModKroneckerProduct<NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $m$, taking the first by
    /// reference, the second by value, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(b, &Natural::from(9u32))
    ///         .to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[254, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(b, &Natural::from(255u32))
    ///         .to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[10000000000000000001, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007; 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(b, &m).to_string(),
    ///     "[80000000000000000007, 30000000000000000021; 50000000000000000005, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: NaturalMatrix, m: &Natural) -> NaturalMatrix {
        mod_kronecker_product(self, &other, m)
    }
}

impl ModKroneckerProduct<&NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $m$, taking the first by
    /// reference, the second by reference, and the modulus by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(&b, Natural::from(9u32))
    ///         .to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[254, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(&b, Natural::from(255u32))
    ///         .to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[10000000000000000001, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007; 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(&b, m).to_string(),
    ///     "[80000000000000000007, 30000000000000000021; 50000000000000000005, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: &NaturalMatrix, m: Natural) -> NaturalMatrix {
        mod_kronecker_product(self, other, &m)
    }
}

impl ModKroneckerProduct<&NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Kronecker product of two [`NaturalMatrix`]s modulo $m$, taking the first by
    /// reference, the second by reference, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProduct;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(&b, &Natural::from(9u32))
    ///         .to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[254, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[3; 128]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(&b, &Natural::from(255u32))
    ///         .to_string(),
    ///     "[252, 6; 127, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[10000000000000000001, 3]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007; 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_kronecker_product(&b, &m).to_string(),
    ///     "[80000000000000000007, 30000000000000000021; 50000000000000000005, 15]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product(self, other: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
        mod_kronecker_product(self, other, m)
    }
}

impl ModKroneckerProductAssign<Self, Natural> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Kronecker product with another [`NaturalMatrix`]
    /// modulo $m$, taking the second by value, and the modulus by value.
    ///
    /// See the documentation for the [`ModKroneckerProduct`] implementation that takes all
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProductAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// a.mod_kronecker_product_assign(b, Natural::from(9u32));
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let mut a = NaturalMatrix::from_str("[10000000000000000001]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007, 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_kronecker_product_assign(b, m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[80000000000000000007, 50000000000000000005]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product_assign(&mut self, other: Self, m: Natural) {
        *self = mod_kronecker_product(self, &other, &m);
    }
}

impl ModKroneckerProductAssign<Self, &Natural> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Kronecker product with another [`NaturalMatrix`]
    /// modulo $m$, taking the second by value, and the modulus by reference.
    ///
    /// See the documentation for the [`ModKroneckerProduct`] implementation that takes all
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProductAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// a.mod_kronecker_product_assign(b, &Natural::from(9u32));
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let mut a = NaturalMatrix::from_str("[10000000000000000001]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007, 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_kronecker_product_assign(b, &m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[80000000000000000007, 50000000000000000005]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product_assign(&mut self, other: Self, m: &Natural) {
        *self = mod_kronecker_product(self, &other, m);
    }
}

impl ModKroneckerProductAssign<&Self, Natural> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Kronecker product with another [`NaturalMatrix`]
    /// modulo $m$, taking the second by reference, and the modulus by value.
    ///
    /// See the documentation for the [`ModKroneckerProduct`] implementation that takes all
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProductAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// a.mod_kronecker_product_assign(&b, Natural::from(9u32));
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let mut a = NaturalMatrix::from_str("[10000000000000000001]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007, 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_kronecker_product_assign(&b, m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[80000000000000000007, 50000000000000000005]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product_assign(&mut self, other: &Self, m: Natural) {
        *self = mod_kronecker_product(self, other, &m);
    }
}

impl ModKroneckerProductAssign<&Self, &Natural> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Kronecker product with another [`NaturalMatrix`]
    /// modulo $m$, taking the second by reference, and the modulus by reference.
    ///
    /// See the documentation for the [`ModKroneckerProduct`] implementation that takes all
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of either matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModKroneckerProductAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 5; 6, 7]").unwrap();
    /// a.mod_kronecker_product_assign(&b, &Natural::from(9u32));
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[0, 5, 0, 1; 6, 7, 3, 5; 0, 6, 0, 2; 0, 3, 6, 1]"
    /// );
    ///
    /// let mut a = NaturalMatrix::from_str("[10000000000000000001]").unwrap();
    /// let b = NaturalMatrix::from_str("[10000000000000000007, 5]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_kronecker_product_assign(&b, &m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[80000000000000000007, 50000000000000000005]"
    /// );
    /// ```
    #[inline]
    fn mod_kronecker_product_assign(&mut self, other: &Self, m: &Natural) {
        *self = mod_kronecker_product(self, other, m);
    }
}
