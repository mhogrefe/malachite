// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::from::{
    PrimitiveFloatFromSignedError, PrimitiveFloatFromUnsignedError, SignedFromFloatError,
    UnsignedFromFloatError,
};
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_gen_var_13, primitive_float_gen_var_14, signed_gen,
    signed_gen_var_7, unsigned_gen, unsigned_gen_var_18,
};
use std::fmt::Debug;
use std::panic::catch_unwind;

#[allow(clippy::needless_pass_by_value)]
#[test]
pub fn test_try_from() {
    fn test_double_primitive_int<
        T: PrimitiveFloat,
        U: TryFrom<NiceFloat<T>, Error = E> + Copy + Debug + Eq,
        E: Debug + Eq,
    >(
        n_in: T,
        n_out: Result<U, E>,
    ) {
        assert_eq!(U::try_from(NiceFloat(n_in)), n_out);
    }
    test_double_primitive_int::<_, u8, _>(0.0f32, Ok(0));
    test_double_primitive_int::<_, u8, _>(-0.0f32, Ok(0));
    test_double_primitive_int::<_, u8, _>(123.0f32, Ok(123));
    test_double_primitive_int::<_, i8, _>(-123.0f32, Ok(-123));
    test_double_primitive_int::<_, u8, _>(-123.0f32, Err(UnsignedFromFloatError::FloatNegative));
    test_double_primitive_int::<_, u8, _>(
        500.0f32,
        Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange),
    );
    test_double_primitive_int::<_, u8, _>(
        123.1f32,
        Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange),
    );
    test_double_primitive_int::<_, u8, _>(
        f32::NAN,
        Err(UnsignedFromFloatError::FloatInfiniteOrNan),
    );
    test_double_primitive_int::<_, u8, _>(
        f32::INFINITY,
        Err(UnsignedFromFloatError::FloatInfiniteOrNan),
    );
    test_double_primitive_int::<_, u8, _>(
        f32::NEGATIVE_INFINITY,
        Err(UnsignedFromFloatError::FloatInfiniteOrNan),
    );
    test_double_primitive_int::<_, u8, _>(255.0f32, Ok(255));
    test_double_primitive_int::<_, u8, _>(
        256.0f32,
        Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange),
    );
    test_double_primitive_int::<_, i8, _>(127.0f32, Ok(127));
    test_double_primitive_int::<_, i8, _>(
        128.0f32,
        Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange),
    );
    test_double_primitive_int::<_, i8, _>(-128.0f32, Ok(-128));
    test_double_primitive_int::<_, i8, _>(
        -129.0f32,
        Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange),
    );

    fn test_double_primitive_float<T, U: PrimitiveFloat, E: Debug + Eq>(
        n_in: T,
        n_out: Result<U, E>,
    ) where
        NiceFloat<U>: TryFrom<T, Error = E>,
    {
        assert_eq!(NiceFloat::<U>::try_from(n_in), n_out.map(NiceFloat));
    }
    test_double_primitive_float::<_, f32, _>(0u8, Ok(0.0));
    test_double_primitive_float::<_, f32, _>(123u8, Ok(123.0));
    test_double_primitive_float::<_, f32, _>(-123i8, Ok(-123.0));
    test_double_primitive_float::<_, f32, _>(u128::MAX, Err(PrimitiveFloatFromUnsignedError));
    test_double_primitive_float::<_, f32, _>(i128::MIN, Ok(-1.7014118e38));
    test_double_primitive_float::<_, f32, _>(i128::MIN + 1, Err(PrimitiveFloatFromSignedError));
    test_double_primitive_float::<_, f32, _>(u32::MAX, Err(PrimitiveFloatFromUnsignedError));
    test_double_primitive_float::<_, f32, _>(i32::MIN, Ok(-2147483600.0));
    test_double_primitive_float::<_, f32, _>(i32::MIN + 1, Err(PrimitiveFloatFromSignedError));
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

    fn test_double_primitive_int<
        T: PrimitiveFloat,
        U: Copy + Debug + Eq + TryFrom<NiceFloat<T>>,
    >(
        n_in: T,
        n_out: U,
    ) {
        assert_eq!(U::exact_from(NiceFloat(n_in)), n_out);
    }
    test_double_primitive_int(0.0f32, 0u8);
    test_double_primitive_int(-0.0f32, 0u8);
    test_double_primitive_int(123.0f32, 123u8);
    test_double_primitive_int(-123.0f32, -123i8);
    test_double_primitive_int(255.0f32, 255u8);
    test_double_primitive_int(127.0f32, 127i8);
    test_double_primitive_int(-128.0f32, -128i8);

    fn test_double_primitive_float<T, U: PrimitiveFloat>(n_in: T, n_out: U)
    where
        NiceFloat<U>: TryFrom<T>,
    {
        assert_eq!(NiceFloat::<U>::exact_from(n_in), NiceFloat(n_out));
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
    assert_panic!(u8::exact_from(NiceFloat(-123.0f32)));
    assert_panic!(u8::exact_from(NiceFloat(500.0f32)));
    assert_panic!(u8::exact_from(NiceFloat(123.1f32)));
    assert_panic!(u8::exact_from(NiceFloat(f32::NAN)));
    assert_panic!(u8::exact_from(NiceFloat(f32::INFINITY)));
    assert_panic!(u8::exact_from(NiceFloat(f32::NEGATIVE_INFINITY)));
    assert_panic!(u8::exact_from(NiceFloat(256.0f32)));
    assert_panic!(i8::exact_from(NiceFloat(128.0f32)));
    assert_panic!(i8::exact_from(NiceFloat(-129.0f32)));
    assert_panic!(NiceFloat::<f32>::exact_from(u128::MAX));
    assert_panic!(NiceFloat::<f32>::exact_from(i128::MIN + 1));
    assert_panic!(NiceFloat::<f32>::exact_from(u32::MAX));
    assert_panic!(NiceFloat::<f32>::exact_from(i32::MIN + 1));
}

fn try_from_and_exact_from_helper_unsigned_primitive_float<
    T: TryFrom<NiceFloat<U>, Error = UnsignedFromFloatError> + PrimitiveUnsigned + RoundingFrom<U>,
    U: PrimitiveFloat + RoundingFrom<T>,
>()
where
    NiceFloat<U>: TryFrom<T>,
{
    primitive_float_gen::<U>().test_properties(|f| {
        let f = NiceFloat(f);
        let result = T::try_from(f);
        if let Ok(u) = result {
            assert_eq!(u, T::exact_from(f));
            assert_eq!(
                NiceFloat(f.0.abs_negative_zero()),
                NiceFloat::<U>::exact_from(u)
            );
        }
    });

    primitive_float_gen_var_13::<U, T>().test_properties(|f| {
        let f = NiceFloat(f);
        let u = T::exact_from(f);
        assert_eq!(NiceFloat::<U>::exact_from(u), f);
        assert_eq!(T::try_from(f).unwrap(), u);
        assert_eq!(T::rounding_from(f.0, Exact).0, u);
    });
}

fn try_from_and_exact_from_helper_signed_primitive_float<
    T: TryFrom<NiceFloat<U>, Error = SignedFromFloatError> + PrimitiveSigned + RoundingFrom<U>,
    U: PrimitiveFloat + RoundingFrom<T>,
>()
where
    NiceFloat<U>: TryFrom<T>,
{
    primitive_float_gen::<U>().test_properties(|f| {
        let f = NiceFloat(f);
        let result = T::try_from(f);
        if let Ok(i) = result {
            assert_eq!(i, T::exact_from(f));
            assert_eq!(
                NiceFloat(f.0.abs_negative_zero()),
                NiceFloat::<U>::exact_from(i)
            );
        }
    });

    primitive_float_gen_var_14::<U, T>().test_properties(|f| {
        let f = NiceFloat(f);
        let i = T::exact_from(f);
        assert_eq!(NiceFloat::<U>::exact_from(i), f);
        assert_eq!(T::try_from(f).unwrap(), i);
        assert_eq!(T::rounding_from(f.0, Exact).0, i);
    });
}

fn try_from_and_exact_from_helper_primitive_float_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned + RoundingFrom<T> + TryFrom<NiceFloat<T>>,
>()
where
    NiceFloat<T>: TryFrom<U, Error = PrimitiveFloatFromUnsignedError>,
{
    unsigned_gen::<U>().test_properties(|u| {
        let result = NiceFloat::<T>::try_from(u);
        if let Ok(f) = result {
            assert_eq!(f, NiceFloat::<T>::exact_from(u));
            assert_eq!(u, U::exact_from(f));
        }
    });

    unsigned_gen_var_18::<U, T>().test_properties(|u| {
        let f = NiceFloat::<T>::exact_from(u);
        assert_eq!(U::exact_from(f), u);
        assert_eq!(NiceFloat::<T>::try_from(u).unwrap(), f);
        assert_eq!(NiceFloat(T::rounding_from(u, Exact).0), f);
    });
}

fn try_from_and_exact_from_helper_primitive_float_signed<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned + RoundingFrom<T> + TryFrom<NiceFloat<T>>,
>()
where
    NiceFloat<T>: TryFrom<U, Error = PrimitiveFloatFromSignedError>,
{
    signed_gen::<U>().test_properties(|i| {
        let result = NiceFloat::<T>::try_from(i);
        if let Ok(f) = result {
            assert_eq!(f, NiceFloat::<T>::exact_from(i));
            assert_eq!(i, U::exact_from(f));
        }
    });

    signed_gen_var_7::<U, T>().test_properties(|i| {
        let f = NiceFloat::<T>::exact_from(i);
        assert_eq!(U::exact_from(f), i);
        assert_eq!(NiceFloat::<T>::try_from(i).unwrap(), f);
        assert_eq!(NiceFloat(T::rounding_from(i, Exact).0), f);
    });
}

#[test]
fn try_from_and_exact_from_properties() {
    apply_fn_to_unsigneds_and_primitive_floats!(
        try_from_and_exact_from_helper_unsigned_primitive_float
    );
    apply_fn_to_signeds_and_primitive_floats!(
        try_from_and_exact_from_helper_signed_primitive_float
    );
    apply_fn_to_primitive_floats_and_unsigneds!(
        try_from_and_exact_from_helper_primitive_float_unsigned
    );
    apply_fn_to_primitive_floats_and_signeds!(
        try_from_and_exact_from_helper_primitive_float_signed
    );
}
