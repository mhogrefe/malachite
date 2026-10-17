// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_vector::arithmetic::scalar_mul::{
    integers_mul_scalar, integers_mul_scalar_assign,
};
use core::ops::{Mul, MulAssign};
use malachite_base::matrix::Matrix;

impl Mul<Integer> for IntegerMatrix {
    type Output = Self;

    /// Multiplies every element of an [`IntegerMatrix`] by an [`Integer`], taking the matrix by
    /// value and the scalar by value.
    ///
    /// The product is taken element by element, so the result has the same shape as the matrix.
    ///
    /// $$
    /// f(A, c) = cA, \quad \text{where} \quad (cA)_{ij} = cA_{ij}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((a * c).to_string(), "[-3, 6; -9, 0]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(mut self, c: Integer) -> Self {
        self *= &c;
        self
    }
}

impl Mul<&Integer> for IntegerMatrix {
    type Output = Self;

    /// Multiplies every element of an [`IntegerMatrix`] by an [`Integer`], taking the matrix by
    /// value and the scalar by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the matrix and the scalar by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((a * &c).to_string(), "[-3, 6; -9, 0]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(mut self, c: &Integer) -> Self {
        self *= c;
        self
    }
}

impl Mul<Integer> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Multiplies every element of an [`IntegerMatrix`] by an [`Integer`], taking the matrix by
    /// reference and the scalar by value.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the matrix and the scalar by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((&a * c).to_string(), "[-3, 6; -9, 0]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, c: Integer) -> IntegerMatrix {
        self * &c
    }
}

impl Mul<&Integer> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Multiplies every element of an [`IntegerMatrix`] by an [`Integer`], taking the matrix by
    /// reference and the scalar by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the matrix and the scalar by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((&a * &c).to_string(), "[-3, 6; -9, 0]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, c: &Integer) -> IntegerMatrix {
        IntegerMatrix::from_owned_elements(
            self.height(),
            self.width(),
            integers_mul_scalar(&self.elements, c),
        )
    }
}

impl Mul<IntegerMatrix> for Integer {
    type Output = IntegerMatrix;

    /// Multiplies an [`Integer`] by every element of an [`IntegerMatrix`], taking the scalar by
    /// value and the matrix by value.
    ///
    /// Scalar multiplication is commutative, so this is `a * c`; see the documentation for the
    /// [`Mul`] implementation that takes the matrix and the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((c * a).to_string(), "[-3, 6; -9, 0]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: IntegerMatrix) -> IntegerMatrix {
        v * self
    }
}

impl Mul<&IntegerMatrix> for Integer {
    type Output = IntegerMatrix;

    /// Multiplies an [`Integer`] by every element of an [`IntegerMatrix`], taking the scalar by
    /// value and the matrix by reference.
    ///
    /// Scalar multiplication is commutative, so this is `a * c`; see the documentation for the
    /// [`Mul`] implementation that takes the matrix and the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((c * &a).to_string(), "[-3, 6; -9, 0]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: &IntegerMatrix) -> IntegerMatrix {
        v * self
    }
}

impl Mul<IntegerMatrix> for &Integer {
    type Output = IntegerMatrix;

    /// Multiplies an [`Integer`] by every element of an [`IntegerMatrix`], taking the scalar by
    /// reference and the matrix by value.
    ///
    /// Scalar multiplication is commutative, so this is `a * c`; see the documentation for the
    /// [`Mul`] implementation that takes the matrix and the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((&c * a).to_string(), "[-3, 6; -9, 0]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: IntegerMatrix) -> IntegerMatrix {
        v * self
    }
}

impl Mul<&IntegerMatrix> for &Integer {
    type Output = IntegerMatrix;

    /// Multiplies an [`Integer`] by every element of an [`IntegerMatrix`], taking the scalar by
    /// reference and the matrix by reference.
    ///
    /// Scalar multiplication is commutative, so this is `a * c`; see the documentation for the
    /// [`Mul`] implementation that takes the matrix and the scalar by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let c = Integer::from(-3);
    /// assert_eq!((&c * &a).to_string(), "[-3, 6; -9, 0]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: &IntegerMatrix) -> IntegerMatrix {
        v * self
    }
}

impl MulAssign<Integer> for IntegerMatrix {
    /// Multiplies every element of an [`IntegerMatrix`] by an [`Integer`] in place, taking the
    /// scalar by value.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the matrix and the scalar by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// a *= Integer::from(-3);
    /// assert_eq!(a.to_string(), "[-3, 6; -9, 0]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0,
    /// with `vec1` equal to `vec2`.
    #[inline]
    fn mul_assign(&mut self, c: Integer) {
        *self *= &c;
    }
}

impl MulAssign<&Integer> for IntegerMatrix {
    /// Multiplies every element of an [`IntegerMatrix`] by an [`Integer`] in place, taking the
    /// scalar by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes the matrix and the scalar by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of the scalar.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// a *= &Integer::from(-3);
    /// assert_eq!(a.to_string(), "[-3, 6; -9, 0]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0,
    /// with `vec1` equal to `vec2`.
    #[inline]
    fn mul_assign(&mut self, c: &Integer) {
        integers_mul_scalar_assign(&mut self.elements, c);
    }
}
