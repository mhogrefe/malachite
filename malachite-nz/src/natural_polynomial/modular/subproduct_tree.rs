// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::modular::mod_mul::ModMulData;
use crate::natural_polynomial::modular::mod_div_rem::{mod_quotient_with, remainder, to_words};
use crate::natural_polynomial::modular::mod_div_rem_precomputed::{
    mod_quotient_preinv, reversed_inverse,
};
use crate::natural_polynomial::modular::mod_evaluate::{
    mod_evaluate_coefficients, precompute_mod_mul_data,
};
use crate::natural_polynomial::modular::mod_mul::mod_mul_ref_ref;
use crate::natural_polynomial::modular::mod_power_of_2_div_rem::{
    mod_power_of_2_quotient_with, remainder as mod_power_of_2_remainder,
};
use crate::natural_polynomial::modular::mod_power_of_2_div_rem_precomputed::{
    mod_power_of_2_quotient_preinv, reversed_inverse as mod_power_of_2_reversed_inverse,
};
use crate::natural_polynomial::modular::mod_power_of_2_evaluate::*;
use crate::natural_polynomial::modular::mod_power_of_2_mul::mod_power_of_2_mul_ref_ref;
use crate::natural_polynomial::series::mod_div_truncated::mod_div_truncated_dispatch;
use crate::natural_polynomial::series::mod_power_of_2_div_truncated::*;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{
    ModAdd, ModMulPrecomputed, ModNeg, ModPowerOf2Add, ModPowerOf2Mul, ModPowerOf2Neg,
    ModPowerOf2Sub, ModSub,
};
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{
    Modulus, PowerOf2Modulus, TreeModulus, evaluate_many_fast,
};

// Whether evaluating a polynomial of length `n` at `k` points modulo an `m` of more than two limbs
// uses remainders down a subproduct tree rather than Horner's rule at each point. Horner's rule
// with `Natural`s is slow enough that the tree wins from a length of 32, as in FLINT, unless the
// polynomial is that short and there are many more points than coefficients. Measured on an Apple
// M-series machine, 2026-11, with 64-bit limbs and moduli of 192, 512, and 2048 bits.
crate_test_fn! {
#[allow(clippy::missing_const_for_fn)] // crate_test_fn! cannot declare a const fn
mod_evaluate_many_tree_preferred(n: usize, k: usize) -> bool {
    n >= 32 && (n >= 64 || k <= n << 3)
}}

// Whether evaluating a polynomial of length `n` at `k` points modulo a $2^k$ of more than two limbs
// uses remainders down a subproduct tree rather than Horner's rule at each point. Wrapping
// arithmetic keeps Horner's rule cheap, so the tree wins later than modulo a general `m`, and a
// moderately long polynomial keeps Horner's rule when there are many more points than coefficients.
// Measured on an Apple M-series machine, 2026-11, with 64-bit limbs and moduli of 2^192, 2^512, and
// 2^2048.
crate_test_fn! {
#[allow(clippy::missing_const_for_fn)] // crate_test_fn! cannot declare a const fn
mod_power_of_2_evaluate_many_tree_preferred(n: usize, k: usize) -> bool {
    n >= 256 && k >= 256 && (n >= 1024 || k <= n << 2)
}}

// A modulus $m > 1$ that does not fit in two limbs, with the data for multiplying modulo it.
pub(crate) struct NaturalModulus<'a> {
    pub(crate) m: &'a Natural,
    pub(crate) data: ModMulData,
}

impl<'a> NaturalModulus<'a> {
    pub(crate) fn new(m: &'a Natural) -> Self {
        NaturalModulus {
            m,
            data: precompute_mod_mul_data(m),
        }
    }
}

impl TreeModulus<Natural> for NaturalModulus<'_> {
    #[inline]
    fn zero(&self) -> Natural {
        Natural::ZERO
    }

    #[inline]
    fn one(&self) -> Natural {
        Natural::ONE
    }

    #[inline]
    fn neg(&self, x: &Natural) -> Natural {
        x.mod_neg(self.m)
    }

    #[inline]
    fn add(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_add(y, self.m)
    }

    #[inline]
    fn sub(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_sub(y, self.m)
    }

    #[inline]
    fn mul(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_mul_precomputed(y, self.m, &self.data)
    }

    fn mul_to_out(&self, out: &mut [Natural], xs: &[Natural], ys: &[Natural]) {
        let product = mod_mul_ref_ref(xs, ys, self.m).coefficients;
        let n = product.len();
        for (o, p) in out.iter_mut().zip(product) {
            *o = p;
        }
        out[n..].fill(Natural::ZERO);
    }

    fn rem_monic(&self, xs: &[Natural], ys: &[Natural]) -> Vec<Natural> {
        let qs = mod_quotient_with(xs, ys, self.m, &Natural::ONE, mod_div_truncated_dispatch);
        remainder(xs, ys, &qs, self.m)
    }

    #[inline]
    fn reversed_inverse(&self, ys: &[Natural]) -> Vec<Natural> {
        reversed_inverse(ys, self.m, &Natural::ONE)
    }

    #[inline]
    fn rem_monic_preinv(
        &self,
        xs: &[Natural],
        ys: &[Natural],
        inverse: &[Natural],
    ) -> Vec<Natural> {
        remainder(
            xs,
            ys,
            &mod_quotient_preinv(xs, ys, inverse, self.m),
            self.m,
        )
    }

    #[inline]
    fn evaluate(&self, poly: &[Natural], x: &Natural) -> Natural {
        mod_evaluate_coefficients(poly, x, self.m, &self.data)
    }
}

// The values modulo `m` of the polynomial with coefficients `poly` at the points `xs`, computed in
// `T`, which `m` must fit in, by building their subproduct tree.
fn mod_evaluate_many_fast_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    poly: &[Natural],
    xs: &[Natural],
    m: &Natural,
) -> Vec<Natural>
where
    Natural: From<T>,
{
    evaluate_many_fast(
        &to_words::<T>(poly),
        &to_words::<T>(xs),
        &Modulus(T::exact_from(m)),
    )
    .into_iter()
    .map(Natural::from)
    .collect()
}

// The values modulo $m > 1$ of the polynomial with coefficients `poly` at the points `xs`, by
// building their subproduct tree. The coefficients and the points must be reduced. When `m` fits in
// one or two limbs, this is done in word arithmetic.
crate_test_fn! {mod_evaluate_many_fast(
    poly: &[Natural],
    xs: &[Natural],
    m: &Natural,
) -> Vec<Natural> {
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        mod_evaluate_many_fast_word::<Limb>(poly, xs, m)
    } else if bits <= DoubleLimb::WIDTH {
        mod_evaluate_many_fast_word::<DoubleLimb>(poly, xs, m)
    } else {
        evaluate_many_fast(poly, xs, &NaturalModulus::new(m))
    }
}}

// A modulus $2^k$ that does not fit in two limbs; the field is $k$.
#[derive(Clone, Copy)]
pub(crate) struct NaturalPowerOf2Modulus(pub u64);

impl TreeModulus<Natural> for NaturalPowerOf2Modulus {
    #[inline]
    fn zero(&self) -> Natural {
        Natural::ZERO
    }

    #[inline]
    fn one(&self) -> Natural {
        Natural::ONE
    }

    #[inline]
    fn neg(&self, x: &Natural) -> Natural {
        x.mod_power_of_2_neg(self.0)
    }

    #[inline]
    fn add(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_power_of_2_add(y, self.0)
    }

    #[inline]
    fn sub(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_power_of_2_sub(y, self.0)
    }

    #[inline]
    fn mul(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_power_of_2_mul(y, self.0)
    }

    fn mul_to_out(&self, out: &mut [Natural], xs: &[Natural], ys: &[Natural]) {
        let product = mod_power_of_2_mul_ref_ref(xs, ys, self.0).coefficients;
        let n = product.len();
        for (o, p) in out.iter_mut().zip(product) {
            *o = p;
        }
        out[n..].fill(Natural::ZERO);
    }

    // A monic divisor needs no inverse, so this works modulo $2^k$ although most leading
    // coefficients are not invertible.
    fn rem_monic(&self, xs: &[Natural], ys: &[Natural]) -> Vec<Natural> {
        let qs = mod_power_of_2_quotient_with(
            xs,
            ys,
            self.0,
            &Natural::ONE,
            mod_power_of_2_div_truncated_dispatch,
        );
        mod_power_of_2_remainder(xs, ys, &qs, self.0)
    }

    #[inline]
    fn reversed_inverse(&self, ys: &[Natural]) -> Vec<Natural> {
        mod_power_of_2_reversed_inverse(ys, self.0, &Natural::ONE)
    }

    #[inline]
    fn rem_monic_preinv(
        &self,
        xs: &[Natural],
        ys: &[Natural],
        inverse: &[Natural],
    ) -> Vec<Natural> {
        let qs = mod_power_of_2_quotient_preinv(xs, ys, inverse, self.0);
        mod_power_of_2_remainder(xs, ys, &qs, self.0)
    }

    #[inline]
    fn evaluate(&self, poly: &[Natural], x: &Natural) -> Natural {
        mod_power_of_2_evaluate_coefficients(poly, x, self.0)
    }
}

// The values modulo $2^k$, where $k$ is `pow`, of the polynomial with coefficients `poly` at the
// points `xs`, computed in `T`, which $2^k$ must fit in, by building their subproduct tree.
fn mod_power_of_2_evaluate_many_fast_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    poly: &[Natural],
    xs: &[Natural],
    pow: u64,
) -> Vec<Natural>
where
    Natural: From<T>,
{
    evaluate_many_fast(
        &to_words::<T>(poly),
        &to_words::<T>(xs),
        &PowerOf2Modulus(pow),
    )
    .into_iter()
    .map(Natural::from)
    .collect()
}

// The values modulo $2^k$, where $k > 0$ is `pow`, of the polynomial with coefficients `poly` at
// the points `xs`, by building their subproduct tree. The coefficients and the points must be
// reduced. When $2^k$ fits in one or two limbs, this is done in word arithmetic.
crate_test_fn! {mod_power_of_2_evaluate_many_fast(
    poly: &[Natural],
    xs: &[Natural],
    pow: u64,
) -> Vec<Natural> {
    if pow <= Limb::WIDTH {
        mod_power_of_2_evaluate_many_fast_word::<Limb>(poly, xs, pow)
    } else if pow <= DoubleLimb::WIDTH {
        mod_power_of_2_evaluate_many_fast_word::<DoubleLimb>(poly, xs, pow)
    } else {
        evaluate_many_fast(poly, xs, &NaturalPowerOf2Modulus(pow))
    }
}}
