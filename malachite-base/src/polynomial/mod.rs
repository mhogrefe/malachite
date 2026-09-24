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
