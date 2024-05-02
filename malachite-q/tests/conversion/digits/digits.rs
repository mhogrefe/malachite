// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::{count_is_at_most, prefix_to_string};
use malachite_base::num::arithmetic::traits::{Abs, Pow};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{Digits, IsInteger};
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_pair_gen_var_2;
use malachite_q::test_util::generators::{
    rational_natural_pair_gen_var_1, rational_natural_pair_gen_var_2,
};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_digits() {
    let test = |x: &str, base: &str, before_out: &str, after_out: &str| {
        let base = Natural::from_str(base).unwrap();
        let (before, after) = Rational::from_str(x).unwrap().digits(&base);
        assert_eq!(before.to_debug_string(), before_out);
        assert_eq!(prefix_to_string(after, 10), after_out);
    };
    test("0", "3", "[]", "[]");
    test("0", "10", "[]", "[]");
    test("1", "3", "[1]", "[]");
    test("1", "10", "[1]", "[]");
    test("1/2", "3", "[]", "[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ...]");
    test("1/2", "10", "[]", "[5]");
    test("1/3", "3", "[]", "[1]");
    test("1/3", "10", "[]", "[3, 3, 3, 3, 3, 3, 3, 3, 3, 3, ...]");
    test("7/6", "3", "[1]", "[0, 1, 1, 1, 1, 1, 1, 1, 1, 1, ...]");
    test("7/6", "10", "[1]", "[1, 6, 6, 6, 6, 6, 6, 6, 6, 6, ...]");
    test("22/7", "3", "[0, 1]", "[0, 1, 0, 2, 1, 2, 0, 1, 0, 2, ...]");
    test("22/7", "10", "[3]", "[1, 4, 2, 8, 5, 7, 1, 4, 2, 8, ...]");
    test(
        "936851431250/1397",
        "3",
        "[1, 2, 2, 1, 0, 0, 2, 1, 2, 2, 1, 2, 1, 0, 2, 1, 0, 2, 1]",
        "[1, 0, 1, 1, 0, 1, 0, 2, 0, 0, ...]",
    );
    test(
        "936851431250/1397",
        "10",
        "[9, 2, 6, 6, 1, 6, 0, 7, 6]",
        "[3, 8, 4, 3, 9, 5, 1, 3, 2, 4, ...]",
    );
    test(
        "6369051672525773/4503599627370496",
        "10",
        "[1]",
        "[4, 1, 4, 2, 1, 3, 5, 6, 2, 3, ...]",
    );
    test(
        "884279719003555/281474976710656",
        "10",
        "[3]",
        "[1, 4, 1, 5, 9, 2, 6, 5, 3, 5, ...]",
    );
    test(
        "6121026514868073/2251799813685248",
        "10",
        "[2]",
        "[7, 1, 8, 2, 8, 1, 8, 2, 8, 4, ...]",
    );
}

#[test]
#[should_panic]
fn digits_fail_1() {
    Rational::ONE.digits(&Natural::ONE);
}

#[test]
#[should_panic]
fn digits_fail_2() {
    Rational::ONE.digits(&Natural::ZERO);
}

#[test]
fn digits_properties() {
    rational_natural_pair_gen_var_1().test_properties(|(x, base)| {
        let (before_point, after_point) = x.digits(&base);
        let (before_point_alt, after_point_alt) = (-&x).digits(&base);
        assert_eq!(before_point, before_point_alt);
        assert!(Iterator::eq(after_point.take(10), after_point_alt.take(10)));

        let (before_point, after_point) = x.digits(&base);
        let approx = Rational::from_digits(
            &base,
            before_point,
            RationalSequence::from_vec(after_point.take(10).collect()),
        );
        let abs_x = (&x).abs();
        assert!(approx <= abs_x);
        assert!(abs_x - approx < Rational::from(&base).pow(-10i64));

        let after_point = x.digits(&base).1;
        assert_eq!(count_is_at_most(after_point, 0), x.is_integer());
    });

    rational_natural_pair_gen_var_2().test_properties(|(x, base)| {
        let (before_point, after_point) = x.to_digits(&base);
        let (before_point_alt, after_point_alt) = x.digits(&base);
        assert_eq!(before_point, before_point_alt);
        assert!(Iterator::eq(
            after_point.iter().take(10).cloned(),
            after_point_alt.take(10)
        ));
    });

    natural_pair_gen_var_2().test_properties(|(n, base)| {
        let (before_point, after_point) = Rational::from(&n).digits(&base);
        let before_point_alt: Vec<Natural> = n.to_digits_asc(&base);
        assert_eq!(before_point, before_point_alt);
        assert_eq!(Iterator::count(after_point), 0);
    });
}
