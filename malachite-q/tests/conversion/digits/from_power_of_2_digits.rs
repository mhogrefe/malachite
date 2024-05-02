// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::PowerOf2Digits;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::vecs::vec_from_str;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    large_type_gen_var_23, natural_vec_unsigned_pair_gen_var_1,
};
use malachite_q::Rational;

#[test]
fn test_from_power_of_2_digits() {
    let test = |log_base, before: &str, after_nr: &str, after_r: &str, out: &str| {
        let before: Vec<Natural> = vec_from_str(before).unwrap();
        let after_nr: Vec<Natural> = vec_from_str(after_nr).unwrap();
        let after_r: Vec<Natural> = vec_from_str(after_r).unwrap();
        let x = Rational::from_power_of_2_digits_ref(
            log_base,
            &before,
            &RationalSequence::from_slices(&after_nr, &after_r),
        );
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(
            Rational::from_power_of_2_digits(
                log_base,
                before,
                RationalSequence::from_vecs(after_nr, after_r)
            ),
            x
        );
    };
    test(1, "[]", "[]", "[]", "0");
    test(1, "[]", "[0, 0]", "[0]", "0");
    test(10, "[]", "[]", "[]", "0");
    test(1, "[1]", "[]", "[]", "1");
    test(10, "[1]", "[]", "[]", "1");
    test(1, "[]", "[]", "[1]", "1");
    test(1, "[]", "[1]", "[]", "1/2");
    test(10, "[]", "[512]", "[]", "1/2");
    test(1, "[]", "[]", "[0, 1]", "1/3");
    test(10, "[]", "[]", "[341]", "1/3");
    test(1, "[1]", "[0]", "[0, 1]", "7/6");
    test(10, "[1]", "[170]", "[682]", "7/6");
    test(1, "[1, 1]", "[]", "[0, 0, 1]", "22/7");
    test(10, "[3]", "[]", "[146, 292, 585]", "22/7");
    test(
        1,
        "[1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, \
        1]",
        "[]",
        "[0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, \
        0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, \
        1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1]",
        "936851431250/1397",
    );
    test(
        10,
        "[53, 563, 639]",
        "[]",
        "[393, 635, 522, 643, 587, 135, 619]",
        "936851431250/1397",
    );
}

#[test]
#[should_panic]
fn from_power_of_2_digits_fail() {
    Rational::from_power_of_2_digits(0, Vec::new(), RationalSequence::from_vec(Vec::new()));
}

#[test]
#[should_panic]
fn from_power_of_2_digits_ref_fail() {
    Rational::from_power_of_2_digits_ref(0, &[], &RationalSequence::from_vec(Vec::new()));
}

#[test]
fn from_power_of_2_digits_properties() {
    large_type_gen_var_23().test_properties(|(log_base, before_point, after_point)| {
        let x =
            Rational::from_power_of_2_digits(log_base, before_point.clone(), after_point.clone());
        assert!(x.is_valid());
        assert_eq!(
            Rational::from_power_of_2_digits_ref(log_base, &before_point, &after_point),
            x
        );
        assert!(x >= 0u32);
        if before_point.last() != Some(&Natural::ZERO)
            && after_point.slices_ref().1 != [Natural::ZERO]
            && after_point.slices_ref().1 != [Natural::low_mask(log_base)]
            && !(after_point.slices_ref().1.is_empty()
                && after_point.slices_ref().0.last() == Some(&Natural::ZERO))
        {
            assert_eq!(
                x.into_power_of_2_digits(log_base),
                (before_point, after_point)
            );
        }
    });

    natural_vec_unsigned_pair_gen_var_1().test_properties(|(digits, log_base)| {
        assert_eq!(
            Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned()).unwrap(),
            Rational::from_power_of_2_digits(
                log_base,
                digits.to_vec(),
                RationalSequence::from_vec(Vec::new())
            )
        );
    });
}
