// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{
    ModPowerOf2DivAssignRemPrecomputed, ModPowerOf2DivRem, ModPowerOf2DivRemPrecomputed,
    ModPowerOf2QuotientPrecomputed, ModPowerOf2RemPrecomputed,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::modular::mod_power_of_2_mul::*;

// Checks the result of `mod_power_of_2_div_rem_precomputed` in every way of calling it against
// `mod_power_of_2_div_rem`, and returns it.
fn verify(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> (NaturalPolynomial, NaturalPolynomial) {
    let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(q, pow);
    let r = p.mod_power_of_2_div_rem_precomputed(q, pow, &data);
    assert!(r.0.is_valid());
    assert!(r.1.is_valid());
    assert_eq!(
        p.clone()
            .mod_power_of_2_div_rem_precomputed(q.clone(), pow, &data),
        r
    );
    assert_eq!(
        p.clone().mod_power_of_2_div_rem_precomputed(q, pow, &data),
        r
    );
    assert_eq!(
        p.mod_power_of_2_div_rem_precomputed(q.clone(), pow, &data),
        r
    );
    let mut s = p.clone();
    assert_eq!(
        s.mod_power_of_2_div_assign_rem_precomputed(q, pow, &data),
        r.1
    );
    assert_eq!(s, r.0);
    let mut s = p.clone();
    assert_eq!(
        s.mod_power_of_2_div_assign_rem_precomputed(q.clone(), pow, &data),
        r.1
    );
    assert_eq!(s, r.0);
    assert_eq!(p.mod_power_of_2_div_rem(q, pow), r);
    assert_eq!(p.mod_power_of_2_quotient_precomputed(q, pow, &data), r.0);
    assert_eq!(p.mod_power_of_2_rem_precomputed(q, pow, &data), r.1);
    r
}

#[test]
fn test_mod_power_of_2_div_rem_precomputed() {
    let test = |p, q, pow: u64, out: (&str, &str)| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let (s, t) = verify(&p, &q, pow);
        assert_eq!((s.to_string().as_str(), t.to_string().as_str()), out);
    };
    // - modulo 1
    test("0", "0", 0, ("0", "0"));
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 3, ("0", "5"));
    // - a non-monic divisor
    test("x^4+7*x+5", "5*x^2+2", 3, ("5*x^2+6", "7*x+1"));
    // - an exponent of two limbs
    test(
        "x^4+7*x+5",
        "5*x^2+3",
        100,
        (
            "1014120480182583521197362564301*x^2+152118072027387528179604384645",
            "7*x+811296384146066816957890051446",
        ),
    );
    // - an exponent of more than two limbs, the dividend is shorter
    test("5", "3*x+2", 200, ("0", "5"));
    // - an exponent of more than two limbs, a constant divisor
    test(
        "6*x^2+5",
        "3",
        200,
        (
            "2*x^2+535646014752996758513987364113720867507400997927597611767127",
            "0",
        ),
    );
    // - an exponent of more than two limbs, the dividend is longer than twice the divisor, so it is
    //   divided a block at a time
    test(
        "x^4+7*x+5",
        "5*x^2+3",
        200,
        (
            "1285550435407192220433569673872930082017762395026234268241101*x^2+19283256531107883306\
            5035451080939512302664359253935140236165",
            "7*x+1028440348325753776346855739098344065614209916020987414592886",
        ),
    );
    // - an exponent of more than two limbs, the quotient is found with the precomputed inverse
    test("x^3+2*x+1", "x^2", 200, ("x", "2*x+1"));
}

// The kernels, at lengths that reach the block division, against `mod_power_of_2_div_rem`.
#[test]
fn test_mod_power_of_2_div_rem_precomputed_algorithms() {
    for &(p_len, q_len) in &[(10, 5), (9, 5), (6, 5), (500, 250), (300, 2), (500, 30)] {
        for pow in [3, 64, 128, 140, 400] {
            let p = NaturalPolynomial::from_coefficients_asc(
                natural_mod_power_of_2_generated_coefficients(p_len, pow),
            );
            let mut ys = natural_mod_power_of_2_generated_coefficients(q_len, pow);
            *ys.last_mut().unwrap() = Natural::ONE;
            let q = NaturalPolynomial::from_coefficients_asc(ys);
            verify(&p, &q, pow);
        }
    }
}

#[test]
#[should_panic]
fn precompute_mod_power_of_2_div_rem_data_fail_1() {
    let _ =
        NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&NaturalPolynomial::ZERO, 200);
}

#[test]
#[should_panic]
fn precompute_mod_power_of_2_div_rem_data_fail_2() {
    let _ = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(
        &NaturalPolynomial::from_str("6*x^2+3").unwrap(),
        200,
    );
}

#[test]
#[should_panic]
fn precompute_mod_power_of_2_div_rem_data_fail_3() {
    let _ = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(
        &NaturalPolynomial::from_str("2*x+1").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_div_rem_precomputed_fail() {
    let q = NaturalPolynomial::from_str("x+1").unwrap();
    let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&q, 3);
    let _ = NaturalPolynomial::from_str("8*x")
        .unwrap()
        .mod_power_of_2_div_rem_precomputed(q, 3, &data);
}

#[test]
fn mod_power_of_2_div_rem_precomputed_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );
}
