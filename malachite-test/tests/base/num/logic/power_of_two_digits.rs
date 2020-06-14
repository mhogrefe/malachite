use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::PowerOfTwoDigits;
use malachite_base::rounding_mode::RoundingMode;
use malachite_base::slices::slice_leading_zeros::slice_leading_zeros;
use malachite_base::slices::slice_trailing_zeros::slice_trailing_zeros;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{
    test_properties, test_properties_custom_limit_no_special, test_properties_no_special,
    LARGE_LIMIT, SMALL_LIMIT,
};
use malachite_test::inputs::base::{
    pairs_of_u64_and_small_unsigned_var_1, pairs_of_u64_and_unsigned_vec_var_1,
    pairs_of_u64_and_unsigned_vec_var_2, pairs_of_unsigned_and_small_u64_var_1, small_u64s_var_1,
    unsigneds,
};

fn to_power_of_two_digits_asc_properties_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>()
where
    T: PowerOfTwoDigits<U>,
{
    test_properties(
        pairs_of_unsigned_and_small_u64_var_1::<T, U>,
        |&(u, log_base)| {
            let digits = u.to_power_of_two_digits_asc(log_base);
            assert_eq!(T::from_power_of_two_digits_asc(log_base, &digits), u);
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

#[test]
fn to_power_of_two_digits_asc_properties() {
    to_power_of_two_digits_asc_properties_helper::<u8, u8>();
    to_power_of_two_digits_asc_properties_helper::<u8, u16>();
    to_power_of_two_digits_asc_properties_helper::<u8, u32>();
    to_power_of_two_digits_asc_properties_helper::<u8, u64>();
    to_power_of_two_digits_asc_properties_helper::<u8, u128>();
    to_power_of_two_digits_asc_properties_helper::<u8, usize>();
    to_power_of_two_digits_asc_properties_helper::<u16, u8>();
    to_power_of_two_digits_asc_properties_helper::<u16, u16>();
    to_power_of_two_digits_asc_properties_helper::<u16, u32>();
    to_power_of_two_digits_asc_properties_helper::<u16, u64>();
    to_power_of_two_digits_asc_properties_helper::<u16, u128>();
    to_power_of_two_digits_asc_properties_helper::<u16, usize>();
    to_power_of_two_digits_asc_properties_helper::<u32, u8>();
    to_power_of_two_digits_asc_properties_helper::<u32, u16>();
    to_power_of_two_digits_asc_properties_helper::<u32, u32>();
    to_power_of_two_digits_asc_properties_helper::<u32, u64>();
    to_power_of_two_digits_asc_properties_helper::<u32, u128>();
    to_power_of_two_digits_asc_properties_helper::<u32, usize>();
    to_power_of_two_digits_asc_properties_helper::<u64, u8>();
    to_power_of_two_digits_asc_properties_helper::<u64, u16>();
    to_power_of_two_digits_asc_properties_helper::<u64, u32>();
    to_power_of_two_digits_asc_properties_helper::<u64, u64>();
    to_power_of_two_digits_asc_properties_helper::<u64, u128>();
    to_power_of_two_digits_asc_properties_helper::<u64, usize>();
    to_power_of_two_digits_asc_properties_helper::<usize, u8>();
    to_power_of_two_digits_asc_properties_helper::<usize, u16>();
    to_power_of_two_digits_asc_properties_helper::<usize, u32>();
    to_power_of_two_digits_asc_properties_helper::<usize, u64>();
    to_power_of_two_digits_asc_properties_helper::<usize, u128>();
    to_power_of_two_digits_asc_properties_helper::<usize, usize>();
}

fn to_power_of_two_digits_desc_properties_helper<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned,
>()
where
    T: PowerOfTwoDigits<U>,
{
    test_properties(
        pairs_of_unsigned_and_small_u64_var_1::<T, U>,
        |&(u, log_base)| {
            let digits = u.to_power_of_two_digits_desc(log_base);
            assert_eq!(T::from_power_of_two_digits_desc(log_base, &digits), u);
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
fn to_power_of_two_digits_desc_properties() {
    to_power_of_two_digits_desc_properties_helper::<u8, u8>();
    to_power_of_two_digits_desc_properties_helper::<u8, u16>();
    to_power_of_two_digits_desc_properties_helper::<u8, u32>();
    to_power_of_two_digits_desc_properties_helper::<u8, u64>();
    to_power_of_two_digits_desc_properties_helper::<u8, u128>();
    to_power_of_two_digits_desc_properties_helper::<u8, usize>();
    to_power_of_two_digits_desc_properties_helper::<u16, u8>();
    to_power_of_two_digits_desc_properties_helper::<u16, u16>();
    to_power_of_two_digits_desc_properties_helper::<u16, u32>();
    to_power_of_two_digits_desc_properties_helper::<u16, u64>();
    to_power_of_two_digits_desc_properties_helper::<u16, u128>();
    to_power_of_two_digits_desc_properties_helper::<u16, usize>();
    to_power_of_two_digits_desc_properties_helper::<u32, u8>();
    to_power_of_two_digits_desc_properties_helper::<u32, u16>();
    to_power_of_two_digits_desc_properties_helper::<u32, u32>();
    to_power_of_two_digits_desc_properties_helper::<u32, u64>();
    to_power_of_two_digits_desc_properties_helper::<u32, u128>();
    to_power_of_two_digits_desc_properties_helper::<u32, usize>();
    to_power_of_two_digits_desc_properties_helper::<u64, u8>();
    to_power_of_two_digits_desc_properties_helper::<u64, u16>();
    to_power_of_two_digits_desc_properties_helper::<u64, u32>();
    to_power_of_two_digits_desc_properties_helper::<u64, u64>();
    to_power_of_two_digits_desc_properties_helper::<u64, u128>();
    to_power_of_two_digits_desc_properties_helper::<u64, usize>();
    to_power_of_two_digits_desc_properties_helper::<usize, u8>();
    to_power_of_two_digits_desc_properties_helper::<usize, u16>();
    to_power_of_two_digits_desc_properties_helper::<usize, u32>();
    to_power_of_two_digits_desc_properties_helper::<usize, u64>();
    to_power_of_two_digits_desc_properties_helper::<usize, u128>();
    to_power_of_two_digits_desc_properties_helper::<usize, usize>();
}

fn from_power_of_two_digits_asc_properties_helper<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + Rand + SampleRange,
>()
where
    T: PowerOfTwoDigits<U>,
{
    let limit = if T::WIDTH == u8::WIDTH {
        SMALL_LIMIT
    } else {
        LARGE_LIMIT
    };
    test_properties_custom_limit_no_special(
        limit,
        pairs_of_u64_and_unsigned_vec_var_1::<T, U>,
        |&(log_base, ref digits)| {
            let n = T::from_power_of_two_digits_asc(log_base, &digits);
            let digits_rev: Vec<U> = digits.iter().rev().cloned().collect();
            assert_eq!(T::from_power_of_two_digits_desc(log_base, &digits_rev), n);
            let trailing_zeros = slice_trailing_zeros(&digits);
            let trimmed_digits = digits[..digits.len() - trailing_zeros].to_vec();
            assert_eq!(
                PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&n, log_base),
                trimmed_digits
            );
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_small_unsigned_var_1::<U, usize>,
        |&(log_base, u)| {
            assert_eq!(
                T::from_power_of_two_digits_asc(log_base, &vec![U::ZERO; u]),
                T::ZERO
            );
        },
    );
}

#[test]
fn from_power_of_two_digits_asc_properties() {
    from_power_of_two_digits_asc_properties_helper::<u8, u8>();
    from_power_of_two_digits_asc_properties_helper::<u8, u16>();
    from_power_of_two_digits_asc_properties_helper::<u8, u32>();
    from_power_of_two_digits_asc_properties_helper::<u8, u64>();
    from_power_of_two_digits_asc_properties_helper::<u8, usize>();
    from_power_of_two_digits_asc_properties_helper::<u16, u8>();
    from_power_of_two_digits_asc_properties_helper::<u16, u16>();
    from_power_of_two_digits_asc_properties_helper::<u16, u32>();
    from_power_of_two_digits_asc_properties_helper::<u16, u64>();
    from_power_of_two_digits_asc_properties_helper::<u16, usize>();
    from_power_of_two_digits_asc_properties_helper::<u32, u8>();
    from_power_of_two_digits_asc_properties_helper::<u32, u16>();
    from_power_of_two_digits_asc_properties_helper::<u32, u32>();
    from_power_of_two_digits_asc_properties_helper::<u32, u64>();
    from_power_of_two_digits_asc_properties_helper::<u32, usize>();
    from_power_of_two_digits_asc_properties_helper::<u64, u8>();
    from_power_of_two_digits_asc_properties_helper::<u64, u16>();
    from_power_of_two_digits_asc_properties_helper::<u64, u32>();
    from_power_of_two_digits_asc_properties_helper::<u64, u64>();
    from_power_of_two_digits_asc_properties_helper::<u64, usize>();
    from_power_of_two_digits_asc_properties_helper::<u128, u8>();
    from_power_of_two_digits_asc_properties_helper::<u128, u16>();
    from_power_of_two_digits_asc_properties_helper::<u128, u32>();
    from_power_of_two_digits_asc_properties_helper::<u128, u64>();
    from_power_of_two_digits_asc_properties_helper::<u128, usize>();
    from_power_of_two_digits_asc_properties_helper::<usize, u8>();
    from_power_of_two_digits_asc_properties_helper::<usize, u16>();
    from_power_of_two_digits_asc_properties_helper::<usize, u32>();
    from_power_of_two_digits_asc_properties_helper::<usize, u64>();
    from_power_of_two_digits_asc_properties_helper::<usize, usize>();
}

fn from_power_of_two_digits_desc_properties_helper<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + Rand + SampleRange,
>()
where
    T: PowerOfTwoDigits<U>,
{
    let limit = if T::WIDTH == u8::WIDTH {
        SMALL_LIMIT
    } else {
        LARGE_LIMIT
    };
    test_properties_custom_limit_no_special(
        limit,
        pairs_of_u64_and_unsigned_vec_var_2::<T, U>,
        |&(log_base, ref digits)| {
            let n = T::from_power_of_two_digits_desc(log_base, &digits);
            let digits_rev: Vec<U> = digits.iter().rev().cloned().collect();
            assert_eq!(T::from_power_of_two_digits_asc(log_base, &digits_rev), n);
            let leading_zeros = slice_leading_zeros(&digits);
            let trimmed_digits = digits[leading_zeros..].to_vec();
            assert_eq!(
                PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(&n, log_base),
                trimmed_digits
            );
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_small_unsigned_var_1::<U, usize>,
        |&(log_base, u)| {
            assert_eq!(
                T::from_power_of_two_digits_desc(log_base, &vec![U::ZERO; u]),
                T::ZERO
            );
        },
    );
}

#[test]
fn from_power_of_two_digits_desc_properties() {
    from_power_of_two_digits_desc_properties_helper::<u8, u8>();
    from_power_of_two_digits_desc_properties_helper::<u8, u16>();
    from_power_of_two_digits_desc_properties_helper::<u8, u32>();
    from_power_of_two_digits_desc_properties_helper::<u8, u64>();
    from_power_of_two_digits_desc_properties_helper::<u8, usize>();
    from_power_of_two_digits_desc_properties_helper::<u16, u8>();
    from_power_of_two_digits_desc_properties_helper::<u16, u16>();
    from_power_of_two_digits_desc_properties_helper::<u16, u32>();
    from_power_of_two_digits_desc_properties_helper::<u16, u64>();
    from_power_of_two_digits_desc_properties_helper::<u16, usize>();
    from_power_of_two_digits_desc_properties_helper::<u32, u8>();
    from_power_of_two_digits_desc_properties_helper::<u32, u16>();
    from_power_of_two_digits_desc_properties_helper::<u32, u32>();
    from_power_of_two_digits_desc_properties_helper::<u32, u64>();
    from_power_of_two_digits_desc_properties_helper::<u32, usize>();
    from_power_of_two_digits_desc_properties_helper::<u64, u8>();
    from_power_of_two_digits_desc_properties_helper::<u64, u16>();
    from_power_of_two_digits_desc_properties_helper::<u64, u32>();
    from_power_of_two_digits_desc_properties_helper::<u64, u64>();
    from_power_of_two_digits_desc_properties_helper::<u64, usize>();
    from_power_of_two_digits_desc_properties_helper::<u128, u8>();
    from_power_of_two_digits_desc_properties_helper::<u128, u16>();
    from_power_of_two_digits_desc_properties_helper::<u128, u32>();
    from_power_of_two_digits_desc_properties_helper::<u128, u64>();
    from_power_of_two_digits_desc_properties_helper::<u128, usize>();
    from_power_of_two_digits_desc_properties_helper::<usize, u8>();
    from_power_of_two_digits_desc_properties_helper::<usize, u16>();
    from_power_of_two_digits_desc_properties_helper::<usize, u32>();
    from_power_of_two_digits_desc_properties_helper::<usize, u64>();
    from_power_of_two_digits_desc_properties_helper::<usize, usize>();
}
