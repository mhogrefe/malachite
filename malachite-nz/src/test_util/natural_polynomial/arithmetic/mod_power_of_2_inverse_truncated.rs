// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_inverse_truncated::*;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2AddAssign, ModPowerOf2Inverse, ModPowerOf2Mul, ModPowerOf2Neg,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;

// Inverts a polynomial modulo 2^k as a power series, keeping the first `len` coefficients, one
// coefficient at a time, using only the scalar modular arithmetic: $y_0 = x_0^{-1}$ and $y_i =
// -x_0^{-1} \sum_{j=1}^i x_j y_{i-j}$.
pub fn mod_power_of_2_inverse_truncated_naive(
    p: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    if len == 0 || pow == 0 {
        return Some(NaturalPolynomial::ZERO);
    }
    let x0 = p.coefficient(0);
    if *x0 == 0u32 {
        return None;
    }
    let x0_inverse = x0.mod_power_of_2_inverse(pow)?;
    let n = usize::exact_from(len);
    let mut ys: Vec<Natural> = Vec::with_capacity(n);
    ys.push(x0_inverse.clone());
    for i in 1..n {
        let mut sum = Natural::ZERO;
        for j in 1..=i {
            sum.mod_power_of_2_add_assign(
                p.coefficient(u64::exact_from(j))
                    .mod_power_of_2_mul(&ys[i - j], pow),
                pow,
            );
        }
        ys.push(
            sum.mod_power_of_2_mul(&x0_inverse, pow)
                .mod_power_of_2_neg(pow),
        );
    }
    Some(NaturalPolynomial::from_coefficients_asc(ys))
}

// The inverse of a polynomial modulo 2^k as a power series, coefficient by coefficient.
pub fn mod_power_of_2_inverse_truncated_basecase_wrapper(
    p: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    mod_power_of_2_inverse_truncated_with(
        p.coefficients_asc(),
        len,
        pow,
        mod_power_of_2_inverse_truncated_basecase,
    )
}

fn newton_cutoff_2(xs: &[Natural], n: usize, pow: u64, inverse: &Natural) -> Vec<Natural> {
    mod_power_of_2_inverse_truncated_newton(xs, n, pow, inverse, 2)
}

// The inverse of a polynomial modulo 2^k as a power series, by Newton's iteration all the way down
// to a single coefficient.
pub fn mod_power_of_2_inverse_truncated_newton_wrapper(
    p: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    mod_power_of_2_inverse_truncated_with(p.coefficients_asc(), len, pow, newton_cutoff_2)
}
