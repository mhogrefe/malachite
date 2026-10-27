// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2Add;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModPowerOf2MulTruncated, Polynomial};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_compose_truncated::*;

// Composes two polynomials modulo 2^pow as power series, keeping the first `len` coefficients, with
// Horner's rule over whole polynomials, using only the public modular arithmetic: $p(q) =
// (\cdots(p_{n-1} q + p_{n-2}) q + \cdots) q + p_0$, every product truncated.
pub fn mod_power_of_2_compose_truncated_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    let mut result = UnsignedPolynomial::ZERO;
    if len == 0 {
        return result;
    }
    for &c in p.coefficients_asc().iter().rev() {
        result = result
            .mod_power_of_2_mul_truncated(q, len, pow)
            .mod_power_of_2_add(UnsignedPolynomial::from(c), pow);
    }
    result
}

// The composition of two polynomials modulo 2^pow as power series, by Horner's rule.
pub fn mod_power_of_2_compose_truncated_horner_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    UnsignedPolynomial::from_coefficients_asc(mod_power_of_2_compose_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        pow,
        mod_power_of_2_compose_truncated_horner,
    ))
}

// The composition of two polynomials modulo 2^pow as power series, by divide and conquer.
pub fn mod_power_of_2_compose_truncated_divide_and_conquer_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    UnsignedPolynomial::from_coefficients_asc(mod_power_of_2_compose_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        pow,
        mod_power_of_2_compose_truncated_divide_and_conquer,
    ))
}

// The composition of two polynomials modulo 2^pow as power series, by the Brent–Kung algorithm.
pub fn mod_power_of_2_compose_truncated_brent_kung_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    UnsignedPolynomial::from_coefficients_asc(mod_power_of_2_compose_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        pow,
        mod_power_of_2_compose_truncated_brent_kung,
    ))
}
