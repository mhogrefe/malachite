// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec;
use core::ops::{Mul, MulAssign};
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::{One, Zero};

impl Mul<Natural> for NaturalMatrix {
    type Output = Self;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`], taking the matrix by value
    /// and the scalar by value.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((a * c).to_string(), "[3, 6; 9, 12]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(mut self, c: Natural) -> Self {
        self *= &c;
        self
    }
}

impl Mul<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`], taking the matrix by value
    /// and the scalar by reference.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((a * &c).to_string(), "[3, 6; 9, 12]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(mut self, c: &Natural) -> Self {
        self *= c;
        self
    }
}

impl Mul<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`], taking the matrix by
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((&a * c).to_string(), "[3, 6; 9, 12]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, c: Natural) -> NaturalMatrix {
        self * &c
    }
}

impl Mul<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`], taking the matrix by
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((&a * &c).to_string(), "[3, 6; 9, 12]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, c: &Natural) -> NaturalMatrix {
        NaturalMatrix::from_owned_elements(
            self.height(),
            self.width(),
            match *c {
                Natural::ZERO => vec![Natural::ZERO; self.elements.len()],
                Natural::ONE => self.elements.clone(),
                _ => self.elements.iter().map(|x| x * c).collect(),
            },
        )
    }
}

impl Mul<NaturalMatrix> for Natural {
    type Output = NaturalMatrix;

    /// Multiplies a [`Natural`] by every element of a [`NaturalMatrix`], taking the scalar by value
    /// and the matrix by value.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((c * a).to_string(), "[3, 6; 9, 12]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: NaturalMatrix) -> NaturalMatrix {
        v * self
    }
}

impl Mul<&NaturalMatrix> for Natural {
    type Output = NaturalMatrix;

    /// Multiplies a [`Natural`] by every element of a [`NaturalMatrix`], taking the scalar by value
    /// and the matrix by reference.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((c * &a).to_string(), "[3, 6; 9, 12]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: &NaturalMatrix) -> NaturalMatrix {
        v * self
    }
}

impl Mul<NaturalMatrix> for &Natural {
    type Output = NaturalMatrix;

    /// Multiplies a [`Natural`] by every element of a [`NaturalMatrix`], taking the scalar by
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((&c * a).to_string(), "[3, 6; 9, 12]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: NaturalMatrix) -> NaturalMatrix {
        v * self
    }
}

impl Mul<&NaturalMatrix> for &Natural {
    type Output = NaturalMatrix;

    /// Multiplies a [`Natural`] by every element of a [`NaturalMatrix`], taking the scalar by
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let c = Natural::from(3u32);
    /// assert_eq!((&c * &a).to_string(), "[3, 6; 9, 12]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn mul(self, v: &NaturalMatrix) -> NaturalMatrix {
        v * self
    }
}

impl MulAssign<Natural> for NaturalMatrix {
    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] in place, taking the scalar
    /// by value.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a *= Natural::from(3u32);
    /// assert_eq!(a.to_string(), "[3, 6; 9, 12]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0,
    /// with `vec1` equal to `vec2`.
    #[inline]
    fn mul_assign(&mut self, c: Natural) {
        *self *= &c;
    }
}

impl MulAssign<&Natural> for NaturalMatrix {
    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] in place, taking the scalar
    /// by reference.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a *= &Natural::from(3u32);
    /// assert_eq!(a.to_string(), "[3, 6; 9, 12]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0,
    /// with `vec1` equal to `vec2`.
    #[inline]
    fn mul_assign(&mut self, c: &Natural) {
        match *c {
            Natural::ZERO => self.elements.fill(Natural::ZERO),
            Natural::ONE => {}
            _ => {
                for x in &mut self.elements {
                    *x *= c;
                }
            }
        }
    }
}
