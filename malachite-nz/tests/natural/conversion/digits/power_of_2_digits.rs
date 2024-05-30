// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivRound, Pow, Square};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ExactFrom, PowerOf2DigitIterable, PowerOf2DigitIterator, PowerOf2Digits,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::common::test_double_ended_iterator_size_hint;
use malachite_base::test_util::generators::{unsigned_pair_gen_var_18, unsigned_pair_gen_var_5};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_unsigned_bool_vec_triple_gen_var_1, natural_unsigned_bool_vec_triple_gen_var_2,
    natural_unsigned_pair_gen_var_6, natural_unsigned_pair_gen_var_7,
    natural_unsigned_unsigned_triple_gen_var_2, natural_unsigned_unsigned_triple_gen_var_3,
};
use std::panic::catch_unwind;

#[test]
pub fn test_power_of_2_digits_primitive() {
    let n = Natural::from(107u32);
    assert_eq!(
        PowerOf2Digits::<u8>::to_power_of_2_digits_asc(&n, 2),
        &[3, 2, 2, 1]
    );
    let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2);
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

    let n = Natural::from(107u32);
    let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 2);
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), Some(3));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::ZERO;
    let mut digits = PowerOf2DigitIterable::<u32>::power_of_2_digits(&n, 5);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::from(105u32);
    assert_eq!(
        PowerOf2Digits::<u8>::to_power_of_2_digits_asc(&n, 1),
        &[1, 0, 0, 1, 0, 1, 1]
    );
    let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 1);
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

    let n = Natural::from(105u32);
    let mut digits = PowerOf2DigitIterable::<u8>::power_of_2_digits(&n, 1);
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(0));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::from(10u32).pow(12);
    assert_eq!(
        PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&n, 16),
        &[4096, 54437, 232]
    );
    let mut digits = PowerOf2DigitIterable::<u64>::power_of_2_digits(&n, 16);
    assert_eq!(digits.next(), Some(4096));
    assert_eq!(digits.next_back(), Some(232));
    assert_eq!(digits.next_back(), Some(54437));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 4096);
    assert_eq!(digits.get(1), 54437);
    assert_eq!(digits.get(2), 232);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);

    let n = Natural::from(10u32).pow(12);
    assert_eq!(
        PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&n, 17),
        &[69632, 27218, 58]
    );
    let mut digits = PowerOf2DigitIterable::<u64>::power_of_2_digits(&n, 17);
    assert_eq!(digits.next(), Some(69632));
    assert_eq!(digits.next_back(), Some(58));
    assert_eq!(digits.next_back(), Some(27218));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 69632);
    assert_eq!(digits.get(1), 27218);
    assert_eq!(digits.get(2), 58);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);

    let n = Natural::from(10u32).pow(12).square();
    assert_eq!(
        PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&n, 32),
        &[2701131776, 466537709, 54210]
    );
    let mut digits = PowerOf2DigitIterable::<u64>::power_of_2_digits(&n, 32);
    assert_eq!(digits.next(), Some(2701131776));
    assert_eq!(digits.next_back(), Some(54210));
    assert_eq!(digits.next_back(), Some(466537709));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 2701131776);
    assert_eq!(digits.get(1), 466537709);
    assert_eq!(digits.get(2), 54210);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);

    let n = Natural::from(10u32).pow(12).square();
    assert_eq!(
        PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&n, 64),
        &[2003764205206896640, 54210]
    );
    let mut digits = PowerOf2DigitIterable::<u64>::power_of_2_digits(&n, 64);
    assert_eq!(digits.next(), Some(2003764205206896640));
    assert_eq!(digits.next_back(), Some(54210));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 2003764205206896640);
    assert_eq!(digits.get(1), 54210);
    assert_eq!(digits.get(2), 0);
    assert_eq!(digits.get(3), 0);

    let n = Natural::from(10u32).pow(12).square();
    assert_eq!(
        PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&n, 37),
        &[58535706624, 129132033639, 52]
    );
    let mut digits = PowerOf2DigitIterable::<u64>::power_of_2_digits(&n, 37);
    assert_eq!(digits.next(), Some(58535706624));
    assert_eq!(digits.next_back(), Some(52));
    assert_eq!(digits.next_back(), Some(129132033639));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 58535706624);
    assert_eq!(digits.get(1), 129132033639);
    assert_eq!(digits.get(2), 52);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);
}

macro_rules! power_of_2_digits_primitive_fail_helper {
    ($t:ident) => {
        let x = Natural::from(107u32);
        assert_panic!(PowerOf2DigitIterable::<$t>::power_of_2_digits(&x, 0));
        let x = Natural::from(107u32);
        assert_panic!(PowerOf2DigitIterable::<$t>::power_of_2_digits(&x, 200));
    };
}

#[test]
fn power_of_2_digits_fail() {
    apply_to_unsigneds!(power_of_2_digits_primitive_fail_helper);
}

#[test]
pub fn test_power_of_2_digits_natural() {
    let n = Natural::from(107u32);
    assert_eq!(
        PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, 2).to_debug_string(),
        "[3, 2, 2, 1]"
    );
    let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2);
    assert_eq!(digits.next().unwrap(), 3);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next_back().unwrap(), 2);
    assert_eq!(digits.next().unwrap(), 2);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 3);
    assert_eq!(digits.get(1), 2);
    assert_eq!(digits.get(2), 2);
    assert_eq!(digits.get(3), 1);
    assert_eq!(digits.get(4), 0);
    assert_eq!(digits.get(5), 0);

    let n = Natural::from(107u32);
    let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 2);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next().unwrap(), 3);
    assert_eq!(digits.next().unwrap(), 2);
    assert_eq!(digits.next().unwrap(), 2);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::ZERO;
    let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 5);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::from(105u32);
    assert_eq!(
        PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, 1).to_debug_string(),
        "[1, 0, 0, 1, 0, 1, 1]"
    );
    let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 1);
    assert_eq!(digits.next().unwrap(), 1);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next_back().unwrap(), 0);
    assert_eq!(digits.next().unwrap(), 0);
    assert_eq!(digits.next().unwrap(), 0);
    assert_eq!(digits.next().unwrap(), 1);
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

    let n = Natural::from(105u32);
    let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 1);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next().unwrap(), 1);
    assert_eq!(digits.next().unwrap(), 0);
    assert_eq!(digits.next().unwrap(), 0);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next_back().unwrap(), 0);
    assert_eq!(digits.next_back().unwrap(), 1);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let n = Natural::from(10u32).pow(12);
    assert_eq!(
        PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, 16).to_debug_string(),
        "[4096, 54437, 232]"
    );
    let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 16);
    assert_eq!(digits.next().unwrap(), 4096);
    assert_eq!(digits.next_back().unwrap(), 232);
    assert_eq!(digits.next_back().unwrap(), 54437);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 4096);
    assert_eq!(digits.get(1), 54437);
    assert_eq!(digits.get(2), 232);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);

    let n = Natural::from(10u32).pow(12);
    assert_eq!(
        PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, 17).to_debug_string(),
        "[69632, 27218, 58]"
    );
    let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 17);
    assert_eq!(digits.next().unwrap(), 69632);
    assert_eq!(digits.next_back().unwrap(), 58);
    assert_eq!(digits.next_back().unwrap(), 27218);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 69632);
    assert_eq!(digits.get(1), 27218);
    assert_eq!(digits.get(2), 58);
    assert_eq!(digits.get(3), 0);
    assert_eq!(digits.get(4), 0);

    let n = Natural::from(10u32).pow(12).square();
    assert_eq!(
        PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, 32).to_debug_string(),
        "[2701131776, 466537709, 54210]"
    );
    let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 32);
    assert_eq!(digits.next().unwrap(), 2701131776u32);
    assert_eq!(digits.next_back().unwrap(), 54210u32);
    assert_eq!(digits.next_back().unwrap(), 466537709u32);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 2701131776u32);
    assert_eq!(digits.get(1), 466537709u32);
    assert_eq!(digits.get(2), 54210u32);
    assert_eq!(digits.get(3), 0u32);
    assert_eq!(digits.get(4), 0u32);

    let n = Natural::from(10u32).pow(12).square();
    assert_eq!(
        PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, 64).to_debug_string(),
        "[2003764205206896640, 54210]"
    );
    let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 64);
    assert_eq!(digits.next().unwrap(), 2003764205206896640u64);
    assert_eq!(digits.next_back().unwrap(), 54210u64);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 2003764205206896640u64);
    assert_eq!(digits.get(1), 54210u64);
    assert_eq!(digits.get(2), 0u64);
    assert_eq!(digits.get(3), 0u64);

    let n = Natural::from(10u32).pow(12).square();
    assert_eq!(
        PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, 37).to_debug_string(),
        "[58535706624, 129132033639, 52]"
    );
    let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, 37);
    assert_eq!(digits.next().unwrap(), 58535706624u64);
    assert_eq!(digits.next_back().unwrap(), 52u64);
    assert_eq!(digits.next_back().unwrap(), 129132033639u64);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 58535706624u64);
    assert_eq!(digits.get(1), 129132033639u64);
    assert_eq!(digits.get(2), 52u64);
    assert_eq!(digits.get(3), 0u64);
    assert_eq!(digits.get(4), 0u64);
}

#[test]
#[should_panic]
fn natural_power_of_2_digits_natural_fail() {
    PowerOf2DigitIterable::<Natural>::power_of_2_digits(&Natural::from(107u32), 0);
}

fn power_of_2_digits_primitive_properties_helper<T: PrimitiveUnsigned>()
where
    for<'a> &'a Natural: PowerOf2DigitIterable<T>,
    Natural: PowerOf2Digits<T>,
    for<'a> <&'a Natural as PowerOf2DigitIterable<T>>::PowerOf2DigitIterator: Clone,
{
    natural_unsigned_pair_gen_var_6::<T>().test_properties(|(ref n, log_base)| {
        test_double_ended_iterator_size_hint(
            PowerOf2DigitIterable::<T>::power_of_2_digits(n, log_base),
            usize::exact_from(n.significant_bits().div_round(log_base, Ceiling).0),
        );
    });

    natural_unsigned_bool_vec_triple_gen_var_2::<T>().test_properties(
        |(ref n, log_base, ref bs)| {
            let mut digits = PowerOf2DigitIterable::<T>::power_of_2_digits(n, log_base);
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
                PowerOf2Digits::<T>::to_power_of_2_digits_asc(n, log_base),
                digit_vec
            );
        },
    );

    natural_unsigned_unsigned_triple_gen_var_2::<u64, T>().test_properties(
        |(ref n, log_base, i)| {
            let digits = PowerOf2DigitIterable::<T>::power_of_2_digits(n, log_base);
            if i < n.significant_bits().div_round(log_base, Ceiling).0 {
                assert_eq!(
                    digits.get(i),
                    PowerOf2Digits::<T>::to_power_of_2_digits_asc(n, log_base)
                        [usize::exact_from(i)],
                );
            } else {
                assert_eq!(digits.get(i), T::ZERO);
            }
        },
    );

    unsigned_pair_gen_var_5::<u64, T>().test_properties(|(i, log_base)| {
        let n = Natural::ZERO;
        let digits = PowerOf2DigitIterable::<T>::power_of_2_digits(&n, log_base);
        assert_eq!(digits.get(i), T::ZERO);
    });
}

#[test]
fn power_of_2_digits_primitive_properties() {
    apply_fn_to_unsigneds!(power_of_2_digits_primitive_properties_helper);
}

#[test]
fn power_of_2_digits_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(ref n, log_base)| {
        test_double_ended_iterator_size_hint(
            PowerOf2DigitIterable::<Natural>::power_of_2_digits(n, log_base),
            usize::exact_from(n.significant_bits().div_round(log_base, Ceiling).0),
        );
    });

    natural_unsigned_bool_vec_triple_gen_var_1().test_properties(|(ref n, log_base, ref bs)| {
        let mut digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(n, log_base);
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
            PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(n, log_base),
            digit_vec
        );
    });

    natural_unsigned_unsigned_triple_gen_var_3().test_properties(|(ref n, log_base, i)| {
        let digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(n, log_base);
        if i < n.significant_bits().div_round(log_base, Ceiling).0 {
            assert_eq!(
                digits.get(i),
                PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(n, log_base)
                    [usize::exact_from(i)],
            );
        } else {
            assert_eq!(digits.get(i), 0);
        }
    });

    unsigned_pair_gen_var_18().test_properties(|(i, log_base)| {
        let n = Natural::ZERO;
        let digits = PowerOf2DigitIterable::<Natural>::power_of_2_digits(&n, log_base);
        assert_eq!(digits.get(i), 0);
    });
}
