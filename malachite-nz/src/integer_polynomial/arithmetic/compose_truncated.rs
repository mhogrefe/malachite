// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
//      Copyright © 2010, 2011 William Hart
//
//      Copyright © 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::coefficient::trim_coefficients;
use crate::integer_polynomial::arithmetic::mul::mul_greater_to_out;
use crate::integer_polynomial::arithmetic::mul_middle::mul_middle_to_out;
use crate::integer_polynomial::arithmetic::mul_truncated::mul_truncated_to_out;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};
use core::mem::{swap, take};
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{FloorSqrt, NegAssign, Parity};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{ComposeTruncated, ComposeTruncatedAssign};

// The first `n` coefficients of the composition of the polynomials with coefficients `xs` and `ys`,
// where `xs.len() >= 2` and `ys[0]` is 0, by Horner's rule. The running polynomial and the scratch
// product trade places at every step.
//
// This is equivalent to `_fmpz_poly_compose_series_horner` from `fmpz_poly/compose_series.c`, FLINT
// 3.6.0, except that the product is not copied back, and that a product that fits in `n`
// coefficients exactly is computed as a full product.
crate_test_fn! {compose_truncated_horner(xs: &[Integer], ys: &[Integer], n: usize) -> Vec<Integer> {
    if n == 1 {
        return vec![xs[0].clone()];
    }
    let (leading, rest) = xs.split_last().unwrap();
    let mut i = rest.len() - 1;
    let mut acc = Vec::with_capacity(n);
    acc.extend(ys.iter().map(|y| y * leading));
    acc[0] += &xs[i];
    let mut scratch = Vec::with_capacity(n);
    while i > 0 {
        i -= 1;
        let full_len = acc.len() + ys.len() - 1;
        // The product overwrites every element, so stale ones may stay.
        scratch.resize(min(full_len, n), Integer::ZERO);
        if full_len <= n {
            mul_greater_to_out(&mut scratch, &acc, ys);
        } else {
            mul_truncated_to_out(&mut scratch, &acc, ys);
        }
        swap(&mut acc, &mut scratch);
        acc[0] += &xs[i];
    }
    acc.resize(n, Integer::ZERO);
    acc
}}

// The first `n` coefficients of the composition of the polynomials with coefficients `xs` and `ys`,
// where `ys[0]` is 0, by the Brent–Kung algorithm: with $m = \lfloor \sqrt n \rfloor + 1$, the
// products of the $m$ blocks of $m$ coefficients of `xs` with the first $m$ powers of `ys` are the
// rows of a matrix product, and they are combined by Horner's rule in $y^m$.
//
// This is equivalent to `_fmpz_poly_compose_series_brent_kung` from `fmpz_poly/compose_series.c`,
// FLINT 3.6.0, except that the rows of the product are moved out rather than copied, and the
// running polynomial and the scratch product trade places at every step.
crate_test_fn! {compose_truncated_brent_kung(
    xs: &[Integer],
    ys: &[Integer],
    n: usize,
) -> Vec<Integer> {
    if n == 1 {
        return vec![xs[0].clone()];
    }
    let m = n.floor_sqrt() + 1;
    // The rows of B are the blocks of xs.
    let mut b = vec![Integer::ZERO; m * m];
    for (i, chunk) in xs.chunks(m).enumerate() {
        b[i * m..i * m + chunk.len()].clone_from_slice(chunk);
    }
    // The rows of A are the powers of ys.
    let mut a = vec![Integer::ZERO; m * n];
    a[0] = Integer::ONE;
    a[n..n + ys.len()].clone_from_slice(ys);
    for i in 2..m {
        let (previous, current) = a.split_at_mut(i * n);
        mul_truncated_to_out(&mut current[..n], &previous[(i - 1) * n..], ys);
    }
    let m_u64 = u64::exact_from(m);
    let n_u64 = u64::exact_from(n);
    let b = IntegerMatrix::from_owned_elements(m_u64, m_u64, b);
    let a = IntegerMatrix::from_owned_elements(m_u64, n_u64, a);
    let mut c = (&b * &a).into_elements();
    // Combine the rows by Horner's rule in h = ys^m.
    let mut h = vec![Integer::ZERO; n];
    mul_truncated_to_out(&mut h, &a.elements_ref()[(m - 1) * n..], ys);
    let mut acc: Vec<Integer> = c.drain((m - 1) * n..).collect();
    let mut scratch = vec![Integer::ZERO; n];
    for i in (0..m - 1).rev() {
        mul_truncated_to_out(&mut scratch, &acc, &h);
        for (x, y) in scratch.iter_mut().zip(c.drain(i * n..)) {
            *x += y;
        }
        swap(&mut acc, &mut scratch);
    }
    acc
}}

// Zeroes the gaps of a Kronecker-substitution array whose columns start every `stride` elements and
// have their first `filled` elements set: the elements of each column from `filled` up to the next
// column, or to the end of `xs`.
fn zero_gaps(xs: &mut [Integer], stride: usize, filled: usize) {
    for column in xs.chunks_mut(stride) {
        if column.len() > filled {
            for x in &mut column[filled..] {
                *x = Integer::ZERO;
            }
        }
    }
}

// The parameters of one level of the Kinoshita–Li algorithm: the x-degree bound `xn` of $Q_k$, the
// lower y-index `ylo` of the wanted slice, the y-degree bound `ydeg` of $Q_k$, and the offset
// `qoff` of $Q_k$ in the chain of $Q$s.
#[derive(Clone, Copy, Default)]
struct Level {
    xn: usize,
    ylo: usize,
    ydeg: usize,
    qoff: usize,
}

// The first `n` coefficients of the composition of the polynomials with coefficients `xs` and `ys`,
// where `ys[0]` is 0, by the algorithm of Kinoshita and Li, "Power Series Composition in
// Near-Linear Time" (2024).
//
// This is equivalent to `_gr_poly_compose_series_kinoshita_li` from
// `gr_poly/compose_series_kinoshita_li.c`, FLINT 3.6.0, for the integers, except that the division
// by $Q_L(0, y)$ in the base case is a copy, since that series is 1; that only the gaps of the
// Kronecker-substitution arrays are zeroed; and that coefficients are moved rather than copied
// wherever their source is not read again, including each $Q_k$ in the upward pass, its last use.
crate_test_fn! {compose_truncated_kinoshita_li(
    xs: &[Integer],
    ys: &[Integer],
    n: usize,
) -> Vec<Integer> {
    if xs.len() == 1 || n == 1 {
        let mut out = vec![Integer::ZERO; n];
        out[0] = xs[0].clone();
        return out;
    }
    let len1 = min(xs.len(), n);
    let len2 = min(ys.len(), n);
    // Compute level count L = ceil(log2(n)).
    let mut l = 0;
    let mut tmp = n;
    while tmp > 1 {
        tmp = tmp.div_ceil(2);
        l += 1;
    }
    // Precompute level parameters.
    let mut levels = vec![Level::default(); l + 1];
    levels[0].xn = n;
    levels[0].ylo = n - 1;
    levels[0].ydeg = 2;
    let mut qchain_total = 0;
    for k in 0..l {
        levels[k].qoff = qchain_total;
        qchain_total += levels[k].xn * levels[k].ydeg;
        levels[k + 1].xn = levels[k].xn.div_ceil(2);
        levels[k + 1].ylo = levels[k].ylo.saturating_sub(levels[k].ydeg - 1);
        levels[k + 1].ydeg = min((levels[k].ydeg << 1) - 1, n);
    }
    levels[l].qoff = qchain_total;
    qchain_total += levels[l].xn * levels[l].ydeg;
    // Compute scratch_total: maximum KS working memory across all levels.
    let mut scratch_total = 0;
    let mut yn_w_pre = n;
    for k in 0..l {
        let xn = levels[k].xn;
        let ydeg = levels[k].ydeg;
        let xnh = levels[k + 1].xn;
        let s = (ydeg << 1) - 1;
        let qks_len = (xn - 1) * s + ydeg;
        let rks_len = s * ((xnh << 1) - 1);
        scratch_total = scratch_total.max((qks_len << 1) + rks_len);
    }
    for k in (0..l).rev() {
        let xn = levels[k].xn;
        let xnh = levels[k + 1].xn;
        let ydeg = levels[k].ydeg;
        let ylo = levels[k].ylo;
        let ylor = levels[k + 1].ylo;
        let yn_r = n - ylor;
        let yslice_lo = ylo - ylor;
        let s = ydeg + yn_w_pre - 1;
        let qneg_ks_len = (xn - 1) * s + ydeg;
        let w_ks_len = (((xnh - 1) * s) << 1) + yn_w_pre;
        let r_rks_len = (xn - 1) * s + yn_r - yslice_lo;
        scratch_total = scratch_total.max(qneg_ks_len + w_ks_len + r_rks_len);
        yn_w_pre = n - ylo;
    }
    let mut buffer = vec![Integer::ZERO; qchain_total + scratch_total];
    let (qchain, scratch) = buffer.split_at_mut(qchain_total);
    // Initialise Q_0 = 1 - y * g(x).
    qchain[levels[0].qoff] = Integer::ONE;
    for (q, y) in qchain[levels[0].qoff + n..].iter_mut().zip(&ys[..len2]) {
        *q = -y;
    }
    // Downward pass: compute Q_1, Q_2, ..., Q_L via Graeffe steps. Each Q_k is read again in the
    // upward pass, so it is copied.
    for k in 0..l {
        let xn = levels[k].xn;
        let ydeg = levels[k].ydeg;
        let xnh = levels[k + 1].xn;
        let yda = levels[k + 1].ydeg;
        let s = (ydeg << 1) - 1;
        let qks_len = (xn - 1) * s + ydeg;
        let rks_len = s * ((xnh << 1) - 1);
        let (qchain_lo, qk1) = qchain.split_at_mut(levels[k + 1].qoff);
        let qk = &qchain_lo[levels[k].qoff..];
        split_into_chunks_mut!(scratch, qks_len, [qks, qneg_ks], rest);
        let rks = &mut rest[..rks_len];
        zero_gaps(qks, s, ydeg);
        zero_gaps(qneg_ks, s, ydeg);
        // Pack Q and Q(-x, y): x^i y^j -> flat index i * s + j.
        for j in 0..ydeg {
            for i in 0..xn {
                let src = &qk[j * xn + i];
                qks[i * s + j] = src.clone();
                qneg_ks[i * s + j] = if i.odd() { -src } else { src.clone() };
            }
        }
        mul_truncated_to_out(rks, qks, qneg_ks);
        // Unpack even-x columns -> Q_{k+1}.
        for j in 0..yda {
            for i in 0..xnh {
                qk1[j * xnh + i] = take(&mut rks[((i * s) << 1) + j]);
            }
        }
    }
    // Base case: solve P(y) / Q_L(0, y) mod y^n. Since Q_{k+1}(0, y) = Q_k(0, y)^2 and Q_0(0, y) =
    // 1, Q_L(0, y) = 1, and W = P.
    let mut w = vec![Integer::ZERO; n.next_power_of_two()];
    let mut yn_w = n;
    let glen = min(len1, n);
    let ql = &qchain[levels[l].qoff..];
    let jmax = min(levels[l].ydeg, glen);
    debug_assert!(ql[0] == 1u32 && ql[1..jmax].iter().all(|q| *q == 0u32));
    for (w, x) in w[n - glen..n].iter_mut().zip(xs[..glen].iter().rev()) {
        *w = x.clone();
    }
    // Upward pass: for k = L - 1 down to 0, compute the post-multiply.
    for k in (0..l).rev() {
        let xn = levels[k].xn;
        let xnh = levels[k + 1].xn;
        let ydeg = levels[k].ydeg;
        let ylo = levels[k].ylo;
        let ylor = levels[k + 1].ylo;
        let yn_r = n - ylor;
        let s = ydeg + yn_w - 1;
        let qneg_ks_len = (xn - 1) * s + ydeg;
        let w_ks_len = (((xnh - 1) * s) << 1) + yn_w;
        let yslice_lo = ylo - ylor;
        let r_rks_len = (xn - 1) * s + yn_r - yslice_lo;
        let qk = &mut qchain[levels[k].qoff..];
        let (qneg_ks, rest) = scratch.split_at_mut(qneg_ks_len);
        let (w_ks, rest) = rest.split_at_mut(w_ks_len);
        let r_rks = &mut rest[..r_rks_len];
        zero_gaps(qneg_ks, s, ydeg);
        zero_gaps(w_ks, s << 1, yn_w);
        // Pack Qneg_k: x^i y^j -> flat index i * s + j, with sign flip for odd i.
        for j in 0..ydeg {
            for i in 0..xn {
                let dst = &mut qneg_ks[i * s + j];
                *dst = take(&mut qk[j * xn + i]);
                if i.odd() {
                    dst.neg_assign();
                }
            }
        }
        // Pack W(x^2): x^{2i} y^j -> flat index 2 * i * s + j. W is overwritten below.
        for j in 0..yn_w {
            for i in 0..xnh {
                w_ks[((i * s) << 1) + j] = take(&mut w[j * xnh + i]);
            }
        }
        mul_middle_to_out(r_rks, qneg_ks, w_ks, yslice_lo, yslice_lo + r_rks_len);
        // Unpack y-slice [yslice_lo, yn_r) -> W (in place).
        for (jout, jsrc) in (yslice_lo..yn_r).enumerate() {
            for i in 0..xn {
                w[jout * xn + i] = take(&mut r_rks[i * s + jsrc - yslice_lo]);
            }
        }
        yn_w = n - ylo;
    }
    // W[0..n) holds the result f(g(x)) mod x^n.
    w.truncate(n);
    w
}}

// The first `n` coefficients of $p(cx^v)$, where `xs` holds the coefficients of $p$ and $v \geq 1$:
// $p_i c^i$ at $x^{iv}$.
fn compose_truncated_monomial(xs: &[Integer], c: &Integer, v: usize, n: usize) -> Vec<Integer> {
    let mut out = vec![Integer::ZERO; n];
    let mut power = Integer::ONE;
    for (o, x) in out.iter_mut().step_by(v).zip(xs) {
        *o = x * &power;
        power *= c;
    }
    out
}

// Whether Horner's rule is used rather than one of the other algorithms, for an outer polynomial
// with `len1` coefficients, an inner one with `len2`, a result with `n`, and coefficients of at
// most `bits` bits. Each of Horner's `len1` steps multiplies by the inner polynomial, while the
// Brent–Kung algorithm costs about $\sqrt n$ full products. Measured on this machine, Horner's
// rule is about as fast or faster up to $2.5\sqrt n$ outer coefficients, and for any number when
// the inner polynomial has at most 5 coefficients and they are not small, so that the full products
// the other algorithms rely on are expensive. FLINT uses Horner's rule up to 10 outer coefficients,
// whatever the other lengths.
fn horner_preferred(len1: usize, len2: usize, n: usize, bits: u64) -> bool {
    let (len1, n) = (u128::exact_from(len1), u128::exact_from(n));
    (len1 * len1) << 2 <= 25 * n || (len2 <= 5 && bits >= 8)
}

// Whether the Kinoshita–Li algorithm is used rather than the Brent–Kung algorithm, for an outer
// polynomial with `len1` coefficients, a result with `n`, and coefficients of at most `bits` bits.
// The cost of the Brent–Kung algorithm falls with the outer length, and that of the
// Kinoshita–Li algorithm does not, while the latter's intermediate polynomials have large
// coefficients. So it only pays off for long results with small coefficients and a comparably long
// outer polynomial: measured on this machine, from 1024 coefficients when they have at most 1 bit,
// 2048 at 2 or 3 bits, and 4096 at 4 bits, when the outer polynomial has at least three quarters of
// the result's length; at 6 and 8 bits the two are even at 3072, and at 16 bits and more the
// Brent–Kung algorithm is faster throughout the measured range. FLINT switches at 500
// coefficients whatever the lengths and sizes.
const fn kinoshita_li_preferred(len1: usize, n: usize, bits: u64) -> bool {
    let min_n = match bits {
        0..=1 => 1024,
        2..=3 => 2048,
        4 => 4096,
        _ => return false,
    };
    n >= min_n && len1 << 2 >= 3 * n
}

// The first `n` coefficients of the composition of the polynomials with coefficients `xs` and `ys`,
// where `xs.len() >= 2` and `ys[0]` is 0: directly if `ys` has a single nonzero coefficient, and
// otherwise by Horner's rule, the Brent–Kung algorithm, or the Kinoshita–Li algorithm, as
// measured.
//
// This is equivalent to `_fmpz_poly_compose_series` from `fmpz_poly/compose_series.c`, FLINT 3.6.0,
// except for the monomial case and the choice of algorithm (see `horner_preferred` and
// `kinoshita_li_preferred`).
fn compose_truncated_dispatch(xs: &[Integer], ys: &[Integer], n: usize) -> Vec<Integer> {
    let v = ys.iter().position(|y| *y != 0u32).unwrap();
    if ys[v + 1..].iter().all(|y| *y == 0u32) {
        compose_truncated_monomial(xs, &ys[v], v, n)
    } else {
        let bits = max(vec_max_bits(xs).0, vec_max_bits(ys).0);
        if horner_preferred(xs.len(), ys.len(), n, bits) {
            compose_truncated_horner(xs, ys, n)
        } else if kinoshita_li_preferred(xs.len(), n, bits) {
            compose_truncated_kinoshita_li(xs, ys, n)
        } else {
            compose_truncated_brent_kung(xs, ys, n)
        }
    }
}

// The composition of the polynomials with coefficients `xs` and `ys`, keeping only the coefficients
// of $x^i$ for $i$ less than `len`, with `kernel` doing the work in the general case. If $v$ is the
// index of the first nonzero coefficient of `ys`, the term $p_i q^i$ is divisible by $x^{iv}$, so
// only the first $\lceil n/v \rceil$ coefficients of `xs` are read.
//
// This is equivalent to `fmpz_poly_compose_series` from `fmpz_poly/compose_series.c`, FLINT 3.6.0,
// and to its `_horner` and `_brent_kung` variants, which differ only in the kernel, except that
// `xs` is truncated using $v$.
crate_test_fn! {compose_truncated_with(
    xs: &[Integer],
    ys: &[Integer],
    len: u64,
    kernel: fn(&[Integer], &[Integer], usize) -> Vec<Integer>,
) -> IntegerPolynomial {
    if let Some(y) = ys.first() {
        assert_eq!(
            *y, 0u32,
            "the inner polynomial must have constant coefficient 0"
        );
    }
    let n = usize::try_from(len).unwrap_or(usize::MAX);
    if xs.is_empty() || n == 0 {
        return IntegerPolynomial::ZERO;
    }
    if !ys.is_empty() {
        let v = ys.iter().position(|y| *y != 0u32).unwrap();
        let xs = &xs[..min(xs.len(), (n - 1) / v + 1)];
        if xs.len() > 1 {
            let lenr = min((xs.len() - 1).saturating_mul(ys.len() - 1) + 1, n);
            let xs = &xs[..min(xs.len(), lenr)];
            let ys = &ys[..min(ys.len(), lenr)];
            let mut out = kernel(xs, ys, lenr);
            trim_coefficients(&mut out);
            return IntegerPolynomial { coefficients: out };
        }
    }
    IntegerPolynomial::from(xs[0].clone())
}}

#[inline]
fn compose_truncated_ref(xs: &[Integer], ys: &[Integer], len: u64) -> IntegerPolynomial {
    compose_truncated_with(xs, ys, len, compose_truncated_dispatch)
}

impl ComposeTruncated<Self> for IntegerPolynomial {
    type Output = Self;

    /// Composes two [`IntegerPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. The result is trimmed.
    ///
    /// If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled and spread out
    /// directly. Otherwise, Horner's rule is used for short outer polynomials; the Brent–Kung
    /// algorithm, which combines blocks of about $\sqrt n$ coefficients of $p$ with the first
    /// $\sqrt n$ powers of $q$ in a single matrix product and then applies Horner's rule in
    /// $q^{\sqrt n}$, for most others; and the algorithm of Kinoshita and Li, which takes a nearly
    /// linear number of operations, for long results with small coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(IntegerPolynomial::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "3*x^2+2*x+3"
    /// );
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(IntegerPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "4*x^2+2*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^4+x+1").unwrap())
    ///         .compose_truncated(IntegerPolynomial::from_str("x^2-x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2-x+1"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(IntegerPolynomial::from_str("x^2+x").unwrap(), 0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: Self, len: u64) -> Self {
        compose_truncated_ref(&self.coefficients, &other.coefficients, len)
    }
}

impl ComposeTruncated<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Composes two [`IntegerPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. The result is trimmed.
    ///
    /// If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled and spread out
    /// directly. Otherwise, Horner's rule is used for short outer polynomials; the Brent–Kung
    /// algorithm, which combines blocks of about $\sqrt n$ coefficients of $p$ with the first
    /// $\sqrt n$ powers of $q$ in a single matrix product and then applies Horner's rule in
    /// $q^{\sqrt n}$, for most others; and the algorithm of Kinoshita and Li, which takes a nearly
    /// linear number of operations, for long results with small coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(&IntegerPolynomial::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "3*x^2+2*x+3"
    /// );
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(&IntegerPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "4*x^2+2*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^4+x+1").unwrap())
    ///         .compose_truncated(&IntegerPolynomial::from_str("x^2-x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2-x+1"
    /// );
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(&IntegerPolynomial::from_str("x^2+x").unwrap(), 0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: &Self, len: u64) -> Self {
        compose_truncated_ref(&self.coefficients, &other.coefficients, len)
    }
}

impl ComposeTruncated<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Composes two [`IntegerPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. The result is trimmed.
    ///
    /// If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled and spread out
    /// directly. Otherwise, Horner's rule is used for short outer polynomials; the Brent–Kung
    /// algorithm, which combines blocks of about $\sqrt n$ coefficients of $p$ with the first
    /// $\sqrt n$ powers of $q$ in a single matrix product and then applies Horner's rule in
    /// $q^{\sqrt n}$, for most others; and the algorithm of Kinoshita and Li, which takes a nearly
    /// linear number of operations, for long results with small coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(IntegerPolynomial::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "3*x^2+2*x+3"
    /// );
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(IntegerPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "4*x^2+2*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^4+x+1").unwrap())
    ///         .compose_truncated(IntegerPolynomial::from_str("x^2-x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2-x+1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(IntegerPolynomial::from_str("x^2+x").unwrap(), 0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: IntegerPolynomial, len: u64) -> IntegerPolynomial {
        compose_truncated_ref(&self.coefficients, &other.coefficients, len)
    }
}

impl ComposeTruncated<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Composes two [`IntegerPolynomial`]s and truncates the result to its first `len`
    /// coefficients, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. The result is trimmed.
    ///
    /// If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled and spread out
    /// directly. Otherwise, Horner's rule is used for short outer polynomials; the Brent–Kung
    /// algorithm, which combines blocks of about $\sqrt n$ coefficients of $p$ with the first
    /// $\sqrt n$ powers of $q$ in a single matrix product and then applies Horner's rule in
    /// $q^{\sqrt n}$, for most others; and the algorithm of Kinoshita and Li, which takes a nearly
    /// linear number of operations, for long results with small coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(&IntegerPolynomial::from_str("x^2+x").unwrap(), 3)
    ///         .to_string(),
    ///     "3*x^2+2*x+3"
    /// );
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^3+x^2+x+1").unwrap())
    ///         .compose_truncated(&IntegerPolynomial::from_str("2*x").unwrap(), 3)
    ///         .to_string(),
    ///     "4*x^2+2*x+1"
    /// );
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^4+x+1").unwrap())
    ///         .compose_truncated(&IntegerPolynomial::from_str("x^2-x").unwrap(), 4)
    ///         .to_string(),
    ///     "x^2-x+1"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+2*x+3").unwrap())
    ///         .compose_truncated(&IntegerPolynomial::from_str("x^2+x").unwrap(), 0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn compose_truncated(self, other: &IntegerPolynomial, len: u64) -> IntegerPolynomial {
        compose_truncated_ref(&self.coefficients, &other.coefficients, len)
    }
}

impl ComposeTruncatedAssign<Self> for IntegerPolynomial {
    /// Composes an [`IntegerPolynomial`] with another in place and truncates the result to its
    /// first `len` coefficients, taking the second by value.
    ///
    /// $$
    /// p \gets p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. The result is trimmed.
    ///
    /// If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled and spread out
    /// directly. Otherwise, Horner's rule is used for short outer polynomials; the Brent–Kung
    /// algorithm, which combines blocks of about $\sqrt n$ coefficients of $p$ with the first
    /// $\sqrt n$ powers of $q$ in a single matrix product and then applies Horner's rule in
    /// $q^{\sqrt n}$, for most others; and the algorithm of Kinoshita and Li, which takes a nearly
    /// linear number of operations, for long results with small coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncatedAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2+2*x+3").unwrap();
    /// p.compose_truncated_assign(IntegerPolynomial::from_str("x^2+x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "3*x^2+2*x+3");
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// let mut p = IntegerPolynomial::from_str("x^3+x^2+x+1").unwrap();
    /// p.compose_truncated_assign(IntegerPolynomial::from_str("2*x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "4*x^2+2*x+1");
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// let mut p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// p.compose_truncated_assign(IntegerPolynomial::from_str("x^2-x").unwrap(), 4);
    /// assert_eq!(p.to_string(), "x^2-x+1");
    /// let mut p = IntegerPolynomial::from_str("x^2+2*x+3").unwrap();
    /// p.compose_truncated_assign(IntegerPolynomial::from_str("x^2+x").unwrap(), 0);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    #[inline]
    fn compose_truncated_assign(&mut self, other: Self, len: u64) {
        *self = compose_truncated_ref(&self.coefficients, &other.coefficients, len);
    }
}

impl ComposeTruncatedAssign<&Self> for IntegerPolynomial {
    /// Composes an [`IntegerPolynomial`] with another in place and truncates the result to its
    /// first `len` coefficients, taking the second by reference.
    ///
    /// $$
    /// p \gets p(q(x)) \bmod x^n.
    /// $$
    ///
    /// This is composition in the ring of power series modulo $x^n$. Since $q$ has constant
    /// coefficient 0, $q^i$ is divisible by $x^i$, so the coefficients of $x^i$ for $i \geq n$ of
    /// $p$ and $q$ do not affect the result, and if the first nonzero coefficient of $q$ is that of
    /// $x^v$, only the first $\lceil n/v \rceil$ coefficients of $p$ do. The result is trimmed.
    ///
    /// If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled and spread out
    /// directly. Otherwise, Horner's rule is used for short outer polynomials; the Brent–Kung
    /// algorithm, which combines blocks of about $\sqrt n$ coefficients of $p$ with the first
    /// $\sqrt n$ powers of $q$ in a single matrix product and then applies Horner's rule in
    /// $q^{\sqrt n}$, for most others; and the algorithm of Kinoshita and Li, which takes a nearly
    /// linear number of operations, for long results with small coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{5/2} m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Panics
    /// Panics if the constant coefficient of the inner polynomial is nonzero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeTruncatedAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2+2*x+3").unwrap();
    /// p.compose_truncated_assign(&IntegerPolynomial::from_str("x^2+x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "3*x^2+2*x+3");
    /// // A monomial inner polynomial scales and spreads the coefficients.
    /// let mut p = IntegerPolynomial::from_str("x^3+x^2+x+1").unwrap();
    /// p.compose_truncated_assign(&IntegerPolynomial::from_str("2*x").unwrap(), 3);
    /// assert_eq!(p.to_string(), "4*x^2+2*x+1");
    /// // Powers of the inner polynomial from the fourth on vanish modulo x^4.
    /// let mut p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// p.compose_truncated_assign(&IntegerPolynomial::from_str("x^2-x").unwrap(), 4);
    /// assert_eq!(p.to_string(), "x^2-x+1");
    /// let mut p = IntegerPolynomial::from_str("x^2+2*x+3").unwrap();
    /// p.compose_truncated_assign(&IntegerPolynomial::from_str("x^2+x").unwrap(), 0);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    #[inline]
    fn compose_truncated_assign(&mut self, other: &Self, len: u64) {
        *self = compose_truncated_ref(&self.coefficients, &other.coefficients, len);
    }
}
