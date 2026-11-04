// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::test_util::unsigned_polynomial::modular::mod_gcd::{
    NaiveGcdArithmetic, NaiveModulus, gcd_naive, naive_make_monic,
};
use crate::unsigned_polynomial::UnsignedPolynomial;

// The monic LCM, `p * (q / g)` made monic, with the GCD from `gcd_naive`; `None` unless the product
// of the leading coefficients has an inverse.
pub fn lcm_naive<T: PrimitiveUnsigned, A: NaiveGcdArithmetic<T>>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    a: &A,
) -> Option<UnsignedPolynomial<T>> {
    if a.trivial() || *p == UnsignedPolynomial::ZERO || *q == UnsignedPolynomial::ZERO {
        return Some(UnsignedPolynomial::ZERO);
    }
    let leads = a.scale(
        &UnsignedPolynomial::from_coefficients_asc(vec![*p.coefficients_asc().last().unwrap()]),
        *q.coefficients_asc().last().unwrap(),
    );
    naive_make_monic(&leads, a)?;
    let g = gcd_naive(p, q, a)?;
    let cofactor = a.div_rem(q, &g)?.0;
    naive_make_monic(&a.mul(p, &cofactor), a)
}

pub fn mod_lcm_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    lcm_naive(p, q, &NaiveModulus(m))
}
