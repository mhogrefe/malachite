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

#[test]
fn test_mod_power_of_2_divisible_by() {
    // - the dividend and divisor are 0
    assert!(NaturalPolynomial::ZERO.mod_power_of_2_divisible_by(NaturalPolynomial::ZERO, 3));
    // - the divisor is 0
    assert!(
        !NaturalPolynomial::from_str("x+1")
            .unwrap()
            .mod_power_of_2_divisible_by(NaturalPolynomial::ZERO, 3)
    );
    // - modulo 1
    assert!(NaturalPolynomial::ZERO.mod_power_of_2_divisible_by(NaturalPolynomial::ZERO, 0));
    // - divisible
    assert!(
        NaturalPolynomial::from_str("x^2+3*x+2")
            .unwrap()
            .mod_power_of_2_divisible_by(NaturalPolynomial::from_str("x+1").unwrap(), 3)
    );
    // - not divisible
    assert!(
        !NaturalPolynomial::from_str("x^3+2*x+1")
            .unwrap()
            .mod_power_of_2_divisible_by(NaturalPolynomial::from_str("x^2+3").unwrap(), 3)
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_divisible_by_fail() {
    // The divisor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("2*x^2+x")
        .unwrap()
        .mod_power_of_2_divisible_by(NaturalPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
fn mod_power_of_2_divisible_by_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5().test_properties(
        |(p, q, pow)| {
            let divisible = (&p).mod_power_of_2_divisible_by(&q, pow);
            assert_eq!(
                p.clone().mod_power_of_2_divisible_by(q.clone(), pow),
                divisible
            );
            assert_eq!(
                (&p).checked_mod_power_of_2_div_exact(&q, pow).is_some(),
                divisible
            );
            assert_eq!(
                divisible,
                (&p).mod_power_of_2_rem(&q, pow) == NaturalPolynomial::ZERO
            );
        },
    );
}
