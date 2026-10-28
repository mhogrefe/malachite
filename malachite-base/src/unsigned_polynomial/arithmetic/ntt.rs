// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Multiplication of polynomials with word coefficients modulo $2^k$ by number-theoretic transforms
// modulo several word primes. Only the product modulo $2^k$ is wanted, so the inputs are reduced
// modulo $2^k$, which makes every coefficient of their product over the integers less than $l
// 2^{2k}$, where $l$ is the length of the shorter factor; the product is computed exactly modulo
// enough primes for their product to exceed that bound, and each coefficient is reconstructed
// modulo $2^W$ by Garner's algorithm, whose mixed-radix digits are combined in wrapping arithmetic.
// For small $k$ a single prime suffices, so the cost falls with $k$.
//
// Each prime is $p = c 2^{40} + 1$, with $2^{61} < p < 2^{62}$, so transforms of up to $2^{40}$
// points exist modulo every prime, and Montgomery multiplication with $R = 2^{64}$ never overflows.
// The forward transform is decimation in frequency, taking its input in natural order and leaving
// its output in bit-reversed order, and the inverse is decimation in time, taking its input in
// bit-reversed order, so no bit reversal is ever done. The data stay in normal form; only the
// twiddle factors are in Montgomery form, so multiplying by one is a Montgomery multiplication.

use crate::num::arithmetic::traits::{
    CeilingLogBase2, PowerOf2, WrappingMulAssign, WrappingSubMul, XMulYToZZ,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::logic::traits::BitIterable;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::mul_wrapping;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;

// The primes $p$, each with a root of unity of order exactly $2^{40}$ modulo $p$. They were found
// by a script, which took the largest primes of the form $c 2^{40} + 1$ below $2^{62}$, and for
// each a quadratic non-residue $g$, whose power $g^c$ then has order $2^{40}$.
const PRIMES: [(u64, u64); 6] = [
    (4611615649683210241, 611971507056309289),
    (4611613450659954689, 291604889638457747),
    (4611549678985543681, 1291242423054889089),
    (4611546380450660353, 3446508872214126660),
    (4611524390218104833, 3830756528182632641),
    (4611496902427410433, 1505342663865148807),
];

// The base-2 logarithm of the order of the roots of unity in `PRIMES`, the longest transform.
const MAX_LOG_LEN: u64 = 40;

// A lower bound on the number of bits of every prime in `PRIMES`: each exceeds $2^{61}$.
const PRIME_BITS: u64 = 61;

// Arithmetic modulo one of the primes, with Montgomery multiplication for $R = 2^{64}$.
#[derive(Clone, Copy)]
struct Prime {
    p: u64,
    // $2p$, the bound of lazily reduced values.
    two_p: u64,
    // $p^{-1} \bmod 2^{64}$.
    inverse: u64,
    // $-p^{-1} \bmod 2^{64}$.
    neg_inverse: u64,
    // $R^2 \bmod p$, for converting into Montgomery form.
    r_squared: u64,
}

impl Prime {
    fn new(p: u64) -> Self {
        // Newton's iteration doubles the number of correct low bits of the inverse each time,
        // starting from 3 (p is odd, so p * p = 1 mod 8).
        let mut inverse = p;
        for _ in 0..5 {
            inverse.wrapping_mul_assign(2u64.wrapping_sub_mul(p, inverse));
        }
        let r = ((1u128 << 64) % u128::from(p)) as u64;
        Self {
            p,
            two_p: p << 1,
            inverse,
            neg_inverse: inverse.wrapping_neg(),
            r_squared: ((u128::from(r) * u128::from(r)) % u128::from(p)) as u64,
        }
    }

    #[inline]
    const fn add(self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p { s - self.p } else { s }
    }

    #[inline]
    const fn sub(self, a: u64, b: u64) -> u64 {
        if a >= b { a - b } else { a + self.p - b }
    }

    // $abR^{-1} \bmod p$, for $a, b < p$.
    #[inline]
    const fn mul(self, a: u64, b: u64) -> u64 {
        let t = a as u128 * b as u128;
        let m = (t as u64).wrapping_mul(self.neg_inverse);
        // t + m * p < p^2 + 2^64 p < 2^128, since p < 2^62, and it is divisible by 2^64.
        let u = ((t + m as u128 * self.p as u128) >> 64) as u64;
        if u >= self.p { u - self.p } else { u }
    }

    // A value congruent to $abR^{-1}$ modulo $p$ and in $(0, 2p)$, for any 64-bit $a$ and $b < 2p$
    // with $ab < 2^{64} p$: with $m = t p^{-1} \bmod 2^{64}$, where $t = ab$, $t - mp$ is divisible
    // by $2^{64}$, and its quotient, the difference of the high words of $t$ and $mp$, is in $(-p,
    // p)$, so adding $p$ leaves it in $(0, 2p)$.
    #[inline]
    fn mul_lazy(self, a: u64, b: u64) -> u64 {
        let (t_hi, t_lo) = u64::x_mul_y_to_zz(a, b);
        let m = t_lo.wrapping_mul(self.inverse);
        let h = u64::x_mul_y_to_zz(m, self.p).0;
        t_hi.wrapping_sub(h).wrapping_add(self.p)
    }

    // $x$ reduced from $[0, 2p)$ into $[0, p)$.
    #[inline]
    const fn reduce_once(self, x: u64) -> u64 {
        if x >= self.p { x - self.p } else { x }
    }

    // $aR \bmod p$, the Montgomery form of $a < p$.
    #[inline]
    const fn to_montgomery(self, a: u64) -> u64 {
        self.mul(a, self.r_squared)
    }

    // $x \bmod p$, for any 64-bit $x$: since $p > 2^{61}$, at most three subtractions are needed.
    #[inline]
    const fn reduce(self, mut x: u64) -> u64 {
        while x >= self.p {
            x -= self.p;
        }
        x
    }

    // $x \bmod p$, for any 128-bit $x$.
    #[inline]
    const fn reduce_wide(self, x: u128) -> u64 {
        let hi = self.reduce((x >> 64) as u64);
        // hi * 2^64 = hi * R, which is the Montgomery multiplication of hi by R^2.
        self.add(self.mul(hi, self.r_squared), self.reduce(x as u64))
    }

    // $a^e \bmod p$ in Montgomery form, where `a` is in Montgomery form.
    fn pow_montgomery(self, a: u64, e: u64) -> u64 {
        let mut result = self.to_montgomery(1);
        let mut base = a;
        for bit in e.bits() {
            if bit {
                result = self.mul(result, base);
            }
            base = self.mul(base, base);
        }
        result
    }

    // $a^{-1} \bmod p$ in Montgomery form, where `a` is in Montgomery form, by Fermat's little
    // theorem.
    fn inverse_montgomery(self, a: u64) -> u64 {
        self.pow_montgomery(a, self.p - 2)
    }
}

// The twiddle factors of the transforms of length $N = 2^{\text{log\_len}}$, where `log_len` is
// positive, modulo the prime, in Montgomery form: for each level, whose butterflies span $2h$
// elements, the powers $w_{2h}^j$, for $0 \leq j < h$, of a primitive $2h$th root of unity, stored
// contiguously at offset $h - 1$, so that each level reads its factors in order; and the same for
// the inverse root, after them. Since $w^{N/2} = -1$ for a primitive $N$th root $w$, $w^{-j} =
// -w^{N/2 - j}$ for $0 < j < N/2$.
fn twiddles(prime: Prime, root: u64, log_len: u64) -> Vec<u64> {
    let w = prime.pow_montgomery(
        prime.to_montgomery(root),
        u64::power_of_2(MAX_LOG_LEN - log_len),
    );
    let n = usize::power_of_2(log_len);
    let half_n = n >> 1;
    let mut powers = Vec::with_capacity(half_n);
    let mut power = prime.to_montgomery(1);
    for _ in 0..half_n {
        powers.push(power);
        power = prime.mul(power, w);
    }
    let mut tables = vec![0; (n - 1) << 1];
    let (table, inverse_table) = tables.split_at_mut(n - 1);
    let mut h = 1;
    while h < n {
        let stride = half_n / h;
        let level = &mut table[h - 1..(h << 1) - 1];
        let inverse_level = &mut inverse_table[h - 1..(h << 1) - 1];
        for (j, (t, u)) in level.iter_mut().zip(inverse_level.iter_mut()).enumerate() {
            let k = j * stride;
            *t = powers[k];
            *u = if k == 0 {
                powers[0]
            } else {
                prime.p - powers[half_n - k]
            };
        }
        h <<= 1;
    }
    tables
}

// The forward butterfly: $(x, y) \mapsto (x + y, (x - y) w)$, lazily reduced into $[0, 2p)$.
#[inline]
fn forward_butterfly(prime: Prime, a: &mut u64, b: &mut u64, w: u64) {
    let (x, y) = (*a, *b);
    let s = x + y;
    *a = if s >= prime.two_p { s - prime.two_p } else { s };
    *b = prime.mul_lazy(x + prime.two_p - y, w);
}

// The forward butterfly with $w = 1$.
#[inline]
const fn forward_butterfly_1(prime: Prime, a: &mut u64, b: &mut u64) {
    let (x, y) = (*a, *b);
    let s = x + y;
    *a = if s >= prime.two_p { s - prime.two_p } else { s };
    let d = x + prime.two_p - y;
    *b = if d >= prime.two_p { d - prime.two_p } else { d };
}

// The inverse butterfly: $(x, y) \mapsto (x + yw, x - yw)$, lazily reduced into $[0, 2p)$.
#[inline]
fn inverse_butterfly(prime: Prime, a: &mut u64, b: &mut u64, w: u64) {
    let x = *a;
    let t = prime.mul_lazy(*b, w);
    let s = x + t;
    *a = if s >= prime.two_p { s - prime.two_p } else { s };
    let d = x + prime.two_p - t;
    *b = if d >= prime.two_p { d - prime.two_p } else { d };
}

// Transforms `xs`, whose length $N$ is a power of 2 no less than 2, in place, by decimation in
// frequency: the result is the evaluation of the polynomial with coefficients `xs` at the powers of
// the root whose twiddle factors are `table` (see `twiddles`), in bit-reversed order. The elements
// are reduced lazily, as in Harvey's butterflies: they stay in $[0, 2p)$. The factor $w^0 = 1$,
// which begins every block and makes up the whole last level, is not multiplied by.
fn forward(prime: Prime, xs: &mut [u64], table: &[u64]) {
    let mut h = xs.len() >> 1;
    while h > 1 {
        let level = &table[h - 1..(h << 1) - 1];
        for block in xs.chunks_exact_mut(h << 1) {
            let (lo, hi) = block.split_at_mut(h);
            forward_butterfly_1(prime, &mut lo[0], &mut hi[0]);
            for ((a, b), &w) in lo[1..].iter_mut().zip(hi[1..].iter_mut()).zip(&level[1..]) {
                forward_butterfly(prime, a, b, w);
            }
        }
        h >>= 1;
    }
    for [a, b] in xs.as_chunks_mut::<2>().0 {
        forward_butterfly_1(prime, a, b);
    }
}

// Undoes `forward`, except for the factor $N$: transforms `xs`, in bit-reversed order, by
// decimation in time with the inverse root, whose twiddle factors are `inverse_table` (see
// `twiddles`), leaving the result in natural order, multiplied by $N$. The elements stay in $[0,
// 2p)$.
fn inverse(prime: Prime, xs: &mut [u64], inverse_table: &[u64]) {
    let n = xs.len();
    for [a, b] in xs.as_chunks_mut::<2>().0 {
        forward_butterfly_1(prime, a, b);
    }
    let mut h = 2;
    while h < n {
        let level = &inverse_table[h - 1..(h << 1) - 1];
        for block in xs.chunks_exact_mut(h << 1) {
            let (lo, hi) = block.split_at_mut(h);
            forward_butterfly_1(prime, &mut lo[0], &mut hi[0]);
            for ((a, b), &w) in lo[1..].iter_mut().zip(hi[1..].iter_mut()).zip(&level[1..]) {
                inverse_butterfly(prime, a, b, w);
            }
        }
        h <<= 1;
    }
}

// The residues modulo the prime of the coefficients of the polynomial with coefficients `xs`, each
// reduced modulo $2^k$ first, where $k$ is `pow`, reduced modulo $x^n - 1$: the element at index
// $j$ is added at index $j \bmod n$, and the result is padded with zeros to length `n`.
fn residues<T: PrimitiveUnsigned>(prime: Prime, xs: &[T], pow: u64, n: usize) -> Vec<u64> {
    let mut out = vec![0; n];
    for chunk in xs.chunks(n) {
        for (o, &x) in out.iter_mut().zip(chunk) {
            let x: u128 = x.mod_power_of_2(pow).wrapping_into();
            let r = if T::WIDTH <= 64 {
                prime.reduce(x as u64)
            } else {
                prime.reduce_wide(x)
            };
            *o = prime.add(*o, r);
        }
    }
    out
}

// The number of primes needed for the product of polynomials whose coefficients are less than
// $2^k$, where $k$ is `pow`, the shorter of which has `len` coefficients: enough for the product of
// the primes to exceed every coefficient of the product over the integers, which is less than
// $\text{len}\, 2^{2k}$.
fn prime_count(pow: u64, len: usize) -> usize {
    let bits = (pow << 1) + u64::wrapping_from(len).ceiling_log_base_2() + 1;
    let count = usize::wrapping_from(bits.div_ceil(PRIME_BITS));
    assert!(count <= PRIMES.len());
    count
}

// Sets each element of `out` to the coefficient, modulo $2^W$, with residues `residues[i][j]`
// modulo the first primes, where `j` is its index, by Garner's algorithm: the mixed-radix digits
// $v_i$ are computed modulo the primes, and $\sum_i v_i \prod_{j < i} p_j$ is evaluated in wrapping
// arithmetic.
fn reconstruct<T: PrimitiveUnsigned>(out: &mut [T], residues: &[Vec<u64>]) {
    reconstruct_with(out, residues, &garner_inverses(residues.len()));
}

// The inverses that Garner's algorithm needs for the first `count` primes: element `[i][j]` is
// $p_j^{-1} \bmod p_i$, in Montgomery form, for $j < i$.
fn garner_inverses(count: usize) -> Vec<Vec<u64>> {
    let primes: Vec<Prime> = PRIMES[..count]
        .iter()
        .map(|&(p, _)| Prime::new(p))
        .collect();
    primes
        .iter()
        .enumerate()
        .map(|(i, &pi)| {
            primes[..i]
                .iter()
                .map(|pj| pi.inverse_montgomery(pi.to_montgomery(pi.reduce(pj.p))))
                .collect()
        })
        .collect()
}

// `reconstruct`, with the inverses from `garner_inverses` precomputed.
fn reconstruct_with<T: PrimitiveUnsigned>(
    out: &mut [T],
    residues: &[Vec<u64>],
    inverses: &[Vec<u64>],
) {
    let count = residues.len();
    let primes: Vec<Prime> = PRIMES[..count]
        .iter()
        .map(|&(p, _)| Prime::new(p))
        .collect();
    // radices[i] is the product of the first i primes, modulo 2^W.
    let mut radices = Vec::with_capacity(count);
    let mut radix = T::ONE;
    for prime in &primes {
        radices.push(radix);
        radix.wrapping_mul_assign(T::wrapping_from(prime.p));
    }
    let mut digits = vec![0u64; count];
    for (k, o) in out.iter_mut().enumerate() {
        for i in 0..count {
            let pi = primes[i];
            let mut t = residues[i][k];
            for (j, &d) in digits[..i].iter().enumerate() {
                t = pi.mul(pi.sub(t, pi.reduce(d)), inverses[i][j]);
            }
            digits[i] = t;
        }
        let mut sum = T::ZERO;
        for (&d, &r) in digits.iter().zip(&radices) {
            sum.wrapping_add_assign(T::wrapping_from(d).wrapping_mul(r));
        }
        *o = sum;
    }
}

// Sets `out`, whose length is at most $N = 2^{\text{log\_len}}$, to the first `out.len()`
// coefficients of the cyclic convolution of length $N$ of `xs` and `ys`, modulo $2^W$, computed
// exactly modulo the first `count` primes, whose product must exceed every coefficient of the
// convolution of `xs` and `ys` reduced modulo $2^k$, where $k$ is `pow`. If `ys` is `None`, `xs` is
// squared.
fn cyclic_ntt<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: Option<&[T]>,
    pow: u64,
    log_len: u64,
    count: usize,
) {
    assert!(log_len <= MAX_LOG_LEN);
    let n = usize::power_of_2(log_len);
    let len = out.len();
    let mut all_residues = Vec::with_capacity(count);
    for &(p, root) in &PRIMES[..count] {
        let prime = Prime::new(p);
        let tables = twiddles(prime, root, log_len);
        let (table, inverse_table) = tables.split_at(n - 1);
        let mut a = residues(prime, xs, pow, n);
        forward(prime, &mut a, table);
        if let Some(ys) = ys {
            let mut b = residues(prime, ys, pow, n);
            forward(prime, &mut b, table);
            for (x, y) in a.iter_mut().zip(&b) {
                *x = prime.mul_lazy(*x, *y);
            }
        } else {
            for x in &mut a {
                *x = prime.mul_lazy(*x, *x);
            }
        }
        inverse(prime, &mut a, inverse_table);
        all_residues.push(scaled_product(prime, &a[..len], transform_scale(prime, n)));
    }
    reconstruct(out, &all_residues);
}

// The constant that undoes the factors left by a product of transforms of length `n`: the pointwise
// products carry a factor $R^{-1}$, and the inverse transform a factor $n$, so multiplying by
// $n^{-1} R^2$, by a Montgomery multiplication, leaves the product. The inverse of $n$ is in
// Montgomery form, $n^{-1} R$, and multiplying it by $R^2$ gives $n^{-1} R^2$.
fn transform_scale(prime: Prime, n: usize) -> u64 {
    prime.mul(
        prime.inverse_montgomery(prime.to_montgomery(prime.reduce(u64::wrapping_from(n)))),
        prime.r_squared,
    )
}

// The elements of `xs`, the output of an inverse transform, multiplied by `scale` (see
// `transform_scale`) and fully reduced.
fn scaled_product(prime: Prime, xs: &[u64], scale: u64) -> Vec<u64> {
    xs.iter()
        .map(|&x| prime.reduce_once(prime.mul_lazy(x, scale)))
        .collect()
}

// The data for one prime of an `NttFixedFactor`: the prime, the twiddle factors of the transforms
// and of their inverses (see `twiddles`), the transform of the fixed factor, and the scale (see
// `transform_scale`).
struct FixedFactorPrime {
    prime: Prime,
    tables: Vec<u64>,
    transform: Vec<u64>,
    scale: u64,
}

// A polynomial prepared for being multiplied by many others, keeping only the first `n`
// coefficients of each product, by number-theoretic transforms: its transforms, the twiddle
// factors, and Garner's inverses are computed once. Each product then costs one forward transform
// and one inverse per prime rather than two forward and one inverse.
//
// With $\ell$ the length of the fixed factor after truncation to `n` coefficients, and the other
// factor at most `n` long, a product has at most $L = n + \ell - 1$ coefficients, so transforms of
// length $N \geq L$, a power of 2, compute it exactly. When $L$ exceeds $N/2$ by only $t \leq N /
// 8$, transforms of length $N/2$ are used instead, as in `mul_truncated_ntt_helper`: the
// coefficients that wrap around come from a short product of the top coefficients of the two
// factors, so the top $t$ coefficients of the fixed factor are kept.
pub(crate) struct NttFixedFactor<T: PrimitiveUnsigned> {
    n: usize,
    log_len: u64,
    pow: u64,
    // The length of the fixed factor, after truncation to `n` coefficients.
    len: usize,
    // The top coefficients of the fixed factor, for correcting wrapped coefficients; empty if
    // nothing can wrap.
    top: Vec<T>,
    primes: Vec<FixedFactorPrime>,
    inverses: Vec<Vec<u64>>,
}

impl<T: PrimitiveUnsigned> NttFixedFactor<T> {
    // Prepares the polynomial with coefficients `ys`, which is nonempty, for products truncated to
    // `n` coefficients, correct modulo $2^k$, where $k$ is `pow`.
    pub(crate) fn new(ys: &[T], n: usize, pow: u64) -> Self {
        let ys = &ys[..min(ys.len(), n)];
        let max_len = n + ys.len() - 1;
        let mut size = max_len.next_power_of_two().max(2);
        let half = size >> 1;
        let wraps = half >= 4 && max_len > half && max_len - half <= half / WRAP_DIVISOR;
        // Each coefficient of a cyclic convolution that wraps around is a sum of at most twice as
        // many products, so one more bit is needed.
        let count = if wraps {
            size = half;
            prime_count(pow + 1, min(n, ys.len()))
        } else {
            prime_count(pow, min(n, ys.len()))
        };
        let log_len = u64::wrapping_from(size).ceiling_log_base_2();
        assert!(log_len <= MAX_LOG_LEN);
        let top = if wraps {
            ys[ys.len().saturating_sub(max_len - size)..].to_vec()
        } else {
            Vec::new()
        };
        let primes = PRIMES[..count]
            .iter()
            .map(|&(p, root)| {
                let prime = Prime::new(p);
                let tables = twiddles(prime, root, log_len);
                let mut transform = residues(prime, ys, pow, size);
                forward(prime, &mut transform, &tables[..size - 1]);
                FixedFactorPrime {
                    prime,
                    tables,
                    transform,
                    scale: transform_scale(prime, size),
                }
            })
            .collect();
        Self {
            n,
            log_len,
            pow,
            len: ys.len(),
            top,
            primes,
            inverses: garner_inverses(count),
        }
    }

    // Sets `out`, which is nonempty and at most `n` long, to the first `out.len()` coefficients of
    // the product of the polynomial with coefficients `xs`, which is nonempty, and the fixed
    // factor, correct modulo $2^k$ but not reduced. The elements of `xs` need not be reduced.
    pub(crate) fn mul_truncated(&self, out: &mut [T], xs: &[T]) {
        let len = out.len();
        assert!(len <= self.n);
        let xs = &xs[..min(xs.len(), len)];
        let size = usize::power_of_2(self.log_len);
        let full_len = xs.len() + self.len - 1;
        let low_len = min(len, size);
        let mut all_residues = Vec::with_capacity(self.primes.len());
        for data in &self.primes {
            let prime = data.prime;
            let (table, inverse_table) = data.tables.split_at(size - 1);
            let mut a = residues(prime, xs, self.pow, size);
            forward(prime, &mut a, table);
            for (x, &y) in a.iter_mut().zip(&data.transform) {
                *x = prime.mul_lazy(*x, y);
            }
            inverse(prime, &mut a, inverse_table);
            all_residues.push(scaled_product(prime, &a[..low_len], data.scale));
        }
        reconstruct_with(&mut out[..low_len], &all_residues, &self.inverses);
        if full_len > size {
            // The coefficients c_i for i >= size wrapped around onto c_{i - size}; they involve
            // only the top `excess` coefficients of each factor.
            let excess = full_len - size;
            let start1 = xs.len().saturating_sub(excess);
            let start2 = self.len.saturating_sub(excess);
            let top_ys = &self.top[self.top.len() - (self.len - start2)..];
            let top_xs = &xs[start1..];
            let mut top = vec![T::ZERO; top_xs.len() + top_ys.len() - 1];
            mul_wrapping(&mut top, top_xs, top_ys, self.pow);
            let high = &top[size - start1 - start2..];
            for (o, &h) in out.iter_mut().zip(high) {
                o.wrapping_sub_assign(h);
            }
            if len > size {
                out[size..].copy_from_slice(&high[..len - size]);
            }
        }
    }
}

// The fraction of a transform length, as a divisor, by which a product may exceed it and still be
// computed with a transform of that length, the excess being wrapped around and corrected.
const WRAP_DIVISOR: usize = 4;

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, all three nonempty, modulo $2^k$, where $k$ is `pow`, which must be
// no greater than `T::WIDTH`; `out.len()` must be at most `xs.len() + ys.len() - 1`. The elements
// of `xs` and `ys` need not be reduced, and those of `out` are correct modulo $2^k$ but not
// reduced. If `ys` is `None`, `xs` is squared, with one transform fewer.
//
// A product of $L$ coefficients needs a transform of length $N \geq L$, a power of 2. When $L$
// exceeds $N/2$ by only $t \leq N / 8$, the cyclic convolution of length $N/2$ is computed instead:
// its coefficient $i$ is $c_i + c_{i + N/2}$, and the $t$ coefficients $c_{N/2}, \ldots, c_{L - 1}$
// come from the product of the top $t$ coefficients of each factor, which is short.
fn mul_truncated_ntt_helper<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: Option<&[T]>,
    pow: u64,
) {
    let len = out.len();
    let xs = &xs[..min(xs.len(), len)];
    let ys = ys.map(|ys| &ys[..min(ys.len(), len)]);
    let (len1, len2) = (xs.len(), ys.map_or(xs.len(), <[T]>::len));
    let full_len = len1 + len2 - 1;
    let n = full_len.next_power_of_two().max(2);
    let half = n >> 1;
    let excess = full_len.saturating_sub(half);
    if half < 4 || excess == 0 || excess > half / WRAP_DIVISOR {
        let log_len = u64::wrapping_from(n).ceiling_log_base_2();
        cyclic_ntt(out, xs, ys, pow, log_len, prime_count(pow, min(len1, len2)));
        return;
    }
    // The top coefficients: c_i for i >= half involves only the elements of `xs` from index len1 -
    // excess on, and those of `ys` from len2 - excess on.
    let (start1, start2) = (len1.saturating_sub(excess), len2.saturating_sub(excess));
    let top_xs = &xs[start1..];
    let top_ys = ys.map_or(top_xs, |ys| &ys[start2..]);
    let mut top = vec![T::ZERO; top_xs.len() + top_ys.len() - 1];
    mul_wrapping(&mut top, top_xs, top_ys, pow);
    let high = &top[half - start1 - start2..];
    // Each coefficient of the cyclic convolution is a sum of at most twice as many products, so one
    // more bit is needed.
    let log_len = u64::wrapping_from(half).ceiling_log_base_2();
    let low_len = min(len, half);
    cyclic_ntt(
        &mut out[..low_len],
        xs,
        ys,
        pow,
        log_len,
        prime_count(pow + 1, min(len1, len2)),
    );
    for (o, &h) in out.iter_mut().zip(high) {
        o.wrapping_sub_assign(h);
    }
    if len > half {
        out[half..].copy_from_slice(&high[..len - half]);
    }
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, all three nonempty, modulo $2^k$, where $k$ is `pow`, by
// number-theoretic transforms; `out.len()` must be at most `xs.len() + ys.len() - 1`. The inputs
// need not be reduced, and the output is correct modulo $2^k$ but not reduced.
crate_test_fn! {mul_truncated_ntt<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    mul_truncated_ntt_helper(out, xs, Some(ys), pow);
}}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, both nonempty, modulo $2^k$, where $k$ is `pow`, by number-theoretic
// transforms; `out.len()` must be at most `2 * xs.len() - 1`. The input need not be reduced, and
// the output is correct modulo $2^k$ but not reduced.
crate_test_fn! {square_truncated_ntt<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], pow: u64) {
    mul_truncated_ntt_helper(out, xs, None, pow);
}}

// The operations whose choice between Karatsuba multiplication and number-theoretic transforms is
// measured separately.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum NttOp {
    // A product, whole or truncated.
    Mul,
    // A whole square.
    Square,
    // A truncated square, which Karatsuba multiplication computes at about half the cost of a whole
    // one.
    SquareTruncated,
}

// Whether multiplications by a fixed factor, truncated to `n` coefficients, use an `NttFixedFactor`
// rather than Karatsuba multiplication, where `len` is the length of the shorter factor after
// truncation: each such product costs two transforms per prime rather than three, so the thresholds
// are about a third of `ntt_preferred`'s. Measured on an Apple M-series machine, 2026-10, for 8
// products by one fixed factor, all of the same length: from 224 coefficients with one prime, 864
// with two, and 1792 with three for words of at most 64 bits; and from 960 with four and 1728 with
// five primes for `u128`. The other entries are interpolated.
pub(crate) fn fixed_ntt_preferred<T: PrimitiveUnsigned>(len: usize, pow: u64) -> bool {
    const NARROW: [usize; 6] = [224, 864, 1792, 3072, 4096, 5120];
    const WIDE: [usize; 6] = [224, 640, 768, 960, 1728, 2048];
    let table = if T::WIDTH > 64 { &WIDE } else { &NARROW };
    len >= table[0] && len >= table[prime_count(pow, len) - 1]
}

// Whether a product of polynomials with coefficients of type `T` modulo $2^k$, where $k$ is `pow`,
// the shorter of whose factors (after any truncation) has `len` coefficients, is computed by
// number-theoretic transforms rather than by Karatsuba multiplication. The transforms cost about
// the same per prime whatever `T` is, and Karatsuba multiplication about twice as much for `u128`
// as for narrower types, so the thresholds depend on the number of primes and on whether `T` is
// `u128`. They were measured on an Apple M-series machine, 2026-10, on balanced factors.
pub(crate) fn ntt_preferred<T: PrimitiveUnsigned>(op: NttOp, len: usize, pow: u64) -> bool {
    // The thresholds for 1 to 6 primes.
    const NARROW: [[usize; 6]; 3] = [
        [512, 2048, 6144, 12288, 16384, 20480],
        [512, 2048, 6144, 12288, 16384, 20480],
        [896, 3584, 12288, 16384, 20480, 24576],
    ];
    const WIDE: [[usize; 6]; 3] = [
        [512, 1536, 2560, 3584, 6144, 8192],
        [512, 1536, 2560, 3584, 6144, 8192],
        [896, 2560, 4096, 6144, 7168, 9216],
    ];
    let row = match op {
        NttOp::Mul => 0,
        NttOp::Square => 1,
        NttOp::SquareTruncated => 2,
    };
    // The cheapest case, a single prime, has the lowest threshold; check it before counting.
    let table = if T::WIDTH > 64 { &WIDE } else { &NARROW };
    len >= table[row][0] && len >= table[row][prime_count(pow, len) - 1]
}

// Sets each element of `outs`, all with the same nonzero length $n$, to the first $n$ coefficients
// of the product of the corresponding element of `xss` and `ys`, correct modulo $2^k$, where $k$ is
// `pow`, by number-theoretic transforms with the transforms of `ys` computed once. For measuring
// `fixed_ntt_preferred`.
crate_test_fn! {
#[allow(dead_code)]
mul_truncated_many_by_fixed_ntt<T: PrimitiveUnsigned>(
    outs: &mut [Vec<T>],
    xss: &[Vec<T>],
    ys: &[T],
    pow: u64,
) {
    let factor = NttFixedFactor::<T>::new(ys, outs[0].len(), pow);
    for (out, xs) in outs.iter_mut().zip(xss) {
        factor.mul_truncated(out, xs);
    }
}}
