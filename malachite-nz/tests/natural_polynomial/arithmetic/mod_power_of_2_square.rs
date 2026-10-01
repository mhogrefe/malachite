// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Square, ModPowerOf2SquareAssign,
    Square,
};
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_pair_gen_var_4;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_square::*;

#[test]
fn test_mod_power_of_2_square() {
    let test = |s, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let r = (&p).mod_power_of_2_square(pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_square(pow), r);
        let mut s = p.clone();
        s.mod_power_of_2_square_assign(pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_power_of_2_square_naive(&p, pow), r);
    };
    test("0", 0, "0");
    test("0", 4, "0");
    test("x^2+3*x+2", 3, "x^4+6*x^3+5*x^2+4*x+4");
    test("x^2+3*x+2", 100, "x^4+6*x^3+13*x^2+12*x+4");
    // A constant, which `mod_power_of_2_square_assign` squares in place.
    test("7", 4, "1");
    // A constant whose square vanishes.
    test("4", 4, "0");
    // The leading coefficient vanishes, so the degree drops.
    test("4*x+1", 4, "8*x+1");
}

#[test]
#[should_panic]
fn mod_power_of_2_square_fail() {
    NaturalPolynomial::from_str("x+16")
        .unwrap()
        .mod_power_of_2_square(4);
}

#[test]
fn mod_power_of_2_square_properties() {
    natural_polynomial_unsigned_pair_gen_var_4().test_properties(|(p, pow)| {
        let r = (&p).mod_power_of_2_square(pow);
        assert!(r.is_valid());
        assert!(r.mod_power_of_2_is_reduced(pow));
        // The forms agree.
        assert_eq!(p.clone().mod_power_of_2_square(pow), r);
        let mut s = p.clone();
        s.mod_power_of_2_square_assign(pow);
        assert!(s.is_valid());
        assert_eq!(s, r);

        assert_eq!(mod_power_of_2_square_naive(&p, pow), r);
        assert_eq!((&p).square().mod_power_of_2(pow), r);
        assert_eq!((&p).mod_power_of_2_mul(&p, pow), r);
    });
}
