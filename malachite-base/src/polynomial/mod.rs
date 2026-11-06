// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::PowerOf2;
use crate::num::conversion::traits::ExactFrom;
use crate::num::exponential::traits::FloorLogBase2;
use crate::num::modular::traits::DivisibleBy;
use crate::num::number_theory::traits::{Gcd, GcdAssign};
use crate::vars::{Var, VarScheme};
use alloc::string::String;

// Until the next release, the traits and structs that moved to `polynomial::arithmetic`,
// `polynomial::modular`, `polynomial::series`, `polynomial::mod_polynomial`, and
// `polynomial::calculus` are also re-exported here, so that paths through this module keep
// resolving. These re-exports will then be removed.
pub use self::arithmetic::traits::*;
pub use self::calculus::traits::*;
pub use self::mod_polynomial::ModComposeModPolynomialData;
pub use self::mod_polynomial::traits::*;
pub use self::modular::ModPolynomialData;
pub use self::modular::traits::*;
pub use self::series::traits::*;
use alloc::vec::Vec;

/// Traits for the ring operations on polynomials, division, composition, and evaluation.
pub mod arithmetic;
/// Traits for derivatives and integrals of polynomials.
pub mod calculus;
/// The evaluation of polynomials at square matrices, shared by all the polynomial types and generic
/// over the ring of coefficients.
#[doc(hidden)]
pub mod evaluate_at_matrix;
/// Traits for operations on polynomials modulo a polynomial.
pub mod mod_polynomial;
/// Traits for operations on polynomials whose coefficients are reduced modulo $m$ or $2^k$.
pub mod modular;
/// Traits for operations on polynomials as power series, truncated to a given number of terms.
pub mod series;
/// Traits for special polynomials, such as those whose coefficients are rows of combinatorial
/// triangles.
pub mod special;

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

/// Raises the polynomial with coefficients `xs`, which has length at least 2, to the power `e`,
/// which is at least 3, by left-to-right binary exponentiation with the given squaring and
/// multiplication, each of which returns its result without zeros at the end.
///
/// This is for powers in rings with zero divisors, such as polynomials modulo $m$, where a power
/// can lose its leading coefficients or vanish: the intermediate powers keep only the length they
/// need, and a power that vanishes ends the computation.
///
/// This is not part of the public API; it is public so that `malachite-nz` can raise its
/// polynomials to powers modulo a number in the same way.
#[doc(hidden)]
pub fn pow_binexp_trimmed<T>(
    xs: &[T],
    e: u64,
    square: impl Fn(&[T]) -> Vec<T>,
    mul: impl Fn(&[T], &[T]) -> Vec<T>,
) -> Vec<T> {
    // The bit of e one place below its most significant bit
    let mut bit = u64::power_of_2(e.floor_log_base_2()) >> 1;
    let mut r = square(xs);
    loop {
        if r.is_empty() {
            return r;
        }
        if bit & e != 0 {
            r = mul(&r, xs);
            if r.is_empty() {
                return r;
            }
        }
        bit >>= 1;
        if bit == 0 {
            return r;
        }
        r = square(&r);
    }
}
