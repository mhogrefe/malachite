// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_gcd::{
    GcdOperation, OnePolynomial, checked_mod_gcd_operation, checked_mod_power_of_2_gcd_operation,
};
use alloc::vec::Vec;
use malachite_base::test_util::unsigned_polynomial::modular::mod_gcd::monic_gcd_slices_with;
use malachite_base::unsigned_polynomial::modular::hgcd::gcd_hgcd;
use malachite_base::unsigned_polynomial::modular::mod_gcd::{GcdModulus, gcd_euclidean};

// The monic GCD by Euclid's algorithm alone.
pub struct EuclideanGcd;

impl GcdOperation for EuclideanGcd {
    type Shape = OnePolynomial;

    fn run<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<Vec<C>> {
        monic_gcd_slices_with(xs, ys, m, &gcd_euclidean)
    }
}

// The monic GCD by the half-GCD algorithm whatever the lengths, with the library's inner cutoffs.
pub struct HalfGcd;

impl GcdOperation for HalfGcd {
    type Shape = OnePolynomial;

    fn run<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<Vec<C>> {
        let cutoffs = m.gcd_cutoffs();
        monic_gcd_slices_with(xs, ys, m, &|xs, ys, m| {
            gcd_hgcd(xs, ys, cutoffs.hgcd_iter, cutoffs.hgcd_outer, m)
        })
    }
}

pub fn mod_gcd_euclidean(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    checked_mod_gcd_operation::<EuclideanGcd>(p, q, m)
}

pub fn mod_gcd_hgcd(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    checked_mod_gcd_operation::<HalfGcd>(p, q, m)
}

pub fn mod_power_of_2_gcd_euclidean(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<NaturalPolynomial> {
    checked_mod_power_of_2_gcd_operation::<EuclideanGcd>(p, q, pow)
}

pub fn mod_power_of_2_gcd_hgcd(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<NaturalPolynomial> {
    checked_mod_power_of_2_gcd_operation::<HalfGcd>(p, q, pow)
}
