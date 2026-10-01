// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul::*;

#[test]
fn test_mod_power_of_2_mul() {
    let test = |s, t, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let r = (&p).mod_power_of_2_mul(&q, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_power_of_2_mul(q.clone(), pow), r);
        assert_eq!(p.clone().mod_power_of_2_mul(&q, pow), r);
        assert_eq!(p.clone().mod_power_of_2_mul(q.clone(), pow), r);
        let mut s = p.clone();
        s.mod_power_of_2_mul_assign(&q, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_power_of_2_mul_assign(q.clone(), pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_power_of_2_mul_naive(&p, &q, pow), r);
    };
    test("0", "0", 0, "0");
    test("0", "x+1", 4, "0");
    test("x^2+3*x+2", "2*x+5", 4, "2*x^3+11*x^2+3*x+10");
    test("x^2+3*x+2", "2*x+5", 100, "2*x^3+11*x^2+19*x+10");
    // A constant, which the forms taking the other polynomial by value multiply in place.
    test("x^2+3*x+2", "6", 4, "6*x^2+2*x+12");
    // The leading coefficient vanishes, so the degree drops.
    test("8*x+1", "2*x+1", 4, "10*x+1");
    // Every coefficient vanishes.
    test("2*x+2", "8", 4, "0");
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_fail_1() {
    NaturalPolynomial::from_str("x+16")
        .unwrap()
        .mod_power_of_2_mul(NaturalPolynomial::from_str("x+1").unwrap(), 4);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_fail_2() {
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_mul(NaturalPolynomial::from_str("16*x+1").unwrap(), 4);
}

#[test]
fn mod_power_of_2_mul_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, pow)| {
            let r = (&p).mod_power_of_2_mul(&q, pow);
            assert!(r.is_valid());
            assert!(r.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!((&p).mod_power_of_2_mul(q.clone(), pow), r);
            assert_eq!(p.clone().mod_power_of_2_mul(&q, pow), r);
            assert_eq!(p.clone().mod_power_of_2_mul(q.clone(), pow), r);
            let mut s = p.clone();
            s.mod_power_of_2_mul_assign(&q, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_power_of_2_mul_assign(q.clone(), pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            assert_eq!(mod_power_of_2_mul_naive(&p, &q, pow), r);
            assert_eq!((&p * &q).mod_power_of_2(pow), r);
            // Multiplication is commutative.
            assert_eq!((&q).mod_power_of_2_mul(&p, pow), r);
            if pow == 0 {
                assert_eq!(r, NaturalPolynomial::ZERO);
            }
        },
    );
}
