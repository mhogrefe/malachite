// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{BalancedMod, BalancedModPowerOf2, PowerOf2};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_pair_gen_var_1;

#[test]
fn test_balanced_mod_power_of_2() {
    let test = |s, pow, out| {
        let x = NaturalPolynomial::from_str(s).unwrap();
        let r = (&x).balanced_mod_power_of_2(pow);
        assert_eq!(r.to_string(), out);
        assert_eq!(x.clone().balanced_mod_power_of_2(pow), r);
    };
    test("0", 3, "0");
    test("23*x^2+20*x+19", 3, "-x^2+4*x+3");
    test("8*x^2+7*x+5", 3, "-x-3");
    test("x^2+3", 0, "0");
    test("18446744073709551615*x", 64, "-x");
    test("12345*x", 100, "12345*x");
}

#[test]
fn balanced_mod_power_of_2_properties() {
    natural_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, pow)| {
        let r = (&p).balanced_mod_power_of_2(pow);
        assert_eq!(p.clone().balanced_mod_power_of_2(pow), r);
        // It is balanced_mod with modulus 2^k.
        assert_eq!((&p).balanced_mod(Natural::power_of_2(pow)), r);
        // It is the same as reducing the polynomial as an IntegerPolynomial.
        assert_eq!(
            IntegerPolynomial::from(p.clone()).balanced_mod_power_of_2(pow),
            r
        );
        // Reducing never lengthens the coefficient list.
        assert!(r.coefficients_asc().len() <= p.coefficients_asc().len());
        // Reducing again changes nothing.
        assert_eq!((&r).balanced_mod_power_of_2(pow), r);
    });
}
