// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_div_rem::*;
use crate::natural_polynomial::arithmetic::mod_power_of_2_div_truncated::*;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Inverse, ModPowerOf2Mul, ModPowerOf2SubAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Divides one polynomial by another modulo 2^k by schoolbook long division, using only the scalar
// modular arithmetic: while the remainder is at least as long as the divisor, its leading term is
// cancelled by a multiple of the divisor. Returns `None` if the divisor is zero or its leading
// coefficient is even.
pub fn mod_power_of_2_div_rem_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
    if pow == 0 {
        return Some((NaturalPolynomial::ZERO, NaturalPolynomial::ZERO));
    }
    let ys = q.coefficients_asc();
    let inverse = ys.last()?.mod_power_of_2_inverse(pow)?;
    let mut rs = p.coefficients_asc().to_vec();
    if rs.len() < ys.len() {
        return Some((NaturalPolynomial::ZERO, p.clone()));
    }
    let mut qs = vec![Natural::ZERO; rs.len() - ys.len() + 1];
    for i in (0..qs.len()).rev() {
        let c = (&rs[i + ys.len() - 1]).mod_power_of_2_mul(&inverse, pow);
        for (j, y) in ys.iter().enumerate() {
            rs[i + j].mod_power_of_2_sub_assign((&c).mod_power_of_2_mul(y, pow), pow);
        }
        qs[i] = c;
    }
    rs.truncate(ys.len() - 1);
    Some((
        NaturalPolynomial::from_coefficients_asc(qs),
        NaturalPolynomial::from_coefficients_asc(rs),
    ))
}

// The quotient and remainder of two polynomials modulo 2^k, with the reversals divided as power
// series coefficient by coefficient.
pub fn mod_power_of_2_div_rem_basecase_wrapper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
    mod_power_of_2_div_rem_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        pow,
        mod_power_of_2_div_truncated_basecase,
    )
}

// The quotient and remainder of two polynomials modulo 2^k, with the reversals divided as power
// series by the method of Karp and Markstein.
pub fn mod_power_of_2_div_rem_newton_wrapper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
    mod_power_of_2_div_rem_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        pow,
        mod_power_of_2_div_truncated_newton,
    )
}
