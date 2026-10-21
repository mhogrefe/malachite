// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul, matrix_mul_assign};
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModAssign, ModIsReduced, ModMul, ModMulAssign};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the arguments of a matrix product modulo `m`: `m` is positive, the width of the first
// matrix is the height of the second, and the elements of both are reduced.
fn assert_arguments(a: &NaturalMatrix, b: &NaturalMatrix, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert_eq!(
        a.width, b.height,
        "the width of the first matrix must equal the height of the second"
    );
    assert!(
        a.mod_is_reduced(m),
        "the first matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        b.mod_is_reduced(m),
        "the second matrix must be reduced mod m, but {b} has an element >= {m}"
    );
}

const fn product_shape(a: &NaturalMatrix, b: &NaturalMatrix) -> ProductShape {
    ProductShape {
        ar: a.height,
        br: b.height,
        bc: b.width,
    }
}

// The elements of a matrix whose elements all fit in a limb, as limbs.
fn to_limb_matrix(a: &NaturalMatrix) -> UnsignedMatrix<Limb> {
    UnsignedMatrix::from_owned_elements(
        a.height(),
        a.width(),
        a.elements.iter().map(Limb::wrapping_from).collect(),
    )
}

// The product of `a` and `b` modulo `m`, which must fit in a limb, so that every element does too:
// the product of the matrices as `UnsignedMatrix`es of limbs.
fn limb_product(a: &NaturalMatrix, b: &NaturalMatrix, m: Limb) -> NaturalMatrix {
    NaturalMatrix::from(to_limb_matrix(a).mod_mul(to_limb_matrix(b), m))
}

// The product `a * b` modulo `m`, after checking the arguments. When `m` fits in a limb, the
// product is computed by the `UnsignedMatrix` implementation; otherwise the exact product is
// computed and each of its elements reduced once.
fn mod_mul_matrix(a: &NaturalMatrix, b: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    assert_arguments(a, b, m);
    if let Natural(Small(small_m)) = *m {
        limb_product(a, b, small_m)
    } else {
        let mut elements = matrix_mul(&a.elements, &b.elements, product_shape(a, b));
        for x in &mut elements {
            x.mod_assign(m);
        }
        NaturalMatrix {
            elements,
            height: a.height,
            width: b.width,
        }
    }
}

fn mod_mul_matrix_assign(a: &mut NaturalMatrix, b: &NaturalMatrix, m: &Natural) {
    assert_arguments(a, b, m);
    if let Natural(Small(small_m)) = *m {
        *a = limb_product(a, b, small_m);
    } else {
        let shape = product_shape(a, b);
        matrix_mul_assign(&mut a.elements, &b.elements, shape);
        for x in &mut a.elements {
            x.mod_assign(m);
        }
        a.width = b.width;
    }
}

impl ModMul<Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Multiplies two [`NaturalMatrix`]es modulo a [`Natural`] $m$, taking the first by value, the
    /// second by value, and the modulus by value.
    ///
    /// The elements of both matrices must already be reduced modulo $m$. The product $AB$ of an $h
    /// \times n$ matrix $A$ and an $n \times w$ matrix $B$ is the $h \times w$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $B$,
    /// reduced modulo $m$:
    ///
    /// $$
    /// f(A, B, m)_{ij} = \left ( \sum_{l=0}^{n-1} A_{il}B_{lj} \right ) \bmod m.
    /// $$
    ///
    /// When $n$ is 0, the product is the $h \times w$ zero matrix. When $m$ fits in a limb, the
    /// product is computed as for [`UnsignedMatrix`]es; otherwise the exact product is computed, as
    /// by [`Mul`](core::ops::Mul), and each of its elements reduced once.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(a.mod_mul(b, Natural::from(9u32)).to_string(), "[1, 4; 7, 5]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_mul(b, Natural::from(7u32)).to_string(), "[5, 4; 0, 2]");
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_mul(b, m).to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: Self, m: Natural) -> Self {
        let mut a = self;
        mod_mul_matrix_assign(&mut a, &other, &m);
        a
    }
}

impl ModMul<Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Multiplies two [`NaturalMatrix`]es modulo a [`Natural`] $m$, taking the first by value, the
    /// second by value, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(a.mod_mul(b, &Natural::from(9u32)).to_string(), "[1, 4; 7, 5]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_mul(b, &Natural::from(7u32)).to_string(), "[5, 4; 0, 2]");
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_mul(b, &m).to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: Self, m: &Natural) -> Self {
        let mut a = self;
        mod_mul_matrix_assign(&mut a, &other, m);
        a
    }
}

impl ModMul<&Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Multiplies two [`NaturalMatrix`]es modulo a [`Natural`] $m$, taking the first by value, the
    /// second by reference, and the modulus by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(a.mod_mul(&b, Natural::from(9u32)).to_string(), "[1, 4; 7, 5]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_mul(&b, Natural::from(7u32)).to_string(), "[5, 4; 0, 2]");
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_mul(&b, m).to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: &Self, m: Natural) -> Self {
        let mut a = self;
        mod_mul_matrix_assign(&mut a, other, &m);
        a
    }
}

impl ModMul<&Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Multiplies two [`NaturalMatrix`]es modulo a [`Natural`] $m$, taking the first by value, the
    /// second by reference, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(a.mod_mul(&b, &Natural::from(9u32)).to_string(), "[1, 4; 7, 5]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_mul(&b, &Natural::from(7u32)).to_string(), "[5, 4; 0, 2]");
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_mul(&b, &m).to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: &Self, m: &Natural) -> Self {
        let mut a = self;
        mod_mul_matrix_assign(&mut a, other, m);
        a
    }
}

impl ModMul<NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies two [`NaturalMatrix`]es modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by value, and the modulus by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!((&a).mod_mul(b, Natural::from(9u32)).to_string(), "[1, 4; 7, 5]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_mul(b, Natural::from(7u32)).to_string(), "[5, 4; 0, 2]");
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul(b, m).to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: NaturalMatrix, m: Natural) -> NaturalMatrix {
        mod_mul_matrix(self, &other, &m)
    }
}

impl ModMul<NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies two [`NaturalMatrix`]es modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by value, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!((&a).mod_mul(b, &Natural::from(9u32)).to_string(), "[1, 4; 7, 5]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_mul(b, &Natural::from(7u32)).to_string(), "[5, 4; 0, 2]");
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul(b, &m).to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: NaturalMatrix, m: &Natural) -> NaturalMatrix {
        mod_mul_matrix(self, &other, m)
    }
}

impl ModMul<&NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies two [`NaturalMatrix`]es modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by reference, and the modulus by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!((&a).mod_mul(&b, Natural::from(9u32)).to_string(), "[1, 4; 7, 5]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_mul(&b, Natural::from(7u32)).to_string(), "[5, 4; 0, 2]");
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul(&b, m).to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: &NaturalMatrix, m: Natural) -> NaturalMatrix {
        mod_mul_matrix(self, other, &m)
    }
}

impl ModMul<&NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies two [`NaturalMatrix`]es modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by reference, and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!((&a).mod_mul(&b, &Natural::from(9u32)).to_string(), "[1, 4; 7, 5]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_mul(&b, &Natural::from(7u32)).to_string(), "[5, 4; 0, 2]");
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul(&b, &m).to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, other: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
        mod_mul_matrix(self, other, m)
    }
}

impl ModMulAssign<Self, Natural> for NaturalMatrix {
    /// Multiplies a [`NaturalMatrix`] by another modulo a [`Natural`] $m$ in place, taking the
    /// second matrix by value and the modulus by value.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes all arguments by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// a.mod_mul_assign(b, Natural::from(5u32));
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// a.mod_mul_assign(b, Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[5, 4; 0, 2]");
    ///
    /// let mut a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_mul_assign(b, m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, other: Self, m: Natural) {
        mod_mul_matrix_assign(self, &other, &m);
    }
}

impl ModMulAssign<Self, &Natural> for NaturalMatrix {
    /// Multiplies a [`NaturalMatrix`] by another modulo a [`Natural`] $m$ in place, taking the
    /// second matrix by value and the modulus by reference.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes all arguments by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// a.mod_mul_assign(b, &Natural::from(5u32));
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// a.mod_mul_assign(b, &Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[5, 4; 0, 2]");
    ///
    /// let mut a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_mul_assign(b, &m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, other: Self, m: &Natural) {
        mod_mul_matrix_assign(self, &other, m);
    }
}

impl ModMulAssign<&Self, Natural> for NaturalMatrix {
    /// Multiplies a [`NaturalMatrix`] by another modulo a [`Natural`] $m$ in place, taking the
    /// second matrix by reference and the modulus by value.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes all arguments by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// a.mod_mul_assign(&b, Natural::from(5u32));
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// a.mod_mul_assign(&b, Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[5, 4; 0, 2]");
    ///
    /// let mut a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_mul_assign(&b, m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, other: &Self, m: Natural) {
        mod_mul_matrix_assign(self, other, &m);
    }
}

impl ModMulAssign<&Self, &Natural> for NaturalMatrix {
    /// Multiplies a [`NaturalMatrix`] by another modulo a [`Natural`] $m$ in place, taking the
    /// second matrix by reference and the modulus by reference.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes all arguments by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(h, n, w, m) = O(hnwm \log m \log\log m)$
    ///
    /// $M(h, w, m) = O(hwm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the first matrix, $n$ is
    /// its width, $w$ is the width of the second matrix, and $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the width of the first matrix is not the height of the second, or
    /// if any element of either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// a.mod_mul_assign(&b, &Natural::from(5u32));
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// a.mod_mul_assign(&b, &Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[5, 4; 0, 2]");
    ///
    /// let mut a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let b =
    ///     NaturalMatrix::from_str("[2, 10000000000000000000; 10000000000000000005, 9]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_mul_assign(&b, &m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[50000000000000000015, 27; 60000000000000000019, 60000000000000000009]"
    /// );
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, other: &Self, m: &Natural) {
        mod_mul_matrix_assign(self, other, m);
    }
}
