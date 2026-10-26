// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::rational_matrix::arithmetic::mul::{divide_all, take_matrix};
use crate::rational_matrix::conversion::to_numerators_and_row_denominators::{
    into_scaled_numerators, scaled_numerators,
};
use alloc::vec;
use core::iter::repeat_n;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    Inverse, LcmAssign, Pow, PowAssign, Square, SquareAssign,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;

fn assert_square(a: &RationalMatrix) {
    assert_eq!(
        a.height, a.width,
        "the matrix must be square, but it is {} by {}",
        a.height, a.width
    );
}

// Writing a matrix as $N/d$, with $d$ the common denominator of all its elements, and computing
// $N^k/d^k$ replaces a reduction of every element at every step of the powering with one at the
// end, but makes the numerators larger by up to the ratio of $d$ to each row's own denominator.
// Measured on an aarch64 machine, with dimensions from 2 to 8, elements of 4 to 256 bits, and
// exponents from 3 to 32, it was faster whenever $d$ had at most twice the bits of the largest row
// denominator, and whenever $d$ had at most about 600 bits and at most about three times the bits
// of the largest row denominator; it was slower when $d$ had about 1000 bits or more and more than
// twice the bits, or about 700 bits and more than five times the bits.
const MATWISE_POW_MAX_DENOMINATOR_BITS: u64 = 768;

// The common denominator $d$ of the whole matrix, if the matrix is best raised to a power as $N/d$:
// either $d$ has at most twice the bits of the largest row denominator, so that putting the whole
// matrix over it inflates the numerators little, or $d$ is small and has at most four times the
// bits.
fn matwise_denominator(a: &RationalMatrix) -> Option<Natural> {
    let row_denominators = a.row_denominators();
    let mut d = Natural::ONE;
    let mut max_row_bits = 0;
    for e in &row_denominators {
        d.lcm_assign(e);
        max_row_bits = max_row_bits.max(e.significant_bits());
    }
    let bits = d.significant_bits();
    if bits <= max_row_bits << 1
        || (bits <= MATWISE_POW_MAX_DENOMINATOR_BITS && bits <= max_row_bits << 2)
    {
        Some(d)
    } else {
        None
    }
}

// The `exp`th power of the matrix $N/d$, where $N$ has `numerators` as elements: $N^k/d^k$, reduced
// once.
fn pow_matwise(numerators: IntegerMatrix, n: usize, exp: u64, d: &Natural) -> RationalMatrix {
    divide_all(numerators.pow(exp), n, n, &d.pow(exp))
}

// The `exp`th power of `a`, which is square, for an exponent of at least 3 and a dimension of at
// least 2, by left-to-right binary powering in rational arithmetic; every further power of a zero
// matrix is zero, so the powering stops at one.
fn pow_stepwise(a: &RationalMatrix, exp: u64) -> RationalMatrix {
    let top = exp.significant_bits() - 2;
    let mut t = a.square();
    if exp.get_bit(top) {
        t *= a;
    }
    for i in (0..top).rev() {
        if t.elements.iter().all(|x| *x == 0u32) {
            break;
        }
        t.square_assign();
        if exp.get_bit(i) {
            t *= a;
        }
    }
    t
}

// The power of `a`, always by binary powering in rational arithmetic, whatever its denominators.
private_test_fn! {pow_in_rational_arithmetic(a: &RationalMatrix, exp: u64) -> RationalMatrix {
    assert_square(a);
    pow_trivial(a, exp).unwrap_or_else(|| pow_stepwise(a, exp))
}}

// The power of `a` for the exponents and dimensions that need no powering at all.
fn pow_trivial(a: &RationalMatrix, exp: u64) -> Option<RationalMatrix> {
    match exp {
        0 => Some(RationalMatrix::identity(a.height())),
        1 => Some(a.clone()),
        2 => Some(a.square()),
        _ if a.height == 1 => Some(RationalMatrix {
            elements: vec![(&a.elements[0]).pow(exp)],
            height: 1,
            width: 1,
        }),
        _ => None,
    }
}

fn pow_ref(a: &RationalMatrix, exp: u64) -> RationalMatrix {
    if let Some(p) = pow_trivial(a, exp) {
        return p;
    }
    let n = a.height;
    if let Some(d) = matwise_denominator(a) {
        let numerators = IntegerMatrix::from_owned_elements(
            a.height(),
            a.width(),
            scaled_numerators(&a.elements, repeat_n(&d, n * n)),
        );
        pow_matwise(numerators, n, exp, &d)
    } else {
        pow_stepwise(a, exp)
    }
}

// The power of `a`, taking it by value, so that its numerators are scaled in place when it is
// raised to the power over a common denominator.
fn pow_owned(a: RationalMatrix, exp: u64) -> RationalMatrix {
    if exp == 1 {
        return a;
    }
    if let Some(p) = pow_trivial(&a, exp) {
        return p;
    }
    let n = a.height;
    if let Some(d) = matwise_denominator(&a) {
        let (height, width) = (a.height(), a.width());
        let numerators = IntegerMatrix::from_owned_elements(
            height,
            width,
            into_scaled_numerators(a.elements, repeat_n(&d, n * n)),
        );
        pow_matwise(numerators, n, exp, &d)
    } else {
        pow_stepwise(&a, exp)
    }
}

// The inverse of a matrix being raised to a negative power, which must exist.
const fn expect_inverse(inverse: Option<RationalMatrix>) -> RationalMatrix {
    inverse.expect("a singular matrix cannot be raised to a negative power")
}

// The inverse is taken before the power, since inverting $A^k$ would work on elements that have
// already grown $k$-fold.
fn pow_signed_owned(a: RationalMatrix, exp: i64) -> RationalMatrix {
    let abs_exp = exp.unsigned_abs();
    if exp >= 0 {
        pow_owned(a, abs_exp)
    } else {
        pow_owned(expect_inverse(a.inverse()), abs_exp)
    }
}

fn pow_signed_ref(a: &RationalMatrix, exp: i64) -> RationalMatrix {
    let abs_exp = exp.unsigned_abs();
    if exp >= 0 {
        pow_ref(a, abs_exp)
    } else {
        pow_owned(expect_inverse(a.inverse()), abs_exp)
    }
}

impl Pow<u64> for RationalMatrix {
    type Output = Self;

    /// Raises a square [`RationalMatrix`] to a power, taking it by value.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$; $A^0$ is
    /// the $n \times n$ identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// When the common denominator $d$ of all the elements is small, or not much larger than the
    /// largest common denominator of a row, $A$ is written as $N/d$ for an integer matrix $N$, and
    /// the power is $N^k/d^k$, with $N^k$ computed as by the [`Pow`] implementation for
    /// [`IntegerMatrix`]s and each element reduced once. Otherwise the power is computed by
    /// left-to-right binary powering, with each square computed as by [`Square`] and each product
    /// as by [`Mul`](core::ops::Mul), so that every intermediate power is reduced; it stops early
    /// if a power is the zero matrix. A $1 \times 1$ matrix's element is raised to the power
    /// directly, and the square of a matrix is computed as by [`Square`].
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O(n^3 km \log (km) \log\log (km))$
    ///
    /// $M(n, m, k) = O(n^2 km)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is the
    /// sum of the largest number of significant bits of any of its elements and the number of
    /// significant bits of the least common multiple of its denominators, and $k$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// assert_eq!(a.pow(5u64).to_string(), "[65/32, 29/16; 29/16, 9/8]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// assert_eq!(
    ///     a.pow(3u64).to_string(),
    ///     "[-25/24, 25/144; -25/24, -175/192]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// assert_eq!(a.pow(0u64).to_string(), "[1, 0; 0, 1]");
    ///
    /// let a = RationalMatrix::from_str("[-2/3]").unwrap();
    /// assert_eq!(a.pow(4u64).to_string(), "[16/81]");
    /// ```
    #[inline]
    fn pow(self, exp: u64) -> Self {
        assert_square(&self);
        pow_owned(self, exp)
    }
}

impl Pow<u64> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Raises a square [`RationalMatrix`] to a power, taking it by reference.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$; $A^0$ is
    /// the $n \times n$ identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// When the common denominator $d$ of all the elements is small, or not much larger than the
    /// largest common denominator of a row, $A$ is written as $N/d$ for an integer matrix $N$, and
    /// the power is $N^k/d^k$, with $N^k$ computed as by the [`Pow`] implementation for
    /// [`IntegerMatrix`]s and each element reduced once. Otherwise the power is computed by
    /// left-to-right binary powering, with each square computed as by [`Square`] and each product
    /// as by [`Mul`](core::ops::Mul), so that every intermediate power is reduced; it stops early
    /// if a power is the zero matrix. A $1 \times 1$ matrix's element is raised to the power
    /// directly, and the square of a matrix is computed as by [`Square`].
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O(n^3 km \log (km) \log\log (km))$
    ///
    /// $M(n, m, k) = O(n^2 km)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is the
    /// sum of the largest number of significant bits of any of its elements and the number of
    /// significant bits of the least common multiple of its denominators, and $k$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).pow(5u64).to_string(), "[65/32, 29/16; 29/16, 9/8]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// assert_eq!(
    ///     (&a).pow(3u64).to_string(),
    ///     "[-25/24, 25/144; -25/24, -175/192]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// assert_eq!((&a).pow(0u64).to_string(), "[1, 0; 0, 1]");
    ///
    /// let a = RationalMatrix::from_str("[-2/3]").unwrap();
    /// assert_eq!((&a).pow(4u64).to_string(), "[16/81]");
    /// ```
    fn pow(self, exp: u64) -> RationalMatrix {
        assert_square(self);
        pow_ref(self, exp)
    }
}

impl PowAssign<u64> for RationalMatrix {
    /// Raises a square [`RationalMatrix`] to a power in place.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$; $A^0$ is
    /// the $n \times n$ identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// When the common denominator $d$ of all the elements is small, or not much larger than the
    /// largest common denominator of a row, $A$ is written as $N/d$ for an integer matrix $N$, and
    /// the power is $N^k/d^k$, with $N^k$ computed as by the [`Pow`] implementation for
    /// [`IntegerMatrix`]s and each element reduced once. Otherwise the power is computed by
    /// left-to-right binary powering, with each square computed as by [`Square`] and each product
    /// as by [`Mul`](core::ops::Mul), so that every intermediate power is reduced; it stops early
    /// if a power is the zero matrix. A $1 \times 1$ matrix's element is raised to the power
    /// directly, and the square of a matrix is computed as by [`Square`].
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O(n^3 km \log (km) \log\log (km))$
    ///
    /// $M(n, m, k) = O(n^2 km)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is the
    /// sum of the largest number of significant bits of any of its elements and the number of
    /// significant bits of the least common multiple of its denominators, and $k$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// a.pow_assign(5u64);
    /// assert_eq!(a.to_string(), "[65/32, 29/16; 29/16, 9/8]");
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// a.pow_assign(3u64);
    /// assert_eq!(a.to_string(), "[-25/24, 25/144; -25/24, -175/192]");
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// a.pow_assign(0u64);
    /// assert_eq!(a.to_string(), "[1, 0; 0, 1]");
    ///
    /// let mut a = RationalMatrix::from_str("[-2/3]").unwrap();
    /// a.pow_assign(4u64);
    /// assert_eq!(a.to_string(), "[16/81]");
    /// ```
    fn pow_assign(&mut self, exp: u64) {
        assert_square(self);
        if exp != 1 {
            *self = pow_owned(take_matrix(self), exp);
        }
    }
}

impl Pow<i64> for RationalMatrix {
    type Output = Self;

    /// Raises a square [`RationalMatrix`] to a signed power, taking it by value.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$ when $k
    /// \geq 0$, and the product of $-k$ copies of $A^{-1}$ when $k < 0$; $A^0$ is the $n \times n$
    /// identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// A non-negative power is computed as by the [`Pow<u64>`](Pow) implementation. For a negative
    /// one, the matrix is inverted first, as by [`Inverse`], and the inverse is raised to $-k$,
    /// since inverting $A^{-k}$ would work on elements that have already grown $-k$-fold.
    ///
    /// # Worst-case complexity
    /// $T(n, b, k) = O(n^6 b^2 + n^4 b|k| \log (nb|k|) \log\log (nb|k|))$
    ///
    /// $M(n, b, k) = O(n^4 b + n^3 b|k|)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $b$ is the
    /// maximum number of significant bits of the numerators and denominators of its elements, and
    /// $k$ is `exp`: the inverse has elements of $O(nb)$ bits.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if `exp` is negative and the matrix is singular.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// assert_eq!(a.pow(5i64).to_string(), "[65/32, 29/16; 29/16, 9/8]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// assert_eq!(a.pow(-1i64).to_string(), "[0, 1; 1, -1/2]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// assert_eq!(a.pow(-5i64).to_string(), "[-9/8, 29/16; 29/16, -65/32]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// assert_eq!(
    ///     a.pow(-3i64).to_string(),
    ///     "[-504/625, -96/625; 576/625, -576/625]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[-2/3]").unwrap();
    /// assert_eq!(a.pow(-4i64).to_string(), "[81/16]");
    /// ```
    #[inline]
    fn pow(self, exp: i64) -> Self {
        assert_square(&self);
        pow_signed_owned(self, exp)
    }
}

impl Pow<i64> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Raises a square [`RationalMatrix`] to a signed power, taking it by reference.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$ when $k
    /// \geq 0$, and the product of $-k$ copies of $A^{-1}$ when $k < 0$; $A^0$ is the $n \times n$
    /// identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// A non-negative power is computed as by the [`Pow<u64>`](Pow) implementation. For a negative
    /// one, the matrix is inverted first, as by [`Inverse`], and the inverse is raised to $-k$,
    /// since inverting $A^{-k}$ would work on elements that have already grown $-k$-fold.
    ///
    /// # Worst-case complexity
    /// $T(n, b, k) = O(n^6 b^2 + n^4 b|k| \log (nb|k|) \log\log (nb|k|))$
    ///
    /// $M(n, b, k) = O(n^4 b + n^3 b|k|)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $b$ is the
    /// maximum number of significant bits of the numerators and denominators of its elements, and
    /// $k$ is `exp`: the inverse has elements of $O(nb)$ bits.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if `exp` is negative and the matrix is singular.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).pow(5i64).to_string(), "[65/32, 29/16; 29/16, 9/8]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).pow(-1i64).to_string(), "[0, 1; 1, -1/2]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).pow(-5i64).to_string(), "[-9/8, 29/16; 29/16, -65/32]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// assert_eq!(
    ///     (&a).pow(-3i64).to_string(),
    ///     "[-504/625, -96/625; 576/625, -576/625]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[-2/3]").unwrap();
    /// assert_eq!((&a).pow(-4i64).to_string(), "[81/16]");
    /// ```
    fn pow(self, exp: i64) -> RationalMatrix {
        assert_square(self);
        pow_signed_ref(self, exp)
    }
}

impl PowAssign<i64> for RationalMatrix {
    /// Raises a square [`RationalMatrix`] to a signed power in place.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$ when $k
    /// \geq 0$, and the product of $-k$ copies of $A^{-1}$ when $k < 0$; $A^0$ is the $n \times n$
    /// identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// A non-negative power is computed as by the [`Pow<u64>`](Pow) implementation. For a negative
    /// one, the matrix is inverted first, as by [`Inverse`], and the inverse is raised to $-k$,
    /// since inverting $A^{-k}$ would work on elements that have already grown $-k$-fold.
    ///
    /// # Worst-case complexity
    /// $T(n, b, k) = O(n^6 b^2 + n^4 b|k| \log (nb|k|) \log\log (nb|k|))$
    ///
    /// $M(n, b, k) = O(n^4 b + n^3 b|k|)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $b$ is the
    /// maximum number of significant bits of the numerators and denominators of its elements, and
    /// $k$ is `exp`: the inverse has elements of $O(nb)$ bits.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if `exp` is negative and the matrix is singular.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// a.pow_assign(5i64);
    /// assert_eq!(a.to_string(), "[65/32, 29/16; 29/16, 9/8]");
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1; 1, 0]").unwrap();
    /// a.pow_assign(-5i64);
    /// assert_eq!(a.to_string(), "[-9/8, 29/16; 29/16, -65/32]");
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// a.pow_assign(-3i64);
    /// assert_eq!(a.to_string(), "[-504/625, -96/625; 576/625, -576/625]");
    /// ```
    fn pow_assign(&mut self, exp: i64) {
        assert_square(self);
        if exp != 1 {
            *self = pow_signed_owned(take_matrix(self), exp);
        }
    }
}
