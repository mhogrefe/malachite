// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, Floor, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{Digits, IsInteger};
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_pair_gen_var_2;
use malachite_q::test_util::generators::rational_natural_pair_gen_var_2;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_to_digits() {
    let test = |x: &str, base: &str, before_out: &str, after_out: &str| {
        let x = Rational::from_str(x).unwrap();
        let base = Natural::from_str(base).unwrap();
        let (before, after) = x.to_digits(&base);
        let (before_alt, after_alt) = x.into_digits(&base);
        assert_eq!(before, before_alt);
        assert_eq!(after, after_alt);
        assert_eq!(before.to_debug_string(), before_out);
        assert_eq!(after.to_string(), after_out);
    };
    test("0", "3", "[]", "[]");
    test("0", "10", "[]", "[]");
    test("1", "3", "[1]", "[]");
    test("1", "10", "[1]", "[]");
    test("1/2", "3", "[]", "[[1]]");
    test("1/2", "10", "[]", "[5]");
    test("1/3", "3", "[]", "[1]");
    test("1/3", "10", "[]", "[[3]]");
    test("7/6", "3", "[1]", "[0, [1]]");
    test("7/6", "10", "[1]", "[1, [6]]");
    test("22/7", "3", "[0, 1]", "[[0, 1, 0, 2, 1, 2]]");
    test("22/7", "10", "[3]", "[[1, 4, 2, 8, 5, 7]]");
    test(
        "936851431250/1397",
        "3",
        "[1, 2, 2, 1, 0, 0, 2, 1, 2, 2, 1, 2, 1, 0, 2, 1, 0, 2, 1]",
        "[[1, 0, 1, 1, 0, 1, 0, 2, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 2, 0, 0, 2, 1, 0, 2, 0, \
        2, 0, 0, 1, 1, 2, 2, 2, 1, 2, 2, 0, 0, 2, 0, 2, 2, 0, 0, 2, 1, 2, 2, 1, 1, 1, 0, 2, 0, 2, \
        2, 2, 1, 0, 1, 0, 2, 2, 0, 1, 0, 2, 0, 0, 2, 0, 0, 0, 0, 2, 1, 0, 2, 2, 0, 2, 0, 2, 1, 1, \
        0, 1, 2, 1, 2, 0, 2, 1, 0, 2, 1, 2, 0, 1, 2, 0, 1, 2, 1, 0, 1, 0, 0, 1, 2, 1, 2, 1, 2, 1, \
        2, 0, 0, 1, 0, 0, 0, 1, 2, 2, 2, 0, 2, 0, 0, 1, 0, 1, 0, 2, 1, 0, 0, 2, 1, 1, 1, 2, 1, 1, \
        1, 2, 2, 0, 0, 1, 2, 1, 0, 0, 0, 0, 0, 2, 0, 0, 2, 1, 0, 0, 1, 2, 2, 2, 2, 0, 2, 0, 1, 2, \
        2, 1, 1, 1, 2, 1, 0, 0, 0, 1, 2, 1, 0, 1, 1, 2, 0, 2, 2, 2, 0, 0, 0, 2, 2, 0, 1, 1, 2, 0, \
        1, 2, 1, 1, 2, 2, 0, 2, 2, 0, 0, 1, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 2, 1, 2, 1, 1, 1, 0, 0, \
        0, 2, 0, 1, 0, 2, 2, 0, 2, 2, 1, 0, 1, 1, 1, 0, 2, 1, 1, 1, 1, 1, 1, 0, 2, 0, 1, 1, 0, 0, \
        2, 1, 2, 1, 2, 2, 2, 0, 1, 1, 1, 2, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 2, 0, 2, 1, \
        0, 1, 0, 0, 0, 0, 2, 2, 1, 0, 0, 1, 0, 2, 1, 2, 2, 1, 2, 2, 2, 2, 1, 2, 1, 0, 0, 1, 0, 1, \
        0, 0, 2, 1, 0, 1, 2, 0, 1, 0, 0, 2, 1, 2, 0, 1, 0, 1, 2, 2, 0, 0, 2, 2, 1, 1, 0, 1, 2, 0, \
        0, 1, 2, 0, 1, 1, 0, 2, 2, 2, 2, 0, 0, 0, 1, 0, 1, 1, 0, 2, 2, 1, 0, 0, 2, 2, 1, 2, 0, 2, \
        0, 0, 2, 0, 1, 1, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 0, 1, 1, 1, 1, 2, 1, \
        2, 2, 0, 0, 0, 2, 0, 1, 2, 1, 1, 1, 0, 1, 2, 1, 0, 2, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, \
        2, 0, 1, 0, 1, 2, 0, 1, 1, 2, 1, 1, 2, 2, 2, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 2, 0, \
        2, 1, 1, 1, 0, 1, 0, 2, 1, 1, 2, 1, 1, 2, 1, 0, 2, 0, 2, 1, 2, 0, 2, 0, 2, 0, 2, 1, 2, 2, \
        0, 2, 2, 1, 2, 0, 0, 1, 1, 1, 2, 2, 0, 2, 0, 1, 1, 0, 2, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 1, \
        2, 1, 2, 1, 0, 2, 2, 2, 2, 1, 1, 2, 1, 1, 0, 2, 1, 2, 0, 0, 0, 1, 1, 1, 1, 2, 0, 0, 2, 2, \
        2, 1, 0, 2, 2, 1, 2, 1, 0, 1, 2, 2, 1, 1, 0, 0, 1, 2, 2, 1, 0, 1, 1, 2, 2, 1, 1, 2, 0, 2, \
        2, 0, 1, 1, 0, 1, 2, 2, 0, 2, 1, 1, 1, 1, 2, 2, 2, 2, 1, 0, 2, 1, 0, 0, 0, 2, 2, 1, 1, 2, \
        0]]",
    );
    test(
        "936851431250/1397",
        "10",
        "[9, 2, 6, 6, 1, 6, 0, 7, 6]",
        "[[3, 8, 4, 3, 9, 5, 1, 3, 2, 4, 2, 6, 6, 2, 8, 4, 8, 9, 6, 2, 0, 6, 1, 5, 6, 0, 4, 8, 6, \
        7, 5, 7, 3, 3, 7, 1, 5, 1, 0, 3, 7, 9]]",
    );
    test(
        "6369051672525773/4503599627370496",
        "10",
        "[1]",
        "[4, 1, 4, 2, 1, 3, 5, 6, 2, 3, 7, 3, 0, 9, 5, 1, 4, 5, 4, 7, 4, 6, 2, 1, 8, 5, 8, 7, 3, \
        8, 8, 2, 8, 4, 5, 0, 4, 4, 1, 3, 6, 0, 4, 7, 3, 6, 3, 2, 8, 1, 2, 5]",
    );
    test(
        "884279719003555/281474976710656",
        "10",
        "[3]",
        "[1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 1, 1, 5, 9, 9, 7, 9, 6, 3, 4, 6, 8, 5, 4, \
        4, 1, 8, 5, 1, 6, 1, 5, 9, 0, 5, 7, 6, 1, 7, 1, 8, 7, 5]",
    );
    test(
        "6121026514868073/2251799813685248",
        "10",
        "[2]",
        "[7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 5, 9, 0, 4, 5, 0, 9, 0, 7, 9, 5, 5, 9, 8, 2, 9, 8, 4, 2, \
        7, 6, 4, 8, 8, 4, 2, 3, 3, 4, 7, 4, 7, 3, 1, 4, 4, 5, 3, 1, 2, 5]",
    );
}

#[test]
#[should_panic]
fn to_digits_fail_1() {
    Rational::ONE.to_digits(&Natural::ONE);
}

#[test]
#[should_panic]
fn to_digits_fail_2() {
    Rational::ONE.to_digits(&Natural::ZERO);
}

#[test]
#[should_panic]
fn into_digits_fail_1() {
    Rational::ONE.into_digits(&Natural::ONE);
}

#[test]
#[should_panic]
fn into_digits_fail_2() {
    Rational::ONE.into_digits(&Natural::ZERO);
}

#[test]
fn to_digits_properties() {
    rational_natural_pair_gen_var_2().test_properties(|(x, base)| {
        let (before_point, after_point) = x.to_digits(&base);
        let (before_point_alt, after_point_alt) = x.clone().into_digits(&base);
        assert_eq!(before_point, before_point_alt);
        assert_eq!(after_point, after_point_alt);
        let (before_point, after_point) = (-&x).into_digits(&base);
        assert_eq!(before_point, before_point_alt);
        assert_eq!(after_point, after_point_alt);
        assert_eq!(
            Rational::from_digits_ref(&base, &before_point, &after_point),
            (&x).abs()
        );

        assert_ne!(after_point.slices_ref().1, &[Natural::ZERO]);
        assert_ne!(after_point.slices_ref().1, &[&base - Natural::ONE]);
        assert_eq!(after_point.is_empty(), x.is_integer());

        let before_point_alt: Vec<Natural> = x.abs().floor().unsigned_abs().to_digits_asc(&base);
        assert_eq!(before_point_alt, before_point);
    });

    natural_pair_gen_var_2().test_properties(|(n, base)| {
        assert_eq!(
            Rational::from(&n).into_digits(&base),
            (
                n.to_digits_asc(&base),
                RationalSequence::from_vec(Vec::new())
            )
        );
    });
}
