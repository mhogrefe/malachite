// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::modular::mod_extended_gcd::{
    extended_gcd_naive, monic_extended_gcd_with,
};
use crate::test_util::unsigned_polynomial::modular::mod_gcd::NaivePowerOf2Modulus;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::hgcd::xgcd_hgcd;
use crate::unsigned_polynomial::modular::mod_extended_gcd::xgcd_euclidean;
use crate::unsigned_polynomial::modular::mod_gcd::GcdModulus;
use crate::unsigned_polynomial::modular::subproduct_tree::PowerOf2Modulus;

#[allow(clippy::type_complexity)]
pub fn mod_power_of_2_extended_gcd_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    extended_gcd_naive(p, q, &NaivePowerOf2Modulus(pow))
}

#[allow(clippy::type_complexity)]
pub fn mod_power_of_2_extended_gcd_euclidean<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    monic_extended_gcd_with(p, q, &PowerOf2Modulus(pow), pow == 0, &xgcd_euclidean)
}

#[allow(clippy::type_complexity)]
pub fn mod_power_of_2_extended_gcd_hgcd<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<(
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
    UnsignedPolynomial<T>,
)> {
    let modulus = PowerOf2Modulus(pow);
    let cutoffs = GcdModulus::<T>::gcd_cutoffs(&modulus);
    monic_extended_gcd_with(p, q, &modulus, pow == 0, &|xs, ys, m| {
        xgcd_hgcd(xs, ys, cutoffs.hgcd_iter, cutoffs.xgcd_hgcd, m)
    })
}
