// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::Floor;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::IsInteger;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::conversion::traits::ContinuedFraction;
use malachite_q::test_util::generators::rational_gen;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_continued_fraction() {
    let test = |x: &str, out: &str| {
        let x = Rational::from_str(x).unwrap();
        let (floor, continued_fraction) = (&x).continued_fraction();
        let (floor_alt, continued_fraction_alt) = x.continued_fraction();
        assert_eq!(floor, floor_alt);
        assert_eq!(continued_fraction, continued_fraction_alt);
        let continued_fraction = continued_fraction.collect_vec();
        let s = if continued_fraction.is_empty() {
            format!("[{floor}]")
        } else {
            let s = continued_fraction.to_debug_string();
            format!("[{}; {}]", floor, &s[1..s.len() - 1])
        };
        assert_eq!(s, out);
    };
    test("0", "[0]");
    test("123", "[123]");
    test("-123", "[-123]");
    test("1/2", "[0; 2]");
    test("22/7", "[3; 7]");
    test("-22/7", "[-4; 1, 6]");
    test("99/100", "[0; 1, 99]");
    test("936851431250/1397", "[670616629; 2, 1, 1, 1, 1, 26, 4]");
    test(
        "6369051672525773/4503599627370496",
        "[1; 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 2, 7, 1, 2, 33, \
        2, 7, 5, 2, 1, 1, 16, 2]",
    );
    test(
        "884279719003555/281474976710656",
        "[3; 7, 15, 1, 292, 1, 1, 1, 2, 1, 3, 1, 14, 3, 3, 2, 1, 3, 3, 7, 2, 1, 1, 3, 2, 42, 2]",
    );
    test(
        "6121026514868073/2251799813685248",
        "[2; 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, 1, 1, 10, 1, 1, 12, 1, 1, 11, 1, 1, 1, 11, 5, 1, 1, \
        2, 1, 4, 2, 1, 1, 9, 17, 3]",
    );
}

#[test]
fn continued_fraction_properties() {
    rational_gen().test_properties(|x| {
        let (floor, continued_fraction) = (&x).continued_fraction();
        let (floor_alt, continued_fraction_alt) = x.clone().continued_fraction();
        assert_eq!(floor, floor_alt);
        assert_eq!(continued_fraction, continued_fraction_alt);

        let continued_fraction = continued_fraction.collect_vec();
        assert_eq!(floor, (&x).floor());
        assert_eq!(continued_fraction.is_empty(), x.is_integer());
        assert!(continued_fraction.iter().all(|n| *n > 0u32));
        assert_ne!(continued_fraction.last(), Some(&Natural::ONE));
        assert_eq!(
            Rational::from_continued_fraction(floor, continued_fraction.into_iter()),
            x
        );
    });

    integer_gen().test_properties(|x| {
        let (floor, continued_fraction) = Rational::from(&x).continued_fraction();
        let continued_fraction = continued_fraction.collect_vec();
        assert_eq!(floor, x);
        assert!(continued_fraction.is_empty());
    });
}
