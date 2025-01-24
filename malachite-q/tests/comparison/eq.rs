// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::common::test_eq_helper;
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::test_util::generators::{rational_gen, rational_pair_gen, rational_triple_gen};
use malachite_q::Rational;
use num::BigRational;
use rug;

#[test]
fn test_eq() {
    let strings = &[
        "-1000000000001",
        "-1000000000000",
        "-999999999999",
        "-123",
        "-2",
        "-7/5",
        "-1",
        "-5/7",
        "-3/8",
        "-123/1000000",
        "-1237/1000000000000",
        "0",
        "1237/1000000000000",
        "123/1000000",
        "3/8",
        "5/7",
        "1",
        "7/5",
        "2",
        "123",
        "999999999999",
        "1000000000000",
        "1000000000001",
    ];
    test_eq_helper::<Rational>(strings);
    test_eq_helper::<BigRational>(strings);
    test_eq_helper::<rug::Rational>(strings);
}

#[allow(clippy::cmp_owned, clippy::eq_op)]
#[test]
fn eq_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(BigRational::from(&x) == BigRational::from(&y), eq);
        assert_eq!(rug::Rational::from(&x) == rug::Rational::from(&y), eq);
        assert_eq!(y == x, eq);
    });

    rational_gen().test_properties(|x| {
        assert_eq!(x, x);
    });

    rational_triple_gen().test_properties(|(x, y, z)| {
        if x == y && y == z {
            assert_eq!(x, z);
        }
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x) == Rational::from(&y), x == y);
        assert_eq!(Rational::from(&x) == y, x == y);
        assert_eq!(x == Rational::from(&y), x == y);
    });
}
