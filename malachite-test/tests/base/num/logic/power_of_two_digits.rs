use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::PowerOfTwoDigits;
use malachite_base::round::RoundingMode;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_1, small_u64s_var_1, unsigneds,
};

fn to_power_of_two_digits_asc_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>()
where
    T: PowerOfTwoDigits<U>,
{
    test_properties(
        pairs_of_unsigned_and_small_u64_var_1::<T, U>,
        |&(u, log_base)| {
            let digits = u.to_power_of_two_digits_asc(log_base);
            if u != T::ZERO {
                assert_ne!(*digits.last().unwrap(), U::ZERO);
            }
            assert_eq!(
                digits.iter().cloned().rev().collect::<Vec<U>>(),
                u.to_power_of_two_digits_desc(log_base)
            );
            assert_eq!(
                digits.len(),
                usize::exact_from(
                    u.significant_bits()
                        .div_round(log_base, RoundingMode::Ceiling)
                )
            );
            assert!(digits
                .iter()
                .all(|digit| digit.significant_bits() <= log_base));
        },
    );

    test_properties(unsigneds::<T>, |&u| {
        assert_eq!(
            u.to_power_of_two_digits_asc(1)
                .into_iter()
                .map(|digit: U| digit == U::ONE)
                .collect::<Vec<bool>>(),
            u.to_bits_asc()
        );
    });

    test_properties_no_special(small_u64s_var_1::<U>, |&log_base| {
        assert!(PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&T::ZERO, log_base).is_empty());
    });
}

fn to_power_of_two_digits_desc_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>()
where
    T: PowerOfTwoDigits<U>,
{
    test_properties(
        pairs_of_unsigned_and_small_u64_var_1::<T, U>,
        |&(u, log_base)| {
            let digits = u.to_power_of_two_digits_desc(log_base);
            if u != T::ZERO {
                assert_ne!(digits[0], U::ZERO);
            }
            assert_eq!(
                digits.iter().cloned().rev().collect::<Vec<U>>(),
                u.to_power_of_two_digits_asc(log_base)
            );
            assert_eq!(
                digits.len(),
                usize::exact_from(
                    u.significant_bits()
                        .div_round(log_base, RoundingMode::Ceiling)
                )
            );
            assert!(digits
                .iter()
                .all(|digit| digit.significant_bits() <= log_base));
        },
    );

    test_properties(unsigneds::<T>, |&u| {
        assert_eq!(
            u.to_power_of_two_digits_desc(1)
                .into_iter()
                .map(|digit: U| digit == U::ONE)
                .collect::<Vec<bool>>(),
            u.to_bits_desc()
        );
    });

    test_properties_no_special(small_u64s_var_1::<U>, |&log_base| {
        assert!(PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(&T::ZERO, log_base).is_empty());
    });
}

#[test]
fn to_power_of_two_digits_asc_properties() {
    to_power_of_two_digits_asc_helper::<u8, u8>();
    to_power_of_two_digits_asc_helper::<u8, u16>();
    to_power_of_two_digits_asc_helper::<u8, u32>();
    to_power_of_two_digits_asc_helper::<u8, u64>();
    to_power_of_two_digits_asc_helper::<u8, u128>();
    to_power_of_two_digits_asc_helper::<u8, usize>();
    to_power_of_two_digits_asc_helper::<u16, u8>();
    to_power_of_two_digits_asc_helper::<u16, u16>();
    to_power_of_two_digits_asc_helper::<u16, u32>();
    to_power_of_two_digits_asc_helper::<u16, u64>();
    to_power_of_two_digits_asc_helper::<u16, u128>();
    to_power_of_two_digits_asc_helper::<u16, usize>();
    to_power_of_two_digits_asc_helper::<u32, u8>();
    to_power_of_two_digits_asc_helper::<u32, u16>();
    to_power_of_two_digits_asc_helper::<u32, u32>();
    to_power_of_two_digits_asc_helper::<u32, u64>();
    to_power_of_two_digits_asc_helper::<u32, u128>();
    to_power_of_two_digits_asc_helper::<u32, usize>();
    to_power_of_two_digits_asc_helper::<u64, u8>();
    to_power_of_two_digits_asc_helper::<u64, u16>();
    to_power_of_two_digits_asc_helper::<u64, u32>();
    to_power_of_two_digits_asc_helper::<u64, u64>();
    to_power_of_two_digits_asc_helper::<u64, u128>();
    to_power_of_two_digits_asc_helper::<u64, usize>();
    to_power_of_two_digits_asc_helper::<usize, u8>();
    to_power_of_two_digits_asc_helper::<usize, u16>();
    to_power_of_two_digits_asc_helper::<usize, u32>();
    to_power_of_two_digits_asc_helper::<usize, u64>();
    to_power_of_two_digits_asc_helper::<usize, u128>();
    to_power_of_two_digits_asc_helper::<usize, usize>();
}

#[test]
fn to_power_of_two_digits_desc_properties() {
    to_power_of_two_digits_desc_helper::<u8, u8>();
    to_power_of_two_digits_desc_helper::<u8, u16>();
    to_power_of_two_digits_desc_helper::<u8, u32>();
    to_power_of_two_digits_desc_helper::<u8, u64>();
    to_power_of_two_digits_desc_helper::<u8, u128>();
    to_power_of_two_digits_desc_helper::<u8, usize>();
    to_power_of_two_digits_desc_helper::<u16, u8>();
    to_power_of_two_digits_desc_helper::<u16, u16>();
    to_power_of_two_digits_desc_helper::<u16, u32>();
    to_power_of_two_digits_desc_helper::<u16, u64>();
    to_power_of_two_digits_desc_helper::<u16, u128>();
    to_power_of_two_digits_desc_helper::<u16, usize>();
    to_power_of_two_digits_desc_helper::<u32, u8>();
    to_power_of_two_digits_desc_helper::<u32, u16>();
    to_power_of_two_digits_desc_helper::<u32, u32>();
    to_power_of_two_digits_desc_helper::<u32, u64>();
    to_power_of_two_digits_desc_helper::<u32, u128>();
    to_power_of_two_digits_desc_helper::<u32, usize>();
    to_power_of_two_digits_desc_helper::<u64, u8>();
    to_power_of_two_digits_desc_helper::<u64, u16>();
    to_power_of_two_digits_desc_helper::<u64, u32>();
    to_power_of_two_digits_desc_helper::<u64, u64>();
    to_power_of_two_digits_desc_helper::<u64, u128>();
    to_power_of_two_digits_desc_helper::<u64, usize>();
    to_power_of_two_digits_desc_helper::<usize, u8>();
    to_power_of_two_digits_desc_helper::<usize, u16>();
    to_power_of_two_digits_desc_helper::<usize, u32>();
    to_power_of_two_digits_desc_helper::<usize, u64>();
    to_power_of_two_digits_desc_helper::<usize, u128>();
    to_power_of_two_digits_desc_helper::<usize, usize>();
}
