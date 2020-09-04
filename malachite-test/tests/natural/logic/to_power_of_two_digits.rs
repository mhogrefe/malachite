use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitConvertible, PowerOfTwoDigits, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_1, small_positive_unsigneds, small_u64s_var_1,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_u64_var_3, pairs_of_natural_and_small_unsigned_var_3,
};

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
        Natural::_to_power_of_two_digits_asc_naive,
    );
    to_power_of_two_digits_asc_properties_helper::<u16, _>(
        Natural::_to_power_of_two_digits_asc_naive,
    );
    to_power_of_two_digits_asc_properties_helper::<u32, _>(
        Natural::_to_power_of_two_digits_asc_naive,
    );
    to_power_of_two_digits_asc_properties_helper::<u64, _>(
        Natural::_to_power_of_two_digits_asc_naive,
    );
    to_power_of_two_digits_asc_properties_helper::<u128, _>(
        Natural::_to_power_of_two_digits_asc_naive,
    );
    to_power_of_two_digits_asc_properties_helper::<usize, _>(
        Natural::_to_power_of_two_digits_asc_naive,
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
