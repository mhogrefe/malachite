// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::modular::mod_gcd::{
    NaivePowerOf2Modulus, gcd_naive, monic_gcd_with,
};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::hgcd::gcd_hgcd;
use crate::unsigned_polynomial::modular::mod_gcd::{GcdModulus, gcd_euclidean};
use crate::unsigned_polynomial::modular::subproduct_tree::PowerOf2Modulus;

pub fn mod_power_of_2_gcd_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    gcd_naive(p, q, &NaivePowerOf2Modulus(pow))
}

pub fn mod_power_of_2_gcd_euclidean<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    monic_gcd_with(p, q, &PowerOf2Modulus(pow), pow == 0, &gcd_euclidean)
}

pub fn mod_power_of_2_gcd_hgcd<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    let modulus = PowerOf2Modulus(pow);
    let cutoffs = GcdModulus::<T>::gcd_cutoffs(&modulus);
    monic_gcd_with(p, q, &modulus, pow == 0, &|xs, ys, m| {
        gcd_hgcd(xs, ys, cutoffs.hgcd_iter, cutoffs.hgcd_outer, m)
    })
}
