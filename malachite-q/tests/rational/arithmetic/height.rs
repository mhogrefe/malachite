// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Reciprocal;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_gen;
use std::cmp::max;
use std::str::FromStr;

#[test]
fn test_height() {
    let test = |s, out, out_bits| {
        let x = Rational::from_str(s).unwrap();
        let height = Natural::from_str(out).unwrap();
        assert_eq!(x.to_height(), height);
        assert_eq!(x.clone().into_height(), height);
        assert_eq!(x.height_significant_bits(), out_bits);
    };
    // - the numerator and denominator are equal
    test("0", "1", 1);
    test("1", "1", 1);
    test("-1", "1", 1);
    // - the denominator is larger
    test("1/2", "2", 2);
    test("-22/101", "101", 7);
    // - the numerator is larger
    test("22/7", "22", 5);
    test("-1000000000000000000000/7", "1000000000000000000000", 70);
    // - integers: the height is the absolute value
    test("5", "5", 3);
    test(
        "-98765432123456789012345678990",
        "98765432123456789012345678990",
        97,
    );
}

#[test]
fn height_properties() {
    rational_gen().test_properties(|x| {
        let height = x.to_height();
        assert_eq!(x.clone().into_height(), height);
        assert_eq!(height, max(x.to_numerator(), x.to_denominator()));
        // the height is at least 1, since the denominator is
        assert!(height >= 1u32);
        // bit counts agree with the materialized height
        assert_eq!(x.height_significant_bits(), height.significant_bits());
        // negation preserves the height
        assert_eq!((-&x).to_height(), height);
        // so does taking the reciprocal, away from zero
        if x != 0u32 {
            assert_eq!((&x).reciprocal().to_height(), height);
        }
        assert_eq!(Rational::ONE.to_height(), Natural::ONE);
    });
}
