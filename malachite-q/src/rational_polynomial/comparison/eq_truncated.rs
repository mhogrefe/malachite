// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::{DivExact, Gcd};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{EqTruncated, slices_eq_truncated};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;

// Whether a coefficient is zero. This is a function rather than a closure because inside the impls
// below, a `where` bound on `Integer: PartialEq<T>` would capture a comparison with a literal.
fn integer_is_zero(x: &Integer) -> bool {
    *x == 0u32
}

fn natural_is_zero(x: &Natural) -> bool {
    *x == 0u32
}

// Converts the denominator for the impl over `UnsignedPolynomial<T>`, whose `Integer: From<T>`
// bound would otherwise capture the conversion.
fn natural_to_integer(x: &Natural) -> Integer {
    Integer::from(x)
}

// Whether x / d equals y, which is whether x equals y * d.
fn eq_scaled(x: &Integer, y: Integer, d: &Integer) -> bool {
    *x == y * d
}

impl EqTruncated for RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] and another agree below $x^{\mathrm{len}}$: that
    /// is, whether they have the same coefficient of $x^i$ for every $i$ less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients below $x^{\mathrm{len}}$ and of the denominators.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    ///
    /// This is equivalent to `fmpq_poly_equal_trunc` from `fmpq_poly/equal_trunc.c`, FLINT 3.6.0.
    fn eq_truncated(&self, other: &Self, len: u64) -> bool {
        let xs = self.numerator.coefficients_asc();
        let ys = other.numerator.coefficients_asc();
        if self.denominator == other.denominator {
            return slices_eq_truncated(xs, ys, len, integer_is_zero, integer_is_zero, |x, y| {
                x == y
            });
        }
        // Compare x / d_x with y / d_y as x * (d_y / g) with y * (d_x / g), where g is the
        // denominators' gcd.
        let gcd = (&self.denominator).gcd(&other.denominator);
        let x_factor = Integer::from((&other.denominator).div_exact(&gcd));
        let y_factor = Integer::from((&self.denominator).div_exact(&gcd));
        slices_eq_truncated(xs, ys, len, integer_is_zero, integer_is_zero, |x, y| {
            x * &x_factor == y * &y_factor
        })
    }
}

impl<T: PrimitiveUnsigned> EqTruncated<UnsignedPolynomial<T>> for RationalPolynomial
where
    Integer: From<T> + PartialEq<T>,
{
    /// Determines whether a [`RationalPolynomial`] and an [`UnsignedPolynomial`] agree below
    /// $x^{\mathrm{len}}$: that is, whether they have the same coefficient of $x^i$ for every $i$
    /// less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$ and of the denominator.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    fn eq_truncated(&self, other: &UnsignedPolynomial<T>, len: u64) -> bool {
        let xs = self.numerator.coefficients_asc();
        let ys = other.coefficients_asc();
        if self.denominator == 1u32 {
            return slices_eq_truncated(
                xs,
                ys,
                len,
                integer_is_zero,
                |&y| y == T::ZERO,
                |x, y| x == y,
            );
        }
        // x / d equals y exactly when x equals y * d.
        let d = natural_to_integer(&self.denominator);
        slices_eq_truncated(
            xs,
            ys,
            len,
            integer_is_zero,
            |&y| y == T::ZERO,
            |x, &y| eq_scaled(x, Integer::from(y), &d),
        )
    }
}

impl<T: PrimitiveUnsigned> EqTruncated<RationalPolynomial> for UnsignedPolynomial<T>
where
    Integer: From<T> + PartialEq<T>,
{
    /// Determines whether an [`UnsignedPolynomial`] and a [`RationalPolynomial`] agree below
    /// $x^{\mathrm{len}}$: that is, whether they have the same coefficient of $x^i$ for every $i$
    /// less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$ and of the denominator.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    #[inline]
    fn eq_truncated(&self, other: &RationalPolynomial, len: u64) -> bool {
        other.eq_truncated(self, len)
    }
}

impl EqTruncated<NaturalPolynomial> for RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] and a [`NaturalPolynomial`] agree below
    /// $x^{\mathrm{len}}$: that is, whether they have the same coefficient of $x^i$ for every $i$
    /// less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$ and of the denominator.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    fn eq_truncated(&self, other: &NaturalPolynomial, len: u64) -> bool {
        let xs = self.numerator.coefficients_asc();
        let ys = other.coefficients_asc();
        if self.denominator == 1u32 {
            return slices_eq_truncated(xs, ys, len, integer_is_zero, natural_is_zero, |x, y| {
                x == y
            });
        }
        // x / d equals y exactly when x equals y * d.
        slices_eq_truncated(xs, ys, len, integer_is_zero, natural_is_zero, |x, y| {
            *x == y * &self.denominator
        })
    }
}

impl EqTruncated<RationalPolynomial> for NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] and a [`RationalPolynomial`] agree below
    /// $x^{\mathrm{len}}$: that is, whether they have the same coefficient of $x^i$ for every $i$
    /// less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$ and of the denominator.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    #[inline]
    fn eq_truncated(&self, other: &RationalPolynomial, len: u64) -> bool {
        other.eq_truncated(self, len)
    }
}

impl EqTruncated<IntegerPolynomial> for RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] and an [`IntegerPolynomial`] agree below
    /// $x^{\mathrm{len}}$: that is, whether they have the same coefficient of $x^i$ for every $i$
    /// less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$ and of the denominator.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    fn eq_truncated(&self, other: &IntegerPolynomial, len: u64) -> bool {
        let xs = self.numerator.coefficients_asc();
        let ys = other.coefficients_asc();
        if self.denominator == 1u32 {
            return slices_eq_truncated(xs, ys, len, integer_is_zero, integer_is_zero, |x, y| {
                x == y
            });
        }
        // x / d equals y exactly when x equals y * d.
        let d = Integer::from(&self.denominator);
        slices_eq_truncated(xs, ys, len, integer_is_zero, integer_is_zero, |x, y| {
            *x == y * &d
        })
    }
}

impl EqTruncated<RationalPolynomial> for IntegerPolynomial {
    /// Determines whether an [`IntegerPolynomial`] and a [`RationalPolynomial`] agree below
    /// $x^{\mathrm{len}}$: that is, whether they have the same coefficient of $x^i$ for every $i$
    /// less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$ and of the denominator.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    #[inline]
    fn eq_truncated(&self, other: &RationalPolynomial, len: u64) -> bool {
        other.eq_truncated(self, len)
    }
}
