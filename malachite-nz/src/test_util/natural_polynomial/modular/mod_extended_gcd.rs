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
    GcdOperation, ThreePolynomials, checked_mod_gcd_operation, checked_mod_power_of_2_gcd_operation,
};
use alloc::vec::Vec;
use malachite_base::test_util::unsigned_polynomial::modular::mod_extended_gcd::*;
use malachite_base::unsigned_polynomial::modular::hgcd::xgcd_hgcd;
use malachite_base::unsigned_polynomial::modular::mod_extended_gcd::xgcd_euclidean;
use malachite_base::unsigned_polynomial::modular::mod_gcd::GcdModulus;

// The monic extended GCD by Euclid's algorithm alone.
pub struct EuclideanExtendedGcd;

impl GcdOperation for EuclideanExtendedGcd {
    type Shape = ThreePolynomials;

    fn run<C: Clone, M: GcdModulus<C>>(
        xs: &[C],
        ys: &[C],
        m: &M,
    ) -> Option<(Vec<C>, Vec<C>, Vec<C>)> {
        monic_extended_gcd_slices_with(xs, ys, m, &xgcd_euclidean)
    }
}

// The monic extended GCD by the half-GCD algorithm whatever the lengths, with the library's inner
// cutoffs.
pub struct HalfExtendedGcd;

impl GcdOperation for HalfExtendedGcd {
    type Shape = ThreePolynomials;

    fn run<C: Clone, M: GcdModulus<C>>(
        xs: &[C],
        ys: &[C],
        m: &M,
    ) -> Option<(Vec<C>, Vec<C>, Vec<C>)> {
        let cutoffs = m.gcd_cutoffs();
        monic_extended_gcd_slices_with(xs, ys, m, &|xs, ys, m| {
            xgcd_hgcd(xs, ys, cutoffs.hgcd_iter, cutoffs.xgcd_hgcd, m)
        })
    }
}

#[allow(clippy::type_complexity)]
pub fn mod_extended_gcd_euclidean(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
    checked_mod_gcd_operation::<EuclideanExtendedGcd>(p, q, m)
}

#[allow(clippy::type_complexity)]
pub fn mod_extended_gcd_hgcd(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
    checked_mod_gcd_operation::<HalfExtendedGcd>(p, q, m)
}

#[allow(clippy::type_complexity)]
pub fn mod_power_of_2_extended_gcd_euclidean(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
    checked_mod_power_of_2_gcd_operation::<EuclideanExtendedGcd>(p, q, pow)
}

#[allow(clippy::type_complexity)]
pub fn mod_power_of_2_extended_gcd_hgcd(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<(NaturalPolynomial, NaturalPolynomial, NaturalPolynomial)> {
    checked_mod_power_of_2_gcd_operation::<HalfExtendedGcd>(p, q, pow)
}
