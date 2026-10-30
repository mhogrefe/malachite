// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;

#[test]
fn test_mod_divisible_by() {
    // - the dividend and divisor are 0
    assert!(NaturalPolynomial::ZERO.mod_divisible_by(NaturalPolynomial::ZERO, Natural::from(7u32)));
    // - the divisor is 0
    assert!(
        !NaturalPolynomial::from_str("x+1")
            .unwrap()
            .mod_divisible_by(NaturalPolynomial::ZERO, Natural::from(7u32))
    );
    // - modulo 1
    assert!(NaturalPolynomial::ZERO.mod_divisible_by(NaturalPolynomial::ZERO, Natural::ONE));
    // - divisible
    assert!(
        NaturalPolynomial::from_str("x^2+3*x+2")
            .unwrap()
            .mod_divisible_by(
                NaturalPolynomial::from_str("x+1").unwrap(),
                Natural::from(7u32)
            )
    );
    // - not divisible
    assert!(
        !NaturalPolynomial::from_str("x^3+2*x+1")
            .unwrap()
            .mod_divisible_by(
                NaturalPolynomial::from_str("x^2+3").unwrap(),
                Natural::from(7u32)
            )
    );
}

#[test]
#[should_panic]
fn mod_divisible_by_fail() {
    // The divisor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("2*x^2+x")
        .unwrap()
        .mod_divisible_by(
            NaturalPolynomial::from_str("2*x+1").unwrap(),
            Natural::from(6u32),
        );
}

#[test]
fn mod_divisible_by_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_2().test_properties(
        |(p, q, m)| {
            let divisible = (&p).mod_divisible_by(&q, &m);
            assert_eq!(p.clone().mod_divisible_by(q.clone(), &m), divisible);
            assert_eq!((&p).checked_mod_div_exact(&q, &m).is_some(), divisible);
            assert_eq!(divisible, (&p).mod_rem(&q, &m) == NaturalPolynomial::ZERO);
        },
    );
}
