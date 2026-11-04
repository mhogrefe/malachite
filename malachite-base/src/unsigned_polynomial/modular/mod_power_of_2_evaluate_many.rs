// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::ModPowerOf2IsReduced;
#[cfg(doc)]
use crate::polynomial::modular::traits::ModPowerOf2Evaluate;
use crate::polynomial::modular::traits::ModPowerOf2EvaluateMany;
#[cfg(doc)]
use crate::polynomial::modular::traits::ModPowerOf2EvaluateManyPrecomputed;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::subproduct_tree::{
    PowerOf2Modulus, TreeModulus, evaluate_many_fast, mod_power_of_2_evaluate_many_tree_preferred,
};
use alloc::vec;
use alloc::vec::Vec;

// The number of points evaluated together by `mod_power_of_2_evaluate_many_horner`. Horner's rule
// is a chain of dependent multiplications, so running several points through one pass over the
// coefficients keeps more multiplications in flight.
const MOD_POWER_OF_2_EVALUATE_MANY_BLOCK: usize = 8;

// The shortest polynomial evaluated in blocks of points; shorter ones are evaluated one point at a
// time, since the blocks' setup is not paid back. Measured on an Apple M-series machine, 2026-11,
// for `u64` with moduli of 2^32 and 2^64: blocks of 8 are 1.4 times as fast from length 512 on.
const MOD_POWER_OF_2_EVALUATE_MANY_BLOCKED_MIN_LENGTH: usize = 48;

// The values modulo 2^pow of the polynomial with coefficients `coefficients` at the points `xs`,
// with wrapping Horner's rule for a block of points at a time. The coefficients and the points must
// be reduced; this is not checked.
crate_test_fn! {mod_power_of_2_evaluate_many_horner<T: PrimitiveUnsigned>(
    coefficients: &[T],
    xs: &[T],
    pow: u64,
) -> Vec<T> {
    let mut values = vec![T::ZERO; xs.len()];
    let Some((&leading, rest)) = coefficients.split_last() else {
        return values;
    };
    for (block, xs) in values
        .chunks_mut(MOD_POWER_OF_2_EVALUATE_MANY_BLOCK)
        .zip(xs.chunks(MOD_POWER_OF_2_EVALUATE_MANY_BLOCK))
    {
        block.fill(leading);
        for &c in rest.iter().rev() {
            for (value, &x) in block.iter_mut().zip(xs) {
                *value = value.wrapping_mul(x).wrapping_add(c);
            }
        }
    }
    for value in &mut values {
        value.mod_power_of_2_assign(pow);
    }
    values
}}

fn mod_power_of_2_evaluate_many_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    xs: &[T],
    pow: u64,
) -> Vec<T> {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    for &x in xs {
        assert!(
            x.significant_bits() <= pow,
            "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
        );
    }
    if pow == 0 {
        return vec![T::ZERO; xs.len()];
    }
    let n = p.coefficients.len();
    if mod_power_of_2_evaluate_many_tree_preferred(n, xs.len()) {
        evaluate_many_fast(&p.coefficients, xs, &PowerOf2Modulus(pow))
    } else if n >= MOD_POWER_OF_2_EVALUATE_MANY_BLOCKED_MIN_LENGTH {
        mod_power_of_2_evaluate_many_horner(&p.coefficients, xs, pow)
    } else {
        let m = PowerOf2Modulus(pow);
        xs.iter().map(|x| m.evaluate(&p.coefficients, x)).collect()
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2EvaluateMany<T> for &UnsignedPolynomial<T> {
    type Output = T;

    /// Evaluates an [`UnsignedPolynomial`] at each of several values of its coefficient type,
    /// modulo $2^k$. The coefficients and the values must already be reduced modulo $2^k$, and $k$
    /// may be at most the width of the type.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{l-1}, k) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod 2^k
    /// \right )_{j=0}^{l-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// The result is the same as calling
    /// [`mod_power_of_2_evaluate`](ModPowerOf2Evaluate::mod_power_of_2_evaluate) at each value, but
    /// the polynomial is checked once. Reducing modulo $2^k$ commutes with wrapping arithmetic, so
    /// Horner's rule is carried out with wrapping multiplications and additions, for several values
    /// together in each pass over the coefficients when the polynomial is long enough. For a long
    /// polynomial and many values, the polynomial is instead reduced modulo the products of the
    /// linear factors $x - x_j$ in a subproduct tree, from the top down, as with
    /// [`mod_power_of_2_evaluate_many_precomputed`](
    /// ModPowerOf2EvaluateManyPrecomputed::mod_power_of_2_evaluate_many_precomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(\min(nl, n \log n + l \log^2 l))$
    ///
    /// $M(n, l) = O(n + l \log l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $l$ is `xs.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or any value in
    /// `xs` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateMany;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+3*x+7").unwrap();
    /// // 7, 15, 33, and 61, mod 16
    /// assert_eq!(
    ///     (&p).mod_power_of_2_evaluate_many(&[0, 1, 2, 3], 4),
    ///     &[7, 15, 1, 13]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_evaluate_nmod_vec` from `nmod_poly/evaluate_nmod_vec.c`,
    /// FLINT 3.6.0, with the modulus $2^k$, except that the values must be reduced, and that the
    /// choice between evaluating at each value and the subproduct tree depends on the number of
    /// values as well as on the polynomial's length.
    #[inline]
    fn mod_power_of_2_evaluate_many(self, xs: &[T], pow: u64) -> Vec<T> {
        mod_power_of_2_evaluate_many_helper(self, xs, pow)
    }
}
