// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::mod_mul::{mod_mul_precompute_shoup, mod_mul_shoup, mod_mul_shoup_lazy};
use crate::num::modular::traits::ModIsReduced;
use crate::polynomial::modular::traits::ModEvaluateMany;
#[cfg(doc)]
use crate::polynomial::modular::traits::ModEvaluateManyPrecomputed;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_evaluate::{
    MOD_EVALUATE_SHOUP_THRESHOLD, mod_evaluate_slice,
};
use crate::unsigned_polynomial::modular::subproduct_tree::{
    Modulus, evaluate_many_fast, mod_evaluate_many_tree_preferred,
};
use alloc::vec::Vec;

// The numbers of points evaluated together by `mod_evaluate_many_in_place`, for types at most 32
// bits wide and for wider types. Horner's rule is a chain of dependent multiplications, so running
// several points through one pass over the coefficients keeps more multiplications in flight. Tuned
// on Apple M-series: for `u64` a block of 8 is 3.4 to 4.8 times as fast as one point at a time at
// length 256, and for `u32` a block of 4 is best.
const MOD_EVALUATE_MANY_NARROW_BLOCK: usize = 4;
const MOD_EVALUATE_MANY_WIDE_BLOCK: usize = 8;

// Replaces each of the `N` points in `xs` with the polynomial's value there modulo `m`, with
// Horner's rule for all of them in one pass over the coefficients. `coefficients` must be nonempty,
// and the points and coefficients reduced.
crate_test_fn! {mod_evaluate_horner_block<T: PrimitiveUnsigned, const N: usize>(
    coefficients: &[T],
    xs: &mut [T; N],
    m: T,
) {
    let data = T::precompute_mod_mul_data(&m);
    let (&last, rest) = coefficients.split_last().unwrap();
    let points = *xs;
    let mut values = [last; N];
    for &c in rest.iter().rev() {
        for (value, &x) in values.iter_mut().zip(points.iter()) {
            value.mod_mul_precomputed_assign(x, m, &data);
            value.mod_add_assign(c, m);
        }
    }
    *xs = values;
}}

// Like `mod_evaluate_horner_block`, multiplying by each point with Shoup's method. The top bit of
// `m` must be clear.
crate_test_fn! {mod_evaluate_shoup_block<T: PrimitiveUnsigned, const N: usize>(
    coefficients: &[T],
    xs: &mut [T; N],
    m: T,
) {
    let points = *xs;
    let precomps = points.map(|x| mod_mul_precompute_shoup(x, m));
    let (&last, rest) = coefficients.split_last().unwrap();
    let mut values = [last; N];
    for &c in rest.iter().rev() {
        for ((value, &x), &x_precomp) in values.iter_mut().zip(points.iter()).zip(precomps.iter()) {
            *value = mod_mul_shoup(x, *value, x_precomp, m);
            value.mod_add_assign(c, m);
        }
    }
    *xs = values;
}}

// Like `mod_evaluate_shoup_block`, with lazy reduction as in `mod_evaluate_shoup_lazy`. `m` must be
// at most `T::MAX / 3`. The values are fully reduced at the end.
crate_test_fn! {mod_evaluate_shoup_lazy_block<T: PrimitiveUnsigned, const N: usize>(
    coefficients: &[T],
    xs: &mut [T; N],
    m: T,
) {
    let points = *xs;
    let precomps = points.map(|x| mod_mul_precompute_shoup(x, m));
    let (&last, rest) = coefficients.split_last().unwrap();
    let mut values = [last; N];
    for &c in rest.iter().rev() {
        for ((value, &x), &x_precomp) in values.iter_mut().zip(points.iter()).zip(precomps.iter()) {
            // value is x * value mod m, or that plus m, and then less than 3m - 1
            *value = mod_mul_shoup_lazy(x, *value, x_precomp, m);
            value.wrapping_add_assign(c);
        }
    }
    let two_m = m << 1;
    for value in &mut values {
        if *value >= two_m {
            *value -= two_m;
        } else if *value >= m {
            *value -= m;
        }
    }
    *xs = values;
}}

// Replaces each point in `xs` with the polynomial's value there modulo `m`. The points and the
// coefficients must be reduced; this is not checked. Blocks of points are evaluated together, with
// the method `mod_evaluate_slice` would choose for one point. Evaluates the points in `xs` in
// blocks of `N`, with the method chosen by `mod_evaluate_many_in_place`, and returns the leftover
// points, fewer than `N` of them.
fn mod_evaluate_many_blocks<'a, T: PrimitiveUnsigned, const N: usize>(
    coefficients: &[T],
    xs: &'a mut [T],
    m: T,
    shoup: bool,
    lazy: bool,
) -> &'a mut [T] {
    let (blocks, remainder) = xs.as_chunks_mut::<N>();
    for block in blocks {
        if !shoup {
            mod_evaluate_horner_block(coefficients, block, m);
        } else if lazy {
            mod_evaluate_shoup_lazy_block(coefficients, block, m);
        } else {
            mod_evaluate_shoup_block(coefficients, block, m);
        }
    }
    remainder
}

// Replaces each point in `xs` with the polynomial's value there modulo `m`. The points and the
// coefficients must be reduced; this is not checked. Blocks of points are evaluated together, with
// the method `mod_evaluate_slice` would choose for one point.
crate_test_fn! {mod_evaluate_many_in_place<T: PrimitiveUnsigned>(
    coefficients: &[T],
    xs: &mut [T],
    m: T,
) {
    let len = coefficients.len();
    match len {
        0 => xs.fill(T::ZERO),
        1 => xs.fill(coefficients[0]),
        _ => {
            // As in mod_evaluate_slice: Shoup's method needs the top bit of m clear, and is used
            // for short polynomials only when T is at most 32 bits wide
            let shoup = !m.get_highest_bit()
                && (T::WIDTH <= u32::WIDTH || len >= MOD_EVALUATE_SHOUP_THRESHOLD);
            let lazy = m <= T::MAX / T::from(3u8);
            // Wider types take blocks of the wide size first; the leftover points go through blocks
            // of the narrow size, and then one at a time
            let xs = if T::WIDTH <= u32::WIDTH {
                xs
            } else {
                mod_evaluate_many_blocks::<T, MOD_EVALUATE_MANY_WIDE_BLOCK>(
                    coefficients,
                    xs,
                    m,
                    shoup,
                    lazy,
                )
            };
            let remainder = mod_evaluate_many_blocks::<T, MOD_EVALUATE_MANY_NARROW_BLOCK>(
                coefficients,
                xs,
                m,
                shoup,
                lazy,
            );
            for x in remainder {
                *x = mod_evaluate_slice(coefficients, *x, m);
            }
        }
    }
}}

impl<T: PrimitiveUnsigned> ModEvaluateMany<T> for &UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at each of several values of its coefficient type,
    /// modulo a value of that type. The coefficients and the values must already be reduced modulo
    /// `m`.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m
    /// \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// The result is the same as calling
    /// [`mod_evaluate`](crate::polynomial::modular::traits::ModEvaluate::mod_evaluate) at each
    /// value, but the polynomial is checked once. Usually several values are evaluated together in
    /// each pass over the coefficients, with the same choice between Horner's rule and Shoup's
    /// method; Horner's rule is a chain of dependent multiplications, so interleaving independent
    /// chains keeps the processor's multipliers busy. For a long polynomial and many values, the
    /// polynomial is instead reduced modulo the products of the linear factors $x - x_j$ in a
    /// subproduct tree, from the top down, as with [`mod_evaluate_many_precomputed`](
    /// ModEvaluateManyPrecomputed::mod_evaluate_many_precomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(\min(nk, n \log n + k \log^2 k))$
    ///
    /// $M(n, k) = O(n + k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `xs.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or any value in `xs` is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateMany;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, 61, 99, and 147, mod 13
    /// assert_eq!(
    ///     (&p).mod_evaluate_many(&[0, 1, 2, 3, 4, 5], 13),
    ///     &[7, 2, 7, 9, 8, 4]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod_vec` from `nmod_poly/evaluate_nmod_vec.c`,
    /// FLINT 3.6.0, except that the values must be reduced, and that the choice between evaluating
    /// at each value and the subproduct tree depends on the number of values and on $m$ as well as
    /// on the polynomial's length.
    fn mod_evaluate_many(self, xs: &[T], m: T) -> Vec<T> {
        assert!(
            self.mod_is_reduced(&m),
            "self must be reduced mod m, but {self} has a coefficient >= {m}"
        );
        for &x in xs {
            assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
        }
        if m != T::ONE && mod_evaluate_many_tree_preferred(self.coefficients.len(), xs.len(), m) {
            return evaluate_many_fast(&self.coefficients, xs, &Modulus(m));
        }
        let mut values = xs.to_vec();
        mod_evaluate_many_in_place(&self.coefficients, &mut values, m);
        values
    }
}
