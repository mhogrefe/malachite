// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::*;
use malachite_base::polynomial::*;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_compositional_inverse_truncated::*;

fn verify(p: &NaturalPolynomial, len: u64, pow: u64) -> Option<NaturalPolynomial> {
    let r = p.mod_power_of_2_compositional_inverse_truncated(len, pow);
    if let Some(r) = &r {
        assert!(r.is_valid());
        assert!(r.mod_power_of_2_is_reduced(pow));
    }
    assert_eq!(
        p.clone()
            .mod_power_of_2_compositional_inverse_truncated(len, pow),
        r
    );
    assert_eq!(
        mod_power_of_2_compositional_inverse_truncated_naive(p, len, pow),
        r
    );
    assert_eq!(
        mod_power_of_2_compositional_inverse_truncated_newton(p, len, pow),
        r
    );
    if let Some(r_alt) = mod_power_of_2_compositional_inverse_truncated_lagrange(p, len, pow) {
        assert_eq!(Some(r_alt), r);
    }
    if let Some(r_alt) = mod_power_of_2_compositional_inverse_truncated_lagrange_fast(p, len, pow) {
        assert_eq!(Some(r_alt), r);
    }
    r
}

fn show(p: Option<NaturalPolynomial>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

#[test]
fn test_mod_power_of_2_compositional_inverse_truncated() {
    let test = |p, len, pow, out| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        assert_eq!(show(verify(&p, len, pow)), out);
    };
    // - the length is 0
    test("x", 0, 3, "0");
    // - the length is 1
    test("3*x", 1, 3, "0");
    // - the length is 2: the inverse of the coefficient of x
    test("x^2+3*x", 2, 3, "3*x");
    // - the power is 0
    test("0", 5, 0, "0");
    // - a division by an even index is not possible, so Newton's iteration is used
    test("x^2+x", 6, 3, "6*x^5+3*x^4+2*x^3+7*x^2+x");
    // - a longer result
    test(
        "5*x^3+2*x^2+3*x",
        10,
        8,
        "53*x^9+82*x^8+84*x^7+56*x^6+189*x^5+10*x^4+99*x^3+218*x^2+171*x",
    );
    // - several Newton steps
    test(
        "5*x^3+3*x^2+x",
        12,
        5,
        "9*x^11+x^10+11*x^9+3*x^8+10*x^7+26*x^6+8*x^5+4*x^4+13*x^3+29*x^2+x",
    );
    // - a linear polynomial
    test("5*x", 6, 3, "5*x");
    // - a power too large for two limbs
    test(
        "x^3+x^2+3*x",
        5,
        200,
        "424696017184131860659924137802099672728776099865776981620574*x^4+\
        581936411089675490731245531382807856057423306390476417722309*x^3+\
        416613567030108589955323505421782896950200776165909253596653*x^2+\
        1071292029505993517027974728227441735014801995855195223534251*x",
    );
    // - the constant coefficient is not 0
    test("x+1", 5, 3, "None");
    // - the coefficient of x is 0
    test("x^2", 5, 3, "None");
    // - the coefficient of x is even
    test("x^2+2*x", 5, 3, "None");
    // - the polynomial is 0
    test("0", 5, 3, "None");
    // - a power that fits in a limb
    test(
        "x^2+x",
        8,
        64,
        "132*x^7+18446744073709551574*x^6+14*x^5+18446744073709551611*x^4+2*x^3\
        +18446744073709551615*x^2+x",
    );
    // - a power too large for a limb
    test(
        "7*x^3+5*x^2+3*x",
        6,
        65,
        "15444919084221582616*x^5+22352021854289122900*x^4+6224827218288819887*x^3\
        +25962084251887517089*x^2+12297829382473034411*x",
    );
    // - a power of more than one limb
    test(
        "x^3+x^2+x",
        5,
        100,
        "x^3+1267650600228229401496703205375*x^2+x",
    );
}

#[test]
fn mod_power_of_2_compositional_inverse_truncated_properties() {
    natural_polynomial_unsigned_unsigned_triple_gen_var_7().test_properties(|(p, len, pow)| {
        let q = (&p)
            .mod_power_of_2_compositional_inverse_truncated(len, pow)
            .unwrap();
        assert!(q.is_valid());
        assert_eq!(
            p.clone()
                .mod_power_of_2_compositional_inverse_truncated(len, pow),
            Some(q.clone())
        );
        assert_eq!(
            mod_power_of_2_compositional_inverse_truncated_naive(&p, len, pow),
            Some(q.clone())
        );
        assert_eq!(
            mod_power_of_2_compositional_inverse_truncated_newton(&p, len, pow),
            Some(q.clone())
        );
        if let Some(q_alt) = mod_power_of_2_compositional_inverse_truncated_lagrange(&p, len, pow) {
            assert_eq!(q_alt, q);
        }
        if let Some(q_alt) =
            mod_power_of_2_compositional_inverse_truncated_lagrange_fast(&p, len, pow)
        {
            assert_eq!(q_alt, q);
        }
        if len >= 2 {
            let x = NaturalPolynomial::x();
            assert_eq!((&p).mod_power_of_2_compose_truncated(&q, len, pow), x);
            assert_eq!((&q).mod_power_of_2_compose_truncated(&p, len, pow), x);
        }
    });

    natural_polynomial_unsigned_unsigned_triple_gen_var_4().test_properties(|(p, len, pow)| {
        let q = (&p).mod_power_of_2_compositional_inverse_truncated(len, pow);
        let xs = p.coefficients_asc();
        let reversible = pow == 0 || xs.len() >= 2 && xs[0] == 0u32 && xs[1].odd();
        assert_eq!(q.is_some(), reversible);
        assert_eq!(
            mod_power_of_2_compositional_inverse_truncated_naive(&p, len, pow),
            q
        );
    });
}
