// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::repeat_n;
use itertools::Itertools;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::PowerOf2Digits;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::slices::{slice_leading_zeros, slice_trailing_zeros};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_18, unsigned_pair_gen_var_5, unsigned_vec_gen,
    unsigned_vec_unsigned_pair_gen_var_10, unsigned_vec_unsigned_pair_gen_var_11,
    unsigned_vec_unsigned_pair_gen_var_2, unsigned_vec_unsigned_pair_gen_var_3,
};
use malachite_base::vecs::vec_from_str;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vec_unsigned_pair_gen_var_1, natural_vec_unsigned_pair_gen_var_2,
};
use std::panic::catch_unwind;

#[test]
fn test_from_power_of_2_digits_asc() {
    fn test_ok<T: PrimitiveUnsigned>(log_base: u64, digits: &[T], out: &str)
    where
        Natural: From<T> + PowerOf2Digits<T>,
    {
        assert_eq!(
            Natural::from_power_of_2_digits_asc(log_base, digits.iter().copied())
                .unwrap()
                .to_string(),
            out
        );
        assert_eq!(
            Natural::from_power_of_2_digits_asc_naive(log_base, digits.iter().copied())
                .unwrap()
                .to_string(),
            out
        );
    }
    test_ok::<u8>(1, &[], "0");
    test_ok::<u8>(1, &[0, 0, 0], "0");
    test_ok::<u16>(10, &[123], "123");
    test_ok::<u16>(
        1,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1,
            0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1,
        ],
        "1000000000000",
    );
    test_ok::<u32>(
        3,
        &[0, 0, 0, 0, 1, 2, 1, 5, 4, 2, 3, 4, 6, 1, 0],
        "1000000000000",
    );
    test_ok::<u64>(4, &[0, 0, 0, 1, 5, 10, 4, 13, 8, 14], "1000000000000");
    test_ok::<u32>(32, &[3567587328, 232], "1000000000000");
    test_ok::<u64>(64, &[1000000000000], "1000000000000");
    test_ok::<u64>(
        64,
        &[2003764205206896640, 54210],
        "1000000000000000000000000",
    );

    fn test_err<T: PrimitiveUnsigned>(log_base: u64, digits: &[T])
    where
        Natural: From<T> + PowerOf2Digits<T>,
    {
        assert!(Natural::from_power_of_2_digits_asc(log_base, digits.iter().copied()).is_none());
        assert!(
            Natural::from_power_of_2_digits_asc_naive(log_base, digits.iter().copied()).is_none()
        );
    }
    test_err::<u8>(1, &[2]);
}

fn from_power_of_2_digits_asc_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: PowerOf2Digits<T>,
{
    assert_panic!(Natural::from_power_of_2_digits_asc(
        0,
        [T::ZERO].iter().copied()
    ));
    assert_panic!(Natural::from_power_of_2_digits_asc(
        T::WIDTH + 1,
        [T::TWO].iter().copied()
    ));
}

#[test]
fn from_power_of_2_digits_asc_fail() {
    apply_fn_to_unsigneds!(from_power_of_2_digits_asc_fail_helper);
}

#[test]
fn test_from_power_of_2_digits_desc() {
    fn test_ok<T: PrimitiveUnsigned>(log_base: u64, digits: &[T], out: &str)
    where
        Natural: PowerOf2Digits<T>,
    {
        assert_eq!(
            Natural::from_power_of_2_digits_desc(log_base, digits.iter().copied())
                .unwrap()
                .to_string(),
            out
        );
    }
    test_ok::<u8>(1, &[], "0");
    test_ok::<u8>(1, &[0, 0, 0], "0");
    test_ok::<u16>(10, &[123], "123");
    test_ok::<u16>(
        1,
        &[
            1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        "1000000000000",
    );
    test_ok::<u32>(
        3,
        &[0, 1, 6, 4, 3, 2, 4, 5, 1, 2, 1, 0, 0, 0, 0],
        "1000000000000",
    );
    test_ok::<u64>(4, &[14, 8, 13, 4, 10, 5, 1, 0, 0, 0], "1000000000000");
    test_ok::<u32>(32, &[232, 3567587328], "1000000000000");
    test_ok::<u64>(64, &[1000000000000], "1000000000000");
    test_ok::<u64>(
        64,
        &[54210, 2003764205206896640],
        "1000000000000000000000000",
    );

    fn test_err<T: PrimitiveUnsigned>(log_base: u64, digits: &[T])
    where
        Natural: PowerOf2Digits<T>,
    {
        assert!(Natural::from_power_of_2_digits_desc(log_base, digits.iter().copied()).is_none());
    }
    test_err::<u8>(1, &[2]);
}

fn from_power_of_2_digits_desc_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: PowerOf2Digits<T>,
{
    assert_panic!(Natural::from_power_of_2_digits_desc(
        0,
        [T::ZERO].iter().copied()
    ));
    assert_panic!(Natural::from_power_of_2_digits_desc(
        T::WIDTH + 1,
        [T::TWO].iter().copied()
    ));
}

#[test]
fn from_power_of_2_digits_desc_fail() {
    apply_fn_to_unsigneds!(from_power_of_2_digits_desc_fail_helper);
}

#[test]
fn test_from_power_of_2_digits_asc_natural() {
    let test_ok = |log_base, digits, out| {
        let digits = vec_from_str(digits).unwrap();
        assert_eq!(
            Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned())
                .unwrap()
                .to_string(),
            out
        );
        assert_eq!(
            Natural::from_power_of_2_digits_asc_natural_naive(log_base, digits.iter().cloned())
                .unwrap()
                .to_string(),
            out
        );
    };
    test_ok(1, "[]", "0");
    test_ok(1, "[0, 0, 0]", "0");
    test_ok(10, "[123]", "123");
    test_ok(
        1,
        "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, \
        0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1]",
        "1000000000000",
    );
    test_ok(
        3,
        "[0, 0, 0, 0, 1, 2, 1, 5, 4, 2, 3, 4, 6, 1]",
        "1000000000000",
    );
    test_ok(4, "[0, 0, 0, 1, 5, 10, 4, 13, 8, 14, 0]", "1000000000000");
    test_ok(32, "[3567587328, 232]", "1000000000000");
    test_ok(64, "[1000000000000]", "1000000000000");
    test_ok(
        64,
        "[2003764205206896640, 54210]",
        "1000000000000000000000000",
    );
    test_ok(
        33,
        "[6996099072, 4528236150, 13552]",
        "1000000000000000000000000",
    );

    let test_err = |log_base, digits| {
        let digits = vec_from_str(digits).unwrap();
        assert!(Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned()).is_none());
        assert!(Natural::from_power_of_2_digits_asc_natural_naive(
            log_base,
            digits.iter().cloned()
        )
        .is_none());
    };
    test_err(1, "[2]");
}

#[test]
#[should_panic]
fn from_power_of_2_digits_asc_natural_fail() {
    let digits: Vec<Natural> = vec_from_str("[0, 0, 0]").unwrap();
    Natural::from_power_of_2_digits_asc(0, digits.iter().cloned());
}

#[test]
fn test_from_power_of_2_digits_desc_natural() {
    let test_ok = |log_base, digits, out| {
        let digits: Vec<Natural> = vec_from_str(digits).unwrap();
        assert_eq!(
            Natural::from_power_of_2_digits_desc(log_base, digits.iter().cloned())
                .unwrap()
                .to_string(),
            out
        );
    };
    test_ok(1, "[]", "0");
    test_ok(1, "[0, 0, 0]", "0");
    test_ok(10, "[123]", "123");
    test_ok(
        1,
        "[1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, \
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]",
        "1000000000000",
    );
    test_ok(
        3,
        "[1, 6, 4, 3, 2, 4, 5, 1, 2, 1, 0, 0, 0, 0]",
        "1000000000000",
    );
    test_ok(4, "[0, 14, 8, 13, 4, 10, 5, 1, 0, 0, 0]", "1000000000000");
    test_ok(32, "[232, 3567587328]", "1000000000000");
    test_ok(64, "[1000000000000]", "1000000000000");
    test_ok(
        64,
        "[54210, 2003764205206896640]",
        "1000000000000000000000000",
    );
    test_ok(
        33,
        "[13552, 4528236150, 6996099072]",
        "1000000000000000000000000",
    );

    let test_err = |log_base, digits| {
        let digits: Vec<Natural> = vec_from_str(digits).unwrap();
        assert!(Natural::from_power_of_2_digits_desc(log_base, digits.iter().cloned()).is_none());
    };
    test_err(1, "[2]");
}

#[test]
#[should_panic]
fn from_power_of_2_digits_desc_natural_fail() {
    let digits: Vec<Natural> = vec_from_str("[0, 0, 0]").unwrap();
    Natural::from_power_of_2_digits_desc(0, digits.iter().cloned());
}

fn from_power_of_2_digits_asc_properties_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T> + PowerOf2Digits<T>,
    Limb: PowerOf2Digits<T>,
{
    let mut config = GenConfig::new();
    config.insert("mean_log_base_n", T::WIDTH >> 1);
    config.insert("mean_stripe_n", 64);
    config.insert("mean_digit_count_n", 32);
    unsigned_vec_unsigned_pair_gen_var_11().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Natural::from_power_of_2_digits_asc(log_base, digits.iter().copied());
            assert_eq!(
                n.is_some(),
                digits.iter().all(|x| x.significant_bits() <= log_base),
            );
        },
    );

    unsigned_vec_unsigned_pair_gen_var_10().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Natural::from_power_of_2_digits_asc(log_base, digits.iter().copied()).unwrap();
            assert_eq!(
                Natural::from_power_of_2_digits_asc_naive(log_base, digits.iter().copied())
                    .unwrap(),
                n
            );
            assert_eq!(
                Natural::from_power_of_2_digits_desc(log_base, digits.iter().rev().copied())
                    .unwrap(),
                n
            );
            let trailing_zeros = slice_trailing_zeros(&digits);
            let trimmed_digits = digits[..digits.len() - trailing_zeros].to_vec();
            assert_eq!(
                PowerOf2Digits::<T>::to_power_of_2_digits_asc(&n, log_base),
                trimmed_digits
            );
        },
    );

    unsigned_pair_gen_var_5::<usize, T>().test_properties(|(u, log_base)| {
        assert_eq!(
            Natural::from_power_of_2_digits_asc(log_base, repeat_n(T::ZERO, u)).unwrap(),
            0
        );
    });

    unsigned_vec_unsigned_pair_gen_var_2::<Limb, T>().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Limb::from_power_of_2_digits_asc(log_base, digits.iter().copied()).unwrap();
            assert_eq!(
                Natural::from_power_of_2_digits_asc(log_base, digits.iter().copied()).unwrap(),
                Natural::try_from(Integer::from(n)).unwrap()
            );
        },
    );
}

#[test]
fn from_power_of_2_digits_asc_properties() {
    apply_fn_to_unsigneds!(from_power_of_2_digits_asc_properties_helper);

    let mut config = GenConfig::new();
    config.insert("mean_stripe_n", 64);
    config.insert("mean_length_n", 32);
    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        assert_eq!(
            Natural::from_power_of_2_digits_asc(Limb::WIDTH, xs.iter().copied()).unwrap(),
            Natural::from_limbs_asc(&xs)
        );
    });
}

fn from_power_of_2_digits_desc_properties_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T> + PowerOf2Digits<T>,
    Limb: PowerOf2Digits<T>,
{
    let mut config = GenConfig::new();
    config.insert("mean_log_base_n", T::WIDTH >> 1);
    config.insert("mean_stripe_n", 64);
    config.insert("mean_digit_count_n", 32);
    unsigned_vec_unsigned_pair_gen_var_11().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Natural::from_power_of_2_digits_desc(log_base, digits.iter().copied());
            assert_eq!(
                n.is_some(),
                digits.iter().all(|x| x.significant_bits() <= log_base)
            );
        },
    );

    unsigned_vec_unsigned_pair_gen_var_10().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Natural::from_power_of_2_digits_desc(log_base, digits.iter().copied()).unwrap();
            assert_eq!(
                Natural::from_power_of_2_digits_asc(log_base, digits.iter().rev().copied())
                    .unwrap(),
                n
            );
            let leading_zeros = slice_leading_zeros(&digits);
            let trimmed_digits = digits[leading_zeros..].to_vec();
            assert_eq!(
                PowerOf2Digits::<T>::to_power_of_2_digits_desc(&n, log_base),
                trimmed_digits
            );
        },
    );

    unsigned_pair_gen_var_5::<usize, T>().test_properties(|(u, log_base)| {
        assert_eq!(
            Natural::from_power_of_2_digits_desc(log_base, repeat_n(T::ZERO, u)),
            Some(Natural::ZERO)
        );
    });

    unsigned_vec_unsigned_pair_gen_var_3::<Limb, T>().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Limb::from_power_of_2_digits_desc(log_base, digits.iter().copied()).unwrap();
            let natural_n: Natural = From::<Limb>::from(n);
            assert_eq!(
                Natural::from_power_of_2_digits_desc(log_base, digits.iter().copied()).unwrap(),
                natural_n
            );
        },
    );
}

#[test]
fn from_power_of_2_digits_desc_properties() {
    apply_fn_to_unsigneds!(from_power_of_2_digits_desc_properties_helper);

    let mut config = GenConfig::new();
    config.insert("mean_stripe_n", 64);
    config.insert("mean_length_n", 32);
    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        assert_eq!(
            Natural::from_power_of_2_digits_desc(Limb::WIDTH, xs.iter().copied()).unwrap(),
            Natural::from_limbs_desc(&xs)
        );
    });
}

#[test]
fn from_power_of_2_digits_asc_natural_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_log_base_n", 16);
    config.insert("mean_stripe_n", 64);
    config.insert("mean_digit_count_n", 32);
    natural_vec_unsigned_pair_gen_var_2().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned());
            assert_eq!(
                n.is_some(),
                digits.iter().all(|x| x.significant_bits() <= log_base),
            );
        },
    );

    natural_vec_unsigned_pair_gen_var_1().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned()).unwrap();
            assert_eq!(
                Natural::from_power_of_2_digits_asc_natural_naive(log_base, digits.iter().cloned())
                    .unwrap(),
                n
            );
            assert_eq!(
                Natural::from_power_of_2_digits_desc(log_base, digits.iter().rev().cloned())
                    .unwrap(),
                n
            );
            let trailing_zeros = slice_trailing_zeros(&digits);
            let trimmed_digits = digits[..digits.len() - trailing_zeros].to_vec();
            assert_eq!(
                PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, log_base),
                trimmed_digits
            );
        },
    );

    unsigned_pair_gen_var_18().test_properties(|(u, log_base)| {
        assert_eq!(
            Natural::from_power_of_2_digits_asc(log_base, repeat_n(Natural::ZERO, u)),
            Some(Natural::ZERO)
        );
    });

    unsigned_vec_unsigned_pair_gen_var_10::<Limb>().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Natural::from_power_of_2_digits_asc(log_base, digits.iter().copied());
            let digits = digits.iter().copied().map(Natural::from).collect_vec();
            assert_eq!(
                Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned()),
                n
            );
        },
    );
}

#[test]
fn from_power_of_2_digits_desc_natural_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_log_base_n", 16);
    config.insert("mean_stripe_n", 64);
    config.insert("mean_digit_count_n", 32);
    natural_vec_unsigned_pair_gen_var_2().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Natural::from_power_of_2_digits_desc(log_base, digits.iter().cloned());
            assert_eq!(
                n.is_some(),
                digits.iter().all(|x| x.significant_bits() <= log_base)
            );
        },
    );

    natural_vec_unsigned_pair_gen_var_1().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Natural::from_power_of_2_digits_desc(log_base, digits.iter().cloned()).unwrap();
            assert_eq!(
                Natural::from_power_of_2_digits_asc(log_base, digits.iter().rev().cloned())
                    .unwrap(),
                n
            );
            let leading_zeros = slice_leading_zeros(&digits);
            let trimmed_digits = digits[leading_zeros..].to_vec();
            assert_eq!(
                PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&n, log_base),
                trimmed_digits
            );
        },
    );

    unsigned_pair_gen_var_18().test_properties(|(u, log_base)| {
        assert_eq!(
            Natural::from_power_of_2_digits_desc(log_base, repeat_n(Natural::ZERO, u)),
            Some(Natural::ZERO)
        );
    });

    unsigned_vec_unsigned_pair_gen_var_10::<Limb>().test_properties_with_config(
        &config,
        |(digits, log_base)| {
            let n = Natural::from_power_of_2_digits_desc(log_base, digits.iter().copied()).unwrap();
            let digits = digits.iter().copied().map(Natural::from).collect_vec();
            assert_eq!(
                Natural::from_power_of_2_digits_desc(log_base, digits.iter().cloned()).unwrap(),
                n
            );
        },
    );
}
