// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{DivExact, Lcm, Pow, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::gaussian_integer::arithmetic::root::*;

// Clears denominators by L^exp and hands the Gaussian integer to that type's brute-force roots,
// then divides them back down. Only sensible for small inputs.
pub fn gaussian_rational_checked_roots_naive(
    z: &GaussianRational,
    exp: u64,
) -> Vec<GaussianRational> {
    assert_ne!(exp, 0);
    if *z == 0u32 {
        return vec![GaussianRational::ZERO];
    }
    let l = Integer::from(z.real.denominator_ref().lcm(z.imaginary.denominator_ref()));
    let l_pow = (&l).pow(exp);
    let scaled = GaussianInteger {
        real: Integer::from_sign_and_abs(
            z.real >= 0u32,
            z.real.numerator_ref()
                * (&l_pow)
                    .div_exact(Integer::from(z.real.denominator_ref()))
                    .unsigned_abs(),
        ),
        imaginary: Integer::from_sign_and_abs(
            z.imaginary >= 0u32,
            z.imaginary.numerator_ref()
                * (&l_pow)
                    .div_exact(Integer::from(z.imaginary.denominator_ref()))
                    .unsigned_abs(),
        ),
    };
    gaussian_integer_checked_roots_naive(&scaled, exp)
        .into_iter()
        .map(|root| GaussianRational {
            real: Rational::from_integers_ref(&root.real, &l),
            imaginary: Rational::from_integers(root.imaginary, l.clone()),
        })
        .collect()
}
