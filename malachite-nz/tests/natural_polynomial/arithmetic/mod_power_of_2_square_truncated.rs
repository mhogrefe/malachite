// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Square,
};
use malachite_base::polynomial::{
    ModPowerOf2MulTruncated, ModPowerOf2SquareTruncated, ModPowerOf2SquareTruncatedAssign,
    Polynomial, SquareTruncated,
};
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_square_truncated::*;

#[test]
fn test_mod_power_of_2_square_truncated() {
    let test = |s, len, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let r = (&p).mod_power_of_2_square_truncated(len, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_square_truncated(len, pow), r);
        let mut s = p.clone();
        s.mod_power_of_2_square_truncated_assign(len, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_power_of_2_square_truncated_naive(&p, len, pow), r);
    };
    test("x^2+3*x+2", 0, 3, "0");
    test("x^2+3*x+2", 3, 3, "5*x^2+4*x+4");
    test("x^2+3*x+2", 10, 3, "x^4+6*x^3+5*x^2+4*x+4");
    // A constant, which `mod_power_of_2_square_truncated_assign` squares in place.
    test("7", 1, 4, "1");
    test("7", 0, 4, "0");
    // A constant whose square vanishes.
    test("4", 1, 4, "0");
    test("4*x+1", 3, 4, "8*x+1");
}

#[test]
#[should_panic]
fn mod_power_of_2_square_truncated_fail() {
    NaturalPolynomial::from_str("x+16")
        .unwrap()
        .mod_power_of_2_square_truncated(2, 4);
}

#[test]
fn mod_power_of_2_square_truncated_properties() {
    natural_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().test_properties(
        |(p, len, pow)| {
            let r = (&p).mod_power_of_2_square_truncated(len, pow);
            assert!(r.is_valid());
            assert!(r.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!(p.clone().mod_power_of_2_square_truncated(len, pow), r);
            let mut s = p.clone();
            s.mod_power_of_2_square_truncated_assign(len, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            assert_eq!(mod_power_of_2_square_truncated_naive(&p, len, pow), r);
            assert_eq!((&p).square_truncated(len).mod_power_of_2(pow), r);
            assert_eq!((&p).mod_power_of_2_square(pow).truncate(len), r);
            assert_eq!((&p).mod_power_of_2_mul_truncated(&p, len, pow), r);
            assert!(r.len() <= len);
        },
    );
}
