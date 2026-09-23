// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{integer_gen, natural_polynomial_integer_pair_gen};

#[test]
fn test_partial_eq_integer() {
    let test = |s, t, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let i = Integer::from_str(t).unwrap();
        assert_eq!(p == i, out);
        assert_eq!(i == p, out);
    };
    // The zero polynomial equals 0 and nothing else.
    test("0", "0", true);
    test("0", "1", false);
    test("0", "-1", false);
    test("1", "1", true);
    test("1", "-1", false);
    test("123", "123", true);
    test("123", "-123", false);
    test("123", "5", false);
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        true,
    );
    test(
        "1000000000000000000000000",
        "-1000000000000000000000000",
        false,
    );
    // No polynomial of positive degree equals an Integer, whatever its constant term.
    test("x", "0", false);
    test("x+1", "1", false);
    test("2*x^2+3", "3", false);
}

#[test]
fn partial_eq_integer_properties() {
    natural_polynomial_integer_pair_gen().test_properties(|(p, i)| {
        let eq = p == i;
        assert_eq!(i == p, eq);
        // Only a non-negative Integer can equal a polynomial, and then exactly when the Natural
        // with its value does.
        assert_eq!(eq, i >= 0u32 && p == Natural::exact_from(&i));
    });

    integer_gen().test_properties(|i| {
        let equal_to_constant = i >= 0u32;
        if equal_to_constant {
            let p = NaturalPolynomial::from(Natural::exact_from(&i));
            assert!(p == i);
            assert!(i == p);
        }
        assert_eq!(NaturalPolynomial::ZERO == i, i == 0u32);
        assert_eq!(NaturalPolynomial::one() == i, i == 1u32);
        assert!(NaturalPolynomial::x() != i);
    });
}
