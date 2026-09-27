// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::add::add_or_sub_coefficients;
use alloc::vec::Vec;
use core::mem::{swap, take};
use core::ptr;
use malachite_base::num::arithmetic::traits::{DivExactAssign, GcdAssign, Parity};
use malachite_base::polynomial::{AddTruncated, AddTruncatedAssign, Content, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

// The first `len` elements of `xs`, or all of them if there are fewer.
fn prefix(xs: &[Integer], len: u64) -> &[Integer] {
    &xs[..usize::try_from(len).map_or(xs.len(), |len| len.min(xs.len()))]
}

// Adds $q$ to $p$, or subtracts it when `negate_q` is true, keeping only the coefficients below
// $x^{len}$, where `xs` holds the first `len` coefficients of the numerator of $p$, `a` is its
// denominator, and `p_cut` says whether truncating $p$ removed anything. The storage of `xs` is
// reused.
//
// This is equivalent to `fmpq_poly_add_series_can` from `fmpq_poly/add_series.c`, FLINT 3.6.0,
// where `can` is 1, and, when `negate_q` is true, to `fmpq_poly_sub_series_can` from
// `fmpq_poly/sub_series.c`, FLINT 3.6.0, in the case where the two polynomials are distinct.
pub(crate) fn add_or_sub_truncated(
    xs: Vec<Integer>,
    p_cut: bool,
    a: Natural,
    q: &RationalPolynomial,
    len: u64,
    negate_q: bool,
) -> RationalPolynomial {
    add_or_sub_coefficients(
        xs,
        a,
        prefix(q.numerator.coefficients_asc(), len),
        &q.denominator,
        negate_q,
        p_cut || q.numerator.len() > len,
    )
}

// Adds or subtracts, keeping only the coefficients below $x^{len}$, reusing the storage of $p$.
pub(crate) fn add_or_sub_truncated_owned_ref(
    p: RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
    negate_q: bool,
) -> RationalPolynomial {
    let p_cut = p.numerator.len() > len;
    let mut x = p.numerator;
    x.truncate_assign(len);
    add_or_sub_truncated(
        x.into_coefficients_asc(),
        p_cut,
        p.denominator,
        q,
        len,
        negate_q,
    )
}

// Adds or subtracts, keeping only the coefficients below $x^{len}$, copying only the first `len`
// coefficients of $p$.
pub(crate) fn add_or_sub_truncated_ref_ref(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
    negate_q: bool,
) -> RationalPolynomial {
    add_or_sub_truncated(
        prefix(p.numerator.coefficients_asc(), len).to_vec(),
        p.numerator.len() > len,
        p.denominator.clone(),
        q,
        len,
        negate_q,
    )
}

// Adds two `RationalPolynomial`s taken by value, keeping only the coefficients below $x^{len}$ and
// reusing the storage of the one with the longer numerator.
fn add_truncated_owned_owned(
    mut p: RationalPolynomial,
    mut q: RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    if q.numerator.len() > p.numerator.len() {
        swap(&mut p, &mut q);
    }
    add_or_sub_truncated_owned_ref(p, &q, len, false)
}

// Doubles a polynomial, keeping only the coefficients below $x^{len}$.
//
// This is equivalent to the `poly1 == poly2` case of `fmpq_poly_add_series_can` from
// `fmpq_poly/add_series.c`, FLINT 3.6.0, where `can` is 1.
fn double_truncated(p: &RationalPolynomial, len: u64) -> RationalPolynomial {
    let mut numerator = p.numerator.truncate(len);
    let mut denominator = p.denominator.clone();
    // Halving an even denominator, or doubling the numerator over an odd one, keeps a canonical
    // pair canonical.
    if denominator.even() {
        denominator >>= 1u32;
    } else {
        numerator = IntegerPolynomial::from_coefficients_asc(
            numerator
                .coefficients_asc()
                .iter()
                .map(|c| c << 1u32)
                .collect::<Vec<_>>(),
        );
    }
    // Cutting can leave a common factor. A zero numerator has content 0 and reduces to 0/1.
    if p.numerator.len() > len {
        let mut e = (&numerator).content();
        if e != 1u32 {
            e.gcd_assign(&denominator);
            if e != 1u32 {
                denominator.div_exact_assign(&e);
                numerator.div_exact_assign(Integer::from(e));
            }
        }
    }
    RationalPolynomial {
        numerator,
        denominator,
    }
}

impl AddTruncated<Self> for RationalPolynomial {
    type Output = Self;

    /// Adds two [`RationalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = (p + q) \bmod x^n.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is kept in lowest
    /// terms: cutting a polynomial can remove the coefficients that kept its numerator coprime to
    /// its denominator, so when anything is cut, the whole common factor of the new numerator and
    /// denominator is divided out. The sum is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4")
    ///         .unwrap()
    ///         .add_truncated(
    ///             RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "x^2+x"
    /// );
    /// // The constant terms cancel, and the linear ones sum to 1.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4")
    ///         .unwrap()
    ///         .add_truncated(
    ///             RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add_series` from `fmpq_poly/add_series.c`, FLINT 3.6.0.
    #[inline]
    fn add_truncated(self, other: Self, len: u64) -> Self {
        add_truncated_owned_owned(self, other, len)
    }
}

impl AddTruncated<&Self> for RationalPolynomial {
    type Output = Self;

    /// Adds two [`RationalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = (p + q) \bmod x^n.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is kept in lowest
    /// terms: cutting a polynomial can remove the coefficients that kept its numerator coprime to
    /// its denominator, so when anything is cut, the whole common factor of the new numerator and
    /// denominator is divided out. The sum is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4")
    ///         .unwrap()
    ///         .add_truncated(
    ///             &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "x^2+x"
    /// );
    /// // The constant terms cancel, and the linear ones sum to 1.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4")
    ///         .unwrap()
    ///         .add_truncated(
    ///             &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add_series` from `fmpq_poly/add_series.c`, FLINT 3.6.0.
    #[inline]
    fn add_truncated(self, other: &Self, len: u64) -> Self {
        add_or_sub_truncated_owned_ref(self, other, len, false)
    }
}

impl AddTruncated<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Adds two [`RationalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = (p + q) \bmod x^n.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is kept in lowest
    /// terms: cutting a polynomial can remove the coefficients that kept its numerator coprime to
    /// its denominator, so when anything is cut, the whole common factor of the new numerator and
    /// denominator is divided out. The sum is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap())
    ///         .add_truncated(
    ///             RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "x^2+x"
    /// );
    /// // The constant terms cancel, and the linear ones sum to 1.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap())
    ///         .add_truncated(
    ///             RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add_series` from `fmpq_poly/add_series.c`, FLINT 3.6.0.
    #[inline]
    fn add_truncated(self, other: RationalPolynomial, len: u64) -> RationalPolynomial {
        add_or_sub_truncated_owned_ref(other, self, len, false)
    }
}

impl AddTruncated<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Adds two [`RationalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = (p + q) \bmod x^n.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is kept in lowest
    /// terms: cutting a polynomial can remove the coefficients that kept its numerator coprime to
    /// its denominator, so when anything is cut, the whole common factor of the new numerator and
    /// denominator is divided out. The sum is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap())
    ///         .add_truncated(
    ///             &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "x^2+x"
    /// );
    /// // The constant terms cancel, and the linear ones sum to 1.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap())
    ///         .add_truncated(
    ///             &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add_series` from `fmpq_poly/add_series.c`, FLINT 3.6.0.
    fn add_truncated(self, other: &RationalPolynomial, len: u64) -> RationalPolynomial {
        if ptr::eq(self, other) {
            return double_truncated(self, len);
        }
        add_or_sub_truncated_ref_ref(self, other, len, false)
    }
}

impl AddTruncatedAssign<Self> for RationalPolynomial {
    /// Adds a [`RationalPolynomial`] to a [`RationalPolynomial`] in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by value.
    ///
    /// $$
    /// p \gets (p + q) \bmod x^n.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is kept in lowest
    /// terms: cutting a polynomial can remove the coefficients that kept its numerator coprime to
    /// its denominator, so when anything is cut, the whole common factor of the new numerator and
    /// denominator is divided out. The sum is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap();
    /// p.add_truncated_assign(
    ///     RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "x^2+x");
    ///
    /// // The constant terms cancel, and the linear ones sum to 1.
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap();
    /// p.add_truncated_assign(
    ///     RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///     2,
    /// );
    /// assert_eq!(p.to_string(), "x");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add_series` from `fmpq_poly/add_series.c`, FLINT 3.6.0.
    #[inline]
    fn add_truncated_assign(&mut self, other: Self, len: u64) {
        *self = add_truncated_owned_owned(take(self), other, len);
    }
}

impl AddTruncatedAssign<&Self> for RationalPolynomial {
    /// Adds a [`RationalPolynomial`] to a [`RationalPolynomial`] in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by reference.
    ///
    /// $$
    /// p \gets (p + q) \bmod x^n.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is kept in lowest
    /// terms: cutting a polynomial can remove the coefficients that kept its numerator coprime to
    /// its denominator, so when anything is cut, the whole common factor of the new numerator and
    /// denominator is divided out. The sum is also trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of the numerators and of the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncatedAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap();
    /// p.add_truncated_assign(
    ///     &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "x^2+x");
    ///
    /// // The constant terms cancel, and the linear ones sum to 1.
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3*x+1/4").unwrap();
    /// p.add_truncated_assign(
    ///     &RationalPolynomial::from_str("1/2*x^2+2/3*x-1/4").unwrap(),
    ///     2,
    /// );
    /// assert_eq!(p.to_string(), "x");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_add_series` from `fmpq_poly/add_series.c`, FLINT 3.6.0.
    #[inline]
    fn add_truncated_assign(&mut self, other: &Self, len: u64) {
        *self = add_or_sub_truncated_owned_ref(take(self), other, len, false);
    }
}
