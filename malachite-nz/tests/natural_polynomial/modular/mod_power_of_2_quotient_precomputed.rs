// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::modular::traits::*;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;

// Checks the result of `mod_power_of_2_quotient_precomputed` in every way of calling it against
// `mod_power_of_2_quotient`, and returns it.
fn verify(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
    let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(q, pow);
    let r = p.mod_power_of_2_quotient_precomputed(q, pow, &data);
    assert!(r.is_valid());
    assert_eq!(
        p.clone()
            .mod_power_of_2_quotient_precomputed(q.clone(), pow, &data),
        r
    );
    assert_eq!(
        p.clone().mod_power_of_2_quotient_precomputed(q, pow, &data),
        r
    );
    assert_eq!(
        p.mod_power_of_2_quotient_precomputed(q.clone(), pow, &data),
        r
    );
    let mut s = p.clone();
    s.mod_power_of_2_quotient_precomputed_assign(q, pow, &data);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_power_of_2_quotient_precomputed_assign(q.clone(), pow, &data);
    assert_eq!(s, r);
    assert_eq!(p.mod_power_of_2_quotient(q, pow), r);
    r
}

#[test]
fn test_mod_power_of_2_quotient_precomputed() {
    let test = |p, q, pow: u64, out: &str| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        assert_eq!(verify(&p, &q, pow).to_string(), out);
    };
    // - modulo 1
    test("0", "0", 0, "0");
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 3, "0");
    // - a non-monic divisor
    test("x^4+7*x+5", "5*x^2+2", 3, "5*x^2+6");
    // - an exponent of two limbs
    test(
        "x^4+7*x+5",
        "5*x^2+3",
        100,
        "1014120480182583521197362564301*x^2+152118072027387528179604384645",
    );
    // - an exponent of more than two limbs, the dividend is shorter
    test("5", "3*x+2", 200, "0");
    // - an exponent of more than two limbs, a constant divisor
    test(
        "6*x^2+5",
        "3",
        200,
        "2*x^2+535646014752996758513987364113720867507400997927597611767127",
    );
    // - an exponent of more than two limbs, the dividend is longer than twice the divisor, so it is
    //   divided a block at a time
    test(
        "x^4+7*x+5",
        "5*x^2+3",
        200,
        "1285550435407192220433569673872930082017762395026234268241101*x^2+192832565311078833065035\
        451080939512302664359253935140236165",
    );
    // - an exponent of more than two limbs, the quotient is found with the precomputed inverse
    test("x^3+2*x+1", "x^2", 200, "x");
}

#[test]
fn mod_power_of_2_quotient_precomputed_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );
}
