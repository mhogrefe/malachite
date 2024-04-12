// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::Digits;
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::vecs::vec_from_str;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    large_type_gen_var_25, natural_vec_natural_pair_gen_var_2,
};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_from_digits() {
    let test = |base: &str, before: &str, after_nr: &str, after_r: &str, out: &str| {
        let base = Natural::from_str(base).unwrap();
        let before: Vec<Natural> = vec_from_str(before).unwrap();
        let after_nr: Vec<Natural> = vec_from_str(after_nr).unwrap();
        let after_r: Vec<Natural> = vec_from_str(after_r).unwrap();
        let x = Rational::from_digits_ref(
            &base,
            &before,
            &RationalSequence::from_slices(&after_nr, &after_r),
        );
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(
            Rational::from_digits(
                &base,
                before,
                RationalSequence::from_vecs(after_nr, after_r)
            ),
            x
        );
    };
    test("3", "[]", "[]", "[]", "0");
    test("10", "[]", "[]", "[]", "0");
    test("3", "[1]", "[]", "[]", "1");
    test("10", "[1]", "[]", "[]", "1");
    test("3", "[]", "[]", "[1]", "1/2");
    test("10", "[]", "[5]", "[]", "1/2");
    test("3", "[]", "[1]", "[]", "1/3");
    test("10", "[]", "[]", "[3]", "1/3");
    test("3", "[1]", "[0]", "[1]", "7/6");
    test("10", "[1]", "[1]", "[6]", "7/6");
    test("3", "[0, 1]", "[]", "[0, 1, 0, 2, 1, 2]", "22/7");
    test("10", "[3]", "[]", "[1, 4, 2, 8, 5, 7]", "22/7");
    test(
        "3",
        "[1, 2, 2, 1, 0, 0, 2, 1, 2, 2, 1, 2, 1, 0, 2, 1, 0, 2, 1]",
        "[]",
        "[1, 0, 1, 1, 0, 1, 0, 2, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 2, 0, 0, 2, 1, 0, 2, 0, \
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
        0]",
        "936851431250/1397",
    );
    test(
        "10",
        "[9, 2, 6, 6, 1, 6, 0, 7, 6]",
        "[]",
        "[3, 8, 4, 3, 9, 5, 1, 3, 2, 4, 2, 6, 6, 2, 8, 4, 8, 9, 6, 2, 0, 6, 1, 5, 6, 0, 4, 8, 6, \
        7, 5, 7, 3, 3, 7, 1, 5, 1, 0, 3, 7, 9]",
        "936851431250/1397",
    );
}

#[test]
#[should_panic]
fn from_digits_fail_1() {
    Rational::from_digits(
        &Natural::ONE,
        Vec::new(),
        RationalSequence::from_vec(Vec::new()),
    );
}

#[test]
#[should_panic]
fn from_digits_fail_2() {
    Rational::from_digits(
        &Natural::ZERO,
        Vec::new(),
        RationalSequence::from_vec(Vec::new()),
    );
}

#[test]
#[should_panic]
fn from_digits_ref_fail_1() {
    Rational::from_digits_ref(&Natural::ONE, &[], &RationalSequence::from_vec(Vec::new()));
}

#[test]
#[should_panic]
fn from_digits_ref_fail_2() {
    Rational::from_digits_ref(&Natural::ZERO, &[], &RationalSequence::from_vec(Vec::new()));
}

#[test]
fn from_digits_properties() {
    large_type_gen_var_25().test_properties(|(base, before_point, after_point)| {
        let x = Rational::from_digits(&base, before_point.clone(), after_point.clone());
        assert!(x.is_valid());
        assert_eq!(
            Rational::from_digits_ref(&base, &before_point, &after_point),
            x
        );
        assert!(x >= 0u32);
        if before_point.last() != Some(&Natural::ZERO)
            && after_point.slices_ref().1 != [Natural::ZERO]
            && after_point.slices_ref().1 != [&base - Natural::ONE]
            && !(after_point.slices_ref().1.is_empty()
                && after_point.slices_ref().0.last() == Some(&Natural::ZERO))
        {
            assert_eq!(x.into_digits(&base), (before_point, after_point));
        }
    });

    natural_vec_natural_pair_gen_var_2().test_properties(|(digits, base)| {
        assert_eq!(
            Natural::from_digits_asc(&base, digits.iter().cloned()).unwrap(),
            Rational::from_digits(
                &base,
                digits.to_vec(),
                RationalSequence::from_vec(Vec::new())
            )
        );
    });
}
