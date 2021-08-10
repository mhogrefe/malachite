use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, OverflowingFrom, RoundingFrom,
};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::{
    primitive_float_gen, primitive_float_gen_var_13, primitive_float_gen_var_14, signed_gen,
    signed_gen_var_7, unsigned_gen, unsigned_gen_var_18,
};
use std::fmt::Debug;
use std::panic::catch_unwind;

#[test]
pub fn test_checked_from() {
    fn test_single<T: CheckedFrom<T> + Copy + Debug + Eq>(n: T) {
        assert_eq!(T::checked_from(n), Some(n));
    }
    test_single(0u8);
    test_single(5u64);
    test_single(1000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double_primitive_int<T, U: CheckedFrom<T> + Copy + Debug + Eq>(
        n_in: T,
        n_out: Option<U>,
    ) {
        assert_eq!(U::checked_from(n_in), n_out);
    }
    test_double_primitive_int(0u8, Some(0u16));
    test_double_primitive_int(1000u16, Some(1000i32));
    test_double_primitive_int(-5i16, Some(-5i8));
    test_double_primitive_int(255u8, Some(255u64));

    test_double_primitive_int::<_, u32>(-1i8, None);
    test_double_primitive_int::<_, u16>(u32::MAX, None);
    test_double_primitive_int::<_, u32>(i32::MIN, None);
    test_double_primitive_int::<_, u16>(i32::MIN, None);
    test_double_primitive_int::<_, i16>(i32::MIN, None);
    test_double_primitive_int::<_, u32>(-5i32, None);
    test_double_primitive_int::<_, i32>(3000000000u32, None);
    test_double_primitive_int::<_, i8>(-1000i16, None);

    test_double_primitive_int::<_, u8>(0.0f32, Some(0));
    test_double_primitive_int::<_, u8>(-0.0f32, Some(0));
    test_double_primitive_int::<_, u8>(123.0f32, Some(123));
    test_double_primitive_int::<_, i8>(-123.0f32, Some(-123));
    test_double_primitive_int::<_, u8>(-123.0f32, None);
    test_double_primitive_int::<_, u8>(500.0f32, None);
    test_double_primitive_int::<_, u8>(123.1f32, None);
    test_double_primitive_int::<_, u8>(f32::NAN, None);
    test_double_primitive_int::<_, u8>(f32::POSITIVE_INFINITY, None);
    test_double_primitive_int::<_, u8>(f32::NEGATIVE_INFINITY, None);
    test_double_primitive_int::<_, u8>(255.0f32, Some(255));
    test_double_primitive_int::<_, u8>(256.0f32, None);
    test_double_primitive_int::<_, i8>(127.0f32, Some(127));
    test_double_primitive_int::<_, i8>(128.0f32, None);
    test_double_primitive_int::<_, i8>(-128.0f32, Some(-128));
    test_double_primitive_int::<_, i8>(-129.0f32, None);

    fn test_double_primitive_float<T, U: CheckedFrom<T> + PrimitiveFloat>(
        n_in: T,
        n_out: Option<U>,
    ) {
        assert_eq!(U::checked_from(n_in).map(NiceFloat), n_out.map(NiceFloat));
    }
    test_double_primitive_float::<_, f32>(0u8, Some(0.0));
    test_double_primitive_float::<_, f32>(123u8, Some(123.0));
    test_double_primitive_float::<_, f32>(-123i8, Some(-123.0));
    test_double_primitive_float::<_, f32>(u128::MAX, None);
    test_double_primitive_float::<_, f32>(i128::MIN, Some(-1.7014118e38));
    test_double_primitive_float::<_, f32>(i128::MIN + 1, None);
    test_double_primitive_float::<_, f32>(u32::MAX, None);
    test_double_primitive_float::<_, f32>(i32::MIN, Some(-2147483600.0));
    test_double_primitive_float::<_, f32>(i32::MIN + 1, None);
}

#[test]
pub fn test_exact_from() {
    fn test_single<T: Copy + Debug + Eq + ExactFrom<T>>(n: T) {
        assert_eq!(T::exact_from(n), n);
    }
    test_single(0u8);
    test_single(5u64);
    test_single(1000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double_primitive_int<T, U: Copy + Debug + Eq + ExactFrom<T>>(n_in: T, n_out: U) {
        assert_eq!(U::exact_from(n_in), n_out);
    }
    test_double_primitive_int(0u8, 0u16);
    test_double_primitive_int(1000u16, 1000i32);
    test_double_primitive_int(-5i16, -5i8);
    test_double_primitive_int(255u8, 255u64);

    test_double_primitive_int(0.0f32, 0u8);
    test_double_primitive_int(-0.0f32, 0u8);
    test_double_primitive_int(123.0f32, 123u8);
    test_double_primitive_int(-123.0f32, -123i8);
    test_double_primitive_int(255.0f32, 255u8);
    test_double_primitive_int(127.0f32, 127i8);
    test_double_primitive_int(-128.0f32, -128i8);

    fn test_double_primitive_float<T, U: CheckedFrom<T> + PrimitiveFloat>(n_in: T, n_out: U) {
        assert_eq!(NiceFloat(U::exact_from(n_in)), NiceFloat(n_out));
    }
    test_double_primitive_float(0u8, 0.0f32);
    test_double_primitive_float(123u8, 123.0f32);
    test_double_primitive_float(-123i8, -123.0f32);
    test_double_primitive_float(i128::MIN, -1.7014118e38f32);
    test_double_primitive_float(i32::MIN, -2147483600.0f32);
}

#[test]
fn exact_from_fail() {
    assert_panic!(u32::exact_from(-1i8));
    assert_panic!(u16::exact_from(u32::MAX));
    assert_panic!(u32::exact_from(i32::MIN));
    assert_panic!(u16::exact_from(i32::MIN));
    assert_panic!(i16::exact_from(i32::MIN));
    assert_panic!(u32::exact_from(-5i32));
    assert_panic!(i32::exact_from(3000000000u32));
    assert_panic!(i8::exact_from(-1000i16));
    assert_panic!(u8::exact_from(-123.0f32));
    assert_panic!(u8::exact_from(500.0f32));
    assert_panic!(u8::exact_from(123.1f32));
    assert_panic!(u8::exact_from(f32::NAN));
    assert_panic!(u8::exact_from(f32::POSITIVE_INFINITY));
    assert_panic!(u8::exact_from(f32::NEGATIVE_INFINITY));
    assert_panic!(u8::exact_from(256.0f32));
    assert_panic!(i8::exact_from(128.0f32));
    assert_panic!(i8::exact_from(-129.0f32));
    assert_panic!(f32::exact_from(u128::MAX));
    assert_panic!(f32::exact_from(i128::MIN + 1));
    assert_panic!(f32::exact_from(u32::MAX));
    assert_panic!(f32::exact_from(i32::MIN + 1));
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

fn checked_from_and_exact_from_helper_unsigned_primitive_float<
    T: CheckedFrom<U> + PrimitiveUnsigned + RoundingFrom<U>,
    U: CheckedFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>() {
    primitive_float_gen::<U>().test_properties(|f| {
        let result = T::checked_from(f);
        if let Some(u) = result {
            assert_eq!(u, T::exact_from(f));
            assert_eq!(f, U::exact_from(u));
        }
    });

    primitive_float_gen_var_13::<U, T>().test_properties(|f| {
        let u = T::exact_from(f);
        assert_eq!(U::exact_from(u), f);
        assert_eq!(T::checked_from(f).unwrap(), u);
        assert_eq!(T::rounding_from(f, RoundingMode::Exact), u);
    });
}

fn checked_from_and_exact_from_helper_signed_primitive_float<
    T: CheckedFrom<U> + PrimitiveSigned + RoundingFrom<U>,
    U: CheckedFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>() {
    primitive_float_gen::<U>().test_properties(|f| {
        let result = T::checked_from(f);
        if let Some(i) = result {
            assert_eq!(i, T::exact_from(f));
            assert_eq!(f, U::exact_from(i));
        }
    });

    primitive_float_gen_var_14::<U, T>().test_properties(|f| {
        let i = T::exact_from(f);
        assert_eq!(U::exact_from(i), f);
        assert_eq!(T::checked_from(f).unwrap(), i);
        assert_eq!(T::rounding_from(f, RoundingMode::Exact), i);
    });
}

fn checked_from_and_exact_from_helper_primitive_float_unsigned<
    T: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: CheckedFrom<T> + PrimitiveUnsigned + RoundingFrom<T>,
>() {
    unsigned_gen::<U>().test_properties(|u| {
        let result = T::checked_from(u);
        if let Some(f) = result {
            assert_eq!(f, T::exact_from(u));
            assert_eq!(u, U::exact_from(f));
        }
    });

    unsigned_gen_var_18::<U, T>().test_properties(|u| {
        let f = T::exact_from(u);
        assert_eq!(U::exact_from(f), u);
        assert_eq!(T::checked_from(u).unwrap(), f);
        assert_eq!(T::rounding_from(u, RoundingMode::Exact), f);
    });
}

fn checked_from_and_exact_from_helper_primitive_float_signed<
    T: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: CheckedFrom<T> + PrimitiveSigned + RoundingFrom<T>,
>() {
    signed_gen::<U>().test_properties(|i| {
        let result = T::checked_from(i);
        if let Some(f) = result {
            assert_eq!(f, T::exact_from(i));
            assert_eq!(i, U::exact_from(f));
        }
    });

    signed_gen_var_7::<U, T>().test_properties(|i| {
        let f = T::exact_from(i);
        assert_eq!(U::exact_from(f), i);
        assert_eq!(T::checked_from(i).unwrap(), f);
        assert_eq!(T::rounding_from(i, RoundingMode::Exact), f);
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
    apply_fn_to_unsigneds_and_primitive_floats!(
        checked_from_and_exact_from_helper_unsigned_primitive_float
    );
    apply_fn_to_signeds_and_primitive_floats!(
        checked_from_and_exact_from_helper_signed_primitive_float
    );
    apply_fn_to_primitive_floats_and_unsigneds!(
        checked_from_and_exact_from_helper_primitive_float_unsigned
    );
    apply_fn_to_primitive_floats_and_signeds!(
        checked_from_and_exact_from_helper_primitive_float_signed
    );
}
