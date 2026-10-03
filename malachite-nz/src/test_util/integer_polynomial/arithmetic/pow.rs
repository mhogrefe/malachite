// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::test_util::integer_polynomial::arithmetic::mul::mul_naive;
use malachite_base::num::arithmetic::traits::{AddMulAssign, DivExactAssign, Pow, SubMulAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;

// Raises the polynomial to the power `e` by `e` schoolbook multiplications, starting from 1.
pub fn pow_naive(p: &IntegerPolynomial, e: u64) -> IntegerPolynomial {
    let mut power = IntegerPolynomial::one();
    for _ in 0..e {
        power = mul_naive(&power, p);
    }
    power
}

// The multinomial kernel in FLINT's form, for `Integer` coefficients only: each term forms the
// product $q_i r_{k - i}$ and adds or subtracts it, scaled, into a single sum. Requires the same as
// `pow_to_out_multinomial`.
//
// This is `_fmpz_poly_pow_multinomial` from `fmpz_poly/pow_multinomial.c`, FLINT 3.6.0, without the
// stripping of zero low coefficients.
pub fn pow_to_out_multinomial_flint(out: &mut [Integer], xs: &[Integer], e: u64) {
    let len = xs.len();
    let mut d = Integer::ZERO;
    out[0] = (&xs[0]).pow(e);
    for k in 1..out.len() {
        let mut sum = Integer::ZERO;
        let mut u = -i128::exact_from(k);
        for i in 1..=k.min(len - 1) {
            let t = &xs[i] * &out[k - i];
            u += i128::from(e) + 1;
            if u >= 0 {
                sum.add_mul_assign(&t, Integer::from(u));
            } else {
                sum.sub_mul_assign(&t, Integer::from(-u));
            }
        }
        d += &xs[0];
        sum.div_exact_assign(&d);
        out[k] = sum;
    }
}
