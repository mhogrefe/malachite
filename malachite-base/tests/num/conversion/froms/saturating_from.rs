// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, SaturatingFrom};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use std::fmt::Debug;

#[test]
pub fn test_saturating_from() {
    fn test_single<T: Copy + Debug + Eq + SaturatingFrom<T>>(n: T) {
        assert_eq!(T::saturating_from(n), n);
    }
    test_single(0u8);
    test_single(5u64);
    test_single(1000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Copy + Debug + Eq + SaturatingFrom<T>>(n_in: T, n_out: U) {
        assert_eq!(U::saturating_from(n_in), n_out);
    }
    test_double(0u8, 0u16);
    test_double(1000u16, 1000i32);
    test_double(-5i16, -5i8);
    test_double(255u8, 255u64);

    test_double(-1i8, 0u32);
    test_double(u32::MAX, u16::MAX);
    test_double(i32::MIN, 0u32);
    test_double(i32::MIN, 0u16);
    test_double(i32::MIN, i16::MIN);
    test_double(-5i32, 0u32);
    test_double(3000000000u32, i32::MAX);
    test_double(-1000i16, i8::MIN);
}

fn saturating_from_helper_primitive_int_unsigned<
    T: TryFrom<U> + ConvertibleFrom<U> + SaturatingFrom<U> + PrimitiveInt,
    U: TryFrom<T> + PrimitiveUnsigned,
>() {
    unsigned_gen::<U>().test_properties(|u| {
        let result = T::saturating_from(u);
        if let Ok(u_u) = T::try_from(u) {
            assert_eq!(result, u_u);
        }
        if let Ok(result_t) = U::try_from(result) {
            assert!(result_t.le_abs(&u));
            assert_eq!(result_t == u, T::convertible_from(u));
        }
    });
}

fn saturating_from_helper_primitive_int_signed<
    T: TryFrom<U> + ConvertibleFrom<U> + SaturatingFrom<U> + PrimitiveInt,
    U: TryFrom<T> + PrimitiveSigned,
>() {
    signed_gen::<U>().test_properties(|i| {
        let result = T::saturating_from(i);
        if let Ok(i_u) = T::try_from(i) {
            assert_eq!(result, i_u);
        }
        if let Ok(result_t) = U::try_from(result) {
            assert!(result_t.le_abs(&i));
            assert_eq!(result_t == i, T::convertible_from(i));
        }
    });
}

#[test]
fn saturating_from_properties() {
    apply_fn_to_primitive_ints_and_unsigneds!(saturating_from_helper_primitive_int_unsigned);
    apply_fn_to_primitive_ints_and_signeds!(saturating_from_helper_primitive_int_signed);
}
