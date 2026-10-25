// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul};
use crate::integer_matrix::arithmetic::square::matrix_square;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Assign, ModPowerOf2IsReduced, ModPowerOf2Pow, ModPowerOf2PowAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the arguments of a power modulo $2^k$: the matrix is square, and its elements are reduced.
fn assert_arguments(a: &NaturalMatrix, pow: u64) {
    assert_eq!(
        a.height, a.width,
        "the matrix must be square, but it is {} by {}",
        a.height, a.width
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

// Reduces every element of `xs` modulo $2^k$.
fn reduce(mut xs: Vec<Natural>, pow: u64) -> Vec<Natural> {
    for x in &mut xs {
        x.mod_power_of_2_assign(pow);
    }
    xs
}

// Returns the elements of the `exp`th power of the `n` by `n` matrix with elements `xs`, which are
// reduced modulo $2^k$, modulo $2^k$. When $k$ is at most the width of a limb, the matrix is raised
// to the power as an `UnsignedMatrix` of limbs. Otherwise the power is computed by left-to-right
// binary powering, with each exact product reduced, stopping once the power is the zero matrix,
// since every further power is zero too.
fn matrix_mod_power_of_2_pow(xs: &[Natural], n: usize, exp: u64, pow: u64) -> Vec<Natural> {
    if pow <= Limb::WIDTH {
        let n = u64::exact_from(n);
        return UnsignedMatrix::<Limb>::from_owned_elements(
            n,
            n,
            xs.iter().map(Limb::wrapping_from).collect(),
        )
        .mod_power_of_2_pow(exp, pow)
        .into_elements()
        .into_iter()
        .map(Natural::from)
        .collect();
    }
    if exp == 0 {
        let mut out = vec![Natural::ZERO; n * n];
        for x in out.iter_mut().step_by(n + 1) {
            *x = Natural::ONE;
        }
        return out;
    }
    if exp == 1 {
        return xs.to_vec();
    }
    let shape = ProductShape {
        ar: n,
        br: n,
        bc: n,
    };
    let top = exp.significant_bits() - 2;
    let mut t = Vec::new();
    for i in (0..=top).rev() {
        // The first squaring reads `xs` itself.
        let u = reduce(matrix_square(if i == top { xs } else { &t }, n), pow);
        t = if exp.get_bit(i) {
            reduce(matrix_mul(&u, xs, shape), pow)
        } else {
            u
        };
        if t.iter().all(|x| *x == 0u32) {
            break;
        }
    }
    t
}

impl ModPowerOf2Pow<u64> for NaturalMatrix {
    type Output = Self;

    /// Raises a square [`NaturalMatrix`] to a power modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The power $A^e$ of an $n
    /// \times n$ matrix $A$ is the product of $e$ copies of $A$, reduced modulo $2^k$; $A^0$ is the
    /// $n \times n$ identity matrix, reduced modulo $2^k$.
    ///
    /// $$
    /// f(A, e, k) = A^e \bmod 2^k.
    /// $$
    ///
    /// The power is computed by left-to-right binary powering, with each product reduced modulo
    /// $2^k$, so the elements never grow; it stops early if a power is the zero matrix. When $k$ is
    /// at most the width of a limb, the matrix is raised to the power as an [`UnsignedMatrix`] of
    /// limbs; otherwise each product is computed exactly and then reduced.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^3 m \log m \log\log m \log e)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is
    /// `pow`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any of its elements is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Pow;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_pow(10, 6).to_string(), "[25, 55; 55, 34]");
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_pow(4, 5).to_string(),
    ///     "[24, 25, 16; 2, 26, 25; 25, 9, 8]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_power_of_2_pow(0, 3).to_string(), "[1, 0; 0, 1]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_pow(100, 100).to_string(),
    ///     "[573147844013817084101, 354224848179261915075; 354224848179261915075, \
    ///     218922995834555169026]"
    /// );
    /// ```
    fn mod_power_of_2_pow(self, exp: u64, pow: u64) -> Self {
        if exp == 1 {
            assert_arguments(&self, pow);
            self
        } else {
            (&self).mod_power_of_2_pow(exp, pow)
        }
    }
}

impl ModPowerOf2Pow<u64> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Raises a square [`NaturalMatrix`] to a power modulo $2^k$, taking it by reference.
    ///
    /// See the documentation for the implementation that takes the matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^3 m \log m \log\log m \log e)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is
    /// `pow`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any of its elements is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Pow;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow(10, 6).to_string(),
    ///     "[25, 55; 55, 34]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow(4, 5).to_string(),
    ///     "[24, 25, 16; 2, 26, 25; 25, 9, 8]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_pow(0, 3).to_string(), "[1, 0; 0, 1]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow(100, 100).to_string(),
    ///     "[573147844013817084101, 354224848179261915075; 354224848179261915075, \
    ///     218922995834555169026]"
    /// );
    /// ```
    fn mod_power_of_2_pow(self, exp: u64, pow: u64) -> NaturalMatrix {
        assert_arguments(self, pow);
        NaturalMatrix {
            elements: matrix_mod_power_of_2_pow(&self.elements, self.height, exp, pow),
            height: self.height,
            width: self.width,
        }
    }
}

impl ModPowerOf2PowAssign<u64> for NaturalMatrix {
    /// Raises a square [`NaturalMatrix`] to a power modulo $2^k$ in place.
    ///
    /// See the documentation for the implementation that takes the matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^3 m \log m \log\log m \log e)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is
    /// `pow`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any of its elements is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2PowAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// a.mod_power_of_2_pow_assign(10, 6);
    /// assert_eq!(a.to_string(), "[25, 55; 55, 34]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.mod_power_of_2_pow_assign(4, 5);
    /// assert_eq!(a.to_string(), "[24, 25, 16; 2, 26, 25; 25, 9, 8]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_pow_assign(0, 3);
    /// assert_eq!(a.to_string(), "[1, 0; 0, 1]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// a.mod_power_of_2_pow_assign(100, 100);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[573147844013817084101, 354224848179261915075; 354224848179261915075, \
    ///     218922995834555169026]"
    /// );
    /// ```
    fn mod_power_of_2_pow_assign(&mut self, exp: u64, pow: u64) {
        assert_arguments(self, pow);
        if exp != 1 {
            self.elements = matrix_mod_power_of_2_pow(&self.elements, self.height, exp, pow);
        }
    }
}
