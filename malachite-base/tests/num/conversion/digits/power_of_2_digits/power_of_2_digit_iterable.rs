// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ExactFrom, PowerOf2DigitIterable, PowerOf2DigitIterator, PowerOf2Digits,
};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::common::test_double_ended_iterator_size_hint;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_4, unsigned_pair_gen_var_5, unsigned_triple_gen_var_3,
    unsigned_unsigned_bool_vec_triple_gen_var_1,
};
use std::panic::catch_unwind;

#[test]
pub fn test_power_of_2_digits() {
    assert_eq!(
        PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&107u32, 2),
        &[3, 2, 2, 1]
    );
    let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(107u32, 2);
    assert_eq!(digits.next(), Some(3));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(2));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 3);
    assert_eq!(digits.get(1), 2);
    assert_eq!(digits.get(2), 2);
    assert_eq!(digits.get(3), 1);
    assert_eq!(digits.get(4), 0);
    assert_eq!(digits.get(5), 0);

    let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(107u32, 2);
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), Some(3));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let mut digits = PowerOf2DigitIterable::<u32>::power_of_2_digits(0u8, 5);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(
        PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&105u32, 1),
        &[1, 0, 0, 1, 0, 1, 1]
    );
    let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(105u32, 1);
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 1);
    assert_eq!(digits.get(1), 0);
    assert_eq!(digits.get(2), 0);
    assert_eq!(digits.get(3), 1);
    assert_eq!(digits.get(4), 0);
    assert_eq!(digits.get(5), 1);
    assert_eq!(digits.get(6), 1);
    assert_eq!(digits.get(7), 0);
    assert_eq!(digits.get(8), 0);

    let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(105u32, 1);
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(0));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);
}

fn power_of_2_digits_fail_helper<
    T: PowerOf2DigitIterable<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!(PowerOf2DigitIterable::<U>::power_of_2_digits(
        T::exact_from(107),
        0
    ));
    assert_panic!(PowerOf2DigitIterable::<U>::power_of_2_digits(
        T::exact_from(107),
        200
    ));
}

#[test]
fn power_of_2_digits_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(power_of_2_digits_fail_helper);
}

fn power_of_2_digit_iterable_helper<
    T: PowerOf2DigitIterable<U> + PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>()
where
    <T as PowerOf2DigitIterable<U>>::PowerOf2DigitIterator: Clone,
{
    unsigned_pair_gen_var_4::<T, U>().test_properties(|(u, log_base)| {
        test_double_ended_iterator_size_hint(
            PowerOf2DigitIterable::<U>::power_of_2_digits(u, log_base),
            usize::exact_from(u.significant_bits().div_round(log_base, Ceiling).0),
        );
    });

    unsigned_unsigned_bool_vec_triple_gen_var_1::<T, U>().test_properties(
        |(u, log_base, ref bs)| {
            let mut digits = PowerOf2DigitIterable::<U>::power_of_2_digits(u, log_base);
            let mut digit_vec = Vec::new();
            let mut i = 0;
            for &b in bs {
                if b {
                    digit_vec.insert(i, digits.next().unwrap());
                    i += 1;
                } else {
                    digit_vec.insert(i, digits.next_back().unwrap());
                }
            }
            assert!(digits.next().is_none());
            assert!(digits.next_back().is_none());
            assert_eq!(
                PowerOf2Digits::<U>::to_power_of_2_digits_asc(&u, log_base),
                digit_vec
            );
        },
    );

    unsigned_triple_gen_var_3::<T, U, u64>().test_properties(|(u, log_base, i)| {
        let digits = PowerOf2DigitIterable::<U>::power_of_2_digits(u, log_base);
        if i < u.significant_bits().div_round(log_base, Ceiling).0 {
            assert_eq!(
                digits.get(i),
                PowerOf2Digits::<U>::to_power_of_2_digits_asc(&u, log_base)[usize::exact_from(i)]
            );
        } else {
            assert_eq!(digits.get(i), U::ZERO);
        }
    });

    unsigned_pair_gen_var_5::<u64, U>().test_properties(|(i, log_base)| {
        let digits = PowerOf2DigitIterable::<U>::power_of_2_digits(T::ZERO, log_base);
        assert_eq!(digits.get(i), U::ZERO);
    });
}

#[test]
fn power_of_2_digit_iterable_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(power_of_2_digit_iterable_helper);
}
