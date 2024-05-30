// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{FloorLogBasePowerOf2, Pow};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, PowerOf2Digits};
use malachite_base::num::logic::traits::{BitConvertible, SignificantBits};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_gen_var_3, unsigned_pair_gen_var_4,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen, natural_unsigned_pair_gen_var_6, natural_unsigned_pair_gen_var_7,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_to_power_of_2_digits_asc() {
    fn test<T: PrimitiveUnsigned, F: Fn(&Natural, u64) -> Vec<T>>(
        to_power_of_2_digits_asc_naive: F,
        n: &str,
        log_base: u64,
        out: &[T],
    ) where
        Natural: PowerOf2Digits<T>,
    {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            PowerOf2Digits::<T>::to_power_of_2_digits_asc(&n, log_base),
            out
        );
        assert_eq!(to_power_of_2_digits_asc_naive(&n, log_base), out);
    }
    test::<u8, _>(Natural::to_power_of_2_digits_asc_naive, "0", 1, &[]);
    test::<u16, _>(Natural::to_power_of_2_digits_asc_naive, "123", 10, &[123]);
    test::<u16, _>(
        Natural::to_power_of_2_digits_asc_naive,
        "1000000000000",
        1,
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1,
            0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1,
        ],
    );
    test::<u32, _>(
        Natural::to_power_of_2_digits_asc_naive,
        "1000000000000",
        3,
        &[0, 0, 0, 0, 1, 2, 1, 5, 4, 2, 3, 4, 6, 1],
    );
    test::<u64, _>(
        Natural::to_power_of_2_digits_asc_naive,
        "1000000000000",
        4,
        &[0, 0, 0, 1, 5, 10, 4, 13, 8, 14],
    );
    test::<u32, _>(
        Natural::to_power_of_2_digits_asc_naive,
        "1000000000000",
        32,
        &[3567587328, 232],
    );
    test::<u64, _>(
        Natural::to_power_of_2_digits_asc_naive,
        "1000000000000",
        64,
        &[1000000000000],
    );
    test::<u64, _>(
        Natural::to_power_of_2_digits_asc_naive,
        "1000000000000000000000000",
        64,
        &[2003764205206896640, 54210],
    );
}

fn to_power_of_2_digits_asc_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: PowerOf2Digits<T>,
{
    assert_panic!(PowerOf2Digits::<T>::to_power_of_2_digits_asc(
        &Natural::from(10u32).pow(12),
        0
    ));
    assert_panic!(PowerOf2Digits::<T>::to_power_of_2_digits_asc(
        &Natural::from(10u32).pow(12),
        T::WIDTH + 1
    ));
}

#[test]
fn to_power_of_2_digits_asc_fail() {
    apply_fn_to_unsigneds!(to_power_of_2_digits_asc_fail_helper);
}

#[test]
fn test_to_power_of_2_digits_desc() {
    fn test<T: PrimitiveUnsigned>(n: &str, log_base: u64, out: &[T])
    where
        Natural: PowerOf2Digits<T>,
    {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            PowerOf2Digits::<T>::to_power_of_2_digits_desc(&n, log_base),
            out
        );
    }
    test::<u8>("0", 1, &[]);
    test::<u16>("123", 10, &[123]);
    test::<u16>(
        "1000000000000",
        1,
        &[
            1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    );
    test::<u32>(
        "1000000000000",
        3,
        &[1, 6, 4, 3, 2, 4, 5, 1, 2, 1, 0, 0, 0, 0],
    );
    test::<u64>("1000000000000", 4, &[14, 8, 13, 4, 10, 5, 1, 0, 0, 0]);
    test::<u32>("1000000000000", 32, &[232, 3567587328]);
    test::<u64>("1000000000000", 64, &[1000000000000]);
    test::<u64>(
        "1000000000000000000000000",
        64,
        &[54210, 2003764205206896640],
    );
}

fn to_power_of_2_digits_desc_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: PowerOf2Digits<T>,
{
    assert_panic!(PowerOf2Digits::<T>::to_power_of_2_digits_desc(
        &Natural::from(10u32).pow(12),
        0
    ));
    assert_panic!(PowerOf2Digits::<T>::to_power_of_2_digits_desc(
        &Natural::from(10u32).pow(12),
        T::WIDTH + 1
    ));
}

#[test]
fn to_power_of_2_digits_desc_fail() {
    apply_fn_to_unsigneds!(to_power_of_2_digits_desc_fail_helper);
}

#[test]
fn test_to_power_of_2_digits_asc_natural() {
    let test = |n, log_base, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&n, log_base).to_debug_string(),
            out
        );
        assert_eq!(
            n.to_power_of_2_digits_asc_natural_naive(log_base)
                .to_debug_string(),
            out
        );
    };
    test("0", 1, "[]");
    test("123", 10, "[123]");
    test(
        "1000000000000",
        1,
        "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, \
        0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1]",
    );
    test(
        "1000000000000",
        3,
        "[0, 0, 0, 0, 1, 2, 1, 5, 4, 2, 3, 4, 6, 1]",
    );
    test("1000000000000", 4, "[0, 0, 0, 1, 5, 10, 4, 13, 8, 14]");
    test("1000000000000", 32, "[3567587328, 232]");
    test("1000000000000", 64, "[1000000000000]");
    test(
        "1000000000000000000000000",
        64,
        "[2003764205206896640, 54210]",
    );
    test(
        "1000000000000000000000000",
        33,
        "[6996099072, 4528236150, 13552]",
    );
}

#[test]
#[should_panic]
fn to_power_of_2_digits_asc_natural_fail() {
    PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&Natural::from(10u32).pow(12), 0);
}

#[test]
fn test_to_power_of_2_digits_desc_natural() {
    let test = |n, log_base, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&n, log_base).to_debug_string(),
            out
        );
    };
    test("0", 1, "[]");
    test("123", 10, "[123]");
    test(
        "1000000000000",
        1,
        "[1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, \
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]",
    );
    test(
        "1000000000000",
        3,
        "[1, 6, 4, 3, 2, 4, 5, 1, 2, 1, 0, 0, 0, 0]",
    );
    test("1000000000000", 4, "[14, 8, 13, 4, 10, 5, 1, 0, 0, 0]");
    test("1000000000000", 32, "[232, 3567587328]");
    test("1000000000000", 64, "[1000000000000]");
    test(
        "1000000000000000000000000",
        64,
        "[54210, 2003764205206896640]",
    );
    test(
        "1000000000000000000000000",
        33,
        "[13552, 4528236150, 6996099072]",
    );
}

#[test]
#[should_panic]
fn to_power_of_2_digits_desc_natural_fail() {
    PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&Natural::from(10u32).pow(12), 0);
}

fn to_power_of_2_digits_asc_properties_helper<T: for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned>()
where
    Limb: PowerOf2Digits<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    natural_unsigned_pair_gen_var_6::<T>().test_properties(|(ref n, log_base)| {
        let digits = n.to_power_of_2_digits_asc(log_base);
        assert_eq!(
            Natural::to_power_of_2_digits_asc_naive::<T>(n, log_base),
            digits
        );
        assert_eq!(
            Natural::from_power_of_2_digits_asc(log_base, digits.iter().copied()).unwrap(),
            *n
        );
        if *n != 0 {
            assert_ne!(*digits.last().unwrap(), T::ZERO);
        }
        assert_eq!(
            digits.iter().copied().rev().collect_vec(),
            n.to_power_of_2_digits_desc(log_base)
        );
        if *n != Natural::ZERO {
            assert_eq!(
                u64::exact_from(digits.len()),
                n.floor_log_base_power_of_2(log_base) + 1
            );
        }
        assert!(digits
            .iter()
            .all(|digit| digit.significant_bits() <= log_base));

        assert_eq!(
            PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(n, log_base),
            digits.iter().copied().map(Natural::from).collect_vec()
        );
    });

    natural_gen().test_properties(|n| {
        assert_eq!(
            n.to_power_of_2_digits_asc(1)
                .into_iter()
                .map(|digit: T| digit == T::ONE)
                .collect_vec(),
            n.to_bits_asc()
        );
    });

    unsigned_gen_var_3::<T>().test_properties(|log_base| {
        assert!(PowerOf2Digits::<T>::to_power_of_2_digits_asc(&Natural::ZERO, log_base).is_empty());
    });

    unsigned_pair_gen_var_4::<Limb, T>().test_properties(|(u, log_base)| {
        let n: Natural = From::from(u);
        assert_eq!(
            PowerOf2Digits::<T>::to_power_of_2_digits_asc(&u, log_base),
            PowerOf2Digits::<T>::to_power_of_2_digits_asc(&n, log_base)
        );
    });
}

#[test]
fn to_power_of_2_digits_asc_properties() {
    apply_fn_to_unsigneds!(to_power_of_2_digits_asc_properties_helper);

    natural_gen().test_properties(|ref n| {
        assert_eq!(
            PowerOf2Digits::<Limb>::to_power_of_2_digits_asc(n, Limb::WIDTH),
            n.to_limbs_asc()
        );
    });
}

fn to_power_of_2_digits_desc_properties_helper<
    T: for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned,
>()
where
    Limb: PowerOf2Digits<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    natural_unsigned_pair_gen_var_6::<T>().test_properties(|(ref n, log_base)| {
        let digits = n.to_power_of_2_digits_desc(log_base);
        assert_eq!(
            Natural::from_power_of_2_digits_desc(log_base, digits.iter().copied()).unwrap(),
            *n
        );
        if *n != 0 {
            assert_ne!(digits[0], T::ZERO);
        }
        assert_eq!(
            digits.iter().copied().rev().collect_vec(),
            n.to_power_of_2_digits_asc(log_base)
        );
        if *n != Natural::ZERO {
            assert_eq!(
                u64::exact_from(digits.len()),
                n.floor_log_base_power_of_2(log_base) + 1
            );
        }
        assert!(digits
            .iter()
            .all(|digit| digit.significant_bits() <= log_base));

        assert_eq!(
            PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(n, log_base),
            digits.iter().copied().map(Natural::from).collect_vec()
        );
    });

    natural_gen().test_properties(|n| {
        assert_eq!(
            n.to_power_of_2_digits_desc(1)
                .into_iter()
                .map(|digit: T| digit == T::ONE)
                .collect_vec(),
            n.to_bits_desc()
        );
    });

    unsigned_gen_var_3::<T>().test_properties(|log_base| {
        assert!(
            PowerOf2Digits::<T>::to_power_of_2_digits_desc(&Natural::ZERO, log_base).is_empty()
        );
    });

    unsigned_pair_gen_var_4::<Limb, T>().test_properties(|(u, log_base)| {
        let n: Natural = From::from(u);
        assert_eq!(
            PowerOf2Digits::<T>::to_power_of_2_digits_desc(&u, log_base),
            PowerOf2Digits::<T>::to_power_of_2_digits_desc(&n, log_base)
        );
    });
}

#[test]
fn to_power_of_2_digits_desc_properties() {
    apply_fn_to_unsigneds!(to_power_of_2_digits_desc_properties_helper);

    natural_gen().test_properties(|ref n| {
        assert_eq!(
            PowerOf2Digits::<Limb>::to_power_of_2_digits_desc(n, Limb::WIDTH),
            n.to_limbs_desc()
        );
    });
}

#[test]
fn to_power_of_2_digits_asc_natural_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(ref n, log_base)| {
        let digits: Vec<Natural> = n.to_power_of_2_digits_asc(log_base);
        assert_eq!(n.to_power_of_2_digits_asc_natural_naive(log_base), digits);
        assert_eq!(
            Natural::from_power_of_2_digits_asc(log_base, digits.iter().cloned()).unwrap(),
            *n
        );
        if *n != 0 {
            assert_ne!(*digits.last().unwrap(), 0);
        }
        assert_eq!(
            digits.iter().cloned().rev().collect_vec(),
            PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(n, log_base)
        );
        if *n != Natural::ZERO {
            assert_eq!(
                u64::exact_from(digits.len()),
                n.floor_log_base_power_of_2(log_base) + 1
            );
        }
        assert!(digits
            .iter()
            .all(|digit| digit.significant_bits() <= log_base));
    });

    natural_gen().test_properties(|n| {
        assert_eq!(
            n.to_power_of_2_digits_asc(1)
                .into_iter()
                .map(|digit: Natural| digit == 1)
                .collect_vec(),
            n.to_bits_asc()
        );
    });

    unsigned_gen_var_11().test_properties(|log_base| {
        assert!(
            PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&Natural::ZERO, log_base)
                .is_empty()
        );
    });
}

#[test]
fn to_power_of_2_digits_desc_natural_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(ref n, log_base)| {
        let digits: Vec<Natural> = n.to_power_of_2_digits_desc(log_base);
        assert_eq!(
            Natural::from_power_of_2_digits_desc(log_base, digits.iter().cloned()).unwrap(),
            *n
        );
        if *n != 0 {
            assert_ne!(digits[0], 0);
        }
        assert_eq!(
            digits.iter().cloned().rev().collect_vec(),
            PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(n, log_base)
        );
        if *n != Natural::ZERO {
            assert_eq!(
                u64::exact_from(digits.len()),
                n.floor_log_base_power_of_2(log_base) + 1
            );
        }
        assert!(digits
            .iter()
            .all(|digit| digit.significant_bits() <= log_base));
    });

    natural_gen().test_properties(|n| {
        assert_eq!(
            n.to_power_of_2_digits_desc(1)
                .into_iter()
                .map(|digit: Natural| digit == 1)
                .collect_vec(),
            n.to_bits_desc()
        );
    });

    unsigned_gen_var_11().test_properties(|log_base| {
        assert!(
            PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&Natural::ZERO, log_base)
                .is_empty()
        );
    });
}
