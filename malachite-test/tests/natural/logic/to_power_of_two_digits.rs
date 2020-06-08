use std::str::FromStr;

use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitConvertible, PowerOfTwoDigits, SignificantBits};
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_1, small_positive_unsigneds, small_u64s_var_1,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_u64_var_3, pairs_of_natural_and_small_unsigned_var_3,
};

#[test]
fn test_to_power_of_two_digits_asc() {
    fn test<T: PrimitiveUnsigned, F: Fn(&Natural, u64) -> Vec<T>>(
        to_power_of_two_digits_asc_naive: F,
        n: &str,
        log_base: u64,
        out: Vec<T>,
    ) where
        Natural: PowerOfTwoDigits<T>,
    {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(&n, log_base),
            out
        );
        assert_eq!(to_power_of_two_digits_asc_naive(&n, log_base), out);
    };
    test::<u8, _>(
        Natural::_to_power_of_two_digits_asc_u8_naive,
        "0",
        1,
        vec![],
    );
    test::<u16, _>(
        Natural::_to_power_of_two_digits_asc_u16_naive,
        "123",
        10,
        vec![123],
    );
    test::<u16, _>(
        Natural::_to_power_of_two_digits_asc_u16_naive,
        "1000000000000",
        1,
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1,
            0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1,
        ],
    );
    test::<u32, _>(
        Natural::_to_power_of_two_digits_asc_u32_naive,
        "1000000000000",
        3,
        vec![0, 0, 0, 0, 1, 2, 1, 5, 4, 2, 3, 4, 6, 1],
    );
    test::<u64, _>(
        Natural::_to_power_of_two_digits_asc_u64_naive,
        "1000000000000",
        4,
        vec![0, 0, 0, 1, 5, 10, 4, 13, 8, 14],
    );
    test::<u32, _>(
        Natural::_to_power_of_two_digits_asc_u32_naive,
        "1000000000000",
        32,
        vec![3567587328, 232],
    );
    test::<u64, _>(
        Natural::_to_power_of_two_digits_asc_u64_naive,
        "1000000000000",
        64,
        vec![1000000000000],
    );
    test::<u64, _>(
        Natural::_to_power_of_two_digits_asc_u64_naive,
        "1000000000000000000000000",
        64,
        vec![2003764205206896640, 54210],
    );
}

macro_rules! to_power_of_two_digits_asc_fail_helper {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            PowerOfTwoDigits::<$t>::to_power_of_two_digits_asc(&Natural::trillion(), 0);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            PowerOfTwoDigits::<$t>::to_power_of_two_digits_asc(&Natural::trillion(), $t::WIDTH + 1);
        }
    };
}

to_power_of_two_digits_asc_fail_helper!(
    u8,
    to_power_of_two_digits_asc_u8_fail_1,
    to_power_of_two_digits_asc_u8_fail_2
);
to_power_of_two_digits_asc_fail_helper!(
    u16,
    to_power_of_two_digits_asc_u16_fail_1,
    to_power_of_two_digits_asc_u16_fail_2
);
to_power_of_two_digits_asc_fail_helper!(
    u32,
    to_power_of_two_digits_asc_u32_fail_1,
    to_power_of_two_digits_asc_u32_fail_2
);
to_power_of_two_digits_asc_fail_helper!(
    u64,
    to_power_of_two_digits_asc_u64_fail_1,
    to_power_of_two_digits_asc_u64_fail_2
);
to_power_of_two_digits_asc_fail_helper!(
    u128,
    to_power_of_two_digits_asc_u128_fail_1,
    to_power_of_two_digits_asc_u128_fail_2
);
to_power_of_two_digits_asc_fail_helper!(
    usize,
    to_power_of_two_digits_asc_usize_fail_1,
    to_power_of_two_digits_asc_usize_fail_2
);

#[test]
fn test_to_power_of_two_digits_desc() {
    fn test<T: PrimitiveUnsigned>(n: &str, log_base: u64, out: Vec<T>)
    where
        Natural: PowerOfTwoDigits<T>,
    {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(&n, log_base),
            out
        );
    };
    test::<u8>("0", 1, vec![]);
    test::<u16>("123", 10, vec![123]);
    test::<u16>(
        "1000000000000",
        1,
        vec![
            1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    );
    test::<u32>(
        "1000000000000",
        3,
        vec![1, 6, 4, 3, 2, 4, 5, 1, 2, 1, 0, 0, 0, 0],
    );
    test::<u64>("1000000000000", 4, vec![14, 8, 13, 4, 10, 5, 1, 0, 0, 0]);
    test::<u32>("1000000000000", 32, vec![232, 3567587328]);
    test::<u64>("1000000000000", 64, vec![1000000000000]);
    test::<u64>(
        "1000000000000000000000000",
        64,
        vec![54210, 2003764205206896640],
    );
}

macro_rules! to_power_of_two_digits_desc_fail_helper {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            PowerOfTwoDigits::<$t>::to_power_of_two_digits_desc(&Natural::trillion(), 0);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            PowerOfTwoDigits::<$t>::to_power_of_two_digits_desc(
                &Natural::trillion(),
                $t::WIDTH + 1,
            );
        }
    };
}

to_power_of_two_digits_desc_fail_helper!(
    u8,
    to_power_of_two_digits_desc_u8_fail_1,
    to_power_of_two_digits_desc_u8_fail_2
);
to_power_of_two_digits_desc_fail_helper!(
    u16,
    to_power_of_two_digits_desc_u16_fail_1,
    to_power_of_two_digits_desc_u16_fail_2
);
to_power_of_two_digits_desc_fail_helper!(
    u32,
    to_power_of_two_digits_desc_u32_fail_1,
    to_power_of_two_digits_desc_u32_fail_2
);
to_power_of_two_digits_desc_fail_helper!(
    u64,
    to_power_of_two_digits_desc_u64_fail_1,
    to_power_of_two_digits_desc_u64_fail_2
);
to_power_of_two_digits_desc_fail_helper!(
    u128,
    to_power_of_two_digits_desc_u128_fail_1,
    to_power_of_two_digits_desc_u128_fail_2
);
to_power_of_two_digits_desc_fail_helper!(
    usize,
    to_power_of_two_digits_desc_usize_fail_1,
    to_power_of_two_digits_desc_usize_fail_2
);

#[test]
fn test_to_power_of_two_digits_asc_natural() {
    let test = |n, log_base, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            format!(
                "{:?}",
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, log_base)
            ),
            out
        );
        assert_eq!(
            format!(
                "{:?}",
                n._to_power_of_two_digits_asc_natural_naive(log_base)
            ),
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
fn to_power_of_two_digits_asc_natural_fail() {
    PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&Natural::trillion(), 0);
}

#[test]
fn test_to_power_of_two_digits_desc_natural() {
    let test = |n, log_base, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(
            format!(
                "{:?}",
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&n, log_base)
            ),
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
fn to_power_of_two_digits_desc_natural_fail() {
    PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&Natural::trillion(), 0);
}

fn to_power_of_two_digits_asc_properties_helper<
    T: PrimitiveUnsigned,
    F: Fn(&Natural, u64) -> Vec<T>,
>(
    to_power_of_two_digits_asc_naive: F,
) where
    Natural: From<T> + PowerOfTwoDigits<T>,
    Limb: PowerOfTwoDigits<T>,
{
    test_properties(
        pairs_of_natural_and_small_u64_var_3::<T>,
        |&(ref n, log_base)| {
            let digits = n.to_power_of_two_digits_asc(log_base);
            assert_eq!(to_power_of_two_digits_asc_naive(n, log_base), digits);
            assert_eq!(Natural::from_power_of_two_digits_asc(log_base, &digits), *n);
            if *n != 0 {
                assert_ne!(*digits.last().unwrap(), T::ZERO);
            }
            assert_eq!(
                digits.iter().cloned().rev().collect::<Vec<T>>(),
                n.to_power_of_two_digits_desc(log_base)
            );
            assert_eq!(
                digits.len(),
                usize::exact_from(
                    n.significant_bits()
                        .div_round(log_base, RoundingMode::Ceiling)
                )
            );
            assert!(digits
                .iter()
                .all(|digit| digit.significant_bits() <= log_base));

            assert_eq!(
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(n, log_base),
                digits
                    .iter()
                    .cloned()
                    .map(Natural::from)
                    .collect::<Vec<Natural>>()
            );
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(
            n.to_power_of_two_digits_asc(1)
                .into_iter()
                .map(|digit: T| digit == T::ONE)
                .collect::<Vec<bool>>(),
            n.to_bits_asc()
        );
    });

    test_properties_no_special(small_u64s_var_1::<T>, |&log_base| {
        assert!(
            PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(&Natural::ZERO, log_base).is_empty()
        );
    });

    test_properties(
        pairs_of_unsigned_and_small_u64_var_1::<Limb, T>,
        |&(u, log_base)| {
            let n: Natural = From::from(u);
            assert_eq!(
                PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(&u, log_base),
                PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(&n, log_base)
            );
        },
    );
}

#[test]
fn to_power_of_two_digits_asc_properties() {
    to_power_of_two_digits_asc_properties_helper::<u8, _>(
        Natural::_to_power_of_two_digits_asc_u8_naive,
    );
    to_power_of_two_digits_asc_properties_helper::<u16, _>(
        Natural::_to_power_of_two_digits_asc_u16_naive,
    );
    to_power_of_two_digits_asc_properties_helper::<u32, _>(
        Natural::_to_power_of_two_digits_asc_u32_naive,
    );
    to_power_of_two_digits_asc_properties_helper::<u64, _>(
        Natural::_to_power_of_two_digits_asc_u64_naive,
    );
    to_power_of_two_digits_asc_properties_helper::<u128, _>(
        Natural::_to_power_of_two_digits_asc_u128_naive,
    );
    to_power_of_two_digits_asc_properties_helper::<usize, _>(
        Natural::_to_power_of_two_digits_asc_usize_naive,
    );

    test_properties(naturals, |n| {
        assert_eq!(
            PowerOfTwoDigits::<Limb>::to_power_of_two_digits_asc(n, Limb::WIDTH),
            n.to_limbs_asc()
        );
    });
}

fn to_power_of_two_digits_desc_properties_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T> + PowerOfTwoDigits<T>,
    Limb: PowerOfTwoDigits<T>,
{
    test_properties(
        pairs_of_natural_and_small_u64_var_3::<T>,
        |&(ref n, log_base)| {
            let digits = n.to_power_of_two_digits_desc(log_base);
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, &digits),
                *n
            );
            if *n != 0 {
                assert_ne!(digits[0], T::ZERO);
            }
            assert_eq!(
                digits.iter().cloned().rev().collect::<Vec<T>>(),
                n.to_power_of_two_digits_asc(log_base)
            );
            assert_eq!(
                digits.len(),
                usize::exact_from(
                    n.significant_bits()
                        .div_round(log_base, RoundingMode::Ceiling)
                )
            );
            assert!(digits
                .iter()
                .all(|digit| digit.significant_bits() <= log_base));

            assert_eq!(
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(n, log_base),
                digits
                    .iter()
                    .cloned()
                    .map(Natural::from)
                    .collect::<Vec<Natural>>()
            );
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(
            n.to_power_of_two_digits_desc(1)
                .into_iter()
                .map(|digit: T| digit == T::ONE)
                .collect::<Vec<bool>>(),
            n.to_bits_desc()
        );
    });

    test_properties_no_special(small_u64s_var_1::<T>, |&log_base| {
        assert!(
            PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(&Natural::ZERO, log_base).is_empty()
        );
    });

    test_properties(
        pairs_of_unsigned_and_small_u64_var_1::<Limb, T>,
        |&(u, log_base)| {
            let n: Natural = From::from(u);
            assert_eq!(
                PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(&u, log_base),
                PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(&n, log_base)
            );
        },
    );
}

#[test]
fn to_power_of_two_digits_desc_properties() {
    to_power_of_two_digits_desc_properties_helper::<u8>();
    to_power_of_two_digits_desc_properties_helper::<u16>();
    to_power_of_two_digits_desc_properties_helper::<u32>();
    to_power_of_two_digits_desc_properties_helper::<u64>();
    to_power_of_two_digits_desc_properties_helper::<u128>();
    to_power_of_two_digits_desc_properties_helper::<usize>();

    test_properties(naturals, |n| {
        assert_eq!(
            PowerOfTwoDigits::<Limb>::to_power_of_two_digits_desc(n, Limb::WIDTH),
            n.to_limbs_desc()
        );
    });
}

#[test]
fn to_power_of_two_digits_asc_natural_properties() {
    test_properties(
        pairs_of_natural_and_small_unsigned_var_3,
        |&(ref n, log_base)| {
            let digits = n.to_power_of_two_digits_asc(log_base);
            assert_eq!(
                n._to_power_of_two_digits_asc_natural_naive(log_base),
                digits
            );
            assert_eq!(Natural::from_power_of_two_digits_asc(log_base, &digits), *n);
            if *n != 0 {
                assert_ne!(*digits.last().unwrap(), 0);
            }
            assert_eq!(
                digits.iter().cloned().rev().collect::<Vec<Natural>>(),
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(n, log_base)
            );
            assert_eq!(
                digits.len(),
                usize::exact_from(
                    n.significant_bits()
                        .div_round(log_base, RoundingMode::Ceiling)
                )
            );
            assert!(digits
                .iter()
                .all(|digit| digit.significant_bits() <= log_base));
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(
            n.to_power_of_two_digits_asc(1)
                .into_iter()
                .map(|digit: Natural| digit == 1)
                .collect::<Vec<bool>>(),
            n.to_bits_asc()
        );
    });

    test_properties_no_special(small_positive_unsigneds, |&log_base| {
        assert!(
            PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&Natural::ZERO, log_base)
                .is_empty()
        );
    });
}

#[test]
fn to_power_of_two_digits_desc_natural_properties() {
    test_properties(
        pairs_of_natural_and_small_unsigned_var_3,
        |&(ref n, log_base)| {
            let digits = n.to_power_of_two_digits_desc(log_base);
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, &digits),
                *n
            );
            if *n != 0 {
                assert_ne!(digits[0], 0);
            }
            assert_eq!(
                digits.iter().cloned().rev().collect::<Vec<Natural>>(),
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(n, log_base)
            );
            assert_eq!(
                digits.len(),
                usize::exact_from(
                    n.significant_bits()
                        .div_round(log_base, RoundingMode::Ceiling)
                )
            );
            assert!(digits
                .iter()
                .all(|digit| digit.significant_bits() <= log_base));
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(
            n.to_power_of_two_digits_desc(1)
                .into_iter()
                .map(|digit: Natural| digit == 1)
                .collect::<Vec<bool>>(),
            n.to_bits_desc()
        );
    });

    test_properties_no_special(small_positive_unsigneds, |&log_base| {
        assert!(
            PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&Natural::ZERO, log_base)
                .is_empty()
        );
    });
}
