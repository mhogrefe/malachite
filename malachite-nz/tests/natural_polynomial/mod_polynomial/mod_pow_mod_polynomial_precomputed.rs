// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::{
    ModPowModPolynomial, ModPowModPolynomialPrecomputed, ModPowModPolynomialPrecomputedAssign,
};
use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::mod_polynomial::mod_pow_mod_polynomial::*;

// Checks `mod_pow_mod_polynomial_precomputed` in every way of calling it against the naive version,
// and returns its result.
fn verify(
    a: &NaturalPolynomial,
    exp: u64,
    f: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    let data = NaturalPolynomial::precompute_mod_div_rem_data(f, m);
    let r = a.mod_pow_mod_polynomial_precomputed(exp, f, m, &data);
    assert!(r.is_valid());
    assert_eq!(
        a.clone()
            .mod_pow_mod_polynomial_precomputed(exp, f.clone(), m.clone(), &data),
        r
    );
    assert_eq!(
        a.clone()
            .mod_pow_mod_polynomial_precomputed(exp, f, m, &data),
        r
    );
    assert_eq!(
        a.mod_pow_mod_polynomial_precomputed(exp, f.clone(), m, &data),
        r
    );
    assert_eq!(
        a.mod_pow_mod_polynomial_precomputed(exp, f, m.clone(), &data),
        r
    );
    let mut c = a.clone();
    c.mod_pow_mod_polynomial_precomputed_assign(exp, f, m, &data);
    assert_eq!(c, r);
    let mut c = a.clone();
    c.mod_pow_mod_polynomial_precomputed_assign(exp, f.clone(), m.clone(), &data);
    assert_eq!(c, r);
    assert_eq!(mod_pow_mod_polynomial_naive(a, exp, f, m), r);
    assert_eq!(a.mod_pow_mod_polynomial(exp, f, m), r);
    assert!(r.len() < f.len() || r == NaturalPolynomial::ZERO);
    r
}

#[test]
fn test_mod_pow_mod_polynomial_precomputed() {
    let test = |a, exp, f, m: &str, out| {
        let a = NaturalPolynomial::from_str(a).unwrap();
        let f = NaturalPolynomial::from_str(f).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(verify(&a, exp, &f, &m).to_string(), out);
    };
    // - modulo 1
    test("0", 5, "0", "1", "0");
    // - a constant modulus
    test("0", 5, "3", "7", "0");
    // - a modulus of degree 1
    test("4", 5, "2*x+3", "7", "2");
    // - exponent 0
    test("3*x^2+x+5", 0, "x^3+2*x+1", "7", "1");
    // - exponent 0, base 0
    test("0", 0, "x^3+2*x+1", "7", "1");
    // - exponent 1
    test("3*x^2+x+5", 1, "x^3+2*x+1", "7", "3*x^2+x+5");
    // - exponent 2
    test("3*x^2+x+5", 2, "x^3+2*x+1", "7", "6*x^2+3*x+5");
    // - base 0
    test("0", 3, "x^3+2*x+1", "7", "0");
    // - binary exponentiation
    test("3*x^2+x+5", 10, "x^3+2*x+1", "7", "2*x+2");
    // - a non-monic modulus
    test("2*x+3", 5, "3*x^2+x+5", "7", "6*x");
    // - a composite modulus
    test("3*x+1", 7, "x^2+x+1", "6", "3*x+1");
    // - a base of x, exponent 2
    test("x", 2, "x^3+2*x+1", "7", "x^2");
    // - a base of x by the sliding window, the smallest exponent
    test("x", 3, "x^3+2*x+1", "7", "5*x+6");
    // - a base of x by the sliding window
    test("x", 100, "x^3+2*x+1", "7", "x^2+3*x+5");
    // - a base of x by the sliding window, with a window of more than 1
    test(
        "x",
        1000,
        "x^9+3*x^2+1",
        "7",
        "2*x^8+5*x^7+4*x^6+x^5+6*x^4+4*x^2+6*x+6",
    );
    // - a modulus of two limbs
    test(
        "3*x^2+x+5",
        10,
        "x^3+2*x+1",
        "1267650600228229401496703205653",
        "4988221*x^2+1267650600228229401496700913022*x+11402463",
    );
    // - a modulus of two limbs, a base of x
    test(
        "x",
        100,
        "x^3+2*x+1",
        "1267650600228229401496703205653",
        "43223594001800766*x^2+1267650600228171584964975050260*x+1267650600228194302167400019797",
    );
    // - a modulus of more than two limbs
    test(
        "3*x^2+x+5",
        10,
        "x^3+2*x+1",
        "1361129467683753853853498429727072845827",
        "4988221*x^2+1361129467683753853853498429727070553196*x+11402463",
    );
    // - a modulus of more than two limbs, a base of x
    test(
        "x",
        1000,
        "x^9+3*x^2+1",
        "1361129467683753853853498429727072845827",
        "1316315150229107988176480800972021526070*x^8+706775857301129022747808936442182099175*x^7+\
        1125841426374980645490231732787632955227*x^6+1242597189720615312475433075492548301829*x^5+\
        767941053853099998707785099080078013192*x^4+625431240386815322902187053871082714847*x^3+71\
        3960284584825714354858573326022440152*x^2+435833392520670687263146780329376187103*x+187228\
        158988258607316801023372260654950",
    );
}

#[test]
#[should_panic]
fn mod_pow_mod_polynomial_precomputed_fail() {
    let f = NaturalPolynomial::from_str("x^2+1").unwrap();
    let data = NaturalPolynomial::precompute_mod_div_rem_data(&f, &Natural::from(7u32));
    let _ = NaturalPolynomial::from_str("x^2")
        .unwrap()
        .mod_pow_mod_polynomial_precomputed(3, f, Natural::from(7u32), &data);
}

#[test]
fn mod_pow_mod_polynomial_precomputed_properties() {
    natural_polynomial_unsigned_natural_polynomial_natural_quadruple_gen_var_1().test_properties(
        |(a, exp, f, m)| {
            verify(&a, exp, &f, &m);
        },
    );
}
