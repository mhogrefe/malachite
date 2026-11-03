// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_div_rem::to_words;
use crate::natural_polynomial::modular::subproduct_tree::{
    NaturalPowerOf2Modulus, mod_power_of_2_evaluate_many_tree_preferred,
};
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::ModPowerOf2IsReduced;
use malachite_base::polynomial::Polynomial;
#[cfg(doc)]
use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateMany;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{
    TreeModulus, evaluate_many_fast,
};

// The values modulo $2^k$, where $k$ is `pow`, of the polynomial with coefficients `coefficients`
// at the points `xs`, computed in `T`, which $2^k$ must fit in, by the evaluation of
// `UnsignedPolynomial`s.
fn mod_power_of_2_evaluate_many_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    coefficients: &[Natural],
    xs: &[Natural],
    pow: u64,
) -> Vec<Natural>
where
    Natural: From<T>,
{
    (&UnsignedPolynomial::from_coefficients_asc(to_words::<T>(coefficients)))
        .mod_power_of_2_evaluate_many(&to_words::<T>(xs), pow)
        .into_iter()
        .map(Natural::from)
        .collect()
}

fn mod_power_of_2_evaluate_many(p: &NaturalPolynomial, xs: &[Natural], pow: u64) -> Vec<Natural> {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    for x in xs {
        assert!(
            x.significant_bits() <= pow,
            "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
        );
    }
    if pow <= Limb::WIDTH {
        return mod_power_of_2_evaluate_many_word::<Limb>(&p.coefficients, xs, pow);
    } else if pow <= DoubleLimb::WIDTH {
        return mod_power_of_2_evaluate_many_word::<DoubleLimb>(&p.coefficients, xs, pow);
    }
    if mod_power_of_2_evaluate_many_tree_preferred(p.coefficients.len(), xs.len()) {
        return evaluate_many_fast(&p.coefficients, xs, &NaturalPowerOf2Modulus(pow));
    }
    let m = NaturalPowerOf2Modulus(pow);
    xs.iter().map(|x| m.evaluate(&p.coefficients, x)).collect()
}

impl ModPowerOf2EvaluateMany<Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at each of several [`Natural`]s, modulo $2^k$. The
    /// coefficients and the values must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{l-1}, k) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod 2^k
    /// \right )_{j=0}^{l-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// The result is the same as calling
    /// [`mod_power_of_2_evaluate`](ModPowerOf2Evaluate::mod_power_of_2_evaluate)
    /// at each value, but the polynomial is checked once. When $2^k$ fits in one or two limbs, the
    /// evaluation is done in words. Otherwise, Horner's rule is used at each value, or, for a long
    /// polynomial and many values, the polynomial is reduced modulo the products of the linear
    /// factors $x - x_j$ in a subproduct tree, from the top down.
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O(\min(nlk \log k \log\log k, N \log^2 N \log\log N))$
    ///
    /// $M(n, l, k) = O((n + l \log l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $l$ is `xs.len()`, $k$ is
    /// `pow`, and $N$ is $(n + l)k$.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or any value in `xs` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2EvaluateMany;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// // 7, 15, 33, and 61, mod 16
    /// assert_eq!(
    ///     (&p).mod_power_of_2_evaluate_many(&xs, 4).to_debug_string(),
    ///     "[7, 15, 1, 13]"
    /// );
    /// assert_eq!(
    ///     (&p).mod_power_of_2_evaluate_many(&xs, 100)
    ///         .to_debug_string(),
    ///     "[7, 15, 33, 61]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_evaluate_fmpz_vec` from
    /// `fmpz_mod_poly/evaluate_fmpz_vec.c`, FLINT 3.6.0, with the modulus $2^k$, except that the
    /// values must be reduced, and that the choice between evaluating at each value and the
    /// subproduct tree depends on the number of values as well as on the polynomial's length.
    #[inline]
    fn mod_power_of_2_evaluate_many(self, xs: &[Natural], pow: u64) -> Vec<Natural> {
        mod_power_of_2_evaluate_many(self, xs, pow)
    }
}
