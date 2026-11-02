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
    ModDivAssignRemPrecomputed, ModDivRem, ModDivRemPrecomputed, ModQuotientPrecomputed,
    ModRemPrecomputed,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::modular::mod_mul::*;

// Checks the result of `mod_div_rem_precomputed` in every way of calling it against `mod_div_rem`,
// and returns it.
fn verify(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> (NaturalPolynomial, NaturalPolynomial) {
    let data = NaturalPolynomial::precompute_mod_div_rem_data(q, m);
    let r = p.mod_div_rem_precomputed(q, m, &data);
    assert!(r.0.is_valid());
    assert!(r.1.is_valid());
    assert_eq!(
        p.clone()
            .mod_div_rem_precomputed(q.clone(), m.clone(), &data),
        r
    );
    assert_eq!(p.clone().mod_div_rem_precomputed(q, m, &data), r);
    assert_eq!(p.mod_div_rem_precomputed(q.clone(), m, &data), r);
    assert_eq!(p.mod_div_rem_precomputed(q, m.clone(), &data), r);
    let mut s = p.clone();
    assert_eq!(s.mod_div_assign_rem_precomputed(q, m, &data), r.1);
    assert_eq!(s, r.0);
    let mut s = p.clone();
    assert_eq!(
        s.mod_div_assign_rem_precomputed(q.clone(), m.clone(), &data),
        r.1
    );
    assert_eq!(s, r.0);
    assert_eq!(p.mod_div_rem(q, m), r);
    assert_eq!(p.mod_quotient_precomputed(q, m, &data), r.0);
    assert_eq!(p.mod_rem_precomputed(q, m, &data), r.1);
    r
}

#[test]
fn test_mod_div_rem_precomputed() {
    let test = |p, q, m: &str, out: (&str, &str)| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        let (s, t) = verify(&p, &q, &m);
        assert_eq!((s.to_string().as_str(), t.to_string().as_str()), out);
    };
    // - modulo 1
    test("0", "0", "1", ("0", "0"));
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", "7", ("0", "5"));
    // - a constant divisor
    test("6*x^2+5", "3", "7", ("2*x^2+4", "0"));
    // - a non-monic divisor
    test(
        "x^5+3*x^4+2*x+1",
        "3*x^2+x+1",
        "7",
        ("5*x^3+4*x^2+4*x+2", "3*x+6"),
    );
    // - a modulus of two limbs
    test(
        "x^4+3*x+1",
        "5*x^2+7",
        "1267650600228229401496703205653",
        (
            "760590360136937640898021923392*x^2+963414456173454345137494436296",
            "3*x+862002408155195993017758179847",
        ),
    );
    // - a modulus of more than two limbs, the dividend is shorter
    test(
        "5",
        "3*x+2",
        "1361129467683753853853498429727072845827",
        ("0", "5"),
    );
    // - a modulus of more than two limbs, a constant divisor
    test(
        "6*x^2+5",
        "3",
        "1361129467683753853853498429727072845827",
        ("2*x^2+453709822561251284617832809909024281944", "0"),
    );
    // - a modulus of more than two limbs, the dividend is longer than twice the divisor, so it is
    //   divided a block at a time
    test(
        "x^5+3*x^4+2*x+1",
        "3*x^2+x+1",
        "1361129467683753853853498429727072845827",
        (
            "907419645122502569235665619818048563885*x^3+151236607520417094872610936636341427315*x^\
            2+100824405013611396581740624424227618209*x+1277109130172411023368714576040216497319",
            "1344325400181485287756541658989701576128*x+84020337511342830484783853686856348509",
        ),
    );
    // - a modulus of more than two limbs, the quotient is found with the precomputed inverse
    test(
        "x^3+2*x+1",
        "x^2",
        "1361129467683753853853498429727072845827",
        ("x", "2*x+1"),
    );
}

// The kernels, at lengths that reach the block division, against `mod_div_rem`.
#[test]
fn test_mod_div_rem_precomputed_algorithms() {
    for &(p_len, q_len) in &[(10, 5), (9, 5), (6, 5), (300, 150), (300, 2), (500, 30)] {
        for m in [
            "7",
            "18446744073709551557",
            "1267650600228229401496703205653",
            "1361129467683753853853498429727072845827",
        ] {
            let m = Natural::from_str(m).unwrap();
            let p = NaturalPolynomial::from_coefficients_asc(natural_mod_generated_coefficients(
                p_len, &m,
            ));
            let mut ys = natural_mod_generated_coefficients(q_len, &m);
            *ys.last_mut().unwrap() = Natural::ONE;
            let q = NaturalPolynomial::from_coefficients_asc(ys);
            verify(&p, &q, &m);
        }
    }
}

#[test]
#[should_panic]
fn precompute_mod_div_rem_data_fail_1() {
    let _ = NaturalPolynomial::precompute_mod_div_rem_data(
        &NaturalPolynomial::ZERO,
        &Natural::from_str("1361129467683753853853498429727072845827").unwrap(),
    );
}

#[test]
#[should_panic]
fn precompute_mod_div_rem_data_fail_2() {
    let _ = NaturalPolynomial::precompute_mod_div_rem_data(
        &NaturalPolynomial::from_str("3*x+1").unwrap(),
        &Natural::from_str("4083388403051261561560495289181218537481").unwrap(),
    );
}

#[test]
#[should_panic]
fn precompute_mod_div_rem_data_fail_3() {
    let _ = NaturalPolynomial::precompute_mod_div_rem_data(
        &NaturalPolynomial::from_str("2*x+1").unwrap(),
        &Natural::from(4u32),
    );
}

#[test]
#[should_panic]
fn mod_div_rem_precomputed_fail() {
    let m = Natural::from(7u32);
    let q = NaturalPolynomial::from_str("x+1").unwrap();
    let data = NaturalPolynomial::precompute_mod_div_rem_data(&q, &m);
    let _ = NaturalPolynomial::from_str("8*x")
        .unwrap()
        .mod_div_rem_precomputed(q, m, &data);
}

#[test]
fn mod_div_rem_precomputed_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_2().test_properties(
        |(p, q, m)| {
            verify(&p, &q, &m);
        },
    );
}
