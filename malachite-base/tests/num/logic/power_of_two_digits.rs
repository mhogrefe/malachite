use std::panic::catch_unwind;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::PowerOfTwoDigits;

#[test]
pub fn test_to_power_of_two_digits_asc() {
    fn test<T: PowerOfTwoDigits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        x: T,
        log_base: u64,
        out: &[U],
    ) {
        assert_eq!(
            PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&x, log_base),
            out
        );
    };

    test::<u8, u64>(0, 6, &[]);
    test::<u16, u64>(2, 6, &[2]);
    test::<u32, u16>(123, 3, &[3, 7, 1]);
    test::<u32, u8>(1000000, 8, &[64, 66, 15]);
    test::<u32, u64>(1000000, 8, &[64, 66, 15]);
    test::<u64, u32>(1000, 1, &[0, 0, 0, 1, 0, 1, 1, 1, 1, 1]);
}

fn to_power_of_two_digits_asc_fail_helper<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!(PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(
        &T::exact_from(100),
        U::WIDTH + 1
    ));
    assert_panic!(PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(
        &T::exact_from(100),
        0
    ));
}

#[test]
fn to_power_of_two_digits_asc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(to_power_of_two_digits_asc_fail_helper);
}

#[test]
pub fn test_to_power_of_two_digits_desc() {
    fn test<T: PowerOfTwoDigits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        x: T,
        log_base: u64,
        out: &[U],
    ) {
        assert_eq!(
            PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(&x, log_base),
            out
        );
    };

    test::<u8, u64>(0, 6, &[]);
    test::<u16, u64>(2, 6, &[2]);
    test::<u32, u16>(123, 3, &[1, 7, 3]);
    test::<u32, u8>(1000000, 8, &[15, 66, 64]);
    test::<u32, u64>(1000000, 8, &[15, 66, 64]);
    test::<u64, u32>(1000, 1, &[1, 1, 1, 1, 1, 0, 1, 0, 0, 0]);
}

fn to_power_of_two_digits_desc_fail_helper<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!(PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(
        &T::exact_from(100),
        U::WIDTH + 1
    ));
    assert_panic!(PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(
        &T::exact_from(100),
        0
    ));
}

#[test]
fn to_power_of_two_digits_desc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(to_power_of_two_digits_desc_fail_helper);
}

#[test]
pub fn test_from_power_of_two_digits_asc() {
    fn test<T: PowerOfTwoDigits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        log_base: u64,
        digits: &[U],
        out: T,
    ) {
        assert_eq!(T::from_power_of_two_digits_asc(log_base, digits), out);
    };

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
        T::from_power_of_two_digits_asc(U::WIDTH + 1, digits);
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_power_of_two_digits_asc(0, digits);
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE; 100];
        T::from_power_of_two_digits_asc(4, digits);
    });
    assert_panic!({
        let digits: &[U] = &[U::TWO];
        T::from_power_of_two_digits_asc(1, digits);
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
        assert_eq!(T::from_power_of_two_digits_desc(log_base, digits), out);
    };

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
        T::from_power_of_two_digits_desc(U::WIDTH + 1, digits);
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_power_of_two_digits_desc(0, digits);
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE; 100];
        T::from_power_of_two_digits_desc(4, digits);
    });
    assert_panic!({
        let digits: &[U] = &[U::TWO];
        T::from_power_of_two_digits_desc(1, digits);
    });
}

#[test]
fn from_power_of_two_digits_desc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(from_power_of_two_digits_desc_fail_helper);
}
