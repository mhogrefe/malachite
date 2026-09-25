// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{EvaluateModPowerOf2, Polynomial};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_base::test_util::unsigned_polynomial::arithmetic::evaluate::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_evaluate_mod_power_of_2() {
    fn test<T: PrimitiveUnsigned>(s: &str, x: T, pow: u64, out: T) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        assert_eq!((&p).evaluate_mod_power_of_2(x, pow), out);
        assert_eq!(p.evaluate_mod_power_of_2(x, pow), out);
    }
    test::<u8>("0", 0, 0, 0);
    test::<u8>("0", 5, 3, 0);
    test::<u8>("5*x^2+3*x+7", 5, 3, 3);
    test::<u8>("5*x^2+3*x+7", 6, 4, 13);
    test::<u8>("5*x^2+3*x+7", 6, 8, 205);
    // p(0) is the constant term.
    test::<u8>("5*x^2+3*x+7", 0, 4, 7);
    // The intermediate values wrap around the width of `T`, which is harmless modulo 2^pow.
    test::<u8>("255*x+255", 255, 8, 0);
    test::<u8>("x^7", 2, 8, 128);
    test::<u8>("x^8", 2, 8, 0);
    test::<u64>("3*x^3+2*x+1", 12345, 20, 232110);
    test::<u64>("3*x^3+2*x+1", 12345, 64, 5644097915566);
    test::<u64>("18446744073709551615*x^2+1", u64::MAX, 64, 0);
    test::<u128>("x^2+1", 1 << 127, 128, 1);
    test::<u128>("340282366920938463463374607431768211455*x+5", 3, 128, 2);
}

#[test]
#[should_panic]
fn evaluate_mod_power_of_2_fail_1() {
    // pow is wider than `T`.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .evaluate_mod_power_of_2(1, 9);
}

#[test]
#[should_panic]
fn evaluate_mod_power_of_2_fail_2() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("4*x+1")
        .unwrap()
        .evaluate_mod_power_of_2(1, 2);
}

#[test]
#[should_panic]
fn evaluate_mod_power_of_2_fail_3() {
    // x is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .evaluate_mod_power_of_2(4, 2);
}

#[test]
#[should_panic]
fn evaluate_mod_power_of_2_fail_4() {
    // By reference, x is not reduced.
    (&UnsignedPolynomial::<u64>::from_str("x+1").unwrap()).evaluate_mod_power_of_2(1, 0);
}

fn evaluate_mod_power_of_2_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, x, pow)| {
        let y = (&p).evaluate_mod_power_of_2(x, pow);
        assert!(y.mod_power_of_2_is_reduced(pow));
        assert_eq!(p.clone().evaluate_mod_power_of_2(x, pow), y);
        assert_eq!(evaluate_mod_power_of_2_naive(&p, x, pow), y);

        // Reducing further agrees with evaluating the reduced polynomial at the reduced value.
        for smaller in [0, pow >> 1, pow.saturating_sub(1)] {
            assert_eq!(
                y.mod_power_of_2(smaller),
                (&p).mod_power_of_2(smaller)
                    .evaluate_mod_power_of_2(x.mod_power_of_2(smaller), smaller)
            );
        }

        // p(0) is the constant term, and p(1) the sum of the coefficients, mod 2^pow.
        assert_eq!((&p).evaluate_mod_power_of_2(T::ZERO, pow), p.coefficient(0));
        if pow != 0 {
            assert_eq!(
                (&p).evaluate_mod_power_of_2(T::ONE, pow),
                p.coefficients_asc()
                    .iter()
                    .fold(T::ZERO, |sum, &c| sum.mod_power_of_2_add(c, pow))
            );
        }
        // Everything is 0 mod 2^0.
        if pow == 0 {
            assert_eq!(y, T::ZERO);
        }
    });
}

#[test]
fn evaluate_mod_power_of_2_properties() {
    apply_fn_to_unsigneds!(evaluate_mod_power_of_2_properties_helper);
}
