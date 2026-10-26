// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2025 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    AddMul, Mod, ModAddAssign, ModMulPrecomputed, ModMulPrecomputedAssign, ModPow, ModPowerOf2,
    ModPowerOf2AddAssign, ModPowerOf2Inverse, ModPowerOf2MulAssign, ModPowerOf2SubAssign,
    ModSubAssign, Parity, PowerOf2,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::logic::traits::TrailingZeros;
use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

// The largest dimension for which Ryser's formula can be used: the subsets of rows are indexed by a
// `u64`, and there are $2^n$ of them. Glynn's formula, with $2^{n-1}$ sign vectors, could go one
// further, but larger matrices are expanded anyway.
const SUBSET_MAX_DIMENSION: usize = 63;

/// A ring with the arithmetic that `permanent_structured` needs: the integers, for malachite-nz's
/// `IntegerMatrix`s, or the integers modulo some number. `permanent_dense` computes the permanent
/// of a matrix of dimension at most `DENSE_MAX_DIMENSION` without looking for structure.
#[doc(hidden)]
pub trait PermanentRing {
    type Element: Clone;

    const DENSE_MAX_DIMENSION: usize;

    fn zero(&self) -> Self::Element;

    fn one(&self) -> Self::Element;

    fn is_zero(&self, x: &Self::Element) -> bool;

    fn add_assign(&self, x: &mut Self::Element, y: &Self::Element);

    fn mul_assign(&self, x: &mut Self::Element, y: &Self::Element);

    // x * y + z * w.
    fn mul_add_mul(
        &self,
        x: &Self::Element,
        y: &Self::Element,
        z: &Self::Element,
        w: &Self::Element,
    ) -> Self::Element {
        let mut p = x.clone();
        self.mul_assign(&mut p, y);
        let mut q = z.clone();
        self.mul_assign(&mut q, w);
        self.add_assign(&mut p, &q);
        p
    }

    fn permanent_dense(&self, xs: &[Self::Element], n: usize) -> Self::Element;
}

/// A ring of integers modulo some number, in which every element is reduced, with the further
/// arithmetic that Ryser's and Glynn's formulas need.
#[doc(hidden)]
pub trait PermanentModRing: PermanentRing {
    fn sub_assign(&self, x: &mut Self::Element, y: &Self::Element);

    // The inverse of 2^k, or `None` if 2 is not invertible, in which case Glynn's formula, which
    // divides by a power of 2, cannot be used.
    fn inverse_power_of_2(&self, k: u64) -> Option<Self::Element>;
}

/// The arithmetic on the elements of both `UnsignedMatrix`s and malachite-nz's `NaturalMatrix`s
/// that the rings of integers modulo $2^k$ and modulo $m$ are built on, and that splitting a
/// modulus into a power of 2 and an odd part needs. Each method is the operation of the same name
/// in `num::arithmetic::traits`.
#[doc(hidden)]
pub trait PermanentElement: Clone + Sized {
    type ModMulData;

    fn zero() -> Self;

    fn one() -> Self;

    fn is_zero(&self) -> bool;

    fn is_one(&self) -> bool;

    // Whether every number modulo 2^pow can be represented.
    fn power_of_2_fits(pow: u64) -> bool;

    // The number of trailing zeros of a positive number.
    fn trailing_zeros(&self) -> u64;

    fn shr(&self, bits: u64) -> Self;

    fn add_mul(self, y: Self, z: Self) -> Self;

    fn mod_power_of_2(&self, pow: u64) -> Self;

    fn mod_power_of_2_add_assign(&mut self, other: &Self, pow: u64);

    fn mod_power_of_2_sub_assign(&mut self, other: &Self, pow: u64);

    fn mod_power_of_2_mul_assign(&mut self, other: &Self, pow: u64);

    fn mod_power_of_2_inverse(&self, pow: u64) -> Option<Self>;

    fn mod_op(&self, m: &Self) -> Self;

    fn precompute_mod_mul_data(m: &Self) -> Self::ModMulData;

    fn mod_add_assign(&mut self, other: &Self, m: &Self);

    fn mod_sub_assign(&mut self, other: &Self, m: &Self);

    fn mod_mul_precomputed_assign(&mut self, other: &Self, m: &Self, data: &Self::ModMulData);

    fn mod_pow(&self, exp: u64, m: &Self) -> Self;
}

impl<T: PrimitiveUnsigned> PermanentElement for T {
    type ModMulData = <T as ModMulPrecomputed<T, T>>::Data;

    #[inline]
    fn zero() -> T {
        T::ZERO
    }

    #[inline]
    fn one() -> T {
        T::ONE
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == T::ZERO
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == T::ONE
    }

    #[inline]
    fn power_of_2_fits(pow: u64) -> bool {
        pow <= T::WIDTH
    }

    #[inline]
    fn trailing_zeros(&self) -> u64 {
        TrailingZeros::trailing_zeros(*self)
    }

    #[inline]
    fn shr(&self, bits: u64) -> T {
        *self >> bits
    }

    #[inline]
    fn add_mul(self, y: T, z: T) -> T {
        AddMul::add_mul(self, y, z)
    }

    #[inline]
    fn mod_power_of_2(&self, pow: u64) -> T {
        ModPowerOf2::mod_power_of_2(*self, pow)
    }

    #[inline]
    fn mod_power_of_2_add_assign(&mut self, other: &T, pow: u64) {
        ModPowerOf2AddAssign::mod_power_of_2_add_assign(self, *other, pow);
    }

    #[inline]
    fn mod_power_of_2_sub_assign(&mut self, other: &T, pow: u64) {
        ModPowerOf2SubAssign::mod_power_of_2_sub_assign(self, *other, pow);
    }

    #[inline]
    fn mod_power_of_2_mul_assign(&mut self, other: &T, pow: u64) {
        ModPowerOf2MulAssign::mod_power_of_2_mul_assign(self, *other, pow);
    }

    #[inline]
    fn mod_power_of_2_inverse(&self, pow: u64) -> Option<T> {
        ModPowerOf2Inverse::mod_power_of_2_inverse(*self, pow)
    }

    #[inline]
    fn mod_op(&self, m: &T) -> T {
        Mod::mod_op(*self, *m)
    }

    #[inline]
    fn precompute_mod_mul_data(m: &T) -> Self::ModMulData {
        <T as ModMulPrecomputed<T, T>>::precompute_mod_mul_data(m)
    }

    #[inline]
    fn mod_add_assign(&mut self, other: &T, m: &T) {
        ModAddAssign::mod_add_assign(self, *other, *m);
    }

    #[inline]
    fn mod_sub_assign(&mut self, other: &T, m: &T) {
        ModSubAssign::mod_sub_assign(self, *other, *m);
    }

    #[inline]
    fn mod_mul_precomputed_assign(&mut self, other: &T, m: &T, data: &Self::ModMulData) {
        ModMulPrecomputedAssign::mod_mul_precomputed_assign(self, *other, *m, data);
    }

    #[inline]
    fn mod_pow(&self, exp: u64, m: &T) -> T {
        ModPow::mod_pow(*self, exp, *m)
    }
}

/// The integers modulo $2^k$.
#[doc(hidden)]
pub struct ModPowerOf2Ring<E> {
    pub pow: u64,
    phantom: PhantomData<E>,
}

impl<E> ModPowerOf2Ring<E> {
    pub const fn new(pow: u64) -> Self {
        Self {
            pow,
            phantom: PhantomData,
        }
    }
}

impl<E: PermanentElement> PermanentRing for ModPowerOf2Ring<E> {
    type Element = E;

    const DENSE_MAX_DIMENSION: usize = SUBSET_MAX_DIMENSION;

    #[inline]
    fn zero(&self) -> E {
        E::zero()
    }

    #[inline]
    fn one(&self) -> E {
        E::one().mod_power_of_2(self.pow)
    }

    #[inline]
    fn is_zero(&self, x: &E) -> bool {
        x.is_zero()
    }

    #[inline]
    fn add_assign(&self, x: &mut E, y: &E) {
        x.mod_power_of_2_add_assign(y, self.pow);
    }

    #[inline]
    fn mul_assign(&self, x: &mut E, y: &E) {
        x.mod_power_of_2_mul_assign(y, self.pow);
    }

    // Glynn's formula, which needs half as many terms as Ryser's, is used modulo a larger power of
    // 2 when it fits; see `permanent_glynn_mod_power_of_2`.
    fn permanent_dense(&self, xs: &[E], n: usize) -> E {
        if n > 4 && E::power_of_2_fits(self.pow + u64::wrapping_from(n - 1)) {
            permanent_glynn_mod_power_of_2(xs, n, self.pow)
        } else {
            permanent_dense(self, xs, n)
        }
    }
}

impl<E: PermanentElement> PermanentModRing for ModPowerOf2Ring<E> {
    #[inline]
    fn sub_assign(&self, x: &mut E, y: &E) {
        x.mod_power_of_2_sub_assign(y, self.pow);
    }

    #[inline]
    fn inverse_power_of_2(&self, _k: u64) -> Option<E> {
        None
    }
}

/// The integers modulo a positive $m$, with the data for multiplying modulo it precomputed.
#[doc(hidden)]
pub struct ModRing<E: PermanentElement> {
    pub m: E,
    data: E::ModMulData,
}

impl<E: PermanentElement> ModRing<E> {
    pub fn new(m: E) -> Self {
        let data = E::precompute_mod_mul_data(&m);
        Self { m, data }
    }
}

impl<E: PermanentElement> PermanentRing for ModRing<E> {
    type Element = E;

    const DENSE_MAX_DIMENSION: usize = SUBSET_MAX_DIMENSION;

    #[inline]
    fn zero(&self) -> E {
        E::zero()
    }

    #[inline]
    fn one(&self) -> E {
        if self.m.is_one() { E::zero() } else { E::one() }
    }

    #[inline]
    fn is_zero(&self, x: &E) -> bool {
        x.is_zero()
    }

    #[inline]
    fn add_assign(&self, x: &mut E, y: &E) {
        x.mod_add_assign(y, &self.m);
    }

    #[inline]
    fn mul_assign(&self, x: &mut E, y: &E) {
        x.mod_mul_precomputed_assign(y, &self.m, &self.data);
    }

    #[inline]
    fn permanent_dense(&self, xs: &[E], n: usize) -> E {
        permanent_dense(self, xs, n)
    }
}

impl<E: PermanentElement> PermanentModRing for ModRing<E> {
    #[inline]
    fn sub_assign(&self, x: &mut E, y: &E) {
        x.mod_sub_assign(y, &self.m);
    }

    fn inverse_power_of_2(&self, k: u64) -> Option<E> {
        if self.m.is_one() {
            Some(E::zero())
        } else if self.m.trailing_zeros() != 0 {
            None
        } else {
            // The inverse of 2 modulo an odd m is (m + 1) / 2.
            Some(
                self.m
                    .shr(1)
                    .add_mul(E::one(), E::one())
                    .mod_pow(k, &self.m),
            )
        }
    }
}

/// The permanent modulo $m$ of the $n \times n$ matrix with elements `xs`, reduced modulo $m$.
/// Writing $m = 2^k q$ with $q$ odd, the permanent is found by `mod_power_of_2_permanent` modulo
/// $2^k$, where Ryser's formula is cheap, and by `mod_odd_permanent` modulo $q$, where Glynn's
/// formula needs half as many terms, and the two are combined by the Chinese remainder theorem. An
/// odd $m$ is handled directly, so that `mod_odd_permanent` is only called with a smaller modulus.
#[doc(hidden)]
pub fn mod_permanent_by_parts<E: PermanentElement>(
    xs: Vec<E>,
    n: usize,
    m: &E,
    mod_power_of_2_permanent: impl Fn(Vec<E>, u64) -> E,
    mod_odd_permanent: impl Fn(Vec<E>, &E) -> E,
) -> E {
    let pow = m.trailing_zeros();
    if pow == 0 {
        return permanent_structured(&ModRing::new(m.clone()), xs, n);
    }
    let q = m.shr(pow);
    if q.is_one() {
        return mod_power_of_2_permanent(xs, pow);
    }
    let xs_q = xs.iter().map(|x| x.mod_op(&q)).collect();
    let xs_2 = xs.iter().map(|x| x.mod_power_of_2(pow)).collect();
    let r_q = mod_odd_permanent(xs_q, &q);
    let mut t = mod_power_of_2_permanent(xs_2, pow);
    // x = r_q + q t, where t = (r_2 - r_q) / q mod 2^k.
    t.mod_power_of_2_sub_assign(&r_q.mod_power_of_2(pow), pow);
    t.mod_power_of_2_mul_assign(
        &q.mod_power_of_2(pow).mod_power_of_2_inverse(pow).unwrap(),
        pow,
    );
    r_q.add_mul(q, t)
}

// The permanent of the matrix formed by the rows `rows` and the columns `col..` of the n by n
// matrix with elements `xs`, by cofactor expansion along the first column. `scratch` holds the row
// lists of the deeper levels.
//
// This is equivalent to `_gr_mat_permanent_cofactor_recursive` from `gr_mat/permanent.c`, FLINT
// 3.6.0.
fn permanent_cofactor_recursive<R: PermanentRing>(
    ring: &R,
    xs: &[R::Element],
    n: usize,
    rows: &[usize],
    col: usize,
    scratch: &mut [usize],
) -> R::Element {
    let m = rows.len();
    if m == 1 {
        return xs[rows[0] * n + col].clone();
    }
    let (r, rest) = scratch.split_at_mut(m - 1);
    let mut sum = ring.zero();
    for i in 0..m {
        r[..i].copy_from_slice(&rows[..i]);
        r[i..].copy_from_slice(&rows[i + 1..]);
        let mut t = permanent_cofactor_recursive(ring, xs, n, r, col + 1, rest);
        ring.mul_assign(&mut t, &xs[rows[i] * n + col]);
        ring.add_assign(&mut sum, &t);
    }
    sum
}

// The permanent of the n by n matrix with elements `xs` by cofactor expansion along the first
// column, in time $O(n \cdot n!)$.
//
// This is equivalent to `gr_mat_permanent_cofactor` from `gr_mat/permanent.c`, FLINT 3.6.0, except
// that it has no limit on the dimension.
crate_test_fn! {
permanent_cofactor<R: PermanentRing>(ring: &R, xs: &[R::Element], n: usize) -> R::Element {
    if n == 0 {
        return ring.one();
    }
    let rows: Vec<usize> = (0..n).collect();
    permanent_cofactor_recursive(ring, xs, n, &rows, 0, &mut vec![0; n * n])
}}

// The product of the elements of `s`, stopping early once it is 0.
fn product<R: PermanentRing>(ring: &R, s: &[R::Element]) -> R::Element {
    let mut p = ring.one();
    for x in s {
        ring.mul_assign(&mut p, x);
        if ring.is_zero(&p) {
            break;
        }
    }
    p
}

// The permanent of the n by n matrix with elements `xs`, where $1 \leq n \leq 63$, by Ryser's
// formula,
// $$
// \operatorname{perm}(A) = (-1)^n \sum_{S} (-1)^{|S|} \prod_j \sum_{i \in S} a_{ij},
// $$
// where $S$ runs over the nonempty sets of rows, visited in Gray-code order so that each step adds
// or removes one row and the column sums are updated by adding or subtracting it. It divides by
// nothing, so it works in any ring.
//
// This is equivalent to `gr_mat_permanent_ryser` from `gr_mat/permanent.c`, FLINT 3.6.0.
crate_test_fn! {
permanent_ryser<R: PermanentModRing>(ring: &R, xs: &[R::Element], n: usize) -> R::Element {
    assert!(
        (1..=SUBSET_MAX_DIMENSION).contains(&n),
        "the dimension must be between 1 and 63"
    );
    let mut s = vec![ring.zero(); n];
    let mut res = ring.zero();
    let mut j_prev = 0u64;
    for i in 1..u64::power_of_2(u64::wrapping_from(n)) {
        let j = i ^ (i >> 1);
        let d = usize::exact_from((j ^ j_prev).trailing_zeros());
        let row = &xs[d * n..(d + 1) * n];
        if j > j_prev {
            for (x, y) in s.iter_mut().zip(row) {
                ring.add_assign(x, y);
            }
        } else {
            for (x, y) in s.iter_mut().zip(row) {
                ring.sub_assign(x, y);
            }
        }
        j_prev = j;
        let p = product(ring, &s);
        if (u64::wrapping_from(n) + i).odd() {
            ring.sub_assign(&mut res, &p);
        } else {
            ring.add_assign(&mut res, &p);
        }
    }
    res
}}

// The permanent of the n by n matrix with elements `xs`, where $1 \leq n \leq 63$, by Glynn's
// formula,
// $$
// \operatorname{perm}(A) = \frac{1}{2^{n-1}} \sum_\delta \left ( \prod_k \delta_k \right )
//     \prod_i \sum_j \delta_j a_{ij},
// $$
// where $\delta$ runs over the sign vectors with $\delta_0 = 1$, visited in Gray-code order so that
// each step changes one sign and the row sums are updated by adding or subtracting twice a column.
// `inverse` is the inverse of $2^{n-1}$, which must exist.
//
// This is equivalent to `gr_mat_permanent_glynn` from `gr_mat/permanent.c`, FLINT 3.6.0.
crate_test_fn! {
permanent_glynn<R: PermanentModRing>(
    ring: &R,
    xs: &[R::Element],
    n: usize,
    inverse: &R::Element,
) -> R::Element {
    assert!(
        (1..=SUBSET_MAX_DIMENSION).contains(&n),
        "the dimension must be between 1 and 63"
    );
    // The columns of the matrix, doubled.
    let mut a2 = vec![ring.zero(); n * n];
    for i in 0..n {
        for j in 0..n {
            let x = &xs[i * n + j];
            let y = &mut a2[j * n + i];
            *y = x.clone();
            ring.add_assign(y, x);
        }
    }
    let mut s: Vec<R::Element> = xs
        .chunks_exact(n)
        .map(|row| {
            let mut sum = ring.zero();
            for x in row {
                ring.add_assign(&mut sum, x);
            }
            sum
        })
        .collect();
    let mut res = product(ring, &s);
    let mut j_prev = 0u64;
    for i in 1..u64::power_of_2(u64::wrapping_from(n - 1)) {
        let j = i ^ (i >> 1);
        let d = usize::exact_from((j ^ j_prev).trailing_zeros());
        let column = &a2[d * n..(d + 1) * n];
        if j > j_prev {
            for (x, y) in s.iter_mut().zip(column) {
                ring.sub_assign(x, y);
            }
        } else {
            for (x, y) in s.iter_mut().zip(column) {
                ring.add_assign(x, y);
            }
        }
        j_prev = j;
        let p = product(ring, &s);
        if i.odd() {
            ring.sub_assign(&mut res, &p);
        } else {
            ring.add_assign(&mut res, &p);
        }
    }
    ring.mul_assign(&mut res, inverse);
    res
}}

// The permanent modulo $2^k$ of the n by n matrix with elements `xs`, reduced modulo $2^k$, where
// $1 \leq n \leq 63$, by Glynn's formula, although 2 is not invertible: the formula's sum, which is
// $2^{n-1}$ times the permanent, is computed modulo $2^{k+n-1}$, and its low $n - 1$ bits, which
// are 0, are shifted out. Every number modulo $2^{k+n-1}$ must be representable.
crate_test_fn! {
permanent_glynn_mod_power_of_2<E: PermanentElement>(xs: &[E], n: usize, pow: u64) -> E {
    let shift = u64::wrapping_from(n - 1);
    let wide_pow = pow + shift;
    assert!(E::power_of_2_fits(wide_pow), "2^(k + n - 1) must fit in the element type");
    let ring = ModPowerOf2Ring::new(wide_pow);
    permanent_glynn(&ring, xs, n, &ring.one()).shr(shift)
}}

// The permanent of the n by n matrix with elements `xs`, without looking for structure: by cofactor
// expansion up to dimension 4, and otherwise by Glynn's formula if 2 is invertible and by Ryser's
// formula if not.
//
// This is equivalent to `gr_mat_permanent_generic` from `gr_mat/permanent.c`, FLINT 3.6.0, without
// the threaded variant.
crate_test_fn! {
permanent_dense<R: PermanentModRing>(ring: &R, xs: &[R::Element], n: usize) -> R::Element {
    if n <= 4 {
        permanent_cofactor(ring, xs, n)
    } else if let Some(inverse) = ring.inverse_power_of_2(u64::wrapping_from(n - 1)) {
        permanent_glynn(ring, xs, n, &inverse)
    } else {
        permanent_ryser(ring, xs, n)
    }
}}

// The elements of the submatrix of the matrix `xs`, whose rows have length `stride`, formed by the
// rows `rows` and the columns `cols`.
fn submatrix<E: Clone>(xs: &[E], stride: usize, rows: &[usize], cols: &[usize]) -> Vec<E> {
    let mut ys = Vec::with_capacity(rows.len() * cols.len());
    for &i in rows {
        ys.extend(cols.iter().map(|&j| xs[i * stride + j].clone()));
    }
    ys
}

// The elements of the n by n matrix `xs` without row `i` and column `j`.
fn minor<E: Clone>(xs: &[E], n: usize, i: usize, j: usize) -> Vec<E> {
    let rows: Vec<usize> = (0..n).filter(|&k| k != i).collect();
    let cols: Vec<usize> = (0..n).filter(|&k| k != j).collect();
    submatrix(xs, n, &rows, &cols)
}

// The connected components of the bipartite graph whose vertices are the rows and columns of the n
// by n matrix `xs` and whose edges are its nonzero elements, as pairs of row and column lists.
fn components<R: PermanentRing>(
    ring: &R,
    xs: &[R::Element],
    n: usize,
) -> Vec<(Vec<usize>, Vec<usize>)> {
    // Vertices 0..n are rows and n..2n are columns.
    let mut component = vec![usize::MAX; n << 1];
    let mut result = Vec::new();
    for start in 0..n << 1 {
        if component[start] != usize::MAX {
            continue;
        }
        let c = result.len();
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        let mut stack = vec![start];
        component[start] = c;
        while let Some(v) = stack.pop() {
            if v < n {
                rows.push(v);
                for j in 0..n {
                    if !ring.is_zero(&xs[v * n + j]) && component[n + j] == usize::MAX {
                        component[n + j] = c;
                        stack.push(n + j);
                    }
                }
            } else {
                let j = v - n;
                cols.push(j);
                for (i, ci) in component[..n].iter_mut().enumerate() {
                    if !ring.is_zero(&xs[i * n + j]) && *ci == usize::MAX {
                        *ci = c;
                        stack.push(i);
                    }
                }
            }
        }
        rows.sort_unstable();
        cols.sort_unstable();
        result.push((rows, cols));
    }
    result
}

// The sum, over the nonzero elements `x` of row `i` of the n by n matrix `xs`, of `x` times the
// permanent of its minor, or the same along column `i` if `along_column` is true.
fn expand<R: PermanentRing>(
    ring: &R,
    xs: &[R::Element],
    n: usize,
    i: usize,
    along_column: bool,
) -> R::Element {
    let mut sum = ring.zero();
    for k in 0..n {
        let (row, col) = if along_column { (k, i) } else { (i, k) };
        let x = &xs[row * n + col];
        if !ring.is_zero(x) {
            let mut p = permanent_structured(ring, minor(xs, n, row, col), n - 1);
            ring.mul_assign(&mut p, x);
            ring.add_assign(&mut sum, &p);
        }
    }
    sum
}

// A square matrix being reduced in place: the elements stay where they are, with rows of length
// `stride`, and only the rows `rows` and the columns `cols` remain. `counts` holds the numbers of
// nonzero elements of the remaining part of each row, indexed by the original row, followed by
// those of each column, indexed by `stride` plus the original column. They are updated as rows and
// columns are removed, so that each reduction takes time linear in the dimension.
struct Reduction<'a, R: PermanentRing> {
    ring: &'a R,
    xs: Vec<R::Element>,
    stride: usize,
    rows: Vec<usize>,
    cols: Vec<usize>,
    counts: Vec<usize>,
}

impl<'a, R: PermanentRing> Reduction<'a, R> {
    fn new(ring: &'a R, xs: Vec<R::Element>, n: usize) -> Self {
        let mut counts = vec![0; n << 1];
        for (i, row) in xs.chunks_exact(n).enumerate() {
            for (j, x) in row.iter().enumerate() {
                if !ring.is_zero(x) {
                    counts[i] += 1;
                    counts[n + j] += 1;
                }
            }
        }
        Self {
            ring,
            xs,
            stride: n,
            rows: (0..n).collect(),
            cols: (0..n).collect(),
            counts,
        }
    }

    fn is_nonzero(&self, i: usize, j: usize) -> bool {
        !self.ring.is_zero(&self.xs[i * self.stride + j])
    }

    // The two remaining columns with nonzero elements in row `i`, which has exactly two, or the
    // first one if it has exactly one.
    fn nonzero_cols(&self, i: usize) -> (usize, Option<usize>) {
        let mut nonzero = self.cols.iter().copied().filter(|&j| self.is_nonzero(i, j));
        (nonzero.next().unwrap(), nonzero.next())
    }

    // The same for column `j`.
    fn nonzero_rows(&self, j: usize) -> (usize, Option<usize>) {
        let mut nonzero = self.rows.iter().copied().filter(|&i| self.is_nonzero(i, j));
        (nonzero.next().unwrap(), nonzero.next())
    }

    // Removes row `i`, keeping the column counts up to date.
    fn remove_row(&mut self, i: usize) {
        for k in 0..self.cols.len() {
            let j = self.cols[k];
            if self.is_nonzero(i, j) {
                self.counts[self.stride + j] -= 1;
            }
        }
        let k = self.rows.iter().position(|&r| r == i).unwrap();
        self.rows.swap_remove(k);
    }

    // Removes column `j`, keeping the row counts up to date.
    fn remove_col(&mut self, j: usize) {
        for k in 0..self.rows.len() {
            let i = self.rows[k];
            if self.is_nonzero(i, j) {
                self.counts[i] -= 1;
            }
        }
        let k = self.cols.iter().position(|&c| c == j).unwrap();
        self.cols.swap_remove(k);
    }

    // Replaces the element in row `i` and column `j` by `y` times itself plus `x` times the element
    // in row `i_2` and column `j_2`, keeping the counts of row `i` and column `j` up to date.
    fn combine(
        &mut self,
        i: usize,
        j: usize,
        i_2: usize,
        j_2: usize,
        x: &R::Element,
        y: &R::Element,
    ) {
        let ring = self.ring;
        let was_nonzero = self.is_nonzero(i, j);
        let z = ring.mul_add_mul(
            y,
            &self.xs[i * self.stride + j],
            x,
            &self.xs[i_2 * self.stride + j_2],
        );
        let is_nonzero = !ring.is_zero(&z);
        self.xs[i * self.stride + j] = z;
        if was_nonzero && !is_nonzero {
            self.counts[i] -= 1;
            self.counts[self.stride + j] -= 1;
        } else if !was_nonzero && is_nonzero {
            self.counts[i] += 1;
            self.counts[self.stride + j] += 1;
        }
    }

    // Row `i` has exactly two nonzero elements, $x$ in column `j_1` and $y$ in column `j_2`. Row
    // `i` and column `j_2` are removed, and column `j_1` is replaced by $y$ times itself plus $x$
    // times column `j_2`. Expanding along row `i` gives $x \operatorname{perm}(A_{i, j_1}) + y
    // \operatorname{perm}(A_{i, j_2})$, and the two minors differ only in the column they keep, so
    // by linearity in that column, and since the permanent does not depend on the order of the
    // columns, the sum is the permanent of the result.
    fn contract_row(&mut self, i: usize, j_1: usize, j_2: usize) {
        let x = self.xs[i * self.stride + j_1].clone();
        let y = self.xs[i * self.stride + j_2].clone();
        self.remove_row(i);
        for k in 0..self.rows.len() {
            let r = self.rows[k];
            self.combine(r, j_1, r, j_2, &x, &y);
        }
        self.remove_col(j_2);
    }

    // The same as `contract_row`, with rows and columns exchanged.
    fn contract_col(&mut self, j: usize, i_1: usize, i_2: usize) {
        let x = self.xs[i_1 * self.stride + j].clone();
        let y = self.xs[i_2 * self.stride + j].clone();
        self.remove_col(j);
        for k in 0..self.cols.len() {
            let c = self.cols[k];
            self.combine(i_1, c, i_2, c, &x, &y);
        }
        self.remove_row(i_2);
    }

    // The elements of the remaining matrix.
    fn remaining(&self) -> Vec<R::Element> {
        submatrix(&self.xs, self.stride, &self.rows, &self.cols)
    }
}

/// The permanent in `ring` of the $n \times n$ matrix with elements `xs`, using its structure
/// before falling back to `ring.permanent_dense`. A zero row or column gives 0. A row or column
/// with a single nonzero element $x$ gives $x$ times the permanent of the minor. A row or column
/// with two nonzero elements is contracted, lowering the dimension by 1 without branching. These
/// reductions update the counts of nonzero elements as they go, each in time linear in the
/// dimension. If the nonzero elements then split into independent blocks, the permanent is the
/// product of the blocks' permanents, or 0 if some block is not square. Otherwise, if the dimension
/// is greater than `R::DENSE_MAX_DIMENSION`, the permanent is expanded along the sparsest row or
/// column.
#[doc(hidden)]
pub fn permanent_structured<R: PermanentRing>(
    ring: &R,
    xs: Vec<R::Element>,
    n: usize,
) -> R::Element {
    if n <= 2 {
        return ring.permanent_dense(&xs, n);
    }
    let mut a = Reduction::new(ring, xs, n);
    let mut factor = ring.one();
    loop {
        if ring.is_zero(&factor) {
            return factor;
        }
        let n = a.rows.len();
        if n <= 2 {
            ring.mul_assign(&mut factor, &ring.permanent_dense(&a.remaining(), n));
            return factor;
        }
        if a.rows.iter().any(|&i| a.counts[i] == 0)
            || a.cols.iter().any(|&j| a.counts[a.stride + j] == 0)
        {
            return ring.zero();
        }
        if let Some(&i) = a.rows.iter().find(|&&i| a.counts[i] == 1) {
            let j = a.nonzero_cols(i).0;
            ring.mul_assign(&mut factor, &a.xs[i * a.stride + j]);
            a.remove_row(i);
            a.remove_col(j);
            continue;
        }
        if let Some(&j) = a.cols.iter().find(|&&j| a.counts[a.stride + j] == 1) {
            let i = a.nonzero_rows(j).0;
            ring.mul_assign(&mut factor, &a.xs[i * a.stride + j]);
            a.remove_row(i);
            a.remove_col(j);
            continue;
        }
        if let Some(&i) = a.rows.iter().find(|&&i| a.counts[i] == 2) {
            let (j_1, j_2) = a.nonzero_cols(i);
            a.contract_row(i, j_1, j_2.unwrap());
            continue;
        }
        if let Some(&j) = a.cols.iter().find(|&&j| a.counts[a.stride + j] == 2) {
            let (i_1, i_2) = a.nonzero_rows(j);
            a.contract_col(j, i_1, i_2.unwrap());
            continue;
        }
        let xs = a.remaining();
        let blocks = components(ring, &xs, n);
        if blocks.len() > 1 {
            if blocks.iter().any(|(rows, cols)| rows.len() != cols.len()) {
                return ring.zero();
            }
            for (rows, cols) in &blocks {
                let block = submatrix(&xs, n, rows, cols);
                ring.mul_assign(&mut factor, &permanent_structured(ring, block, rows.len()));
                if ring.is_zero(&factor) {
                    return factor;
                }
            }
            return factor;
        }
        let p = if n > R::DENSE_MAX_DIMENSION {
            let (min_row, min_row_count) = (0..n)
                .map(|k| (k, a.counts[a.rows[k]]))
                .min_by_key(|&(_, c)| c)
                .unwrap();
            let (min_col, min_col_count) = (0..n)
                .map(|k| (k, a.counts[a.stride + a.cols[k]]))
                .min_by_key(|&(_, c)| c)
                .unwrap();
            if min_row_count <= min_col_count {
                expand(ring, &xs, n, min_row, false)
            } else {
                expand(ring, &xs, n, min_col, true)
            }
        } else {
            ring.permanent_dense(&xs, n)
        };
        ring.mul_assign(&mut factor, &p);
        return factor;
    }
}
