// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::modular::traits::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;

// Checks the result of `mod_quotient_precomputed` in every way of calling it against
// `mod_quotient`, and returns it.
fn verify(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    let data = NaturalPolynomial::precompute_mod_quotient_data(q, m);
    let r = p.mod_quotient_precomputed(q, m, &data);
    assert!(r.is_valid());
    assert_eq!(
        p.clone()
            .mod_quotient_precomputed(q.clone(), m.clone(), &data),
        r
    );
    assert_eq!(p.clone().mod_quotient_precomputed(q, m, &data), r);
    assert_eq!(p.mod_quotient_precomputed(q.clone(), m, &data), r);
    assert_eq!(p.mod_quotient_precomputed(q, m.clone(), &data), r);
    let mut s = p.clone();
    s.mod_quotient_precomputed_assign(q, m, &data);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_quotient_precomputed_assign(q.clone(), m.clone(), &data);
    assert_eq!(s, r);
    assert_eq!(p.mod_quotient(q, m), r);
    r
}

#[test]
fn test_mod_quotient_precomputed() {
    let test = |p, q, m: &str, out: &str| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(verify(&p, &q, &m).to_string(), out);
    };
    // - modulo 1
    test("0", "0", "1", "0");
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", "7", "0");
    // - a constant divisor
    test("6*x^2+5", "3", "7", "2*x^2+4");
    // - a non-monic divisor
    test("x^5+3*x^4+2*x+1", "3*x^2+x+1", "7", "5*x^3+4*x^2+4*x+2");
    // - a modulus of two limbs
    test(
        "x^4+3*x+1",
        "5*x^2+7",
        "1267650600228229401496703205653",
        "760590360136937640898021923392*x^2+963414456173454345137494436296",
    );
    // - a modulus of more than two limbs, the dividend is shorter
    test(
        "5",
        "3*x+2",
        "1361129467683753853853498429727072845827",
        "0",
    );
    // - a modulus of more than two limbs, a constant divisor
    test(
        "6*x^2+5",
        "3",
        "1361129467683753853853498429727072845827",
        "2*x^2+453709822561251284617832809909024281944",
    );
    // - a modulus of more than two limbs, the dividend is longer than twice the divisor, so it is
    //   divided a block at a time
    test(
        "x^5+3*x^4+2*x+1",
        "3*x^2+x+1",
        "1361129467683753853853498429727072845827",
        "907419645122502569235665619818048563885*x^3+151236607520417094872610936636341427315*x^2+10\
        0824405013611396581740624424227618209*x+1277109130172411023368714576040216497319",
    );
    // - a modulus of more than two limbs, the quotient is found with the precomputed inverse
    test(
        "x^3+2*x+1",
        "x^2",
        "1361129467683753853853498429727072845827",
        "x",
    );
}

#[test]
fn mod_quotient_precomputed_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_2().test_properties(
        |(p, q, m)| {
            verify(&p, &q, &m);
        },
    );
}
