// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::{DivExact, Lcm, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::gaussian_integer::arithmetic::sqrt::*;

// Clears denominators by the LCM twice over and hands the Gaussian integer to that type's naive
// root, then divides the root back down.
pub fn gaussian_rational_checked_sqrt_naive(z: &GaussianRational) -> Option<GaussianRational> {
    if *z == 0u32 {
        return Some(GaussianRational::ZERO);
    }
    let l = Integer::from(z.real.denominator_ref().lcm(z.imaginary.denominator_ref()));
    let l_squared = (&l) * (&l);
    let scaled = GaussianInteger {
        real: Integer::from_sign_and_abs(
            z.real >= 0u32,
            z.real.numerator_ref()
                * (&l_squared)
                    .div_exact(Integer::from(z.real.denominator_ref()))
                    .unsigned_abs(),
        ),
        imaginary: Integer::from_sign_and_abs(
            z.imaginary >= 0u32,
            z.imaginary.numerator_ref()
                * (&l_squared)
                    .div_exact(Integer::from(z.imaginary.denominator_ref()))
                    .unsigned_abs(),
        ),
    };
    let root = gaussian_integer_checked_sqrt_naive(&scaled)?;
    Some(GaussianRational {
        real: Rational::from_integers_ref(&root.real, &l),
        imaginary: Rational::from_integers(root.imaginary, l),
    })
}
