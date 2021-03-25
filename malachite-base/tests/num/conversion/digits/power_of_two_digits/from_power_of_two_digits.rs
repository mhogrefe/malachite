use itertools::repeat_n;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::PowerOfTwoDigits;
use malachite_base::slices::{slice_leading_zeros, slice_trailing_zeros};
use malachite_base_test_util::generators::{
    unsigned_pair_gen_var_5, unsigned_vec_unsigned_pair_gen_var_2,
    unsigned_vec_unsigned_pair_gen_var_3,
};
use std::panic::catch_unwind;

#[test]
pub fn test_from_power_of_two_digits_asc() {
    fn test<T: PowerOfTwoDigits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        log_base: u64,
        digits: &[U],
        out: T,
    ) {
        assert_eq!(
            T::from_power_of_two_digits_asc(log_base, digits.iter().cloned()),
            out
        );
    }
    test::<u8, u64>(6, &[], 0);
    test::<u16, u64>(6, &[2], 2);
    test::<u32, u16>(3, &[3, 7, 1], 123);
    test::<u32, u8>(8, &[64, 66, 15], 1000000);
    test::<u32, u64>(8, &[64, 66, 15], 1000000);
    test::<u64, u32>(1, &[0, 0, 0, 1, 0, 1, 1, 1, 1, 1], 1000);
}

fn from_power_of_two_digits_asc_fail_helper<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_power_of_two_digits_asc(U::WIDTH + 1, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_power_of_two_digits_asc(0, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE; 100];
        T::from_power_of_two_digits_asc(4, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::TWO];
        T::from_power_of_two_digits_asc(1, digits.iter().cloned());
    });
}

#[test]
fn from_power_of_two_digits_asc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(from_power_of_two_digits_asc_fail_helper);
}

#[test]
pub fn test_from_power_of_two_digits_desc() {
    fn test<T: PowerOfTwoDigits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        log_base: u64,
        digits: &[U],
        out: T,
    ) {
        assert_eq!(
            T::from_power_of_two_digits_desc(log_base, digits.iter().cloned()),
            out
        );
    }
    test::<u8, u64>(6, &[], 0);
    test::<u16, u64>(6, &[2], 2);
    test::<u32, u16>(3, &[1, 7, 3], 123);
    test::<u32, u8>(8, &[15, 66, 64], 1000000);
    test::<u32, u64>(8, &[15, 66, 64], 1000000);
    test::<u64, u32>(1, &[1, 1, 1, 1, 1, 0, 1, 0, 0, 0], 1000);
}

fn from_power_of_two_digits_desc_fail_helper<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_power_of_two_digits_desc(U::WIDTH + 1, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_power_of_two_digits_desc(0, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE; 100];
        T::from_power_of_two_digits_desc(4, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::TWO];
        T::from_power_of_two_digits_desc(1, digits.iter().cloned());
    });
}

#[test]
fn from_power_of_two_digits_desc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(from_power_of_two_digits_desc_fail_helper);
}

fn from_power_of_two_digits_asc_helper<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    unsigned_vec_unsigned_pair_gen_var_2::<T, U>().test_properties(|(digits, log_base)| {
        let n = T::from_power_of_two_digits_asc(log_base, digits.iter().cloned());
        assert_eq!(
            T::from_power_of_two_digits_desc(log_base, digits.iter().rev().cloned()),
            n
        );
        let trailing_zeros = slice_trailing_zeros(&digits);
        assert_eq!(
            PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&n, log_base),
            &digits[..digits.len() - trailing_zeros]
        );
    });

    unsigned_pair_gen_var_5::<usize, U>().test_properties(|(u, log_base)| {
        assert_eq!(
            T::from_power_of_two_digits_asc(log_base, repeat_n(U::ZERO, u)),
            T::ZERO
        );
    });
}

#[test]
fn from_power_of_two_digits_asc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(from_power_of_two_digits_asc_helper);
}

fn from_power_of_two_digits_desc_helper<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    unsigned_vec_unsigned_pair_gen_var_3::<T, U>().test_properties(|(digits, log_base)| {
        let n = T::from_power_of_two_digits_desc(log_base, digits.iter().cloned());
        assert_eq!(
            T::from_power_of_two_digits_asc(log_base, digits.iter().rev().cloned()),
            n
        );
        let leading_zeros = slice_leading_zeros(&digits);
        assert_eq!(
            PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(&n, log_base),
            &digits[leading_zeros..]
        );
    });

    unsigned_pair_gen_var_5::<usize, U>().test_properties(|(u, log_base)| {
        assert_eq!(
            T::from_power_of_two_digits_desc(log_base, repeat_n(U::ZERO, u)),
            T::ZERO
        );
    });
}

#[test]
fn from_power_of_two_digits_desc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(from_power_of_two_digits_desc_helper);
}
