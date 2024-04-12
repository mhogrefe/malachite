// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, Floor, IsPowerOf2, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{IsInteger, PowerOf2Digits};
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_7;
use malachite_q::test_util::generators::rational_unsigned_pair_gen_var_2;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_to_power_of_2_digits() {
    let test = |x: &str, log_base: u64, before_out: &str, after_out: &str| {
        let x = Rational::from_str(x).unwrap();
        let (before, after) = x.to_power_of_2_digits(log_base);
        let (before_alt, after_alt) = x.into_power_of_2_digits(log_base);
        assert_eq!(before, before_alt);
        assert_eq!(after, after_alt);
        assert_eq!(before.to_debug_string(), before_out);
        assert_eq!(after.to_string(), after_out);
    };
    test("0", 1, "[]", "[]");
    test("0", 10, "[]", "[]");
    test("1", 1, "[1]", "[]");
    test("1", 10, "[1]", "[]");
    test("1/2", 1, "[]", "[1]");
    test("1/2", 10, "[]", "[512]");
    test("1/3", 1, "[]", "[[0, 1]]");
    test("1/3", 10, "[]", "[[341]]");
    test("7/6", 1, "[1]", "[0, [0, 1]]");
    test("7/6", 10, "[1]", "[170, [682]]");
    test("22/7", 1, "[1, 1]", "[[0, 0, 1]]");
    test("22/7", 10, "[3]", "[[146, 292, 585]]");
    test(
        "936851431250/1397",
        1,
        "[1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, \
        1]",
        "[[0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, \
        0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, \
        1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1]]",
    );
    test(
        "936851431250/1397",
        10,
        "[53, 563, 639]",
        "[[393, 635, 522, 643, 587, 135, 619]]",
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
fn to_power_of_2_digits_fail() {
    Rational::ONE.to_power_of_2_digits(0);
}

#[test]
#[should_panic]
fn into_power_of_2_digits_fail() {
    Rational::ONE.into_power_of_2_digits(0);
}

#[test]
fn to_power_of_2_digits_properties() {
    rational_unsigned_pair_gen_var_2().test_properties(|(x, log_base)| {
        let (before_point, after_point) = x.to_power_of_2_digits(log_base);
        let (before_point_alt, after_point_alt) = x.clone().into_power_of_2_digits(log_base);
        assert_eq!(before_point, before_point_alt);
        assert_eq!(after_point, after_point_alt);
        let (before_point, after_point) = (-&x).into_power_of_2_digits(log_base);
        assert_eq!(before_point, before_point_alt);
        assert_eq!(after_point, after_point_alt);
        assert_eq!(
            Rational::from_power_of_2_digits_ref(log_base, &before_point, &after_point),
            (&x).abs()
        );

        assert_ne!(after_point.slices_ref().1, &[Natural::ZERO]);
        assert_ne!(after_point.slices_ref().1, &[Natural::low_mask(log_base)]);
        assert_eq!(after_point.is_empty(), x.is_integer());
        assert_eq!(
            after_point.slices_ref().1.is_empty(),
            x.denominator_ref().is_power_of_2()
        );

        let before_point_alt: Vec<Natural> = x
            .abs()
            .floor()
            .unsigned_abs()
            .to_power_of_2_digits_asc(log_base);
        assert_eq!(before_point_alt, before_point);
    });

    natural_unsigned_pair_gen_var_7().test_properties(|(n, log_base)| {
        assert_eq!(
            Rational::from(&n).into_power_of_2_digits(log_base),
            (
                n.to_power_of_2_digits_asc(log_base),
                RationalSequence::from_vec(Vec::new())
            )
        );
    });
}
