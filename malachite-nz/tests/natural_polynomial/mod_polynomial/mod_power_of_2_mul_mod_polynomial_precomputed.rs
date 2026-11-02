// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2MulModPolynomial;
use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::mod_polynomial::mod_power_of_2_mul_mod_polynomial::*;

// Checks `mod_power_of_2_mul_mod_polynomial_precomputed` in every way of calling it against the
// naive version, and returns its result.
fn verify(
    a: &NaturalPolynomial,
    b: &NaturalPolynomial,
    f: &NaturalPolynomial,
    pow: u64,
) -> NaturalPolynomial {
    let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(f, pow);
    let r = a.mod_power_of_2_mul_mod_polynomial_precomputed(b, f, pow, &data);
    assert!(r.is_valid());
    assert_eq!(
        a.clone()
            .mod_power_of_2_mul_mod_polynomial_precomputed(b.clone(), f.clone(), pow, &data),
        r
    );
    assert_eq!(
        a.clone()
            .mod_power_of_2_mul_mod_polynomial_precomputed(b, f.clone(), pow, &data),
        r
    );
    assert_eq!(
        a.clone()
            .mod_power_of_2_mul_mod_polynomial_precomputed(b.clone(), f, pow, &data),
        r
    );
    assert_eq!(
        a.mod_power_of_2_mul_mod_polynomial_precomputed(b.clone(), f.clone(), pow, &data),
        r
    );
    let mut c = a.clone();
    c.mod_power_of_2_mul_mod_polynomial_precomputed_assign(b, f, pow, &data);
    assert_eq!(c, r);
    let mut c = a.clone();
    c.mod_power_of_2_mul_mod_polynomial_precomputed_assign(b.clone(), f.clone(), pow, &data);
    assert_eq!(c, r);
    assert_eq!(mod_power_of_2_mul_mod_polynomial_naive(a, b, f, pow), r);
    assert_eq!(a.mod_power_of_2_mul_mod_polynomial(b, f, pow), r);
    assert!(r.len() < f.len() || r == NaturalPolynomial::ZERO);
    r
}

#[test]
fn test_mod_power_of_2_mul_mod_polynomial_precomputed() {
    let test = |a, b, f, pow: u64, out| {
        let a = NaturalPolynomial::from_str(a).unwrap();
        let b = NaturalPolynomial::from_str(b).unwrap();
        let f = NaturalPolynomial::from_str(f).unwrap();

        assert_eq!(verify(&a, &b, &f, pow).to_string(), out);
    };
    // - modulo 1
    test("0", "0", "0", 0, "0");
    // - a constant modulus
    test("0", "0", "3", 3, "0");
    // - the first factor is 0
    test("0", "x+1", "x^2+1", 3, "0");
    // - the product is shorter than f
    test("x+2", "3", "x^2+1", 3, "3*x+6");
    // - a product reduced modulo f
    test("3*x^2+x+5", "x^2+4*x+2", "x^3+2*x+1", 3, "x^2+x+5");
    // - a non-monic f
    test("2*x+3", "4*x+1", "3*x^2+x+5", 3, "6*x+3");
    // - the full width of a byte
    test("255*x+1", "253*x+3", "x^2+1", 8, "250*x");
    // - an exponent of two limbs
    test(
        "3*x^2+x+5",
        "x^2+4*x+2",
        "x^3+2*x+1",
        100,
        "9*x^2+1267650600228229401496703205369*x+1267650600228229401496703205373",
    );
    // - an exponent of more than two limbs
    test(
        "3*x^2+x+5",
        "x^2+4*x+2",
        "x^3+2*x+1",
        200,
        "9*x^2+1606938044258990275541962092341162602522202993782792835301369*x+1606938044258990275\
        541962092341162602522202993782792835301373",
    );
    // - an exponent of more than two limbs, a non-monic f
    test(
        "2*x+3",
        "4*x+1",
        "3*x^2+x+5",
        200,
        "1071292029505993517027974728227441735014801995855195223534262*x+5356460147529967585139873\
        64113720867507400997927597611767115",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_mod_polynomial_precomputed_fail() {
    let f = NaturalPolynomial::from_str("x^2+1").unwrap();
    let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    let _ = NaturalPolynomial::from_str("x^2")
        .unwrap()
        .mod_power_of_2_mul_mod_polynomial_precomputed(
            NaturalPolynomial::from_str("1").unwrap(),
            f,
            3,
            &data,
        );
}

#[test]
fn mod_power_of_2_mul_mod_polynomial_precomputed_properties() {
    natural_polynomial_triple_unsigned_quadruple_gen_var_1().test_properties(|(a, b, f, pow)| {
        verify(&a, &b, &f, pow);
    });
}
