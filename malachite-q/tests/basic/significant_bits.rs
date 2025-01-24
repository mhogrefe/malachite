// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::test_util::generators::rational_gen;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(Rational::from_str(n).unwrap().significant_bits(), out);
    };
    test("0", 1);
    test("1", 2);
    test("-1", 2);
    test("2/3", 4);
    test("-2/3", 4);
    test("100/101", 14);
    test("-100/101", 14);
    test("22/7", 8);
    test("-22/7", 8);
}

#[test]
fn significant_bits_properties() {
    rational_gen().test_properties(|x| {
        let bits = x.significant_bits();
        assert!(bits > 0);
        assert_eq!((-x).significant_bits(), bits);
    });

    integer_gen().test_properties(|n| {
        assert_eq!(
            Rational::from(&n).significant_bits(),
            n.significant_bits() + 1
        );
    });
}
