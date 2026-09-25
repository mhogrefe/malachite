// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

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
pub trait EvaluateModPowerOf2<T> {
    /// The type of the polynomial's value.
    type Output;

    /// Evaluates a polynomial at `x`, modulo $2^k$.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn evaluate_mod_power_of_2(self, x: T, pow: u64) -> Self::Output;
}

/// Evaluates a polynomial at a value, modulo $m$. The value must already be reduced modulo $m$, and
/// so must the polynomial's coefficients, unless an implementation says otherwise.
pub trait EvaluateMod<T, M = T> {
    /// The type of the polynomial's value.
    type Output;

    /// Evaluates a polynomial at `x`, modulo `m`.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn evaluate_mod(self, x: T, m: M) -> Self::Output;
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
pub trait EvaluateManyMod<T, M = T> {
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
    fn evaluate_many_mod(self, xs: &[T], m: M) -> Vec<Self::Output>;
}

/// Evaluates a polynomial at the first terms of a geometric progression starting at 1, modulo $m$.
/// The ratio must already be reduced modulo $m$, and so must the polynomial's coefficients.
pub trait EvaluateGeometricMod<T, M = T> {
    /// The type of the polynomial's values.
    type Output;

    /// Evaluates a polynomial at $1, q, q^2, \ldots, q^{k-1}$, modulo `m`.
    ///
    /// $$
    /// f(p, q, k, m) = \left ( \sum_{i=0}^{n-1} c_i q^{ij} \bmod m \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn evaluate_geometric_mod(self, q: T, k: u64, m: M) -> Vec<Self::Output>;
}
