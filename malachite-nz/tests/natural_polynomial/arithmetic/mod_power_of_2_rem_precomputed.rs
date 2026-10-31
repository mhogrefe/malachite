// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::*;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;

// Checks the result of `mod_power_of_2_rem_precomputed` in every way of calling it against
// `mod_power_of_2_rem`, and returns it.
fn verify(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
    let data = NaturalPolynomial::precompute_mod_power_of_2_rem_data(q, pow);
    let r = p.mod_power_of_2_rem_precomputed(q, pow, &data);
    assert!(r.is_valid());
    assert_eq!(
        p.clone()
            .mod_power_of_2_rem_precomputed(q.clone(), pow, &data),
        r
    );
    assert_eq!(p.clone().mod_power_of_2_rem_precomputed(q, pow, &data), r);
    assert_eq!(p.mod_power_of_2_rem_precomputed(q.clone(), pow, &data), r);
    let mut s = p.clone();
    s.mod_power_of_2_rem_precomputed_assign(q, pow, &data);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_power_of_2_rem_precomputed_assign(q.clone(), pow, &data);
    assert_eq!(s, r);
    assert_eq!(p.mod_power_of_2_rem(q, pow), r);
    r
}

#[test]
fn test_mod_power_of_2_rem_precomputed() {
    let test = |p, q, pow: u64, out: &str| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        assert_eq!(verify(&p, &q, pow).to_string(), out);
    };
    // - modulo 1
    test("0", "0", 0, "0");
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 3, "5");
    // - a non-monic divisor
    test("x^4+7*x+5", "5*x^2+2", 3, "7*x+1");
    // - an exponent of two limbs
    test(
        "x^4+7*x+5",
        "5*x^2+3",
        100,
        "7*x+811296384146066816957890051446",
    );
    // - an exponent of more than two limbs, the dividend is shorter
    test("5", "3*x+2", 200, "5");
    // - an exponent of more than two limbs, a constant divisor
    test("6*x^2+5", "3", 200, "0");
    // - an exponent of more than two limbs, the dividend is longer than twice the divisor, so it is
    //   divided a block at a time
    test(
        "x^4+7*x+5",
        "5*x^2+3",
        200,
        "7*x+1028440348325753776346855739098344065614209916020987414592886",
    );
    // - an exponent of more than two limbs, the quotient is found with the precomputed inverse
    test("x^3+2*x+1", "x^2", 200, "2*x+1");
}

#[test]
fn mod_power_of_2_rem_precomputed_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );
}
