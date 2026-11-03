// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::modular::mod_mul::{mod_mul_precompute_shoup, mod_mul_shoup};
use crate::num::modular::traits::ModIsReduced;
use crate::polynomial::modular::traits::ModEvaluateGeometric;
#[cfg(doc)]
use crate::polynomial::modular::traits::ModEvaluateMany;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_evaluate_many::mod_evaluate_many_in_place;
use crate::unsigned_polynomial::modular::mod_mul_middle::mod_mul_middle_karatsuba;
use crate::unsigned_polynomial::series::mod_mul_truncated::mod_mul_truncated_to_out;
use crate::unsigned_vector::modular::mod_dot::ModData;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::max;

// Whether geometric evaluation of a polynomial of length `n` at `k` points modulo `m` uses
// Bluestein's trick rather than evaluating at each power of q: when the polynomial and the number
// of points are both long enough. Evaluating at each power is fast with Shoup's multiplication, and
// fastest with its lazy form, which needs the top two bits of `m` clear; when the top bit is set,
// Shoup's multiplication is unavailable and Bluestein's trick wins much sooner. The middle product
// accumulates in three words once `m` has more than about W - 8 bits, where W is `T::WIDTH`, which
// slows it. Measured on an Apple M-series machine, 2026-10, for `u64` with moduli of 20, 30, 40,
// 50, 62, 63, and 64 bits; the boundaries for other types are scaled by their width without being
// measured.
fn mod_evaluate_geometric_fast_preferred<T: PrimitiveUnsigned>(n: usize, k: usize, m: T) -> bool {
    let bits = m.significant_bits();
    let width = T::WIDTH;
    let (min_n, min_k) = if bits == width {
        (32, 32)
    } else if bits == width - 1 {
        (256, 256)
    } else if bits + 8 > width {
        (512, 1024)
    } else if bits << 4 > width * 5 {
        (256, 256)
    } else {
        (128, 128)
    };
    n >= min_n && k >= min_k
}

// Evaluates the polynomial with the given coefficients, reduced modulo `m`, at $1, q, q^2, \ldots,
// q^{k-1}$ modulo `m`, generating the powers of q and then evaluating at them all, several points
// at a time.
//
// This is equivalent to `_nmod_poly_evaluate_geometric_nmod_vec_iter` from
// `nmod_poly/evaluate_geometric_nmod_vec.c`, FLINT 3.6.0, with the ratio q in place of FLINT's
// $r^2$.
crate_test_fn! {mod_evaluate_geometric_iter<T: PrimitiveUnsigned>(
    coefficients: &[T],
    q: T,
    k: usize,
    m: T,
) -> Vec<T> {
    let mut values = Vec::with_capacity(k);
    if k != 0 {
        let mut power = T::ONE % m;
        values.push(power);
        if m.get_highest_bit() {
            let data = T::precompute_mod_mul_data(&m);
            for _ in 1..k {
                power.mod_mul_precomputed_assign(q, m, &data);
                values.push(power);
            }
        } else {
            let q_precomp = mod_mul_precompute_shoup(q, m);
            for _ in 1..k {
                power = mod_mul_shoup(q, power, q_precomp, m);
                values.push(power);
            }
        }
    }
    mod_evaluate_many_in_place(coefficients, &mut values, m);
    values
}}

// Evaluates the polynomial with the given coefficients, reduced modulo `m`, at $1, q, q^2, \ldots,
// q^{k-1}$ modulo `m` with Bluestein's trick, where `q_inverse` is the inverse of q modulo `m`.
//
// With $\binom{t}{2} = t(t-1)/2$, every product $ij$ is $\binom{i+j}{2} - \binom{i}{2} -
// \binom{j}{2}$, so
// $$
// \sum_i c_i q^{ij} = q^{-\binom{j}{2}} \sum_i \left ( c_i q^{-\binom{i}{2}} \right )
// q^{\binom{i+j}{2}}.
// $$
// The sum is coefficient $n - 1 + j$ of the product of the reverse of $(c_i q^{-\binom{i}{2}})_i$
// and $(q^{\binom{t}{2}})_t$, so all $k$ values are one middle product, the $k$ coefficients of
// that product from coefficient $n - 1$ on. Leading zero coefficients of the polynomial are
// skipped, shortening $n$.
//
// This is equivalent to `_nmod_poly_evaluate_geometric_nmod_vec_fast` from
// `nmod_poly/evaluate_geometric_nmod_vec.c`, FLINT 3.6.0, with exponents $\binom{t}{2}$ in place of
// FLINT's $t^2/2$, so that no square root of q is needed.
crate_test_fn! {mod_evaluate_geometric_fast<T: PrimitiveUnsigned>(
    coefficients: &[T],
    q: T,
    q_inverse: T,
    k: usize,
    m: T,
) -> Vec<T> {
    if k == 0 {
        return Vec::new();
    }
    let Some(start) = coefficients.iter().position(|&c| c != T::ZERO) else {
        return vec![T::ZERO; k];
    };
    let n = coefficients.len();
    let a_len = n - start;
    let data = T::precompute_mod_mul_data(&m);
    // binomial_powers[t] = q^C(t, 2), for t < n + k - 1.
    let b_len = n + k - 1;
    let mut binomial_powers = Vec::with_capacity(b_len);
    let mut power = T::ONE % m;
    let mut step = T::ONE % m;
    for _ in 0..b_len {
        binomial_powers.push(power);
        power.mod_mul_precomputed_assign(step, m, &data);
        step.mod_mul_precomputed_assign(q, m, &data);
    }
    // inverse_powers[t] = q^-C(t, 2), for t < max(n, k).
    let w_len = max(n, k);
    let mut inverse_powers = Vec::with_capacity(w_len);
    let mut power = T::ONE % m;
    let mut step = T::ONE % m;
    for _ in 0..w_len {
        inverse_powers.push(power);
        power.mod_mul_precomputed_assign(step, m, &data);
        step.mod_mul_precomputed_assign(q_inverse, m, &data);
    }
    // The scaled coefficients, from the first nonzero one on, reversed.
    let scaled: Vec<T> = coefficients[start..]
        .iter()
        .zip(&inverse_powers[start..])
        .rev()
        .map(|(&c, &w)| c.mod_mul_precomputed(w, m, &data))
        .collect();
    let mut product = vec![T::ZERO; k];
    mod_mul_middle_karatsuba(
        &mut product,
        &scaled,
        &binomial_powers[start..start + a_len + k - 1],
        &ModData::new(m, a_len),
    );
    product
        .iter()
        .zip(&inverse_powers)
        .map(|(&z, &w)| z.mod_mul_precomputed(w, m, &data))
        .collect()
}}

// Evaluates like `mod_evaluate_geometric_fast`, but computes the sums with a truncated product of
// length $n + k - 1$ and keeps its last $k$ coefficients, discarding the first $n - 1$, rather than
// with a middle product. It is kept to measure what the middle product saves.
crate_test_fn! {
#[allow(dead_code)]
mod_evaluate_geometric_fast_truncated<T: PrimitiveUnsigned>(
    coefficients: &[T],
    q: T,
    q_inverse: T,
    k: usize,
    m: T,
) -> Vec<T> {
    if k == 0 {
        return Vec::new();
    }
    let Some(start) = coefficients.iter().position(|&c| c != T::ZERO) else {
        return vec![T::ZERO; k];
    };
    let n = coefficients.len();
    let a_len = n - start;
    let data = T::precompute_mod_mul_data(&m);
    // binomial_powers[t] = q^C(t, 2), for t < n + k - 1.
    let b_len = n + k - 1;
    let mut binomial_powers = Vec::with_capacity(b_len);
    let mut power = T::ONE % m;
    let mut step = T::ONE % m;
    for _ in 0..b_len {
        binomial_powers.push(power);
        power.mod_mul_precomputed_assign(step, m, &data);
        step.mod_mul_precomputed_assign(q, m, &data);
    }
    // inverse_powers[t] = q^-C(t, 2), for t < max(n, k).
    let w_len = max(n, k);
    let mut inverse_powers = Vec::with_capacity(w_len);
    let mut power = T::ONE % m;
    let mut step = T::ONE % m;
    for _ in 0..w_len {
        inverse_powers.push(power);
        power.mod_mul_precomputed_assign(step, m, &data);
        step.mod_mul_precomputed_assign(q_inverse, m, &data);
    }
    // The scaled coefficients, from the first nonzero one on, reversed.
    let scaled: Vec<T> = coefficients[start..]
        .iter()
        .zip(&inverse_powers[start..])
        .rev()
        .map(|(&c, &w)| c.mod_mul_precomputed(w, m, &data))
        .collect();
    let mut product = vec![T::ZERO; a_len + k - 1];
    mod_mul_truncated_to_out(
        &mut product,
        &scaled,
        &binomial_powers[start..start + a_len + k - 1],
        m,
    );
    product[a_len - 1..]
        .iter()
        .zip(&inverse_powers)
        .map(|(&z, &w)| z.mod_mul_precomputed(w, m, &data))
        .collect()
}}

impl<T: PrimitiveUnsigned> ModEvaluateGeometric<T> for &UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at $1, q, q^2, \ldots, q^{k-1}$, modulo a value of its
    /// coefficient type. The coefficients and `q` must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, k, m) = \left ( \sum_{i=0}^{n-1} c_i q^{ij} \bmod m \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// The powers of `q` are computed with Shoup's method when the top bit of `m` is clear, since
    /// every multiplication is by `q`, and are then evaluated as by
    /// [`mod_evaluate_many`](ModEvaluateMany::mod_evaluate_many), in place.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk)$
    ///
    /// $M(k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `k`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `q` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModEvaluateGeometric;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // At 1, 2, 4, and 8: 15, 33, 99, and 351, mod 13
    /// assert_eq!((&p).mod_evaluate_geometric(2, 4, 13), &[2, 7, 8, 0]);
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_geometric_nmod_vec_iter` from
    /// `nmod_poly/evaluate_geometric_nmod_vec.c`, FLINT 3.6.0, with `q` in place of FLINT's $r^2$:
    /// FLINT evaluates at the powers of the square of its argument.
    fn mod_evaluate_geometric(self, q: T, k: u64, m: T) -> Vec<T> {
        assert!(
            self.mod_is_reduced(&m),
            "self must be reduced mod m, but {self} has a coefficient >= {m}"
        );
        assert!(q < m, "q must be reduced mod m, but {q} >= {m}");
        let k = usize::exact_from(k);
        let n = self.coefficients.len();
        if mod_evaluate_geometric_fast_preferred(n, k, m)
            && q != T::ZERO
            && let Some(q_inverse) = q.mod_inverse(m)
        {
            mod_evaluate_geometric_fast(&self.coefficients, q, q_inverse, k, m)
        } else {
            mod_evaluate_geometric_iter(&self.coefficients, q, k, m)
        }
    }
}
