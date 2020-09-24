use std::fmt::Debug;

use malachite_base_test_util::generators::{signed_gen, unsigned_gen};

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ExactFrom, OverflowingFrom};

#[test]
pub fn test_checked_from() {
    fn test_single<T: CheckedFrom<T> + Copy + Debug + Eq>(n: T) {
        assert_eq!(T::checked_from(n), Some(n));
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: CheckedFrom<T> + Copy + Debug + Eq>(n_in: T, n_out: Option<U>) {
        assert_eq!(U::checked_from(n_in), n_out);
    };
    test_double(0u8, Some(0u16));
    test_double(1000u16, Some(1000i32));
    test_double(-5i16, Some(-5i8));
    test_double(255u8, Some(255u64));

    test_double::<_, u32>(-1i8, None);
    test_double::<_, u16>(u32::MAX, None);
    test_double::<_, u32>(i32::MIN, None);
    test_double::<_, u16>(i32::MIN, None);
    test_double::<_, i16>(i32::MIN, None);
    test_double::<_, u32>(-5i32, None);
    test_double::<_, i32>(3000000000u32, None);
    test_double::<_, i8>(-1000i16, None);
}

#[test]
pub fn test_exact_from() {
    fn test_single<T: Copy + Debug + Eq + ExactFrom<T>>(n: T) {
        assert_eq!(T::exact_from(n), n);
    };
    test_single(0u8);
    test_single(5u64);
    test_single(1000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Copy + Debug + Eq + ExactFrom<T>>(n_in: T, n_out: U) {
        assert_eq!(U::exact_from(n_in), n_out);
    };
    test_double(0u8, 0u16);
    test_double(1000u16, 1000i32);
    test_double(-5i16, -5i8);
    test_double(255u8, 255u64);
}

#[test]
#[should_panic]
fn exact_from_fail_1() {
    u32::exact_from(-1i8);
}

#[test]
#[should_panic]
fn exact_from_fail_2() {
    u16::exact_from(u32::MAX);
}

#[test]
#[should_panic]
fn exact_from_fail_3() {
    u32::exact_from(i32::MIN);
}

#[test]
#[should_panic]
fn exact_from_fail_4() {
    u16::exact_from(i32::MIN);
}

#[test]
#[should_panic]
fn exact_from_fail_5() {
    i16::exact_from(i32::MIN);
}

#[test]
#[should_panic]
fn exact_from_fail_6() {
    u32::exact_from(-5i32);
}

#[test]
#[should_panic]
fn exact_from_fail_7() {
    i32::exact_from(3000000000u32);
}

#[test]
#[should_panic]
fn exact_from_fail_8() {
    i8::exact_from(-1000i16);
}

fn checked_from_and_exact_from_helper_primitive_int_unsigned<
    T: CheckedFrom<U> + OverflowingFrom<U> + PrimitiveInt,
    U: ExactFrom<T> + PrimitiveUnsigned,
>() {
    unsigned_gen::<U>().test_properties(|u| {
        let result = T::checked_from(u);
        assert_eq!(result.is_none(), T::overflowing_from(u).1);
        if let Some(x) = result {
            assert_eq!(x, T::exact_from(u));
            assert_eq!(u, U::exact_from(x));
        }
    });
}

fn checked_from_and_exact_from_helper_primitive_int_signed<
    T: CheckedFrom<U> + OverflowingFrom<U> + PrimitiveInt,
    U: ExactFrom<T> + PrimitiveSigned,
>() {
    signed_gen::<U>().test_properties(|i| {
        let result = T::checked_from(i);
        assert_eq!(result.is_none(), T::overflowing_from(i).1);
        if let Some(x) = result {
            assert_eq!(x, T::exact_from(i));
            assert_eq!(i, U::exact_from(x));
        }
    });
}

#[test]
fn checked_from_and_exact_from_properties() {
    apply_fn_to_primitive_ints_and_unsigneds!(
        checked_from_and_exact_from_helper_primitive_int_unsigned
    );
    apply_fn_to_primitive_ints_and_signeds!(
        checked_from_and_exact_from_helper_primitive_int_signed
    );
}
