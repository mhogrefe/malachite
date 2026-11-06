// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
//      Copyright © 2011, 2012 Sebastian Pancratz
//
//      Copyright © 2013, 2014 Martin Lee
//
//      Copyright © 2020, 2023 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::FloorSqrt;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::modular::traits::{ModIsReduced, ModMul};
use crate::polynomial::Polynomial;
use crate::polynomial::mod_polynomial::traits::{
    ModComposeModPolynomial, ModComposeModPolynomialAssign,
};
use crate::polynomial::modular::ModPolynomialData;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_div_rem_precomputed::{
    mod_div_rem_blocks, precompute_helper,
};
use crate::unsigned_polynomial::modular::mod_mul::mod_mul_to_out;
use alloc::vec::Vec;
use core::mem::take;

// The arithmetic that modular composition needs, in a ring of coefficients modulo something and of
// polynomials modulo a polynomial $h$ with $n + 1$ coefficients. Polynomials reduced modulo $h$ are
// slices of exactly $n$ coefficients, padded with zeros.
//
// This plays the part of the `nmod_t` or `gr_ctx_t` that FLINT's functions take.
#[doc(hidden)]
pub trait ComposeModRing {
    type C: Clone + Eq;

    fn zero(&self) -> Self::C;

    fn one(&self) -> Self::C;

    fn add(&self, x: &Self::C, y: &Self::C) -> Self::C;

    fn mul(&self, x: &Self::C, y: &Self::C) -> Self::C;

    // The product of two polynomials of length $n$, reduced modulo $h$, with length $n$.
    fn mul_mod(&self, xs: &[Self::C], ys: &[Self::C]) -> Vec<Self::C>;

    // The product of two polynomials of length $n$, with length $2n - 1$.
    fn poly_mul(&self, xs: &[Self::C], ys: &[Self::C]) -> Vec<Self::C>;

    // The remainder of a polynomial of length $2n - 1$ divided by $h$, with length $n$.
    fn rem(&self, xs: &[Self::C]) -> Vec<Self::C>;

    // The product of matrices given by their rows, all of the same length.
    fn matrix_mul(&self, b: &[Vec<Self::C>], a: &[Vec<Self::C>]) -> Vec<Vec<Self::C>>;

    // The least length of the outer polynomial for which the Brent–Kung algorithm is faster than
    // Horner's rule, when $h$ has $n + 1$ coefficients.
    fn brent_kung_cutoff(&self, n: usize) -> usize;
}

fn vec_add<R: ComposeModRing>(ring: &R, xs: &mut [R::C], ys: &[R::C]) {
    for (x, y) in xs.iter_mut().zip(ys) {
        *x = ring.add(x, y);
    }
}

fn evaluate<R: ComposeModRing>(ring: &R, f: &[R::C], x: &R::C) -> R::C {
    let mut y = ring.zero();
    for c in f.iter().rev() {
        y = ring.add(&ring.mul(&y, x), c);
    }
    y
}

// The result of composing `f` with `g` modulo $h$ when it doesn't need the general algorithms: when
// $h$ is a constant, so that every result is 0, when `f` is constant, or when $h$ is linear, so
// that `g` is a constant and composition is evaluation.
fn compose_mod_special<R: ComposeModRing>(
    ring: &R,
    f: &[R::C],
    g: &[R::C],
    n: usize,
) -> Option<Vec<R::C>> {
    if n == 0 {
        Some(Vec::new())
    } else if f.len() <= 1 {
        let mut res = vec![ring.zero(); n];
        if let Some(c) = f.first() {
            res[0] = c.clone();
        }
        Some(res)
    } else if n == 1 {
        Some(vec![evaluate(ring, f, &g[0])])
    } else {
        None
    }
}

// The composition $f(g)$ modulo $h$, by Horner's rule. `g` has length $n$.
//
// This is equivalent to `_nmod_poly_compose_mod_horner` from `nmod_poly/compose_mod_horner.c`,
// FLINT 3.6.0.
#[doc(hidden)]
pub fn compose_mod_horner<R: ComposeModRing>(
    ring: &R,
    f: &[R::C],
    g: &[R::C],
    n: usize,
) -> Vec<R::C> {
    if let Some(res) = compose_mod_special(ring, f, g, n) {
        return res;
    }
    let mut i = f.len() - 1;
    let mut res: Vec<R::C> = g.iter().map(|x| ring.mul(x, &f[i])).collect();
    i -= 1;
    res[0] = ring.add(&res[0], &f[i]);
    while i > 0 {
        i -= 1;
        res = ring.mul_mod(&res, g);
        res[0] = ring.add(&res[0], &f[i]);
    }
    res
}

// Splits `f` into rows of `m` coefficients each, padding the last with zeros.
fn segments<R: ComposeModRing>(ring: &R, f: &[R::C], m: usize) -> Vec<Vec<R::C>> {
    f.chunks(m)
        .map(|chunk| {
            let mut row = chunk.to_vec();
            row.resize(m, ring.zero());
            row
        })
        .collect()
}

// The polynomials $g^0, g^1, \ldots, g^{m-1}$ modulo $h$, each of length $n$, with each power above
// the first the product of two lower ones of about half its exponent.
//
// This is equivalent to `_nmod_poly_powers_mod_preinv_naive` from `nmod_poly/powers_mod.c`, FLINT
// 3.6.0.
#[doc(hidden)]
pub fn powers_mod<R: ComposeModRing>(ring: &R, g: &[R::C], n: usize, m: usize) -> Vec<Vec<R::C>> {
    let mut a = Vec::with_capacity(m);
    let mut one = vec![ring.zero(); n];
    one[0] = ring.one();
    a.push(one);
    if m > 1 {
        a.push(g.to_vec());
    }
    for i in 2..m {
        let power = ring.mul_mod(&a[i >> 1], &a[i.div_ceil(2)]);
        a.push(power);
    }
    a
}

// Evaluates $\sum_{i} A_i h^i$ modulo the polynomial modulus, where $A_i$ is row $i$ of `a`, by
// Horner's rule.
fn mod_matrix_rows_evaluate_horner<R: ComposeModRing>(
    ring: &R,
    a: &[Vec<R::C>],
    h: &[R::C],
) -> Vec<R::C> {
    let len = a.len();
    let mut res = a[len - 1].clone();
    for i in (0..len - 1).rev() {
        res = ring.mul_mod(&res, h);
        vec_add(ring, &mut res, &a[i]);
    }
    res
}

// As `mod_matrix_rows_evaluate_horner`, by rectangular splitting: the powers $h^0, \ldots, h^k$ are
// precomputed, each block of $k$ rows is combined with full products and one reduction, and the
// blocks are combined by Horner's rule in $h^k$.
fn mod_matrix_rows_evaluate_rectangular<R: ComposeModRing>(
    ring: &R,
    a: &[Vec<R::C>],
    h: &[R::C],
    n: usize,
) -> Vec<R::C> {
    let len = a.len();
    let m = (len.floor_sqrt() + 1).min(30);
    let r = len.div_ceil(m);
    // Compute powers of h
    let xs = powers_mod(ring, h, n, m + 1);
    let mut s = vec![ring.zero(); (n << 1) - 1];
    s[..n].clone_from_slice(&a[(r - 1) * m]);
    for j in 0..len - (r - 1) * m - 1 {
        let t = ring.poly_mul(&xs[1 + j], &a[(r - 1) * m + 1 + j]);
        vec_add(ring, &mut s, &t);
    }
    let mut res = ring.rem(&s);
    for i in (0..r - 1).rev() {
        let mut s = vec![ring.zero(); (n << 1) - 1];
        s[..n].clone_from_slice(&a[i * m]);
        for j in 1..m {
            let t = ring.poly_mul(&xs[j], &a[i * m + j]);
            vec_add(ring, &mut s, &t);
        }
        let u = ring.rem(&s);
        res = ring.mul_mod(&res, &xs[m]);
        vec_add(ring, &mut res, &u);
    }
    res
}

// This is equivalent to `_nmod_poly_mod_matrix_rows_evaluate` from
// `nmod_poly/mod_matrix_rows_evaluate.c`, FLINT 3.6.0.
fn mod_matrix_rows_evaluate<R: ComposeModRing>(
    ring: &R,
    a: &[Vec<R::C>],
    h: &[R::C],
    n: usize,
) -> Vec<R::C> {
    if a.len() <= 10 {
        mod_matrix_rows_evaluate_horner(ring, a, h)
    } else {
        mod_matrix_rows_evaluate_rectangular(ring, a, h, n)
    }
}

// The rows of the Brent–Kung matrix for `g` modulo $h$ when `f` may be as long as $h$: $g^0,
// \ldots, g^{m-1}$ with $m = \lfloor\sqrt n\rfloor + 1$.
//
// This is equivalent to `_nmod_poly_precompute_matrix` from
// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0.
#[doc(hidden)]
#[inline]
pub fn precompute_matrix<R: ComposeModRing>(ring: &R, g: &[R::C], n: usize) -> Vec<Vec<R::C>> {
    powers_mod(ring, g, n, n.floor_sqrt() + 1)
}

// The composition $f(g)$ modulo $h$ by the Brent–Kung algorithm, given the powers $g^0, \ldots,
// g^{m-1}$ as the rows of `a`: `f` is split into blocks of $m$ coefficients, the blocks are
// composed with `g` all at once as the product of a matrix of blocks and `a`, and the results are
// combined by evaluation at $g^m$. `f` may have any length.
//
// This is equivalent to `_nmod_poly_compose_mod_brent_kung_precomp_preinv` from
// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `f` may be longer
// than $h$, in which case there are more blocks than rows of `a`.
#[doc(hidden)]
pub fn compose_mod_brent_kung_with_powers<R: ComposeModRing>(
    ring: &R,
    f: &[R::C],
    a: &[Vec<R::C>],
    n: usize,
) -> Vec<R::C> {
    if let Some(res) = compose_mod_special(ring, f, &a[1], n) {
        return res;
    }
    let m = a.len();
    let mut c = ring.matrix_mul(&segments(ring, f, m), a);
    if c.len() == 1 {
        return c.pop().unwrap();
    }
    // Evaluate block composition
    let h = ring.mul_mod(&a[m >> 1], &a[m - (m >> 1)]);
    mod_matrix_rows_evaluate(ring, &c, &h, n)
}

// The composition $f(g)$ modulo $h$ by the Brent–Kung algorithm, with blocks of $\lfloor\sqrt{l -
// 1}\rfloor + 1$ coefficients, where $l$ is the length of `f`. `g` has length $n$.
//
// This is equivalent to `_nmod_poly_compose_mod_brent_kung_preinv` from
// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `f` may have any length and
// the block size depends on its length rather than on that of $h$.
#[doc(hidden)]
pub fn compose_mod_brent_kung<R: ComposeModRing>(
    ring: &R,
    f: &[R::C],
    g: &[R::C],
    n: usize,
) -> Vec<R::C> {
    if let Some(res) = compose_mod_special(ring, f, g, n) {
        return res;
    }
    let a = powers_mod(ring, g, n, (f.len() - 1).floor_sqrt() + 1);
    compose_mod_brent_kung_with_powers(ring, f, &a, n)
}

// This is equivalent to `_nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
// except that the choice between Horner's rule and the Brent–Kung algorithm depends mostly on the
// length of `f` rather than on that of $h$, and the Brent–Kung algorithm is also used when `f` is
// at least as long as $h$.
#[doc(hidden)]
pub fn compose_mod_dispatch<R: ComposeModRing>(
    ring: &R,
    f: &[R::C],
    g: &[R::C],
    n: usize,
) -> Vec<R::C> {
    if f.len() < ring.brent_kung_cutoff(n) {
        compose_mod_horner(ring, f, g, n)
    } else {
        compose_mod_brent_kung(ring, f, g, n)
    }
}

// The block size for composing polynomials of the lengths `lens` with the same polynomial by the
// Brent–Kung algorithm: the $m \geq 2$ that minimizes the number of multiplications modulo $h$,
// $m - 1$ for the powers $g^2, \ldots, g^m$ and $\lceil l/m \rceil - 1$ for combining the blocks of
// each polynomial of length $l$. The search starts from $\lfloor\sqrt{s}\rfloor + 1$, where $s$ is
// the sum of the lengths minus their number, and stops at twice that.
fn brent_kung_vec_block_size(lens: &[usize]) -> usize {
    let s: usize = lens.iter().map(|len| len - 1).sum();
    let cost = |m: usize| m - 1 + lens.iter().map(|len| len.div_ceil(m) - 1).sum::<usize>();
    let start = s.floor_sqrt() + 1;
    let mut best = start;
    let mut best_cost = cost(start);
    for m in 2..=start << 1 {
        let c = cost(m);
        if c < best_cost {
            best = m;
            best_cost = c;
        }
    }
    best
}

// The compositions $f_i(g)$ modulo $h$ of the polynomials $f_i$ in `fs` with the same `g`, by the
// Brent–Kung algorithm: the powers $g^0, \ldots, g^{m-1}$ are computed once, the blocks of $m$
// coefficients of all the $f_i$ are stacked into one matrix, which is multiplied by the matrix of
// powers in a single product, and each $f_i$'s rows of the product are combined by evaluation at
// $g^m$. $m$ is chosen by `brent_kung_vec_block_size`. `g` has length $n$, and each result has
// length $n$.
//
// This is equivalent to `_nmod_poly_compose_mod_brent_kung_vec_preinv` from
// `nmod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, except that the $f_i$ may have any
// lengths, so the block size depends on their lengths rather than on that of $h$; that the $f_i$
// that need no matrix, as in `compose_mod_special`, are composed directly; and that $g^m$ is only
// computed if some $f_i$ has more than one block.
#[doc(hidden)]
pub fn compose_mod_brent_kung_vec<R: ComposeModRing, F: AsRef<[R::C]>>(
    ring: &R,
    fs: &[F],
    g: &[R::C],
    n: usize,
) -> Vec<Vec<R::C>> {
    let res: Vec<Option<Vec<R::C>>> = fs
        .iter()
        .map(|f| compose_mod_special(ring, f.as_ref(), g, n))
        .collect();
    let lens: Vec<usize> = fs
        .iter()
        .zip(&res)
        .filter(|(_, r)| r.is_none())
        .map(|(f, _)| f.as_ref().len())
        .collect();
    if lens.is_empty() {
        return res.into_iter().map(Option::unwrap).collect();
    }
    let m = brent_kung_vec_block_size(&lens);
    let a = powers_mod(ring, g, n, m);
    // Set rows of b to the segments of the polynomials
    let mut b = Vec::new();
    for (f, r) in fs.iter().zip(&res) {
        if r.is_none() {
            for chunk in f.as_ref().chunks(m) {
                let mut row = chunk.to_vec();
                row.resize(m, ring.zero());
                b.push(row);
            }
        }
    }
    let mut c = ring.matrix_mul(&b, &a).into_iter();
    let h = if lens.iter().any(|&len| len > m) {
        Some(ring.mul_mod(&a[m >> 1], &a[m - (m >> 1)]))
    } else {
        None
    };
    let mut lens = lens.into_iter();
    // Evaluate block composition
    res.into_iter()
        .map(|r| {
            r.unwrap_or_else(|| {
                let mut rows: Vec<Vec<R::C>> =
                    c.by_ref().take(lens.next().unwrap().div_ceil(m)).collect();
                if rows.len() == 1 {
                    rows.pop().unwrap()
                } else {
                    mod_matrix_rows_evaluate(ring, &rows, h.as_ref().unwrap(), n)
                }
            })
        })
        .collect()
}

// The least total length of the polynomials composed by `compose_mod_dispatch_vec` for which
// composing them together by the Brent–Kung algorithm is faster than composing each on its own.
// Measured with 64-bit coefficients, modulo a number and modulo a power of 2, 2026-11-06.
const BRENT_KUNG_VEC_CUTOFF: usize = 11;

// The compositions $f_i(g)$ modulo $h$ of the polynomials $f_i$ in `fs` with the same `g`: a single
// polynomial, or polynomials whose total length is less than `BRENT_KUNG_VEC_CUTOFF`, are composed
// one at a time by `compose_mod_dispatch`, and the rest together by `compose_mod_brent_kung_vec`.
#[doc(hidden)]
pub fn compose_mod_dispatch_vec<R: ComposeModRing, F: AsRef<[R::C]>>(
    ring: &R,
    fs: &[F],
    g: &[R::C],
    n: usize,
) -> Vec<Vec<R::C>> {
    if fs.len() == 1 || fs.iter().map(|f| f.as_ref().len()).sum::<usize>() < BRENT_KUNG_VEC_CUTOFF {
        fs.iter()
            .map(|f| compose_mod_dispatch(ring, f.as_ref(), g, n))
            .collect()
    } else {
        compose_mod_brent_kung_vec(ring, fs, g, n)
    }
}

// The ring of polynomials with coefficients modulo $m$, modulo a polynomial with coefficients `h`,
// with at least 3 of them, whose reversal has the inverse `inverse` as a power series.
pub(crate) struct ModRing<'a, T: PrimitiveUnsigned> {
    pub(crate) m: T,
    pub(crate) h: &'a [T],
    pub(crate) inverse: &'a [T],
}

impl<T: PrimitiveUnsigned> ComposeModRing for ModRing<'_, T> {
    type C = T;

    fn zero(&self) -> T {
        T::ZERO
    }

    fn one(&self) -> T {
        T::ONE
    }

    #[inline]
    fn add(&self, x: &T, y: &T) -> T {
        x.mod_add(*y, self.m)
    }

    #[inline]
    fn mul(&self, x: &T, y: &T) -> T {
        x.mod_mul(*y, self.m)
    }

    #[inline]
    fn mul_mod(&self, xs: &[T], ys: &[T]) -> Vec<T> {
        self.rem(&self.poly_mul(xs, ys))
    }

    fn poly_mul(&self, xs: &[T], ys: &[T]) -> Vec<T> {
        let mut out = vec![T::ZERO; xs.len() + ys.len() - 1];
        mod_mul_to_out(&mut out, xs, ys, self.m);
        out
    }

    fn rem(&self, xs: &[T]) -> Vec<T> {
        let mut r = mod_div_rem_blocks(xs, self.h, self.inverse, self.m).1;
        r.resize(self.h.len() - 1, T::ZERO);
        r
    }

    fn matrix_mul(&self, b: &[Vec<T>], a: &[Vec<T>]) -> Vec<Vec<T>> {
        let width = a[0].len();
        let b = UnsignedMatrix::from_owned_elements(
            u64::exact_from(b.len()),
            u64::exact_from(a.len()),
            b.concat(),
        );
        let a = UnsignedMatrix::from_owned_elements(
            u64::exact_from(a.len()),
            u64::exact_from(width),
            a.concat(),
        );
        b.mod_mul(a, self.m)
            .into_elements()
            .chunks(width)
            .map(<[T]>::to_vec)
            .collect()
    }

    // Measured with 64-bit coefficients modulo a 63-bit prime, 2026-11-01.
    fn brent_kung_cutoff(&self, n: usize) -> usize {
        match n {
            0..=5 => 10,
            6..=16 => 7,
            17..=64 => 5,
            _ => 4,
        }
    }
}

// Checks that `g` and `h` are reduced modulo $m$, that `h` is nonzero with an invertible leading
// coefficient, unless $m$ is 1, and that `g` is shorter than `h`, and returns whether $m$ is 1.
pub(crate) fn assert_compose_mod_polynomial_inner_and_modulus_arguments<T: PrimitiveUnsigned>(
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
) -> bool {
    assert!(
        g.mod_is_reduced(&m),
        "g must be reduced mod m, but {g} has a coefficient >= {m}"
    );
    assert!(
        h.mod_is_reduced(&m),
        "h must be reduced mod m, but {h} has a coefficient >= {m}"
    );
    if m == T::ONE {
        return true;
    }
    assert_ne!(*h, UnsignedPolynomial::ZERO, "division by zero");
    assert!(
        h.coefficients.last().unwrap().mod_inverse(m).is_some(),
        "the leading coefficient of h must be invertible modulo m"
    );
    assert!(
        g.len() < h.len(),
        "g must be reduced mod h, but {g} has a degree >= that of {h}"
    );
    false
}

// Checks that `f` is reduced, and the other arguments as
// `assert_compose_mod_polynomial_inner_and_modulus_arguments` does, and returns its result.
pub(crate) fn assert_compose_mod_polynomial_arguments<T: PrimitiveUnsigned>(
    f: &UnsignedPolynomial<T>,
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
) -> bool {
    assert!(
        f.mod_is_reduced(&m),
        "self must be reduced mod m, but {f} has a coefficient >= {m}"
    );
    assert_compose_mod_polynomial_inner_and_modulus_arguments(g, h, m)
}

// The composition $f(g)$ modulo $h$, with coefficients modulo $m$, using `data` if it is given and
// computing it otherwise.
//
// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0, or,
// with `data`, to `gr_poly_compose_mod_preinv` from `gr_poly/compose_mod_preinv.c`, except that `g`
// must be reduced modulo $h$ and that the inverse of the reversal of $h$ is used for every
// reduction.
pub(crate) fn mod_compose_mod_polynomial_helper<T: PrimitiveUnsigned>(
    f: &UnsignedPolynomial<T>,
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
    data: Option<&ModPolynomialData<T>>,
) -> UnsignedPolynomial<T> {
    if assert_compose_mod_polynomial_arguments(f, g, h, m) {
        return UnsignedPolynomial::ZERO;
    }
    let computed;
    let data = if let Some(data) = data {
        data
    } else {
        computed = precompute_helper(h, m);
        &computed
    };
    UnsignedPolynomial::from_coefficients_asc(mod_compose_mod_polynomial_vec(
        &f.coefficients,
        &g.coefficients,
        &h.coefficients,
        m,
        &data.inverse,
    ))
}

// The coefficients of $f(g)$ modulo $h$, with coefficients modulo $m$, where everything is reduced,
// `g` is shorter than `h`, `h` is nonzero with an invertible leading coefficient, and `inverse` is
// the inverse of the reversal of `h`, as a power series, truncated to `h.len()` terms. The result
// has `h.len() - 1` coefficients, or none if `h` is constant.
//
// This is not part of the public API; it is public so that `malachite-nz` can compose
// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
pub fn mod_compose_mod_polynomial_vec<T: PrimitiveUnsigned>(
    f: &[T],
    g: &[T],
    h: &[T],
    m: T,
    inverse: &[T],
) -> Vec<T> {
    let n = h.len() - 1;
    let mut gs = g.to_vec();
    gs.resize(n, T::ZERO);
    compose_mod_dispatch(&ModRing { m, h, inverse }, f, &gs, n)
}

// The powers of `g` modulo `h` that Brent–Kung composition with `g` needs, where everything is
// reduced, `g` is shorter than `h`, `h` has at least 2 coefficients and an invertible leading
// coefficient, and `inverse` is the inverse of the reversal of `h` truncated to `h.len()` terms.
//
// This is not part of the public API; it is public so that `malachite-nz` can compose
// `NaturalPolynomial`s with word-sized coefficients.
#[doc(hidden)]
pub fn mod_compose_mod_polynomial_powers_vec<T: PrimitiveUnsigned>(
    g: &[T],
    h: &[T],
    m: T,
    inverse: &[T],
) -> Vec<Vec<T>> {
    let n = h.len() - 1;
    let mut gs = g.to_vec();
    gs.resize(n, T::ZERO);
    precompute_matrix(&ModRing { m, h, inverse }, &gs, n)
}

// The coefficients of $f(g)$ modulo `h`, where `powers` holds the powers of `g` from
// `mod_compose_mod_polynomial_powers_vec`, and the rest is as for that function.
//
// This is not part of the public API; it is public so that `malachite-nz` can compose
// `NaturalPolynomial`s with word-sized coefficients.
#[doc(hidden)]
#[inline]
pub fn mod_compose_mod_polynomial_with_powers_vec<T: PrimitiveUnsigned>(
    f: &[T],
    h: &[T],
    m: T,
    powers: &[Vec<T>],
    inverse: &[T],
) -> Vec<T> {
    compose_mod_brent_kung_with_powers(&ModRing { m, h, inverse }, f, powers, h.len() - 1)
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomial<Self, Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by value, and `h` by value. The inner polynomial
    /// $g$ must already be reduced modulo $h$, with degree less than that of $h$, and the
    /// coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$ must be
    /// invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, h.clone(), 7).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, h, 7).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: Self, h: Self, m: T) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, &h, m, None)
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomial<Self, &Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by value, and `h` by reference. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$
    /// must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, &h, 7).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, &h, 7).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: Self, h: &Self, m: T) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, h, m, None)
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomial<&Self, Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by reference, and `h` by value. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$
    /// must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, h.clone(), 7).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, h, 7).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: &Self, h: Self, m: T) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, &h, m, None)
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomial<&Self, &Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by reference, and `h` by reference. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$
    /// must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, &h, 7).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, &h, 7).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: &Self, h: &Self, m: T) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, h, m, None)
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomial<UnsignedPolynomial<T>, UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by value, and `h` by value. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$
    /// must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, h.clone(), 7).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, h, 7).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: UnsignedPolynomial<T>,
        h: UnsignedPolynomial<T>,
        m: T,
    ) -> UnsignedPolynomial<T> {
        mod_compose_mod_polynomial_helper(self, &g, &h, m, None)
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomial<UnsignedPolynomial<T>, &UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by value, and `h` by reference. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$
    /// must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, &h, 7).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, &h, 7).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        m: T,
    ) -> UnsignedPolynomial<T> {
        mod_compose_mod_polynomial_helper(self, &g, h, m, None)
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomial<&UnsignedPolynomial<T>, UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by reference, and `h` by value. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$
    /// must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, h.clone(), 7)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, h, 7).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: &UnsignedPolynomial<T>,
        h: UnsignedPolynomial<T>,
        m: T,
    ) -> UnsignedPolynomial<T> {
        mod_compose_mod_polynomial_helper(self, g, &h, m, None)
    }
}

impl<T: PrimitiveUnsigned>
    ModComposeModPolynomial<&UnsignedPolynomial<T>, &UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by reference, and `h` by reference. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$
    /// must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, &h, 7).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, &h, 7).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: &UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        m: T,
    ) -> UnsignedPolynomial<T> {
        mod_compose_mod_polynomial_helper(self, g, h, m, None)
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialAssign<Self, Self, T> for UnsignedPolynomial<T> {
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by value, and `h` by value. The inner polynomial
    /// $g$ must already be reduced modulo $h$, with degree less than that of $h$, and the
    /// coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$ must be
    /// invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, h.clone(), 7);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, h, 7);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: Self, h: Self, m: T) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, &h, m, None);
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialAssign<Self, &Self, T> for UnsignedPolynomial<T> {
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by value, and `h` by reference. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$
    /// must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, &h, 7);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, &h, 7);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: Self, h: &Self, m: T) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, h, m, None);
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialAssign<&Self, Self, T> for UnsignedPolynomial<T> {
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by reference, and `h` by value. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$
    /// must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, h.clone(), 7);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, h, 7);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: &Self, h: Self, m: T) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, &h, m, None);
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialAssign<&Self, &Self, T>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by reference, and `h` by reference. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$
    /// must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, &h, 7);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, &h, 7);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: &Self, h: &Self, m: T) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, h, m, None);
    }
}
