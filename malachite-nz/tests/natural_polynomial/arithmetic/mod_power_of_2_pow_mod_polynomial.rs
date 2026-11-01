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
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_pow_mod_polynomial::*;

// Checks `mod_power_of_2_pow_mod_polynomial` in every way of calling it against the naive version,
// and returns its result.
fn verify(a: &NaturalPolynomial, exp: u64, f: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
    let r = a.mod_power_of_2_pow_mod_polynomial(exp, f, pow);
    assert!(r.is_valid());
    assert_eq!(
        a.clone()
            .mod_power_of_2_pow_mod_polynomial(exp, f.clone(), pow),
        r
    );
    assert_eq!(a.clone().mod_power_of_2_pow_mod_polynomial(exp, f, pow), r);
    assert_eq!(a.mod_power_of_2_pow_mod_polynomial(exp, f.clone(), pow), r);
    let mut c = a.clone();
    c.mod_power_of_2_pow_mod_polynomial_assign(exp, f, pow);
    assert_eq!(c, r);
    let mut c = a.clone();
    c.mod_power_of_2_pow_mod_polynomial_assign(exp, f.clone(), pow);
    assert_eq!(c, r);
    assert_eq!(mod_power_of_2_pow_mod_polynomial_naive(a, exp, f, pow), r);
    assert!(r.len() < f.len() || r == NaturalPolynomial::ZERO);
    r
}

#[test]
fn test_mod_power_of_2_pow_mod_polynomial() {
    let test = |a, exp, f, pow: u64, out| {
        let a = NaturalPolynomial::from_str(a).unwrap();
        let f = NaturalPolynomial::from_str(f).unwrap();

        assert_eq!(verify(&a, exp, &f, pow).to_string(), out);
    };
    // - modulo 1
    test("0", 5, "0", 0, "0");
    // - a constant modulus
    test("0", 5, "3", 3, "0");
    // - a modulus of degree 1
    test("5", 5, "3*x+1", 3, "5");
    // - exponent 0
    test("3*x^2+x+5", 0, "x^3+2*x+1", 3, "1");
    // - exponent 0, base 0
    test("0", 0, "x^3+2*x+1", 3, "1");
    // - exponent 1
    test("3*x^2+x+5", 1, "x^3+2*x+1", 3, "3*x^2+x+5");
    // - exponent 2
    test("3*x^2+x+5", 2, "x^3+2*x+1", 3, "5*x^2+5*x+3");
    // - base 0
    test("0", 3, "x^3+2*x+1", 3, "0");
    // - binary exponentiation
    test("3*x^2+x+5", 10, "x^3+2*x+1", 3, "5*x^2+x+7");
    // - a non-monic modulus
    test("2*x+3", 5, "3*x^2+x+5", 3, "2*x+3");
    // - the full width of a byte
    test("255*x+1", 7, "x^2+1", 8, "8*x+8");
    // - a base of x, exponent 2
    test("x", 2, "x^3+2*x+1", 3, "x^2");
    // - a base of x by the sliding window, the smallest exponent
    test("x", 3, "x^3+2*x+1", 3, "6*x+7");
    // - a base of x by the sliding window
    test("x", 100, "x^3+2*x+1", 3, "6*x^2+7*x");
    // - a base of x by the sliding window, with a window of more than 1
    test("x", 1000, "x^9+3*x^2+1", 3, "4*x^8+2*x^6+3*x^5+4*x^2+4*x+1");
    // - an exponent of two limbs
    test(
        "3*x^2+x+5",
        10,
        "x^3+2*x+1",
        100,
        "4988221*x^2+1267650600228229401496700912745*x+11402463",
    );
    // - an exponent of two limbs, a base of x
    test(
        "x",
        100,
        "x^3+2*x+1",
        100,
        "43223594001800766*x^2+1267650600228171584964975049983*x+1267650600228194302167400019520",
    );
    // - an exponent of more than two limbs
    test(
        "3*x^2+x+5",
        10,
        "x^3+2*x+1",
        200,
        "4988221*x^2+1606938044258990275541962092341162602522202993782792833008745*x+11402463",
    );
    // - an exponent of more than two limbs, a base of x
    test(
        "x",
        1000,
        "x^9+3*x^2+1",
        200,
        "495278115737376501987423294967500010668085950863497293357620*x^8+897082421341558327820304\
        124672045315356906728274845648447600*x^7+1522673914603053728748769336351122496551363492784\
        944309636074*x^6+492190090870198204216719755504970681500051890123705653047219*x^5+47600711\
        3330585098318162801897872109449574647918023712086792*x^4+138931790318355820649493385964611\
        0997576529010316980487398416*x^3+126569821807400970144267821274868023384057880147028872729\
        3916*x^2+621444613447840797383394803679145620651605175430460520297556*x+547315465228093567\
        656250202755851695476458981719222082189449",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_mod_polynomial_fail_1() {
    let _ = NaturalPolynomial::from_str("x^2")
        .unwrap()
        .mod_power_of_2_pow_mod_polynomial(3, NaturalPolynomial::from_str("x^2+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_mod_polynomial_fail_2() {
    let _ =
        NaturalPolynomial::ZERO.mod_power_of_2_pow_mod_polynomial(3, NaturalPolynomial::ZERO, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_pow_mod_polynomial_fail_3() {
    let _ = NaturalPolynomial::from_str("1")
        .unwrap()
        .mod_power_of_2_pow_mod_polynomial(3, NaturalPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
fn mod_power_of_2_pow_mod_polynomial_properties() {
    natural_polynomial_unsigned_natural_polynomial_unsigned_quadruple_gen_var_1().test_properties(
        |(a, exp, f, pow)| {
            verify(&a, exp, &f, pow);
        },
    );
}
