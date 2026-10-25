// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul};
use crate::integer_matrix::arithmetic::square::matrix_square;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModAssign, ModIsReduced, ModPow, ModPowAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the arguments of a power modulo `m`: `m` is positive, the matrix is square, and its
// elements are reduced.
fn assert_arguments(a: &NaturalMatrix, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert_eq!(
        a.height, a.width,
        "the matrix must be square, but it is {} by {}",
        a.height, a.width
    );
    assert!(
        a.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// Reduces every element of `xs` modulo `m`.
fn reduce(mut xs: Vec<Natural>, m: &Natural) -> Vec<Natural> {
    for x in &mut xs {
        x.mod_assign(m);
    }
    xs
}

// Returns the elements of the `exp`th power of the `n` by `n` matrix with elements `xs`, which are
// reduced modulo `m`, modulo `m`. When `m` fits in a limb, the matrix is raised to the power as an
// `UnsignedMatrix` of limbs. Otherwise the power is computed by left-to-right binary powering, with
// each exact product reduced, stopping once the power is the zero matrix, since every further power
// is zero too.
fn matrix_mod_pow(xs: &[Natural], n: usize, exp: u64, m: &Natural) -> Vec<Natural> {
    if let Natural(Small(small_m)) = *m {
        let n = u64::exact_from(n);
        return UnsignedMatrix::<Limb>::from_owned_elements(
            n,
            n,
            xs.iter().map(Limb::wrapping_from).collect(),
        )
        .mod_pow(exp, small_m)
        .into_elements()
        .into_iter()
        .map(Natural::from)
        .collect();
    }
    if exp == 0 {
        // `m` does not fit in a limb, so it is greater than 1.
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
        let u = reduce(matrix_square(if i == top { xs } else { &t }, n), m);
        t = if exp.get_bit(i) {
            reduce(matrix_mul(&u, xs, shape), m)
        } else {
            u
        };
        if t.iter().all(|x| *x == 0u32) {
            break;
        }
    }
    t
}

impl ModPow<u64, Natural> for NaturalMatrix {
    type Output = Self;

    /// Raises a square [`NaturalMatrix`] to a power modulo a [`Natural`] $m$, taking the matrix by
    /// value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The power $A^e$ of an $n
    /// \times n$ matrix $A$ is the product of $e$ copies of $A$, reduced modulo $m$; $A^0$ is the
    /// $n \times n$ identity matrix, reduced modulo $m$.
    ///
    /// $$
    /// f(A, e, m) = A^e \bmod m.
    /// $$
    ///
    /// The power is computed by left-to-right binary powering, with each product reduced modulo
    /// $m$, so the elements never grow; it stops early if a power is the zero matrix. When $m$ fits
    /// in a limb, the matrix is raised to the power as an [`UnsignedMatrix`] of limbs; otherwise
    /// each product is computed exactly and then reduced.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^3 m \log m \log\log m \log e)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_pow(10, Natural::from(100u32)).to_string(),
    ///     "[89, 55; 55, 34]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_pow(4, Natural::from(7u32)).to_string(),
    ///     "[3, 4, 2; 3, 6, 4; 4, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_pow(0, Natural::from(5u32)).to_string(),
    ///     "[1, 0; 0, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_pow(100, m).to_string(),
    ///     "[73147844013817084101, 54224848179261915075; 54224848179261915075, \
    ///     18922995834555169026]"
    /// );
    /// ```
    fn mod_pow(self, exp: u64, m: Natural) -> Self {
        if exp == 1 {
            assert_arguments(&self, &m);
            self
        } else {
            (&self).mod_pow(exp, &m)
        }
    }
}

impl ModPow<u64, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Raises a square [`NaturalMatrix`] to a power modulo a [`Natural`] $m$, taking the matrix by
    /// value and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^3 m \log m \log\log m \log e)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_pow(10, &Natural::from(100u32)).to_string(),
    ///     "[89, 55; 55, 34]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_pow(4, &Natural::from(7u32)).to_string(),
    ///     "[3, 4, 2; 3, 6, 4; 4, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_pow(0, &Natural::from(5u32)).to_string(),
    ///     "[1, 0; 0, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_pow(100, &m).to_string(),
    ///     "[73147844013817084101, 54224848179261915075; 54224848179261915075, \
    ///     18922995834555169026]"
    /// );
    /// ```
    fn mod_pow(self, exp: u64, m: &Natural) -> Self {
        if exp == 1 {
            assert_arguments(&self, m);
            self
        } else {
            (&self).mod_pow(exp, m)
        }
    }
}

impl ModPow<u64, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Raises a square [`NaturalMatrix`] to a power modulo a [`Natural`] $m$, taking the matrix by
    /// reference and the modulus by value.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^3 m \log m \log\log m \log e)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow(10, Natural::from(100u32)).to_string(),
    ///     "[89, 55; 55, 34]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow(4, Natural::from(7u32)).to_string(),
    ///     "[3, 4, 2; 3, 6, 4; 4, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow(0, Natural::from(5u32)).to_string(),
    ///     "[1, 0; 0, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow(100, m).to_string(),
    ///     "[73147844013817084101, 54224848179261915075; 54224848179261915075, \
    ///     18922995834555169026]"
    /// );
    /// ```
    fn mod_pow(self, exp: u64, m: Natural) -> NaturalMatrix {
        assert_arguments(self, &m);
        NaturalMatrix {
            elements: matrix_mod_pow(&self.elements, self.height, exp, &m),
            height: self.height,
            width: self.width,
        }
    }
}

impl ModPow<u64, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Raises a square [`NaturalMatrix`] to a power modulo a [`Natural`] $m$, taking the matrix by
    /// reference and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes all arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^3 m \log m \log\log m \log e)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow(10, &Natural::from(100u32)).to_string(),
    ///     "[89, 55; 55, 34]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow(4, &Natural::from(7u32)).to_string(),
    ///     "[3, 4, 2; 3, 6, 4; 4, 6, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow(0, &Natural::from(5u32)).to_string(),
    ///     "[1, 0; 0, 1]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow(100, &m).to_string(),
    ///     "[73147844013817084101, 54224848179261915075; 54224848179261915075, \
    ///     18922995834555169026]"
    /// );
    /// ```
    fn mod_pow(self, exp: u64, m: &Natural) -> NaturalMatrix {
        assert_arguments(self, m);
        NaturalMatrix {
            elements: matrix_mod_pow(&self.elements, self.height, exp, m),
            height: self.height,
            width: self.width,
        }
    }
}

impl ModPowAssign<u64, Natural> for NaturalMatrix {
    /// Raises a square [`NaturalMatrix`] to a power modulo a [`Natural`] $m$ in place, taking the
    /// modulus by value.
    ///
    /// See the documentation for the [`ModPow`] implementation that takes all arguments by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^3 m \log m \log\log m \log e)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// a.mod_pow_assign(10, Natural::from(100u32));
    /// assert_eq!(a.to_string(), "[89, 55; 55, 34]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.mod_pow_assign(4, Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[3, 4, 2; 3, 6, 4; 4, 6, 1]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_pow_assign(0, Natural::from(5u32));
    /// assert_eq!(a.to_string(), "[1, 0; 0, 1]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_pow_assign(100, m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[73147844013817084101, 54224848179261915075; 54224848179261915075, \
    ///     18922995834555169026]"
    /// );
    /// ```
    fn mod_pow_assign(&mut self, exp: u64, m: Natural) {
        assert_arguments(self, &m);
        if exp != 1 {
            self.elements = matrix_mod_pow(&self.elements, self.height, exp, &m);
        }
    }
}

impl ModPowAssign<u64, &Natural> for NaturalMatrix {
    /// Raises a square [`NaturalMatrix`] to a power modulo a [`Natural`] $m$ in place, taking the
    /// modulus by reference.
    ///
    /// See the documentation for the [`ModPow`] implementation that takes all arguments by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^3 m \log m \log\log m \log e)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// a.mod_pow_assign(10, &Natural::from(100u32));
    /// assert_eq!(a.to_string(), "[89, 55; 55, 34]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.mod_pow_assign(4, &Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[3, 4, 2; 3, 6, 4; 4, 6, 1]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_pow_assign(0, &Natural::from(5u32));
    /// assert_eq!(a.to_string(), "[1, 0; 0, 1]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_pow_assign(100, &m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[73147844013817084101, 54224848179261915075; 54224848179261915075, \
    ///     18922995834555169026]"
    /// );
    /// ```
    fn mod_pow_assign(&mut self, exp: u64, m: &Natural) {
        assert_arguments(self, m);
        if exp != 1 {
            self.elements = matrix_mod_pow(&self.elements, self.height, exp, m);
        }
    }
}
