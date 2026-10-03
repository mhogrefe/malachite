// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivisibleBy, Gcd, GcdAssign};
use crate::num::conversion::traits::ExactFrom;
use crate::vars::{Var, VarScheme};
use alloc::string::String;
use alloc::vec::Vec;

/// What every polynomial type has in common: a polynomial in one variable, stored as its
/// coefficients in ascending order with no trailing zeros.
///
/// The functions here are the ones whose meaning does not depend on what the coefficients are. Each
/// implementation documents its own complexity and gives its own examples.
// There is no `is_empty` to go with `len`. Whether a polynomial is zero is asked by comparing it
// with `ZERO`, which every polynomial type has, and a second spelling of that question, under a
// name that suits a collection better than a polynomial, would add nothing.
#[allow(clippy::len_without_is_empty)]
pub trait Polynomial: Sized {
    /// The type of a coefficient.
    type Coefficient;

    /// What [`coefficient`](Self::coefficient) and
    /// [`leading_coefficient`](Self::leading_coefficient) return.
    ///
    /// This is a reference to a [`Coefficient`](Self::Coefficient) when the polynomial holds its
    /// coefficients as they are, and a [`Coefficient`](Self::Coefficient) itself when a coefficient
    /// is cheap to copy or has to be built on demand.
    type CoefficientOutput<'a>
    where
        Self: 'a;

    /// The constant polynomial 1.
    ///
    /// This is a function rather than an associated constant, because a polynomial holds its
    /// coefficients in a [`Vec`] and a [`Vec`] with anything in it cannot be built at compile time.
    fn one() -> Self;

    /// The constant polynomial 2.
    ///
    /// This is a function rather than an associated constant, for the reason given by
    /// [`one`](Self::one).
    fn two() -> Self;

    /// The polynomial $x$, of degree 1 with leading coefficient 1 and constant term 0.
    ///
    /// This is a function rather than an associated constant, for the reason given by
    /// [`one`](Self::one).
    fn x() -> Self;

    /// Converts a [`Vec`] of coefficients, in ascending order, to a polynomial.
    ///
    /// The first coefficient is the constant term. Trailing zeros are dropped, so the [`Vec`] may
    /// end with as many as it likes, and the empty [`Vec`] is the zero polynomial.
    fn from_coefficients_asc(coefficients: Vec<Self::Coefficient>) -> Self;

    /// Converts a polynomial to a [`Vec`] of its coefficients, in ascending order.
    ///
    /// The [`Vec`] is what [`from_coefficients_asc`](Self::from_coefficients_asc) would take back.
    /// It holds no trailing zeros, and for the zero polynomial it is empty.
    fn into_coefficients_asc(self) -> Vec<Self::Coefficient>;

    /// Returns the degree of a polynomial.
    ///
    /// The zero polynomial has no degree, and gives `None`. Every other polynomial's degree is the
    /// index of its leading coefficient, so that a nonzero constant has degree 0.
    fn degree(&self) -> Option<u64>;

    /// Returns the length of a polynomial: the number of coefficients it holds.
    ///
    /// A polynomial holds no trailing zeros, so its length is one more than its degree, and the
    /// zero polynomial, which has no degree, has length 0.
    fn len(&self) -> u64;

    /// Returns one of a polynomial's coefficients.
    ///
    /// The index is the power of the variable the coefficient belongs to, so that index 0 gives the
    /// constant term. An index past the degree gives zero.
    fn coefficient(&self, index: u64) -> Self::CoefficientOutput<'_>;

    /// Returns a polynomial's leading coefficient.
    ///
    /// The zero polynomial has no leading coefficient, and gives zero.
    fn leading_coefficient(&self) -> Self::CoefficientOutput<'_>;

    /// Determines whether a polynomial is monic: nonzero, with leading coefficient 1.
    ///
    /// The zero polynomial is not monic.
    fn is_monic(&self) -> bool;

    /// Mutates one of a polynomial's coefficients using a provided closure, and then returns
    /// whatever the closure returns.
    ///
    /// An index past the degree is not an error: the closure is handed a zero, and the polynomial
    /// grows to hold the result. Trailing zeros left behind by the closure are dropped.
    fn mutate_coefficient<F: FnOnce(&mut Self::Coefficient) -> T, T>(
        &mut self,
        index: u64,
        f: F,
    ) -> T;

    /// Sets the coefficients of $x^i$ for $i$ in `start..end` to zero.
    ///
    /// Indices past the degree are allowed; the coefficients there are zero already. Zeroing the
    /// leading coefficient lowers the degree.
    ///
    /// # Panics
    /// Panics if `start > end`.
    fn zero_coefficients(&mut self, start: u64, end: u64);

    /// Truncates a polynomial to its first `len` coefficients, taking the polynomial by reference
    /// and returning the result.
    ///
    /// The result is the polynomial reduced modulo $x^{\mathrm{len}}$: every term of degree `len`
    /// or more is dropped. A polynomial with at most `len` coefficients is returned unchanged.
    ///
    /// Unlike [`Vec::truncate`], this does not modify the polynomial; see
    /// [`truncate_assign`](Self::truncate_assign) for that.
    fn truncate(&self, len: u64) -> Self;

    /// Truncates a polynomial to its first `len` coefficients, in place.
    ///
    /// See [`truncate`](Self::truncate).
    fn truncate_assign(&mut self, len: u64);

    /// Reverses the coefficients of a polynomial, considered as having length `len`, taking the
    /// polynomial by reference.
    ///
    /// The polynomial is first truncated, or padded with zeros, to exactly `len` coefficients, and
    /// those are then reversed, so that the result's coefficient of $x^i$ is the polynomial's
    /// coefficient of $x^{\mathrm{len} - 1 - i}$:
    ///
    /// $$
    /// f(p, n) = x^{n-1} \left( p \bmod x^n \right)\!\left(\frac{1}{x}\right).
    /// $$
    ///
    /// A polynomial holds no trailing zeros, so the result may have fewer than `len` coefficients.
    fn reverse(&self, len: u64) -> Self;

    /// Reverses the coefficients of a polynomial, considered as having length `len`, in place.
    ///
    /// See [`reverse`](Self::reverse).
    fn reverse_assign(&mut self, len: u64);

    /// Converts a polynomial to a [`String`], naming its variable with any [`VarScheme`].
    fn to_string_with<S: VarScheme + ?Sized>(&self, var: Var<'_, S>) -> String;

    /// Converts a polynomial to a LaTeX math-mode fragment, naming its variable with any
    /// [`VarScheme`].
    fn to_latex_string_with<S: VarScheme + ?Sized>(&self, var: Var<'_, S>) -> String;

    /// Converts a polynomial to a Typst math-mode fragment, naming its variable with any
    /// [`VarScheme`].
    fn to_typst_string_with<S: VarScheme + ?Sized>(&self, var: Var<'_, S>) -> String;

    /// Converts a [`str`] to a polynomial, reading its variable with any [`VarScheme`].
    ///
    /// Returns `None` if the string is not a polynomial in that variable.
    fn from_string_with<S: VarScheme + ?Sized>(var: Var<'_, S>, s: &str) -> Option<Self>;
}

/// Determines whether two polynomials agree below a given power of the variable.
///
/// This is equality of the two polynomials truncated to their first `len` coefficients, decided
/// without building either truncation.
pub trait EqTruncated<Rhs: ?Sized = Self> {
    /// Determines whether two polynomials have the same coefficient of $x^i$ for every $i$ less
    /// than `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = (p \bmod x^n = q \bmod x^n).
    /// $$
    fn eq_truncated(&self, other: &Rhs, len: u64) -> bool;
}

/// Adds two polynomials, keeping only the coefficients of $x^i$ for $i$ less than a given length.
///
/// This is the sum of the two polynomials truncated to their first `len` coefficients, which is
/// also the truncation of their sum.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials modulo $x^n$, applied to
/// the images of the two polynomials there. The polynomials need not already be truncated: they may
/// have any number of coefficients, and only the first `len` of each are read.
pub trait AddTruncated<Rhs = Self> {
    type Output;

    /// Adds two polynomials and truncates the sum to its first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n) = (p + q) \bmod x^n.
    /// $$
    fn add_truncated(self, other: Rhs, len: u64) -> Self::Output;
}

/// Adds a polynomial to another in place, keeping only the coefficients of $x^i$ for $i$ less than
/// a given length.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials modulo $x^n$, applied to
/// the images of the two polynomials there. The polynomials need not already be truncated: they may
/// have any number of coefficients, and only the first `len` of each are read.
pub trait AddTruncatedAssign<Rhs = Self> {
    /// Adds a polynomial to `self` and truncates the sum to its first `len` coefficients.
    ///
    /// $$
    /// p \gets (p + q) \bmod x^n.
    /// $$
    fn add_truncated_assign(&mut self, other: Rhs, len: u64);
}

/// Subtracts one polynomial from another, keeping only the coefficients of $x^i$ for $i$ less than
/// a given length.
///
/// This is the difference of the two polynomials truncated to their first `len` coefficients, which
/// is also the truncation of their difference.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials modulo $x^n$, applied to
/// the images of the two polynomials there. The polynomials need not already be truncated: they may
/// have any number of coefficients, and only the first `len` of each are read.
pub trait SubTruncated<Rhs = Self> {
    type Output;

    /// Subtracts one polynomial from another and truncates the difference to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, q, n) = (p - q) \bmod x^n.
    /// $$
    fn sub_truncated(self, other: Rhs, len: u64) -> Self::Output;
}

/// Subtracts a polynomial from another in place, keeping only the coefficients of $x^i$ for $i$
/// less than a given length.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials modulo $x^n$, applied to
/// the images of the two polynomials there. The polynomials need not already be truncated: they may
/// have any number of coefficients, and only the first `len` of each are read.
pub trait SubTruncatedAssign<Rhs = Self> {
    /// Subtracts a polynomial from `self` and truncates the difference to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets (p - q) \bmod x^n.
    /// $$
    fn sub_truncated_assign(&mut self, other: Rhs, len: u64);
}

/// Multiplies two polynomials, keeping only the coefficients of $x^i$ for $i$ less than a given
/// length.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials modulo $x^n$, applied
/// to the images of the two polynomials there. The polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait MulTruncated<Rhs = Self> {
    type Output;

    /// Multiplies two polynomials and truncates the product to its first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    fn mul_truncated(self, other: Rhs, len: u64) -> Self::Output;
}

/// Multiplies a polynomial by another in place, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials modulo $x^n$, applied
/// to the images of the two polynomials there. The polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait MulTruncatedAssign<Rhs = Self> {
    /// Multiplies `self` by a polynomial and truncates the product to its first `len` coefficients.
    ///
    /// $$
    /// p \gets pq \bmod x^n.
    /// $$
    fn mul_truncated_assign(&mut self, other: Rhs, len: u64);
}

/// Squares a polynomial, keeping only the coefficients of $x^i$ for $i$ less than a given length.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials modulo $x^n$, applied to
/// the image of the polynomial there. The polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait SquareTruncated {
    type Output;

    /// Squares a polynomial and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// f(p, n) = p^2 \bmod x^n.
    /// $$
    fn square_truncated(self, len: u64) -> Self::Output;
}

/// Squares a polynomial in place, keeping only the coefficients of $x^i$ for $i$ less than a given
/// length.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials modulo $x^n$, applied to
/// the image of the polynomial there. The polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait SquareTruncatedAssign {
    /// Squares `self` and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// p \gets p^2 \bmod x^n.
    /// $$
    fn square_truncated_assign(&mut self, len: u64);
}

/// Raises a polynomial to a power, keeping only the coefficients of $x^i$ for $i$ less than a given
/// length.
///
/// With $n$ equal to `len`, this is powering in the ring of polynomials modulo $x^n$, applied to
/// the image of the polynomial there. The polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait PowTruncated {
    type Output;

    /// Raises a polynomial to the power `exp` and truncates the power to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, e, n) = p^e \bmod x^n.
    /// $$
    fn pow_truncated(self, exp: u64, len: u64) -> Self::Output;
}

/// Raises a polynomial to a power in place, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length.
///
/// With $n$ equal to `len`, this is powering in the ring of polynomials modulo $x^n$, applied to
/// the image of the polynomial there. The polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait PowTruncatedAssign {
    /// Raises `self` to the power `exp` and truncates the power to its first `len` coefficients.
    ///
    /// $$
    /// p \gets p^e \bmod x^n.
    /// $$
    fn pow_truncated_assign(&mut self, exp: u64, len: u64);
}

/// Adds two polynomials modulo $2^k$, keeping only the coefficients of $x^i$ for $i$ less than a
/// given length. The coefficients of both must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2AddTruncated<Rhs = Self> {
    type Output;

    /// Adds two polynomials modulo $2^k$ and truncates the sum to its first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n, k) = ((p + q) \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_add_truncated(self, other: Rhs, len: u64, pow: u64) -> Self::Output;
}

/// Adds a polynomial to another modulo $2^k$ in place, keeping only the coefficients of $x^i$ for
/// $i$ less than a given length. The coefficients of both must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2AddTruncatedAssign<Rhs = Self> {
    /// Adds a polynomial to `self` modulo $2^k$ and truncates the sum to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets ((p + q) \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_add_truncated_assign(&mut self, other: Rhs, len: u64, pow: u64);
}

/// Subtracts one polynomial from another modulo $2^k$, keeping only the coefficients of $x^i$ for
/// $i$ less than a given length. The coefficients of both must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2SubTruncated<Rhs = Self> {
    type Output;

    /// Subtracts one polynomial from another modulo $2^k$ and truncates the difference to its first
    /// `len` coefficients.
    ///
    /// $$
    /// f(p, q, n, k) = ((p - q) \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_sub_truncated(self, other: Rhs, len: u64, pow: u64) -> Self::Output;
}

/// Subtracts a polynomial from another modulo $2^k$ in place, keeping only the coefficients of
/// $x^i$ for $i$ less than a given length. The coefficients of both must already be reduced modulo
/// $2^k$.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2SubTruncatedAssign<Rhs = Self> {
    /// Subtracts a polynomial from `self` modulo $2^k$ and truncates the difference to its first
    /// `len` coefficients.
    ///
    /// $$
    /// p \gets ((p - q) \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_sub_truncated_assign(&mut self, other: Rhs, len: u64, pow: u64);
}

/// Multiplies two polynomials modulo $2^k$, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length. The coefficients of both must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials with coefficients
/// modulo $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2MulTruncated<Rhs = Self> {
    type Output;

    /// Multiplies two polynomials modulo $2^k$ and truncates the product to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_mul_truncated(self, other: Rhs, len: u64, pow: u64) -> Self::Output;
}

/// Multiplies a polynomial by another modulo $2^k$ in place, keeping only the coefficients of $x^i$
/// for $i$ less than a given length. The coefficients of both must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials with coefficients
/// modulo $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2MulTruncatedAssign<Rhs = Self> {
    /// Multiplies `self` by a polynomial modulo $2^k$ and truncates the product to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_mul_truncated_assign(&mut self, other: Rhs, len: u64, pow: u64);
}

/// Squares a polynomial modulo $2^k$, keeping only the coefficients of $x^i$ for $i$ less than a
/// given length. The coefficients must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the
/// coefficients, which must already be reduced, the polynomial need not already be truncated: it
/// may have any number of coefficients, and only the first `len` are read.
pub trait ModPowerOf2SquareTruncated {
    type Output;

    /// Squares a polynomial modulo $2^k$ and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// f(p, n, k) = (p^2 \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_square_truncated(self, len: u64, pow: u64) -> Self::Output;
}

/// Squares a polynomial modulo $2^k$ in place, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length. The coefficients must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the
/// coefficients, which must already be reduced, the polynomial need not already be truncated: it
/// may have any number of coefficients, and only the first `len` are read.
pub trait ModPowerOf2SquareTruncatedAssign {
    /// Squares `self` modulo $2^k$ and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// p \gets (p^2 \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_square_truncated_assign(&mut self, len: u64, pow: u64);
}

/// Adds two polynomials modulo $m$, keeping only the coefficients of $x^i$ for $i$ less than a
/// given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModAddTruncated<Rhs = Self, M = Self> {
    type Output;

    /// Adds two polynomials modulo $m$ and truncates the sum to its first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n, m) = ((p + q) \bmod x^n) \bmod m.
    /// $$
    fn mod_add_truncated(self, other: Rhs, len: u64, m: M) -> Self::Output;
}

/// Adds a polynomial to another modulo $m$ in place, keeping only the coefficients of $x^i$ for $i$
/// less than a given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModAddTruncatedAssign<Rhs = Self, M = Self> {
    /// Adds a polynomial to `self` modulo $m$ and truncates the sum to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets ((p + q) \bmod x^n) \bmod m.
    /// $$
    fn mod_add_truncated_assign(&mut self, other: Rhs, len: u64, m: M);
}

/// Subtracts one polynomial from another modulo $m$, keeping only the coefficients of $x^i$ for $i$
/// less than a given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModSubTruncated<Rhs = Self, M = Self> {
    type Output;

    /// Subtracts one polynomial from another modulo $m$ and truncates the difference to its first
    /// `len` coefficients.
    ///
    /// $$
    /// f(p, q, n, m) = ((p - q) \bmod x^n) \bmod m.
    /// $$
    fn mod_sub_truncated(self, other: Rhs, len: u64, m: M) -> Self::Output;
}

/// Subtracts a polynomial from another modulo $m$ in place, keeping only the coefficients of $x^i$
/// for $i$ less than a given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModSubTruncatedAssign<Rhs = Self, M = Self> {
    /// Subtracts a polynomial from `self` modulo $m$ and truncates the difference to its first
    /// `len` coefficients.
    ///
    /// $$
    /// p \gets ((p - q) \bmod x^n) \bmod m.
    /// $$
    fn mod_sub_truncated_assign(&mut self, other: Rhs, len: u64, m: M);
}

/// Multiplies two polynomials modulo $m$, keeping only the coefficients of $x^i$ for $i$ less than
/// a given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials with coefficients
/// modulo $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModMulTruncated<Rhs = Self, M = Self> {
    type Output;

    /// Multiplies two polynomials modulo $m$ and truncates the product to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
    /// $$
    fn mod_mul_truncated(self, other: Rhs, len: u64, m: M) -> Self::Output;
}

/// Multiplies a polynomial by another modulo $m$ in place, keeping only the coefficients of $x^i$
/// for $i$ less than a given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials with coefficients
/// modulo $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModMulTruncatedAssign<Rhs = Self, M = Self> {
    /// Multiplies `self` by a polynomial modulo $m$ and truncates the product to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod m.
    /// $$
    fn mod_mul_truncated_assign(&mut self, other: Rhs, len: u64, m: M);
}

/// Squares a polynomial modulo $m$, keeping only the coefficients of $x^i$ for $i$ less than a
/// given length. The coefficients must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the coefficients,
/// which must already be reduced, the polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait ModSquareTruncated<M = Self> {
    type Output;

    /// Squares a polynomial modulo $m$ and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// f(p, n, m) = (p^2 \bmod x^n) \bmod m.
    /// $$
    fn mod_square_truncated(self, len: u64, m: M) -> Self::Output;
}

/// Squares a polynomial modulo $m$ in place, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length. The coefficients must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the coefficients,
/// which must already be reduced, the polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait ModSquareTruncatedAssign<M = Self> {
    /// Squares `self` modulo $m$ and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// p \gets (p^2 \bmod x^n) \bmod m.
    /// $$
    fn mod_square_truncated_assign(&mut self, len: u64, m: M);
}

/// Computes the square of a polynomial's $L^2$ norm: the sum of the squares of its coefficients.
///
/// This is exact, unlike the norm itself, which is usually irrational.
pub trait L2NormSquared {
    type Output;

    /// Computes the sum of the squares of a polynomial's coefficients.
    ///
    /// $$
    /// f(p) = \sum_i p_i^2.
    /// $$
    fn l2_norm_squared(self) -> Self::Output;
}

/// Computes the floor of a polynomial's $L^2$ norm: the floor of the square root of the sum of the
/// squares of its coefficients.
pub trait FloorL2Norm {
    type Output;

    /// Computes the floor of the square root of the sum of the squares of a polynomial's
    /// coefficients.
    ///
    /// $$
    /// f(p) = \left \lfloor \sqrt{\sum_i p_i^2} \right \rfloor.
    /// $$
    fn floor_l2_norm(self) -> Self::Output;
}

/// Multiplies a polynomial by $x^n$, which moves every coefficient up by $n$ places.
pub trait MulPowerOfX {
    type Output;

    /// Multiplies a polynomial by $x^n$.
    ///
    /// $$
    /// f(p, n) = x^np.
    /// $$
    fn mul_power_of_x(self, n: u64) -> Self::Output;
}

/// Multiplies a polynomial by $x^n$ in place, which moves every coefficient up by $n$ places.
pub trait MulPowerOfXAssign {
    /// Multiplies a polynomial by $x^n$ in place.
    ///
    /// $$
    /// p \gets x^np.
    /// $$
    fn mul_power_of_x_assign(&mut self, n: u64);
}

/// Divides a polynomial by $x^n$, discarding the remainder: every coefficient moves down by $n$
/// places, and the lowest $n$ are dropped.
pub trait DivPowerOfX {
    type Output;

    /// Divides a polynomial by $x^n$, discarding the remainder.
    ///
    /// $$
    /// f(p, n) = \sum_{i \geq n} p_ix^{i-n}.
    /// $$
    fn div_power_of_x(self, n: u64) -> Self::Output;
}

/// Divides a polynomial by $x^n$ in place, discarding the remainder: every coefficient moves down
/// by $n$ places, and the lowest $n$ are dropped.
pub trait DivPowerOfXAssign {
    /// Divides a polynomial by $x^n$ in place, discarding the remainder.
    ///
    /// $$
    /// p \gets \sum_{i \geq n} p_ix^{i-n}.
    /// $$
    fn div_power_of_x_assign(&mut self, n: u64);
}

/// Composes a polynomial with $x^k$, giving $p(x^k)$: the coefficient of $x^i$ moves to $x^{ik}$.
///
/// When $k$ is 0, the result is the constant $p(1)$, the sum of the coefficients.
pub trait ComposePowerOfX {
    type Output;

    /// Composes a polynomial with $x^k$.
    ///
    /// $$
    /// f(p, k) = p(x^k).
    /// $$
    fn compose_power_of_x(self, k: u64) -> Self::Output;
}

/// Composes a polynomial with $x^k$ in place, replacing $p$ with $p(x^k)$.
///
/// When $k$ is 0, the result is the constant $p(1)$, the sum of the coefficients.
pub trait ComposePowerOfXAssign {
    /// Composes a polynomial with $x^k$ in place.
    ///
    /// $$
    /// p \gets p(x^k).
    /// $$
    fn compose_power_of_x_assign(&mut self, k: u64);
}

/// Computes the greatest common divisor of the exponents at which a polynomial has nonzero
/// coefficients.
///
/// This is the largest $k$ such that $p(x) = q(x^k)$ for some polynomial $q$, when $p$ is not
/// constant. A constant polynomial, including zero, gives 0: its only exponent with a nonzero
/// coefficient, if any, is 0, and the GCD of $\\{0\\}$ and of the empty set are both 0.
pub trait ExponentGcd {
    /// Computes the greatest common divisor of the exponents at which a polynomial has nonzero
    /// coefficients.
    ///
    /// $$
    /// f(p) = \gcd \\{i : p_i \neq 0\\}.
    /// $$
    fn exponent_gcd(&self) -> u64;
}

// Computes the GCD of the indices of the elements of `xs` that are not zero, where `xs` holds a
// polynomial's coefficients in ascending order with a nonzero last element, if any.
//
// The index of the last element always takes part, so the search starts from the GCD of it and the
// first nonzero index after 0. From then on, only the indices that are not multiples of the current
// GCD can lower it, so each block of `gcd` indices is scanned but for its last one; the search
// stops as soon as the GCD is 1.
//
// This is equivalent to `_fmpz_poly_deflation` from `fmpz_poly/deflation.c`, FLINT 3.6.0, except
// that a constant gives 0 rather than 1.
#[doc(hidden)]
pub fn slice_exponent_gcd<T>(xs: &[T], is_zero: impl Fn(&T) -> bool) -> u64 {
    let len = xs.len();
    if len <= 1 {
        return 0;
    }
    let mut i = 1;
    while is_zero(&xs[i]) {
        i += 1;
    }
    let mut gcd = (len - 1).gcd(i);
    while gcd > 1 && i + gcd < len {
        let mut j = 0;
        while j + 1 < gcd {
            i += 1;
            if !is_zero(&xs[i]) {
                gcd.gcd_assign(i);
            }
            j += 1;
        }
        if j + 1 == gcd {
            i += 1;
        }
    }
    u64::exact_from(gcd)
}

/// Deflates a polynomial by $n$, giving the polynomial $q$ with $q(x^n) = p(x)$: the coefficient of
/// $x^{in}$ moves to $x^i$.
///
/// This is the inverse of [`ComposePowerOfX`]. It exists only when every exponent at which $p$ has
/// a nonzero coefficient is a multiple of $n$, that is, when $n$ divides
/// [`exponent_gcd`](ExponentGcd::exponent_gcd); implementations panic otherwise, and when $n$ is 0.
/// A constant polynomial deflates to itself for every positive $n$.
pub trait DeflatePowerOfX {
    type Output;

    /// Deflates a polynomial by $n$.
    ///
    /// $$
    /// f(p, n) = q, \quad \text{where} \quad q(x^n) = p(x).
    /// $$
    fn deflate_power_of_x(self, n: u64) -> Self::Output;
}

/// Deflates a polynomial by $n$ in place, replacing $p$ with the polynomial $q$ such that $q(x^n) =
/// p(x)$.
///
/// This is the inverse of [`ComposePowerOfXAssign`]. It exists only when every exponent at which
/// $p$ has a nonzero coefficient is a multiple of $n$, that is, when $n$ divides
/// [`exponent_gcd`](ExponentGcd::exponent_gcd); implementations panic otherwise, and when $n$ is 0.
/// A constant polynomial deflates to itself for every positive $n$.
pub trait DeflatePowerOfXAssign {
    /// Deflates a polynomial by $n$ in place.
    ///
    /// $$
    /// p \gets q, \quad \text{where} \quad q(x^n) = p(x).
    /// $$
    fn deflate_power_of_x_assign(&mut self, n: u64);
}

// Checks that the polynomial whose coefficients `xs` holds, in ascending order with a nonzero last
// element if any, can be deflated by `n`. Returns `n` as a `usize`, or `None` when deflating
// changes nothing: when `n` is 1 or the polynomial is constant.
fn deflation_step<T>(xs: &[T], n: u64, is_zero: impl Fn(&T) -> bool) -> Option<usize> {
    assert_ne!(n, 0, "Cannot deflate a polynomial by 0");
    if n == 1 || xs.len() <= 1 {
        return None;
    }
    assert!(
        slice_exponent_gcd(xs, is_zero).divisible_by(n),
        "Cannot deflate a polynomial by {n}: it has a nonzero coefficient at an exponent that is \
        not a multiple of {n}"
    );
    // n divides the degree, so it fits in a usize.
    Some(usize::exact_from(n))
}

// Deflates, by `n`, the polynomial whose coefficients `xs` holds in ascending order with a nonzero
// last element if any, in place. The coefficient at index `i * n` moves to index `i`, from the
// bottom up, so each lands on a place whose old value is no longer needed; then the vector is
// truncated. The leading coefficient stays nonzero.
//
// This is equivalent to `fmpz_poly_deflate` from `fmpz_poly/deflate.c`, FLINT 3.6.0, except that it
// panics when some nonzero coefficient is not at a multiple of `n`.
#[doc(hidden)]
pub fn vec_deflate_power_of_x<T>(xs: &mut Vec<T>, n: u64, is_zero: impl Fn(&T) -> bool) {
    if let Some(n) = deflation_step(xs, n, is_zero) {
        let new_len = (xs.len() - 1) / n + 1;
        for i in 1..new_len {
            xs.swap(i, i * n);
        }
        xs.truncate(new_len);
    }
}

// Deflates, by `n`, the polynomial whose coefficients `xs` holds in ascending order with a nonzero
// last element if any, returning the coefficients of the result.
//
// This is equivalent to `fmpz_poly_deflate` from `fmpz_poly/deflate.c`, FLINT 3.6.0, except that it
// panics when some nonzero coefficient is not at a multiple of `n`.
#[doc(hidden)]
pub fn slice_deflate_power_of_x<T: Clone>(
    xs: &[T],
    n: u64,
    is_zero: impl Fn(&T) -> bool,
) -> Vec<T> {
    match deflation_step(xs, n, is_zero) {
        None => xs.to_vec(),
        Some(n) => xs.iter().step_by(n).cloned().collect(),
    }
}

// Determines whether two coefficient slices, each holding a polynomial's coefficients in ascending
// order, agree below index `len`.
//
// Only the first `len` coefficients of each count. Where one polynomial has more of those than the
// other, the extra ones must be zero, since the other polynomial's coefficients there are; where
// both have them, `eq` decides. Nothing is allocated.
//
// This is equivalent to `fmpz_poly_equal_trunc` from `fmpz_poly/equal_trunc.c`, FLINT 3.6.0, with
// `eq` and the zero tests standing in for `fmpz_equal` and `fmpz_is_zero`.
#[doc(hidden)]
pub fn slices_eq_truncated<A, B>(
    xs: &[A],
    ys: &[B],
    len: u64,
    x_is_zero: impl Fn(&A) -> bool,
    y_is_zero: impl Fn(&B) -> bool,
    eq: impl Fn(&A, &B) -> bool,
) -> bool {
    let len = usize::try_from(len).unwrap_or(usize::MAX);
    let xs = &xs[..len.min(xs.len())];
    let ys = &ys[..len.min(ys.len())];
    let common = xs.len().min(ys.len());
    xs[common..].iter().all(x_is_zero)
        && ys[common..].iter().all(y_is_zero)
        && xs[..common]
            .iter()
            .zip(&ys[..common])
            .all(|(x, y)| eq(x, y))
}

/// Computes the derivative of a polynomial, $\sum_i ia_ix^{i-1}$.
pub trait Derivative {
    type Output;

    /// Computes the derivative of a polynomial.
    ///
    /// $$
    /// f(p) = p'.
    /// $$
    fn derivative(self) -> Self::Output;
}

/// Replaces a polynomial with its derivative, $\sum_i ia_ix^{i-1}$.
pub trait DerivativeAssign {
    /// Replaces a polynomial with its derivative.
    ///
    /// $$
    /// p \gets p'.
    /// $$
    fn derivative_assign(&mut self);
}

/// Computes the integral of a polynomial whose constant term is zero, $\sum_i a_ix^{i+1}/(i+1)$.
pub trait Integral {
    type Output;

    /// Computes the integral of a polynomial whose constant term is zero.
    ///
    /// $$
    /// f(p) = \int_0^x p(t)\,dt.
    /// $$
    fn integral(self) -> Self::Output;
}

/// Replaces a polynomial with its integral whose constant term is zero, $\sum_i a_ix^{i+1}/(i+1)$.
pub trait IntegralAssign {
    /// Replaces a polynomial with its integral whose constant term is zero.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt.
    /// $$
    fn integral_assign(&mut self);
}

/// Computes the derivative of a polynomial modulo $m$. The coefficients must already be reduced
/// modulo $m$.
///
/// Each coefficient $a_i$ becomes $ia_i \bmod m$, which can be zero even when $a_i$ is not, so the
/// derivative can lose any number of degrees.
pub trait ModDerivative<M> {
    type Output;

    /// Computes the derivative of a polynomial modulo $m$.
    ///
    /// $$
    /// f(p, m) = p' \bmod m.
    /// $$
    fn mod_derivative(self, m: M) -> Self::Output;
}

/// Replaces a polynomial with its derivative modulo $m$. The coefficients must already be reduced
/// modulo $m$.
///
/// Each coefficient $a_i$ becomes $ia_i \bmod m$, which can be zero even when $a_i$ is not, so the
/// derivative can lose any number of degrees.
pub trait ModDerivativeAssign<M> {
    /// Replaces a polynomial with its derivative modulo $m$.
    ///
    /// $$
    /// p \gets p' \bmod m.
    /// $$
    fn mod_derivative_assign(&mut self, m: M);
}

/// Computes the integral modulo $m$ of a polynomial whose constant term is zero. The coefficients
/// must already be reduced modulo $m$.
///
/// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ for which that
/// coefficient is nonzero must be a unit modulo $m$.
pub trait ModIntegral<M> {
    type Output;

    /// Computes the integral modulo $m$ of a polynomial whose constant term is zero.
    ///
    /// $$
    /// f(p, m) = \int_0^x p(t)\,dt \bmod m.
    /// $$
    fn mod_integral(self, m: M) -> Self::Output;
}

/// Replaces a polynomial with its integral modulo $m$ whose constant term is zero. The coefficients
/// must already be reduced modulo $m$.
///
/// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ for which that
/// coefficient is nonzero must be a unit modulo $m$.
pub trait ModIntegralAssign<M> {
    /// Replaces a polynomial with its integral modulo $m$ whose constant term is zero.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt \bmod m.
    /// $$
    fn mod_integral_assign(&mut self, m: M);
}

/// Computes the derivative of a polynomial modulo $2^k$. The coefficients must already be reduced
/// modulo $2^k$.
///
/// Each coefficient $a_i$ becomes $ia_i \bmod 2^k$, which can be zero even when $a_i$ is not, so
/// the derivative can lose any number of degrees.
pub trait ModPowerOf2Derivative {
    type Output;

    /// Computes the derivative of a polynomial modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p' \bmod 2^k.
    /// $$
    fn mod_power_of_2_derivative(self, pow: u64) -> Self::Output;
}

/// Replaces a polynomial with its derivative modulo $2^k$. The coefficients must already be reduced
/// modulo $2^k$.
///
/// Each coefficient $a_i$ becomes $ia_i \bmod 2^k$, which can be zero even when $a_i$ is not, so
/// the derivative can lose any number of degrees.
pub trait ModPowerOf2DerivativeAssign {
    /// Replaces a polynomial with its derivative modulo $2^k$.
    ///
    /// $$
    /// p \gets p' \bmod 2^k.
    /// $$
    fn mod_power_of_2_derivative_assign(&mut self, pow: u64);
}

/// Computes the integral modulo $2^k$ of a polynomial whose constant term is zero. The coefficients
/// must already be reduced modulo $2^k$.
///
/// The coefficient of $x^{i-1}$ is divided by $i$ and moved to $x^i$. Only odd numbers are units
/// modulo $2^k$, so every nonzero coefficient must belong to an even power of $x$.
pub trait ModPowerOf2Integral {
    type Output;

    /// Computes the integral modulo $2^k$ of a polynomial whose constant term is zero.
    ///
    /// $$
    /// f(p, k) = \int_0^x p(t)\,dt \bmod 2^k.
    /// $$
    fn mod_power_of_2_integral(self, pow: u64) -> Self::Output;
}

/// Replaces a polynomial with its integral modulo $2^k$ whose constant term is zero. The
/// coefficients must already be reduced modulo $2^k$.
///
/// The coefficient of $x^{i-1}$ is divided by $i$ and moved to $x^i$. Only odd numbers are units
/// modulo $2^k$, so every nonzero coefficient must belong to an even power of $x$.
pub trait ModPowerOf2IntegralAssign {
    /// Replaces a polynomial with its integral modulo $2^k$ whose constant term is zero.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt \bmod 2^k.
    /// $$
    fn mod_power_of_2_integral_assign(&mut self, pow: u64);
}

/// Computes the $n$th derivative of a polynomial, $\sum_i i^{\underline n}a_ix^{i-n}$, where
/// $i^{\underline n} = i(i-1)\cdots(i-n+1)$ is a falling factorial.
pub trait NthDerivative {
    type Output;

    /// Computes the $n$th derivative of a polynomial.
    ///
    /// $$
    /// f(p, n) = p^{(n)}.
    /// $$
    fn nth_derivative(self, n: u64) -> Self::Output;
}

/// Replaces a polynomial with its $n$th derivative, $\sum_i i^{\underline n}a_ix^{i-n}$, where
/// $i^{\underline n} = i(i-1)\cdots(i-n+1)$ is a falling factorial.
pub trait NthDerivativeAssign {
    /// Replaces a polynomial with its $n$th derivative.
    ///
    /// $$
    /// p \gets p^{(n)}.
    /// $$
    fn nth_derivative_assign(&mut self, n: u64);
}

/// Computes the $n$th derivative of a polynomial modulo $m$. The coefficients must already be
/// reduced modulo $m$.
///
/// Each coefficient $a_i$ becomes $i^{\underline n}a_i \bmod m$, where $i^{\underline n}$ is a
/// falling factorial; this can be zero even when $a_i$ is not, so the derivative can lose any
/// number of degrees. Every $i^{\underline n}$ is a multiple of $n!$, so if $m$ divides $n!$, the
/// result is zero.
pub trait ModNthDerivative<M> {
    type Output;

    /// Computes the $n$th derivative of a polynomial modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = p^{(n)} \bmod m.
    /// $$
    fn mod_nth_derivative(self, n: u64, m: M) -> Self::Output;
}

/// Replaces a polynomial with its $n$th derivative modulo $m$. The coefficients must already be
/// reduced modulo $m$.
///
/// Each coefficient $a_i$ becomes $i^{\underline n}a_i \bmod m$, where $i^{\underline n}$ is a
/// falling factorial; this can be zero even when $a_i$ is not, so the derivative can lose any
/// number of degrees. Every $i^{\underline n}$ is a multiple of $n!$, so if $m$ divides $n!$, the
/// result is zero.
pub trait ModNthDerivativeAssign<M> {
    /// Replaces a polynomial with its $n$th derivative modulo $m$.
    ///
    /// $$
    /// p \gets p^{(n)} \bmod m.
    /// $$
    fn mod_nth_derivative_assign(&mut self, n: u64, m: M);
}

/// Computes the $n$th derivative of a polynomial modulo $2^k$. The coefficients must already be
/// reduced modulo $2^k$.
///
/// Each coefficient $a_i$ becomes $i^{\underline n}a_i \bmod 2^k$, where $i^{\underline n}$ is a
/// falling factorial; this can be zero even when $a_i$ is not, so the derivative can lose any
/// number of degrees. Every $i^{\underline n}$ is a multiple of $n!$, so if $2^k$ divides $n!$, the
/// result is zero.
pub trait ModPowerOf2NthDerivative {
    type Output;

    /// Computes the $n$th derivative of a polynomial modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = p^{(n)} \bmod 2^k.
    /// $$
    fn mod_power_of_2_nth_derivative(self, n: u64, pow: u64) -> Self::Output;
}

/// Replaces a polynomial with its $n$th derivative modulo $2^k$. The coefficients must already be
/// reduced modulo $2^k$.
///
/// Each coefficient $a_i$ becomes $i^{\underline n}a_i \bmod 2^k$, where $i^{\underline n}$ is a
/// falling factorial; this can be zero even when $a_i$ is not, so the derivative can lose any
/// number of degrees. Every $i^{\underline n}$ is a multiple of $n!$, so if $2^k$ divides $n!$, the
/// result is zero.
pub trait ModPowerOf2NthDerivativeAssign {
    /// Replaces a polynomial with its $n$th derivative modulo $2^k$.
    ///
    /// $$
    /// p \gets p^{(n)} \bmod 2^k.
    /// $$
    fn mod_power_of_2_nth_derivative_assign(&mut self, n: u64, pow: u64);
}

/// Packs the coefficients of a polynomial into a single number, placing the coefficient of $x^i$ at
/// bit $ib$; that is, evaluates the polynomial at $2^b$.
///
/// When every coefficient's absolute value is less than $2^b$, the coefficients occupy disjoint
/// $b$-bit fields, with a borrow from the next field above each negative one; this is the
/// representation that Kronecker substitution uses to reduce polynomial multiplication to integer
/// multiplication. Wider coefficients overlap the fields above them, and the result is still
/// $p(2^b)$.
pub trait BitPack {
    type Output;

    /// Packs the coefficients of a polynomial into fields of `bits` bits.
    ///
    /// $$
    /// f(p, b) = p(2^b) = \sum_i a_i2^{ib}.
    /// $$
    fn bit_pack(self, bits: u64) -> Self::Output;
}

/// Unpacks a polynomial from the fixed-width fields of a single number, reading the coefficient of
/// $x^i$ from bit $ib$; the result $p$ satisfies $p(2^b) = n$.
///
/// This inverts [`BitPack`] on polynomials whose coefficients fit their fields.
pub trait BitUnpack<T>: Sized {
    /// Unpacks a polynomial from fields of `bits` bits.
    ///
    /// $$
    /// f(n, b) = p, \quad \text{where} \quad p(2^b) = n.
    /// $$
    fn bit_unpack(n: T, bits: u64) -> Self;
}

/// Computes the content of a polynomial.
///
/// For a polynomial with integer coefficients, the content is the greatest common divisor of its
/// coefficients. For a polynomial with rational coefficients, it is the non-negative rational $c$
/// for which $p/c$ is a primitive polynomial with integer coefficients. Either way it is
/// non-negative, and the content of the zero polynomial is zero.
pub trait Content {
    /// The type of the content.
    type Output;

    /// Computes the content of a polynomial.
    fn content(self) -> Self::Output;
}

/// Computes the primitive part of a polynomial: the polynomial divided by its content, with the
/// sign chosen so that the leading coefficient is non-negative.
///
/// $$
/// p = \operatorname{sgn}(\operatorname{lc}(p)) \operatorname{cont}(p) \operatorname{pp}(p),
/// $$
///
/// where $\operatorname{lc}(p)$ is the leading coefficient of $p$. The sign matters: without it the
/// identity fails whenever the leading coefficient is negative. The primitive part of the zero
/// polynomial is zero.
pub trait PrimitivePart {
    /// The type of the primitive part.
    type Output;

    /// Computes the primitive part of a polynomial.
    fn primitive_part(self) -> Self::Output;
}

/// Replaces a polynomial with its primitive part.
///
/// See [`PrimitivePart`].
pub trait PrimitivePartAssign {
    /// Replaces a polynomial with its primitive part.
    fn primitive_part_assign(&mut self);
}

/// Computes the content and the primitive part of a polynomial together.
///
/// The primitive part is found by dividing by the content, so computing both at once finds the
/// content only once. See [`Content`] and [`PrimitivePart`].
pub trait ContentAndPrimitivePart {
    /// The type of the content.
    type Content;
    /// The type of the primitive part.
    type PrimitivePart;

    /// Computes the content and the primitive part of a polynomial.
    fn content_and_primitive_part(self) -> (Self::Content, Self::PrimitivePart);
}

/// Makes a polynomial monic, by dividing it by its leading coefficient.
///
/// The zero polynomial has no leading coefficient, and is left as it is.
pub trait MakeMonic {
    /// The type of the monic polynomial.
    type Output;

    /// Makes a polynomial monic.
    fn make_monic(self) -> Self::Output;
}

/// Makes a polynomial monic in place, by dividing it by its leading coefficient.
///
/// See [`MakeMonic`].
pub trait MakeMonicAssign {
    /// Makes a polynomial monic in place.
    fn make_monic_assign(&mut self);
}

/// Makes a polynomial monic modulo $m$, by multiplying it by the inverse of its leading
/// coefficient.
///
/// The polynomial's coefficients must already be reduced modulo $m$. If the leading coefficient is
/// not invertible modulo $m$, its greatest common divisor with $m$, a nontrivial factor of $m$, is
/// returned as the error. The zero polynomial is left as it is.
pub trait ModMakeMonic<M> {
    /// The type of the monic polynomial.
    type Output;
    /// The type of the factor of $m$ returned when the leading coefficient is not invertible.
    type Factor;

    /// Makes a polynomial monic modulo `m`.
    fn mod_make_monic(self, m: M) -> Result<Self::Output, Self::Factor>;
}

/// Makes a polynomial monic modulo $m$ in place.
///
/// See [`ModMakeMonic`]. If the leading coefficient is not invertible, the polynomial is left
/// unchanged.
pub trait ModMakeMonicAssign<M> {
    /// The type of the factor of $m$ returned when the leading coefficient is not invertible.
    type Factor;

    /// Makes a polynomial monic modulo `m` in place.
    fn mod_make_monic_assign(&mut self, m: M) -> Result<(), Self::Factor>;
}

/// Evaluates a polynomial at a value.
///
/// Evaluation substitutes a value for the variable. It maps out of the polynomial rather than
/// staying inside it, so the type of the value decides the type of the result, and a polynomial
/// type may implement this trait for several value types.
pub trait Evaluate<T> {
    /// The type of the polynomial's value.
    type Output;

    /// Evaluates a polynomial at `x`.
    ///
    /// $$
    /// f(p, x) = \sum_{i=0}^{n-1} c_i x^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn evaluate(self, x: T) -> Self::Output;
}

/// Evaluates a polynomial at a value, modulo $2^k$. The polynomial's coefficients and the value
/// must already be reduced modulo $2^k$.
pub trait ModPowerOf2Evaluate<T> {
    /// The type of the polynomial's value.
    type Output;

    /// Evaluates a polynomial at `x`, modulo $2^k$.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn mod_power_of_2_evaluate(self, x: T, pow: u64) -> Self::Output;
}

/// Evaluates a polynomial at a value, modulo $m$. The value must already be reduced modulo $m$, and
/// so must the polynomial's coefficients, unless an implementation says otherwise.
pub trait ModEvaluate<T, M = T> {
    /// The type of the polynomial's value.
    type Output;

    /// Evaluates a polynomial at `x`, modulo `m`.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn mod_evaluate(self, x: T, m: M) -> Self::Output;
}

/// Evaluates a polynomial at each of several values.
pub trait EvaluateMany<T> {
    /// The type of the polynomial's values.
    type Output;

    /// Evaluates a polynomial at each value in `xs`, returning the values in the same order.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn evaluate_many(self, xs: &[T]) -> Vec<Self::Output>;
}

/// Evaluates a polynomial at each of several values, modulo $m$. The values must already be reduced
/// modulo $m$, and so must the polynomial's coefficients, unless an implementation says otherwise.
pub trait ModEvaluateMany<T, M = T> {
    /// The type of the polynomial's values.
    type Output;

    /// Evaluates a polynomial at each value in `xs`, modulo `m`, returning the values in the same
    /// order.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m
    /// \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn mod_evaluate_many(self, xs: &[T], m: M) -> Vec<Self::Output>;
}

/// Evaluates a polynomial at the first terms of a geometric progression starting at 1, modulo $m$.
/// The ratio must already be reduced modulo $m$, and so must the polynomial's coefficients.
pub trait ModEvaluateGeometric<T, M = T> {
    /// The type of the polynomial's values.
    type Output;

    /// Evaluates a polynomial at $1, q, q^2, \ldots, q^{k-1}$, modulo `m`.
    ///
    /// $$
    /// f(p, q, k, m) = \left ( \sum_{i=0}^{n-1} c_i q^{ij} \bmod m \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn mod_evaluate_geometric(self, q: T, k: u64, m: M) -> Vec<Self::Output>;
}
