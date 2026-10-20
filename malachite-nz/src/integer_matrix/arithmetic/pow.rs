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

use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul};
use crate::integer_matrix::arithmetic::square::{assert_square, matrix_square};
use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};

// Returns the elements of the `exp`th power of the `n` by `n` matrix with elements `xs`, in
// row-major order, by left-to-right binary powering.
//
// This is equivalent to `fmpz_mat_pow` from `fmpz_mat/pow.c`, FLINT 3.6.0, except that it stops
// once the power is the zero matrix, as it may be when the matrix is nilpotent.
crate_test_fn! {matrix_pow<C: PolynomialCoefficient>(xs: &[C], n: usize, exp: u64) -> Vec<C> {
    if exp <= 2 || n <= 1 {
        if exp == 0 || n == 0 {
            let mut out = vec![C::ZERO; n * n];
            for x in out.iter_mut().step_by(n + 1) {
                *x = C::ONE;
            }
            out
        } else if n == 1 {
            vec![xs[0].pow_ref(exp)]
        } else if exp == 1 {
            xs.to_vec()
        } else {
            matrix_square(xs, n)
        }
    } else {
        let shape = ProductShape {
            ar: n,
            br: n,
            bc: n,
        };
        let top = exp.significant_bits() - 2;
        let mut t = Vec::new();
        for i in (0..=top).rev() {
            // The first squaring reads `xs` itself, which FLINT copies first.
            let u = matrix_square(if i == top { xs } else { &t }, n);
            t = if exp.get_bit(i) {
                matrix_mul(&u, xs, shape)
            } else {
                u
            };
            // Every further power of the zero matrix is zero.
            if t.iter().all(C::is_zero) {
                break;
            }
        }
        t
    }
}}

impl Pow<u64> for IntegerMatrix {
    type Output = Self;

    /// Raises a square [`IntegerMatrix`] to a power, taking it by value.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$; $A^0$ is
    /// the $n \times n$ identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O(n^3 km \log (km) \log\log (km))$
    ///
    /// $M(n, m, k) = O(n^2 km)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is the
    /// largest number of significant bits of any of its elements, and $k$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.pow(10).to_string(), "[89, 55; 55, 34]");
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// assert_eq!(a.pow(5).to_string(), "[-275, 50; -75, -350]");
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// assert_eq!(a.pow(0).to_string(), "[1, 0; 0, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_pow` from `fmpz_mat/pow.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square, which FLINT does not check.
    fn pow(self, exp: u64) -> Self {
        if exp == 1 {
            assert_square(&self);
            self
        } else {
            (&self).pow(exp)
        }
    }
}

impl Pow<u64> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Raises a square [`IntegerMatrix`] to a power, taking it by reference.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$; $A^0$ is
    /// the $n \times n$ identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O(n^3 km \log (km) \log\log (km))$
    ///
    /// $M(n, m, k) = O(n^2 km)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is the
    /// largest number of significant bits of any of its elements, and $k$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).pow(10).to_string(), "[89, 55; 55, 34]");
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// assert_eq!((&a).pow(5).to_string(), "[-275, 50; -75, -350]");
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// assert_eq!((&a).pow(0).to_string(), "[1, 0; 0, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_pow` from `fmpz_mat/pow.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square, which FLINT does not check.
    fn pow(self, exp: u64) -> IntegerMatrix {
        assert_square(self);
        IntegerMatrix {
            elements: matrix_pow(&self.elements, self.height, exp),
            height: self.height,
            width: self.width,
        }
    }
}

impl PowAssign<u64> for IntegerMatrix {
    /// Raises a square [`IntegerMatrix`] to a power in place.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$; $A^0$ is
    /// the $n \times n$ identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O(n^3 km \log (km) \log\log (km))$
    ///
    /// $M(n, m, k) = O(n^2 km)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is the
    /// largest number of significant bits of any of its elements, and $k$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// a.pow_assign(10);
    /// assert_eq!(a.to_string(), "[89, 55; 55, 34]");
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// a.pow_assign(5);
    /// assert_eq!(a.to_string(), "[-275, 50; -75, -350]");
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// a.pow_assign(0);
    /// assert_eq!(a.to_string(), "[1, 0; 0, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_pow` from `fmpz_mat/pow.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square, which FLINT does not check.
    fn pow_assign(&mut self, exp: u64) {
        assert_square(self);
        if exp != 1 {
            self.elements = matrix_pow(&self.elements, self.height, exp);
        }
    }
}
