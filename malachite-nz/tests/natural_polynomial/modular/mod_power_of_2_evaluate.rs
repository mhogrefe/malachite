// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::modular::traits::{ModPowerOf2, ModPowerOf2IsReduced};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::Evaluate;
use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_natural_unsigned_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::modular::mod_power_of_2_evaluate::*;

#[test]
fn test_mod_power_of_2_evaluate() {
    let test = |s, x, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let x = Natural::from_str(x).unwrap();
        let y = (&p).mod_power_of_2_evaluate(&x, pow);
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);
        assert_eq!((&p).mod_power_of_2_evaluate(x.clone(), pow), y);
        assert_eq!(p.clone().mod_power_of_2_evaluate(&x, pow), y);
        assert_eq!(p.clone().mod_power_of_2_evaluate(x.clone(), pow), y);
        assert_eq!(mod_power_of_2_evaluate_naive(&p, &x, pow), y);
    };
    test("0", "0", 0, "0");
    test("0", "5", 3, "0");
    test("7", "0", 3, "7");
    test("7", "5", 3, "7");
    test("x", "5", 3, "5");
    test("5*x^2+3*x+7", "6", 4, "13");
    test("5*x^2+3*x+7", "0", 4, "7");
    test("5*x^2+3*x+7", "1", 4, "15");
    test("5*x^2+3*x+7", "15", 4, "9");
    test("x^2+x+1", "1", 1, "1");
    test("x^3+1", "1", 2, "2");
    test("x^100+1", "3", 8, "210");
    test("255*x^3+255*x+255", "255", 8, "1");
    test(
        "18446744073709551615*x^2+1",
        "18446744073709551615",
        64,
        "0",
    );
    test(
        "18446744073709551615*x^2+1",
        "18446744073709551615",
        100,
        "55340232221128654848",
    );
    test(
        "123456789012345678901234567890*x+1",
        "98765432109876543210",
        128,
        "209858559491276873124708178321624481781",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_fail_1() {
    // A coefficient is not reduced.
    (&NaturalPolynomial::from_str("16*x+1").unwrap()).mod_power_of_2_evaluate(&Natural::ONE, 4);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_fail_2() {
    // The value is not reduced.
    (&NaturalPolynomial::from_str("x+1").unwrap())
        .mod_power_of_2_evaluate(&Natural::from(16u32), 4);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_fail_3() {
    NaturalPolynomial::from_str("16*x+1")
        .unwrap()
        .mod_power_of_2_evaluate(Natural::ONE, 4);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_fail_4() {
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_evaluate(Natural::from(16u32), 4);
}

#[test]
fn mod_power_of_2_evaluate_properties() {
    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, x, pow)| {
        let y = (&p).mod_power_of_2_evaluate(&x, pow);
        assert!(y.is_valid());
        assert!(y.mod_power_of_2_is_reduced(pow));
        assert_eq!((&p).mod_power_of_2_evaluate(x.clone(), pow), y);
        assert_eq!(p.clone().mod_power_of_2_evaluate(&x, pow), y);
        assert_eq!(p.clone().mod_power_of_2_evaluate(x.clone(), pow), y);
        assert_eq!(mod_power_of_2_evaluate_naive(&p, &x, pow), y);
        assert_eq!((&p).evaluate(&x).mod_power_of_2(pow), y);

        // Reducing further agrees with evaluating the reduced polynomial at the reduced value.
        for smaller in [0, pow >> 1, pow.saturating_sub(1)] {
            assert_eq!(
                (&y).mod_power_of_2(smaller),
                (&p).mod_power_of_2(smaller)
                    .mod_power_of_2_evaluate((&x).mod_power_of_2(smaller), smaller)
            );
        }

        // p(0) is the constant term, and p(1) the sum of the coefficients, mod 2^pow.
        assert_eq!(
            (&p).mod_power_of_2_evaluate(Natural::ZERO, pow),
            *p.coefficient(0)
        );
        if pow != 0 {
            assert_eq!(
                (&p).mod_power_of_2_evaluate(Natural::ONE, pow),
                p.coefficients_asc()
                    .iter()
                    .sum::<Natural>()
                    .mod_power_of_2(pow)
            );
        }
        // Everything is 0 mod 2^0.
        if pow == 0 {
            assert_eq!(y, 0u32);
        }
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().test_properties(
        |(p, x, pow)| {
            // The `u64` and `Natural` evaluations agree.
            assert_eq!(
                NaturalPolynomial::from(p.clone()).mod_power_of_2_evaluate(Natural::from(x), pow),
                p.mod_power_of_2_evaluate(x, pow)
            );
        },
    );
}
