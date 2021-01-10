use itertools::Itertools;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, PowerOfTwoDigits};
use malachite_base::slices::{slice_leading_zeros, slice_trailing_zeros};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rand::distributions::range::SampleRange;
use rand::Rand;
use std::iter::repeat;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_small_u64_and_small_usize_var_2, pairs_of_u64_and_small_unsigned_var_1,
    pairs_of_u64_and_unsigned_vec_var_1, pairs_of_u64_and_unsigned_vec_var_2,
    pairs_of_u64_and_unsigned_vec_var_3, vecs_of_unsigned,
};
use malachite_test::inputs::natural::pairs_of_u64_and_natural_vec_var_1;

fn from_power_of_two_digits_asc_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>()
where
    Integer: From<Limb>,
    Natural: From<T> + PowerOfTwoDigits<T>,
    Limb: PowerOfTwoDigits<T>,
{
    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_3::<T>,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_asc(log_base, digits.iter().cloned());
            assert_eq!(
                Natural::_from_power_of_two_digits_asc_naive(log_base, digits.iter().cloned()),
                n
            );
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, digits.iter().rev().cloned()),
                n
            );
            let trailing_zeros = slice_trailing_zeros(&digits);
            let trimmed_digits = digits[..digits.len() - trailing_zeros].to_vec();
            assert_eq!(
                PowerOfTwoDigits::<T>::to_power_of_two_digits_asc(&n, log_base),
                trimmed_digits
            );
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_small_unsigned_var_1::<T, usize>,
        |&(log_base, u)| {
            assert_eq!(
                Natural::from_power_of_two_digits_asc(log_base, repeat(T::ZERO).take(u)),
                0
            );
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_1::<Limb, T>,
        |&(log_base, ref digits)| {
            let n = Limb::from_power_of_two_digits_asc(log_base, digits.iter().cloned());
            assert_eq!(
                Natural::from_power_of_two_digits_asc(log_base, digits.iter().cloned()),
                Natural::checked_from(Integer::from(n)).unwrap()
            );
        },
    );
}

#[test]
fn from_power_of_two_digits_asc_properties() {
    from_power_of_two_digits_asc_properties_helper::<u8>();
    from_power_of_two_digits_asc_properties_helper::<u16>();
    from_power_of_two_digits_asc_properties_helper::<u32>();
    from_power_of_two_digits_asc_properties_helper::<u64>();
    from_power_of_two_digits_asc_properties_helper::<usize>();

    test_properties(vecs_of_unsigned, |limbs| {
        assert_eq!(
            Natural::from_power_of_two_digits_asc(Limb::WIDTH, limbs.iter().cloned()),
            Natural::from_limbs_asc(&limbs)
        );
    });
}

fn from_power_of_two_digits_desc_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>()
where
    Natural: PowerOfTwoDigits<T>,
    Limb: PowerOfTwoDigits<T>,
{
    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_3::<T>,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_desc(log_base, digits.iter().cloned());
            assert_eq!(
                Natural::from_power_of_two_digits_asc(log_base, digits.iter().rev().cloned()),
                n
            );
            let leading_zeros = slice_leading_zeros(&digits);
            let trimmed_digits = digits[leading_zeros..].to_vec();
            assert_eq!(
                PowerOfTwoDigits::<T>::to_power_of_two_digits_desc(&n, log_base),
                trimmed_digits
            );
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_small_unsigned_var_1::<T, usize>,
        |&(log_base, u)| {
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, repeat(T::ZERO).take(u)),
                0
            );
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_2::<Limb, T>,
        |&(log_base, ref digits)| {
            let n = Limb::from_power_of_two_digits_desc(log_base, digits.iter().cloned());
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, digits.iter().cloned()),
                Natural::from(n)
            );
        },
    );
}

#[test]
fn from_power_of_two_digits_desc_properties() {
    from_power_of_two_digits_desc_properties_helper::<u8>();
    from_power_of_two_digits_desc_properties_helper::<u16>();
    from_power_of_two_digits_desc_properties_helper::<u32>();
    from_power_of_two_digits_desc_properties_helper::<u64>();
    from_power_of_two_digits_desc_properties_helper::<usize>();

    test_properties(vecs_of_unsigned, |limbs| {
        assert_eq!(
            Natural::from_power_of_two_digits_desc(Limb::WIDTH, limbs.iter().cloned()),
            Natural::from_limbs_desc(&limbs)
        );
    });
}

#[test]
fn from_power_of_two_digits_asc_natural_properties() {
    test_properties(
        pairs_of_u64_and_natural_vec_var_1,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_asc(log_base, digits.iter().cloned());
            assert_eq!(
                Natural::_from_power_of_two_digits_asc_natural_naive(
                    log_base,
                    digits.iter().cloned()
                ),
                n
            );
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, digits.iter().rev().cloned()),
                n
            );
            let trailing_zeros = slice_trailing_zeros(&digits);
            let trimmed_digits = digits[..digits.len() - trailing_zeros].to_vec();
            assert_eq!(
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&n, log_base),
                trimmed_digits
            );
        },
    );

    test_properties_no_special(
        pairs_of_small_u64_and_small_usize_var_2,
        |&(log_base, u)| {
            assert_eq!(
                Natural::from_power_of_two_digits_asc(log_base, repeat(Natural::ZERO).take(u)),
                0
            );
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_3::<Limb>,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_asc(log_base, digits.iter().cloned());
            let digits = digits.iter().cloned().map(Natural::from).collect_vec();
            assert_eq!(
                Natural::from_power_of_two_digits_asc(log_base, digits.iter().cloned()),
                n
            );
        },
    );
}

#[test]
fn from_power_of_two_digits_desc_natural_properties() {
    test_properties(
        pairs_of_u64_and_natural_vec_var_1,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_desc(log_base, digits.iter().cloned());
            assert_eq!(
                Natural::from_power_of_two_digits_asc(log_base, digits.iter().rev().cloned()),
                n
            );
            let leading_zeros = slice_leading_zeros(&digits);
            let trimmed_digits = digits[leading_zeros..].to_vec();
            assert_eq!(
                PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&n, log_base),
                trimmed_digits
            );
        },
    );

    test_properties_no_special(
        pairs_of_small_u64_and_small_usize_var_2,
        |&(log_base, u)| {
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, repeat(Natural::ZERO).take(u)),
                0
            );
        },
    );

    test_properties_no_special(
        pairs_of_u64_and_unsigned_vec_var_3::<Limb>,
        |&(log_base, ref digits)| {
            let n = Natural::from_power_of_two_digits_desc(log_base, digits.iter().cloned());
            let digits = digits.iter().cloned().map(Natural::from).collect_vec();
            assert_eq!(
                Natural::from_power_of_two_digits_desc(log_base, digits.iter().cloned()),
                n
            );
        },
    );
}
