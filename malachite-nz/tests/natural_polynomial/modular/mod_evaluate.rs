// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::Evaluate;
use malachite_base::polynomial::modular::traits::{ModEvaluate, ModPowerOf2Evaluate};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_natural_triple_gen_var_1,
    natural_polynomial_natural_unsigned_triple_gen_var_1,
};
use malachite_nz::test_util::natural_polynomial::modular::mod_evaluate::*;

#[test]
fn test_mod_evaluate() {
    let test = |s, x, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let x = Natural::from_str(x).unwrap();
        let m = Natural::from_str(m).unwrap();
        let y = (&p).mod_evaluate(&x, &m);
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);
        assert_eq!((&p).mod_evaluate(&x, m.clone()), y);
        assert_eq!((&p).mod_evaluate(x.clone(), &m), y);
        assert_eq!((&p).mod_evaluate(x.clone(), m.clone()), y);
        assert_eq!(p.clone().mod_evaluate(&x, &m), y);
        assert_eq!(p.clone().mod_evaluate(&x, m.clone()), y);
        assert_eq!(p.clone().mod_evaluate(x.clone(), &m), y);
        assert_eq!(p.clone().mod_evaluate(x.clone(), m.clone()), y);
        assert_eq!(mod_evaluate_naive(&p, &x, &m), y);
    };
    test("0", "0", "1", "0");
    test("0", "5", "7", "0");
    test("7", "0", "11", "7");
    test("7", "5", "11", "7");
    test("x", "5", "7", "5");
    test("5*x^2+3*x+7", "6", "11", "7");
    test("5*x^2+3*x+7", "0", "11", "7");
    test("5*x^2+3*x+7", "1", "11", "4");
    test("5*x^2+3*x+7", "10", "11", "9");
    test("x^2+x+1", "1", "2", "1");
    test("x^100+1", "3", "1000000007", "886041712");
    test("x^2+1", "2", "5", "0");
    test("6*x^3+5*x+4", "6", "7", "0");
    test(
        "18446744073709551615*x^2+1",
        "18446744073709551615",
        "18446744073709551616",
        "0",
    );
    test(
        "123456789012345678901234567890*x+1",
        "98765432109876543210",
        "1000000000000000000000000000057",
        "496570641542447803630239294147",
    );
    test("x^5+x^4+x^3+x^2+x+1", "12", "13", "0");
}

#[test]
#[should_panic]
fn mod_evaluate_fail_1() {
    // A coefficient is not reduced.
    (&NaturalPolynomial::from_str("11*x+1").unwrap())
        .mod_evaluate(&Natural::ONE, &Natural::from(11u32));
}

#[test]
#[should_panic]
fn mod_evaluate_fail_2() {
    // The value is not reduced.
    (&NaturalPolynomial::from_str("x+1").unwrap())
        .mod_evaluate(&Natural::from(11u32), &Natural::from(11u32));
}

#[test]
#[should_panic]
fn mod_evaluate_fail_3() {
    // Nothing is reduced mod 0.
    (&NaturalPolynomial::ZERO).mod_evaluate(&Natural::ZERO, &Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_evaluate_fail_4() {
    NaturalPolynomial::from_str("11*x+1")
        .unwrap()
        .mod_evaluate(Natural::ONE, Natural::from(11u32));
}

#[test]
fn mod_evaluate_properties() {
    natural_polynomial_natural_natural_triple_gen_var_1().test_properties(|(p, x, m)| {
        let y = (&p).mod_evaluate(&x, &m);
        assert!(y.is_valid());
        assert!(y < m);
        assert_eq!((&p).mod_evaluate(x.clone(), m.clone()), y);
        assert_eq!(p.clone().mod_evaluate(&x, &m), y);
        assert_eq!(p.clone().mod_evaluate(x.clone(), m.clone()), y);
        assert_eq!(mod_evaluate_naive(&p, &x, &m), y);
        assert_eq!((&p).evaluate(&x) % &m, y);

        // Everything reduced mod m is reduced mod 2m, and the value mod 2m reduces to the value mod
        // m.
        let m_2 = &m << 1u32;
        assert_eq!((&p).mod_evaluate(&x, &m_2) % &m, y);

        // p(0) is the constant term, and p(1) the sum of the coefficients, mod m.
        assert_eq!((&p).mod_evaluate(Natural::ZERO, &m), *p.coefficient(0));
        if m > 1u32 {
            assert_eq!(
                (&p).mod_evaluate(Natural::ONE, &m),
                p.coefficients_asc().iter().sum::<Natural>() % &m
            );
        }
    });

    // Modulo a power of 2, mod_evaluate agrees with mod_power_of_2_evaluate.
    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, x, pow)| {
        assert_eq!(
            (&p).mod_evaluate(&x, Natural::power_of_2(pow)),
            (&p).mod_power_of_2_evaluate(&x, pow)
        );
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>().test_properties(|(p, x, m)| {
        // The `u64` and `Natural` evaluations agree.
        assert_eq!(
            NaturalPolynomial::from(p.clone()).mod_evaluate(Natural::from(x), Natural::from(m)),
            p.mod_evaluate(x, m)
        );
    });
}
