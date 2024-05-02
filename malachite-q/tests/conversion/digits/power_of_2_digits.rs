// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::{count_is_at_most, prefix_to_string};
use malachite_base::num::arithmetic::traits::{Abs, PowerOf2};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, PowerOf2Digits};
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_7;
use malachite_q::test_util::generators::{
    rational_unsigned_pair_gen_var_2, rational_unsigned_pair_gen_var_3,
};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_power_of_2_digits() {
    let test = |x: &str, log_base: u64, before_out: &str, aftr_out: &str| {
        let (before, after) = Rational::from_str(x).unwrap().power_of_2_digits(log_base);
        assert_eq!(before.to_debug_string(), before_out);
        assert_eq!(prefix_to_string(after, 10), aftr_out);
    };
    test("0", 1, "[]", "[]");
    test("0", 10, "[]", "[]");
    test("1", 1, "[1]", "[]");
    test("1", 10, "[1]", "[]");
    test("1/2", 1, "[]", "[1]");
    test("1/2", 10, "[]", "[512]");
    test("1/3", 1, "[]", "[0, 1, 0, 1, 0, 1, 0, 1, 0, 1, ...]");
    test(
        "1/3",
        10,
        "[]",
        "[341, 341, 341, 341, 341, 341, 341, 341, 341, 341, ...]",
    );
    test("7/6", 1, "[1]", "[0, 0, 1, 0, 1, 0, 1, 0, 1, 0, ...]");
    test(
        "7/6",
        10,
        "[1]",
        "[170, 682, 682, 682, 682, 682, 682, 682, 682, 682, ...]",
    );
    test("22/7", 1, "[1, 1]", "[0, 0, 1, 0, 0, 1, 0, 0, 1, 0, ...]");
    test(
        "22/7",
        10,
        "[3]",
        "[146, 292, 585, 146, 292, 585, 146, 292, 585, 146, ...]",
    );
    test(
        "936851431250/1397",
        1,
        "[1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, \
        1]",
        "[0, 1, 1, 0, 0, 0, 1, 0, 0, 1, ...]",
    );
    test(
        "936851431250/1397",
        10,
        "[53, 563, 639]",
        "[393, 635, 522, 643, 587, 135, 619, 393, 635, 522, ...]",
    );
    test(
        "6369051672525773/4503599627370496",
        10,
        "[1]",
        "[424, 158, 409, 1011, 755, 256]",
    );
    test(
        "884279719003555/281474976710656",
        10,
        "[3]",
        "[144, 1014, 674, 133, 652]",
    );
    test(
        "6121026514868073/2251799813685248",
        10,
        "[2]",
        "[735, 533, 88, 650, 948, 512]",
    );
}

#[test]
#[should_panic]
fn power_of_2_digits_fail() {
    Rational::ONE.power_of_2_digits(0);
}

#[test]
fn power_of_2_digits_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, log_base)| {
        let (before_point, after_point) = x.power_of_2_digits(log_base);
        let (before_point_alt, after_point_alt) = (-&x).power_of_2_digits(log_base);
        assert_eq!(before_point, before_point_alt);
        assert!(Iterator::eq(after_point.take(10), after_point_alt.take(10)));

        let (before_point, after_point) = x.power_of_2_digits(log_base);
        let approx = Rational::from_power_of_2_digits(
            log_base,
            before_point,
            RationalSequence::from_vec(after_point.take(10).collect()),
        );
        let abs_x = (&x).abs();
        assert!(approx <= abs_x);
        assert!(abs_x - approx < Rational::power_of_2(-i64::exact_from(log_base) * 10));

        let after_point = x.power_of_2_digits(log_base).1;
        assert_eq!(count_is_at_most(after_point, 0), x.is_integer());
    });

    rational_unsigned_pair_gen_var_2().test_properties(|(x, log_base)| {
        let (before_point, after_point) = x.to_power_of_2_digits(log_base);
        let (before_point_alt, after_point_alt) = x.power_of_2_digits(log_base);
        assert_eq!(before_point, before_point_alt);
        assert!(Iterator::eq(
            after_point.iter().take(10).cloned(),
            after_point_alt.take(10)
        ));
    });

    natural_unsigned_pair_gen_var_7().test_properties(|(n, log_base)| {
        let (before_point, after_point) = Rational::from(&n).power_of_2_digits(log_base);
        let before_point_alt: Vec<Natural> = n.to_power_of_2_digits_asc(log_base);
        assert_eq!(before_point, before_point_alt);
        assert_eq!(Iterator::count(after_point), 0);
    });
}
