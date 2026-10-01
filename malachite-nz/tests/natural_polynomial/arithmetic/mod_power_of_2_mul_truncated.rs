// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Mul};
use malachite_base::polynomial::{
    ModPowerOf2MulTruncated, ModPowerOf2MulTruncatedAssign, MulTruncated, Polynomial,
};
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;

#[test]
fn test_mod_power_of_2_mul_truncated() {
    let test = |s, t, len, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let r = (&p).mod_power_of_2_mul_truncated(&q, len, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_power_of_2_mul_truncated(q.clone(), len, pow), r);
        assert_eq!(p.clone().mod_power_of_2_mul_truncated(&q, len, pow), r);
        assert_eq!(
            p.clone().mod_power_of_2_mul_truncated(q.clone(), len, pow),
            r
        );
        let mut s = p.clone();
        s.mod_power_of_2_mul_truncated_assign(&q, len, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_power_of_2_mul_truncated_assign(q.clone(), len, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_power_of_2_mul_truncated_naive(&p, &q, len, pow), r);
    };
    test("x^2+3*x+2", "2*x+5", 0, 4, "0");
    test("x^2+3*x+2", "2*x+5", 2, 4, "3*x+10");
    test("x^2+3*x+2", "2*x+5", 10, 4, "2*x^3+11*x^2+3*x+10");
    // A constant, which the forms taking the other polynomial by value multiply in place.
    test("x^2+3*x+2", "6", 2, 4, "2*x+12");
    // The linear coefficient vanishes, and is trimmed.
    test("x+15", "x+1", 2, 4, "15");
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_fail() {
    NaturalPolynomial::from_str("x+16")
        .unwrap()
        .mod_power_of_2_mul_truncated(NaturalPolynomial::from_str("x+1").unwrap(), 2, 4);
}

#[test]
fn mod_power_of_2_mul_truncated_properties() {
    natural_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1().test_properties(
        |(p, q, len, pow)| {
            let r = (&p).mod_power_of_2_mul_truncated(&q, len, pow);
            assert!(r.is_valid());
            assert!(r.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!((&p).mod_power_of_2_mul_truncated(q.clone(), len, pow), r);
            assert_eq!(p.clone().mod_power_of_2_mul_truncated(&q, len, pow), r);
            assert_eq!(
                p.clone().mod_power_of_2_mul_truncated(q.clone(), len, pow),
                r
            );
            let mut s = p.clone();
            s.mod_power_of_2_mul_truncated_assign(&q, len, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_power_of_2_mul_truncated_assign(q.clone(), len, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            assert_eq!(mod_power_of_2_mul_truncated_naive(&p, &q, len, pow), r);
            assert_eq!((&p).mul_truncated(&q, len).mod_power_of_2(pow), r);
            assert_eq!((&p).mod_power_of_2_mul(&q, pow).truncate(len), r);
            assert_eq!((&q).mod_power_of_2_mul_truncated(&p, len, pow), r);
            assert!(r.len() <= len);
        },
    );
}
