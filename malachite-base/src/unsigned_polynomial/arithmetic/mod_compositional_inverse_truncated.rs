// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
//      Copyright © 2011, 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivisibleBy, FloorSqrt, ModIsReduced};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::polynomial::{ModCompositionalInverseTruncated, Polynomial};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_compose_truncated::mod_compose_truncated_vec;
use crate::unsigned_polynomial::arithmetic::mod_div_truncated::mod_div_truncated_vec;
use crate::unsigned_polynomial::arithmetic::mod_inverse_truncated::mod_inverse_truncated_vec;
use crate::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_to_out;
use alloc::vec::Vec;

// The power-series arithmetic that series reversion needs, over some ring of coefficients. Series
// are slices; the functions that return series pad them with zeros to the requested length.
//
// This plays the part of the `gr_ctx_t` that FLINT's `gr_poly` functions take.
#[doc(hidden)]
pub trait SeriesReversionRing {
    type C: Clone;

    fn zero(&self) -> Self::C;

    fn is_zero(&self, x: &Self::C) -> bool;

    fn add(&self, x: &Self::C, y: &Self::C) -> Self::C;

    fn sub(&self, x: &Self::C, y: &Self::C) -> Self::C;

    fn mul(&self, x: &Self::C, y: &Self::C) -> Self::C;

    // The inverse of `x`, or `None` if it is not a unit.
    fn inverse(&self, x: &Self::C) -> Option<Self::C>;

    // `x / i`, or `None` if the division is not possible.
    fn div_by_index(&self, x: &Self::C, i: u64) -> Option<Self::C>;

    // `x * i`.
    fn mul_by_index(&self, x: &Self::C, i: u64) -> Self::C;

    // The first `len` coefficients of the product.
    fn mul_truncated(&self, xs: &[Self::C], ys: &[Self::C], len: usize) -> Vec<Self::C>;

    // The first `len` coefficients of the inverse, where the constant coefficient is a unit.
    fn inverse_truncated(&self, xs: &[Self::C], len: usize) -> Vec<Self::C>;

    // The first `len` coefficients of the quotient, where the constant coefficient of `ys` is a
    // unit.
    fn div_truncated(&self, xs: &[Self::C], ys: &[Self::C], len: usize) -> Vec<Self::C>;

    // The first `len` coefficients of the composition, where `ys` has constant coefficient 0.
    fn compose_truncated(&self, xs: &[Self::C], ys: &[Self::C], len: usize) -> Vec<Self::C>;

    // The coefficient of $x^{n-1}$ in the product of the first `n` coefficients of `xs` and `ys`.
    #[inline]
    fn dot_rev(&self, xs: &[Self::C], ys: &[Self::C], n: usize) -> Self::C {
        dot_rev_generic(self, xs, ys, n)
    }

    // Whether every division by an index $i$ with $2 \leq i < n$ is possible, so that Lagrange
    // inversion to `n` terms succeeds.
    fn lagrange_possible(&self, _n: usize) -> bool {
        true
    }

    // The number of terms from which Newton's iteration is used rather than the fast Lagrange
    // algorithm.
    fn newton_cutoff(&self) -> usize {
        NEWTON_CUTOFF
    }
}

// FLINT's choice, in `_gr_poly_revert_series`.
const NEWTON_CUTOFF: usize = 2500;

// The first `len` coefficients of the derivative of the series `xs`.
fn derivative<R: SeriesReversionRing>(ring: &R, xs: &[R::C], len: usize) -> Vec<R::C> {
    let mut out: Vec<R::C> = xs
        .iter()
        .enumerate()
        .skip(1)
        .take(len)
        .map(|(i, x)| ring.mul_by_index(x, u64::exact_from(i)))
        .collect();
    out.resize(len, ring.zero());
    out
}

// The coefficient of $x^{n-1}$ in the product of the first `n` coefficients of `xs` and `ys`, with
// the ring's own addition and multiplication.
fn dot_rev_generic<R: SeriesReversionRing + ?Sized>(
    ring: &R,
    xs: &[R::C],
    ys: &[R::C],
    n: usize,
) -> R::C {
    let mut sum = ring.zero();
    for (x, y) in xs[..n].iter().zip(ys[..n].iter().rev()) {
        sum = ring.add(&sum, &ring.mul(x, y));
    }
    sum
}

// The reversion of `f` to `n` terms by Lagrange inversion: the coefficient of $x^i$ is that of
// $x^{i-1}$ in $(x/f)^i$, divided by $i$. Returns `None` if one of the divisions by $i$ is not
// possible. `f` must have constant coefficient 0 and a unit coefficient of $x$, and `n` must be at
// least 3.
//
// This is equivalent to `_gr_poly_revert_series_lagrange` from `gr_poly/revert_series.c`, FLINT
// 3.6.0.
#[doc(hidden)]
pub fn compositional_inverse_truncated_lagrange<R: SeriesReversionRing>(
    ring: &R,
    f: &[R::C],
    n: usize,
) -> Option<Vec<R::C>> {
    let flen = f.len().min(n);
    let mut res = vec![ring.zero(); n];
    let r = ring.inverse_truncated(&f[1..flen], n - 1);
    res[1] = r[0].clone();
    let mut s = r.clone();
    for i in 2..n {
        s = ring.mul_truncated(&s, &r, n - 1);
        res[i] = ring.div_by_index(&s[i - 1], u64::exact_from(i))?;
    }
    Some(res)
}

// As `compositional_inverse_truncated_lagrange`, with baby steps and giant steps: the powers $(x /
// f)^j$ for $j \leq \lfloor \sqrt n \rfloor$ are kept, and the other powers are reached by
// multiplying by the largest of them; between multiplications, the needed coefficients are single
// dot products.
//
// This is equivalent to `_gr_poly_revert_series_lagrange_fast` from `gr_poly/revert_series.c`,
// FLINT 3.6.0.
#[doc(hidden)]
pub fn compositional_inverse_truncated_lagrange_fast<R: SeriesReversionRing>(
    ring: &R,
    f: &[R::C],
    n: usize,
) -> Option<Vec<R::C>> {
    let flen = f.len().min(n);
    let m = n.floor_sqrt();
    let mut res = vec![ring.zero(); n];
    // rs[j - 1] is (x / f)^j
    let mut rs: Vec<Vec<R::C>> = Vec::with_capacity(m);
    rs.push(ring.inverse_truncated(&f[1..flen], n - 1));
    for i in 2..=m {
        let power = ring.mul_truncated(&rs[i.div_ceil(2) - 1], &rs[(i >> 1) - 1], n - 1);
        rs.push(power);
    }
    res[1] = rs[0][0].clone();
    for i in 2..m {
        res[i] = ring.div_by_index(&rs[i - 1][i - 1], u64::exact_from(i))?;
    }
    let mut s = rs[m - 1].clone();
    let mut i = m;
    while i < n {
        res[i] = ring.div_by_index(&s[i - 1], u64::exact_from(i))?;
        let mut j = 1;
        while j < m && i + j < n {
            let t = ring.dot_rev(&s, &rs[j - 1], i + j);
            res[i + j] = ring.div_by_index(&t, u64::exact_from(i + j))?;
            j += 1;
        }
        if i + 1 < n {
            s = ring.mul_truncated(&s, &rs[m - 1], n - 1);
        }
        i += m;
    }
    Some(res)
}

// The reversion of `f` to `n` terms by Newton iteration: from an approximation $g$, $g - (f(g) - x)
// g' / f(g)'$ is correct to twice as many terms. Works whenever the coefficient of $x$ in `f` is a
// unit.
//
// This is equivalent to `_gr_poly_revert_series_newton` from `gr_poly/revert_series.c`, FLINT
// 3.6.0.
#[doc(hidden)]
pub fn compositional_inverse_truncated_newton<R: SeriesReversionRing>(
    ring: &R,
    f: &[R::C],
    n: usize,
) -> Vec<R::C> {
    let flen = f.len().min(n);
    let mut steps = vec![n];
    let mut k = n;
    while k > 2 {
        k = k.div_ceil(2);
        steps.push(k);
    }
    let mut res = vec![ring.zero(); n];
    if k == 2 {
        res[1] = ring.inverse(&f[1]).unwrap();
    }
    for &k in steps.iter().rev().skip(1) {
        let mut t = ring.compose_truncated(&f[..flen.min(k)], &res[..k], k);
        let u = derivative(ring, &t, k);
        t[1] = ring.zero();
        let v = ring.div_truncated(&t, &u, k);
        let d = derivative(ring, &res[..k], k);
        let u = ring.mul_truncated(&v, &d, k);
        for (r, x) in res.iter_mut().zip(u.iter()) {
            *r = ring.sub(r, x);
        }
    }
    res
}

// This is equivalent to `_gr_poly_revert_series` from `gr_poly/revert_series.c`, FLINT 3.6.0,
// except that the cutoff depends on the ring, and that when a division by an index is not possible,
// Newton's iteration is chosen at once rather than after the fast Lagrange algorithm fails.
#[doc(hidden)]
pub fn compositional_inverse_truncated_dispatch<R: SeriesReversionRing>(
    ring: &R,
    f: &[R::C],
    n: usize,
) -> Vec<R::C> {
    if n >= ring.newton_cutoff() || !ring.lagrange_possible(n) {
        compositional_inverse_truncated_newton(ring, f, n)
    } else {
        compositional_inverse_truncated_lagrange_fast(ring, f, n).unwrap()
    }
}

// The reversion of `f` to `n` terms, or `None` if `f` has no compositional inverse: if it is
// shorter than 2, has a nonzero constant coefficient, or a coefficient of $x$ that is not a unit.
// The result has `n` coefficients, untrimmed.
//
// The checks are those of `_gr_poly_revert_series` from `gr_poly/revert_series.c`, FLINT 3.6.0,
// except that the coefficient of $x$ is checked for every `n`.
#[doc(hidden)]
pub fn compositional_inverse_truncated_helper<R: SeriesReversionRing>(
    ring: &R,
    f: &[R::C],
    n: usize,
) -> Option<Vec<R::C>> {
    if f.len() < 2 || !ring.is_zero(&f[0]) {
        return None;
    }
    let inverse = ring.inverse(&f[1])?;
    Some(match n {
        0 => Vec::new(),
        1 => vec![ring.zero()],
        2 => vec![ring.zero(), inverse],
        _ => compositional_inverse_truncated_dispatch(ring, f, n),
    })
}

fn padded<T: PrimitiveUnsigned>(mut xs: Vec<T>, len: usize) -> Vec<T> {
    xs.resize(len, T::ZERO);
    xs
}

// The ring of coefficients modulo $m$.
pub(crate) struct ModRing<T: PrimitiveUnsigned> {
    pub(crate) m: T,
}

impl<T: PrimitiveUnsigned> ModRing<T> {
    // `i` modulo $m$.
    fn reduce_index(&self, i: u64) -> T {
        let m: u128 = self.m.wrapping_into();
        T::wrapping_from(u128::from(i) % m)
    }
}

impl<T: PrimitiveUnsigned> SeriesReversionRing for ModRing<T> {
    type C = T;

    #[inline]
    fn zero(&self) -> T {
        T::ZERO
    }

    #[inline]
    fn is_zero(&self, x: &T) -> bool {
        *x == T::ZERO
    }

    #[inline]
    fn add(&self, x: &T, y: &T) -> T {
        x.mod_add(*y, self.m)
    }

    #[inline]
    fn sub(&self, x: &T, y: &T) -> T {
        x.mod_sub(*y, self.m)
    }

    #[inline]
    fn mul(&self, x: &T, y: &T) -> T {
        x.mod_mul(*y, self.m)
    }

    #[inline]
    fn inverse(&self, x: &T) -> Option<T> {
        if *x == T::ZERO {
            None
        } else {
            x.mod_inverse(self.m)
        }
    }

    fn div_by_index(&self, x: &T, i: u64) -> Option<T> {
        Some(x.mod_mul(self.inverse(&self.reduce_index(i))?, self.m))
    }

    #[inline]
    fn mul_by_index(&self, x: &T, i: u64) -> T {
        x.mod_mul(self.reduce_index(i), self.m)
    }

    fn mul_truncated(&self, xs: &[T], ys: &[T], len: usize) -> Vec<T> {
        let mut out = vec![T::ZERO; len];
        if len != 0 && !xs.is_empty() && !ys.is_empty() {
            mod_mul_truncated_to_out(&mut out, xs, ys, self.m);
        }
        out
    }

    #[inline]
    fn inverse_truncated(&self, xs: &[T], len: usize) -> Vec<T> {
        padded(
            mod_inverse_truncated_vec(xs, u64::exact_from(len), self.m).unwrap(),
            len,
        )
    }

    #[inline]
    fn div_truncated(&self, xs: &[T], ys: &[T], len: usize) -> Vec<T> {
        padded(
            mod_div_truncated_vec(xs, ys, u64::exact_from(len), self.m).unwrap(),
            len,
        )
    }

    #[inline]
    fn compose_truncated(&self, xs: &[T], ys: &[T], len: usize) -> Vec<T> {
        padded(
            mod_compose_truncated_vec(xs, ys, u64::exact_from(len), self.m),
            len,
        )
    }

    // When the products fit in a `u128`, they are summed there, and reduced only when the sum would
    // overflow, and at the end.
    fn dot_rev(&self, xs: &[T], ys: &[T], n: usize) -> T {
        if T::WIDTH > u64::WIDTH {
            return dot_rev_generic(self, xs, ys, n);
        }
        let m: u128 = self.m.wrapping_into();
        let mut sum = 0u128;
        for (&x, &y) in xs[..n].iter().zip(ys[..n].iter().rev()) {
            let x: u128 = x.wrapping_into();
            let y: u128 = y.wrapping_into();
            let product = x * y;
            if sum > u128::MAX - product {
                sum %= m;
            }
            sum += product;
        }
        T::wrapping_from(sum % m)
    }

    // Every index $i$ with $2 \leq i < n$ is invertible exactly when no prime factor of $m$ is less
    // than $n$.
    fn lagrange_possible(&self, n: usize) -> bool {
        let m: u128 = self.m.wrapping_into();
        let n = u128::wrapping_from(n);
        let mut i = 2;
        while i < n {
            if i * i > m {
                // $m$ is 1 or prime.
                return m >= n;
            }
            if m.divisible_by(i) {
                return false;
            }
            i += 1;
        }
        true
    }
}

// The compositional inverse of `p` modulo $m$ to `len` terms.
//
// This is equivalent to `nmod_poly_revert_series` from `nmod_poly/revert_series.c`, FLINT 3.6.0,
// except that `None` is returned where FLINT throws.
pub(crate) fn mod_compositional_inverse_truncated_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    assert_ne!(m, T::ZERO);
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    if m == T::ONE {
        // Every polynomial is 0 modulo 1, and so is its reversion.
        return Some(UnsignedPolynomial::ZERO);
    }
    let n = usize::exact_from(len);
    let xs = p.coefficients_asc();
    compositional_inverse_truncated_helper(&ModRing { m }, xs, n)
        .map(UnsignedPolynomial::from_coefficients_asc)
}

impl<T: PrimitiveUnsigned> ModCompositionalInverseTruncated<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Finds the compositional inverse of an [`UnsignedPolynomial`] as a power series modulo $m$,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking it by value. Its
    /// coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is invertible
    /// modulo $m$; otherwise the inverse exists and is unique. When the inverse exists and `len` is
    /// less than 2, the result is the zero polynomial, and when $m$ is 1 the result is always
    /// `Some` of the zero polynomial.
    ///
    /// The inverse is computed by the fast Lagrange algorithm, in which the coefficient of $x^i$ is
    /// that of $x^{i-1}$ in $(x/p)^i$, divided by $i$. When $m$ has a prime factor less than `len`,
    /// so that one of those divisions is not possible, or when `len` is at least 2500, Newton's
    /// iteration is used instead, in which each step doubles the number of correct coefficients
    /// with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompositionalInverseTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap())
    ///         .mod_compositional_inverse_truncated(6, 7)
    ///         .unwrap()
    ///         .to_string(),
    ///     "2*x^4+2*x^3+6*x^2+x"
    /// );
    /// // The coefficient of x is not invertible.
    /// assert!(
    ///     (UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap())
    ///         .mod_compositional_inverse_truncated(5, 10)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_revert_series` from `nmod_poly/revert_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT throws or fails, that the inverse modulo
    /// $x^0$ or $x^1$, or modulo 1, is 0, and that Newton's iteration is chosen at once when a
    /// division by an index is not possible.
    #[inline]
    fn mod_compositional_inverse_truncated(self, len: u64, m: T) -> Option<Self> {
        mod_compositional_inverse_truncated_helper(&self, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModCompositionalInverseTruncated<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Finds the compositional inverse of an [`UnsignedPolynomial`] as a power series modulo $m$,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking it by reference. Its
    /// coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n, m}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is invertible
    /// modulo $m$; otherwise the inverse exists and is unique. When the inverse exists and `len` is
    /// less than 2, the result is the zero polynomial, and when $m$ is 1 the result is always
    /// `Some` of the zero polynomial.
    ///
    /// The inverse is computed by the fast Lagrange algorithm, in which the coefficient of $x^i$ is
    /// that of $x^{i-1}$ in $(x/p)^i$, divided by $i$. When $m$ has a prime factor less than `len`,
    /// so that one of those divisions is not possible, or when `len` is at least 2500, Newton's
    /// iteration is used instead, in which each step doubles the number of correct coefficients
    /// with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModCompositionalInverseTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+x").unwrap())
    ///         .mod_compositional_inverse_truncated(6, 7)
    ///         .unwrap()
    ///         .to_string(),
    ///     "2*x^4+2*x^3+6*x^2+x"
    /// );
    /// // The coefficient of x is not invertible.
    /// assert!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap())
    ///         .mod_compositional_inverse_truncated(5, 10)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_revert_series` from `nmod_poly/revert_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT throws or fails, that the inverse modulo
    /// $x^0$ or $x^1$, or modulo 1, is 0, and that Newton's iteration is chosen at once when a
    /// division by an index is not possible.
    #[inline]
    fn mod_compositional_inverse_truncated(self, len: u64, m: T) -> Option<UnsignedPolynomial<T>> {
        mod_compositional_inverse_truncated_helper(self, len, m)
    }
}
