// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{ComposePowerOfX, ExponentGcd, Polynomial};
use malachite_base::test_util::generators::{
    unsigned_polynomial_gen, unsigned_polynomial_unsigned_pair_gen_var_2,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::exponent_gcd::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_exponent_gcd() {
    let test = |s, out| {
        let p = UnsignedPolynomial::<u64>::from_str(s).unwrap();
        assert_eq!(p.exponent_gcd(), out);
        assert_eq!(exponent_gcd_naive(&p), out);
    };
    // A constant, including zero, gives 0.
    test("0", 0);
    test("5", 0);
    test("x", 1);
    test("x^4", 4);
    test("x^6+2*x^3+1", 3);
    test("x^6+x^4+1", 2);
    test("x^6+x^3+x^2", 1);
    test("x^12+x^8", 4);
    test("x^15+x^10+x^5+1", 5);
    test("x^4+x", 1);
    test("18446744073709551615*x^9+x^6", 3);
}

#[test]
fn exponent_gcd_properties() {
    unsigned_polynomial_gen().test_properties(|p| {
        let d = p.exponent_gcd();
        assert_eq!(exponent_gcd_naive(&p), d);
        // It is 0 exactly for the constants.
        assert_eq!(d == 0, p.len() <= 1);
        if d != 0 {
            // Every exponent with a nonzero coefficient is a multiple of it, and composing the
            // polynomial of the coefficients at those multiples with x^d gives p back.
            let q = UnsignedPolynomial::<u64>::from_coefficients_asc(
                (0..p.len())
                    .step_by(usize::exact_from(d))
                    .map(|i| p.coefficient(i))
                    .collect::<Vec<_>>(),
            );
            assert_eq!(q.compose_power_of_x(d), p);
            for i in 0..p.len() {
                if i % d != 0 {
                    assert!(p.coefficient(i) == 0);
                }
            }
        }
    });

    unsigned_polynomial_unsigned_pair_gen_var_2().test_properties(|(p, k)| {
        // Composing a nonconstant polynomial with x^k multiplies the exponent GCD by k.
        if k != 0 && p.len() > 1 {
            assert_eq!(
                (&p).compose_power_of_x(k).exponent_gcd(),
                p.exponent_gcd() * k
            );
        }
    });
}
