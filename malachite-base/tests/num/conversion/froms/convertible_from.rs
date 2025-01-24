// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};
use std::fmt::Debug;

#[test]
pub fn test_convertible_from() {
    fn test_single<T: ConvertibleFrom<T> + Copy + Debug>(n: T) {
        assert!(T::convertible_from(n));
    }
    test_single(0u8);
    test_single(5u64);
    test_single(1000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: ConvertibleFrom<T>>(n_in: T, convertible: bool) {
        assert_eq!(U::convertible_from(n_in), convertible);
    }
    test_double::<_, u16>(0u8, true);
    test_double::<_, i32>(1000u16, true);
    test_double::<_, i8>(-5i16, true);
    test_double::<_, u64>(255u8, true);

    test_double::<_, u32>(-1i8, false);
    test_double::<_, u16>(u32::MAX, false);
    test_double::<_, u32>(i32::MIN, false);
    test_double::<_, u16>(i32::MIN, false);
    test_double::<_, i16>(i32::MIN, false);
    test_double::<_, u32>(-5i32, false);
    test_double::<_, i32>(3000000000u32, false);
    test_double::<_, i8>(-1000i16, false);

    test_double::<_, u8>(0.0f32, true);
    test_double::<_, u8>(-0.0f32, true);
    test_double::<_, u8>(123.0f32, true);
    test_double::<_, i8>(-123.0f32, true);
    test_double::<_, u8>(-123.0f32, false);
    test_double::<_, u8>(500.0f32, false);
    test_double::<_, u8>(123.1f32, false);
    test_double::<_, u8>(f32::NAN, false);
    test_double::<_, u8>(f32::INFINITY, false);
    test_double::<_, u8>(f32::NEGATIVE_INFINITY, false);
    test_double::<_, u8>(255.0f32, true);
    test_double::<_, u8>(256.0f32, false);
    test_double::<_, i8>(127.0f32, true);
    test_double::<_, i8>(128.0f32, false);
    test_double::<_, i8>(-128.0f32, true);
    test_double::<_, i8>(-129.0f32, false);

    test_double::<_, f32>(0u8, true);
    test_double::<_, f32>(123u8, true);
    test_double::<_, f32>(-123i8, true);
    test_double::<_, f32>(u128::MAX, false);
    test_double::<_, f32>(i128::MIN, true);
    test_double::<_, f32>(i128::MIN + 1, false);
    test_double::<_, f32>(u32::MAX, false);
    test_double::<_, f32>(i32::MIN, true);
    test_double::<_, f32>(i32::MIN + 1, false);
}

fn convertible_from_helper_primitive_int_unsigned<
    T: TryFrom<U> + ConvertibleFrom<U> + PrimitiveInt,
    U: PrimitiveUnsigned,
>() {
    unsigned_gen::<U>().test_properties(|u| {
        let convertible = T::convertible_from(u);
        assert_eq!(convertible, T::try_from(u).is_ok());
    });
}

fn convertible_from_helper_primitive_int_signed<
    T: TryFrom<U> + ConvertibleFrom<U> + PrimitiveInt,
    U: PrimitiveSigned,
>() {
    signed_gen::<U>().test_properties(|i| {
        let convertible = T::convertible_from(i);
        assert_eq!(convertible, T::try_from(i).is_ok());
    });
}

fn convertible_from_helper_primitive_int_primitive_float<
    T: TryFrom<NiceFloat<U>> + ConvertibleFrom<U> + PrimitiveInt,
    U: PrimitiveFloat,
>() {
    primitive_float_gen::<U>().test_properties(|f| {
        let convertible = T::convertible_from(f);
        assert_eq!(convertible, T::try_from(NiceFloat(f)).is_ok());
    });
}

fn convertible_from_helper_primitive_float_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat,
    U: PrimitiveUnsigned,
>()
where
    NiceFloat<T>: TryFrom<U>,
{
    unsigned_gen::<U>().test_properties(|u| {
        let convertible = T::convertible_from(u);
        assert_eq!(convertible, NiceFloat::<T>::try_from(u).is_ok());
    });
}

fn convertible_from_helper_primitive_float_signed<
    T: ConvertibleFrom<U> + PrimitiveFloat,
    U: PrimitiveSigned,
>()
where
    NiceFloat<T>: TryFrom<U>,
{
    signed_gen::<U>().test_properties(|i| {
        let convertible = T::convertible_from(i);
        assert_eq!(convertible, NiceFloat::<T>::try_from(i).is_ok());
    });
}

#[test]
fn convertible_from_properties() {
    apply_fn_to_primitive_ints_and_unsigneds!(convertible_from_helper_primitive_int_unsigned);
    apply_fn_to_primitive_ints_and_signeds!(convertible_from_helper_primitive_int_signed);
    apply_fn_to_primitive_ints_and_primitive_floats!(
        convertible_from_helper_primitive_int_primitive_float
    );
    apply_fn_to_primitive_floats_and_unsigneds!(convertible_from_helper_primitive_float_unsigned);
    apply_fn_to_primitive_floats_and_signeds!(convertible_from_helper_primitive_float_signed);
}
