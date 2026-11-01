// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul_mod_polynomial::*;

// Checks `mod_mul_mod_polynomial_precomputed` in every way of calling it against the naive version,
// and returns its result.
fn verify(
    a: &NaturalPolynomial,
    b: &NaturalPolynomial,
    f: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    let data = NaturalPolynomial::precompute_mod_div_rem_data(f, m);
    let r = a.mod_mul_mod_polynomial_precomputed(b, f, m, &data);
    assert!(r.is_valid());
    assert_eq!(
        a.clone()
            .mod_mul_mod_polynomial_precomputed(b.clone(), f.clone(), m.clone(), &data),
        r
    );
    assert_eq!(
        a.clone()
            .mod_mul_mod_polynomial_precomputed(b, f.clone(), m, &data),
        r
    );
    assert_eq!(
        a.clone()
            .mod_mul_mod_polynomial_precomputed(b.clone(), f, m, &data),
        r
    );
    assert_eq!(
        a.mod_mul_mod_polynomial_precomputed(b.clone(), f.clone(), m, &data),
        r
    );
    assert_eq!(
        a.mod_mul_mod_polynomial_precomputed(b, f, m.clone(), &data),
        r
    );
    let mut c = a.clone();
    c.mod_mul_mod_polynomial_precomputed_assign(b, f, m, &data);
    assert_eq!(c, r);
    let mut c = a.clone();
    c.mod_mul_mod_polynomial_precomputed_assign(b.clone(), f.clone(), m.clone(), &data);
    assert_eq!(c, r);
    assert_eq!(mod_mul_mod_polynomial_naive(a, b, f, m), r);
    assert_eq!(a.mod_mul_mod_polynomial(b, f, m), r);
    assert!(r.len() < f.len() || r == NaturalPolynomial::ZERO);
    r
}

#[test]
fn test_mod_mul_mod_polynomial_precomputed() {
    let test = |a, b, f, m: &str, out| {
        let a = NaturalPolynomial::from_str(a).unwrap();
        let b = NaturalPolynomial::from_str(b).unwrap();
        let f = NaturalPolynomial::from_str(f).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(verify(&a, &b, &f, &m).to_string(), out);
    };
    // - modulo 1
    test("0", "0", "0", "1", "0");
    // - a constant modulus
    test("0", "0", "3", "7", "0");
    // - the first factor is 0
    test("0", "x+1", "x^2+1", "7", "0");
    // - the product is shorter than f
    test("x+2", "3", "x^2+1", "7", "3*x+6");
    // - a product reduced modulo f
    test("3*x^2+x+5", "x^2+4*x+2", "x^3+2*x+1", "7", "2*x^2+4");
    // - a non-monic f
    test("2*x+3", "4*x+1", "3*x^2+x+5", "7", "2*x+6");
    // - a composite modulus
    test("3*x+1", "5*x+2", "x^2+x+1", "6", "2*x+5");
    // - a modulus of two limbs
    test(
        "3*x^2+x+5",
        "x^2+4*x+2",
        "x^3+2*x+1",
        "1267650600228229401496703205653",
        "9*x^2+1267650600228229401496703205646*x+1267650600228229401496703205650",
    );
    // - a modulus of more than two limbs
    test(
        "3*x^2+x+5",
        "x^2+4*x+2",
        "x^3+2*x+1",
        "1361129467683753853853498429727072845827",
        "9*x^2+1361129467683753853853498429727072845820*x+1361129467683753853853498429727072845824",
    );
    // - a modulus of more than two limbs, a non-monic f
    test(
        "2*x+3",
        "4*x+1",
        "3*x^2+x+5",
        "1361129467683753853853498429727072845827",
        "907419645122502569235665619818048563896*x+453709822561251284617832809909024281932",
    );
}

#[test]
#[should_panic]
fn mod_mul_mod_polynomial_precomputed_fail() {
    let f = NaturalPolynomial::from_str("x^2+1").unwrap();
    let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &Natural::from(7u32));
    let _ = NaturalPolynomial::from_str("x^2")
        .unwrap()
        .mod_mul_mod_polynomial_precomputed(
            NaturalPolynomial::from_str("1").unwrap(),
            f,
            Natural::from(7u32),
            &data,
        );
}

#[test]
fn mod_mul_mod_polynomial_precomputed_properties() {
    natural_polynomial_triple_natural_quadruple_gen_var_1().test_properties(|(a, b, f, m)| {
        verify(&a, &b, &f, &m);
    });
}
