use itertools::repeat_n;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{Digits, SaturatingFrom};
use malachite_base::slices::{slice_leading_zeros, slice_trailing_zeros};
use malachite_base_test_util::generators::{
    unsigned_pair_gen_var_10, unsigned_unsigned_vec_pair_gen_var_1,
    unsigned_unsigned_vec_pair_gen_var_2,
};
use std::panic::catch_unwind;

#[test]
pub fn test_from_digits_asc() {
    fn test<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(base: U, digits: &[U], out: T) {
        assert_eq!(T::from_digits_asc(&base, digits.iter().cloned()), out);
    };

    test::<u8, u64>(64, &[], 0);
    test::<u8, u64>(64, &[0, 0, 0], 0);
    test::<u16, u64>(64, &[2], 2);
    test::<u32, u16>(8, &[3, 7, 1], 123);
    test::<u32, u16>(256, &[64, 66, 15], 1000000);
    test::<u32, u64>(256, &[64, 66, 15], 1000000);
    test::<u64, u32>(2, &[0, 0, 0, 1, 0, 1, 1, 1, 1, 1], 1000);

    test::<u64, u32>(3, &[], 0);
    test::<u64, u32>(3, &[2], 2);
    test::<u64, u32>(3, &[0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2], 123456);
    test::<u64, u32>(10, &[6, 5, 4, 3, 2, 1], 123456);
    test::<u64, u32>(100, &[56, 34, 12], 123456);
    test::<u64, u32>(123, &[87, 19, 8], 123456);
    test::<u64, u32>(123, &[87, 19, 8, 0, 0, 0], 123456);
}

fn from_digits_asc_fail_helper<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>() {
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_digits_asc(&U::ZERO, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_digits_asc(&U::ONE, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE; 1000];
        T::from_digits_asc(&U::TWO, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::TWO];
        T::from_digits_asc(&U::TWO, digits.iter().cloned());
    });
}

#[test]
fn from_digits_asc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(from_digits_asc_fail_helper);
    assert_panic!({
        let digits: &[u16] = &[1, 2, 3];
        u8::from_digits_asc(&1000, digits.iter().cloned());
    });
}

#[test]
pub fn test_from_digits_desc() {
    fn test<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(base: U, digits: &[U], out: T) {
        assert_eq!(T::from_digits_desc(&base, digits.iter().cloned()), out);
    };

    test::<u8, u64>(64, &[], 0);
    test::<u8, u64>(64, &[0, 0, 0], 0);
    test::<u16, u64>(64, &[2], 2);
    test::<u32, u16>(8, &[1, 7, 3], 123);
    test::<u32, u16>(256, &[15, 66, 64], 1000000);
    test::<u32, u64>(256, &[15, 66, 64], 1000000);
    test::<u64, u32>(2, &[1, 1, 1, 1, 1, 0, 1, 0, 0, 0], 1000);

    test::<u64, u32>(3, &[], 0);
    test::<u64, u32>(3, &[2], 2);
    test::<u64, u32>(3, &[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0], 123456);
    test::<u64, u32>(10, &[1, 2, 3, 4, 5, 6], 123456);
    test::<u64, u32>(100, &[12, 34, 56], 123456);
    test::<u64, u32>(123, &[8, 19, 87], 123456);
    test::<u64, u32>(123, &[0, 0, 0, 8, 19, 87], 123456);
}

fn from_digits_desc_fail_helper<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>() {
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_digits_desc(&U::ZERO, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_digits_desc(&U::ONE, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE; 1000];
        T::from_digits_desc(&U::TWO, digits.iter().cloned());
    });
    assert_panic!({
        let digits: &[U] = &[U::TWO];
        T::from_digits_desc(&U::TWO, digits.iter().cloned());
    });
}

#[test]
fn from_digits_desc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(from_digits_desc_fail_helper);
    assert_panic!({
        let digits: &[u16] = &[1, 2, 3];
        u8::from_digits_desc(&1000, digits.iter().cloned());
    });
}

fn from_digits_asc_helper<
    T: Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() {
    unsigned_unsigned_vec_pair_gen_var_2::<U, T>().test_properties(|(base, digits)| {
        let n = T::from_digits_asc(&base, digits.iter().cloned());
        assert_eq!(T::from_digits_desc(&base, digits.iter().rev().cloned()), n);
        let trailing_zeros = slice_trailing_zeros(&digits);
        assert_eq!(
            Digits::<U>::to_digits_asc(&n, &base),
            &digits[..digits.len() - trailing_zeros]
        );
    });

    unsigned_pair_gen_var_10::<U, T, usize>().test_properties(|(base, u)| {
        assert_eq!(T::from_digits_asc(&base, repeat_n(U::ZERO, u)), T::ZERO);
    });
}

#[test]
fn from_digits_asc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(from_digits_asc_helper);
}

fn from_digits_desc_helper<
    T: Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() {
    unsigned_unsigned_vec_pair_gen_var_1::<U, T>().test_properties(|(base, digits)| {
        let n = T::from_digits_desc(&base, digits.iter().cloned());
        assert_eq!(T::from_digits_asc(&base, digits.iter().rev().cloned()), n);
        let leading_zeros = slice_leading_zeros(&digits);
        assert_eq!(
            Digits::<U>::to_digits_desc(&n, &base),
            &digits[leading_zeros..]
        );
    });

    unsigned_pair_gen_var_10::<U, T, usize>().test_properties(|(base, u)| {
        assert_eq!(T::from_digits_desc(&base, repeat_n(U::ZERO, u)), T::ZERO);
    });
}

#[test]
fn from_digits_desc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(from_digits_desc_helper);
}
