// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{EqTruncated, slices_eq_truncated};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Whether a coefficient is zero. This is a function rather than a closure because inside the impls
// below, a `where` bound on `Integer: PartialEq<T>` would capture a comparison with a literal.
fn integer_is_zero(x: &Integer) -> bool {
    *x == 0u32
}

impl EqTruncated for IntegerPolynomial {
    /// Determines whether an [`IntegerPolynomial`] and another agree below $x^{\mathrm{len}}$: that
    /// is, whether they have the same coefficient of $x^i$ for every $i$ less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    ///
    /// This is equivalent to `fmpz_poly_equal_trunc` from `fmpz_poly/equal_trunc.c`, FLINT 3.6.0.
    #[inline]
    fn eq_truncated(&self, other: &Self, len: u64) -> bool {
        slices_eq_truncated(
            &self.coefficients,
            &other.coefficients,
            len,
            integer_is_zero,
            |y| *y == 0u32,
            |x, y| x == y,
        )
    }
}

impl<T: PrimitiveUnsigned> EqTruncated<UnsignedPolynomial<T>> for IntegerPolynomial
where
    Integer: PartialEq<T>,
{
    /// Determines whether an [`IntegerPolynomial`] and an [`UnsignedPolynomial`] agree below
    /// $x^{\mathrm{len}}$: that is, whether they have the same coefficient of $x^i$ for every $i$
    /// less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    #[inline]
    fn eq_truncated(&self, other: &UnsignedPolynomial<T>, len: u64) -> bool {
        slices_eq_truncated(
            &self.coefficients,
            other.coefficients_asc(),
            len,
            integer_is_zero,
            |&y| y == T::ZERO,
            |x, y| x == y,
        )
    }
}

impl<T: PrimitiveUnsigned> EqTruncated<IntegerPolynomial> for UnsignedPolynomial<T>
where
    Integer: PartialEq<T>,
{
    /// Determines whether an [`UnsignedPolynomial`] and an [`IntegerPolynomial`] agree below
    /// $x^{\mathrm{len}}$: that is, whether they have the same coefficient of $x^i$ for every $i$
    /// less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    #[inline]
    fn eq_truncated(&self, other: &IntegerPolynomial, len: u64) -> bool {
        other.eq_truncated(self, len)
    }
}

impl EqTruncated<NaturalPolynomial> for IntegerPolynomial {
    /// Determines whether an [`IntegerPolynomial`] and a [`NaturalPolynomial`] agree below
    /// $x^{\mathrm{len}}$: that is, whether they have the same coefficient of $x^i$ for every $i$
    /// less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    #[inline]
    fn eq_truncated(&self, other: &NaturalPolynomial, len: u64) -> bool {
        slices_eq_truncated(
            &self.coefficients,
            other.coefficients_asc(),
            len,
            integer_is_zero,
            |y| *y == 0u32,
            |x, y| x == y,
        )
    }
}

impl EqTruncated<IntegerPolynomial> for NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] and an [`IntegerPolynomial`] agree below
    /// $x^{\mathrm{len}}$: that is, whether they have the same coefficient of $x^i$ for every $i$
    /// less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients below $x^{\mathrm{len}}$.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    #[inline]
    fn eq_truncated(&self, other: &IntegerPolynomial, len: u64) -> bool {
        other.eq_truncated(self, len)
    }
}
