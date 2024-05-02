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
use malachite_base::num::conversion::traits::{ConvertibleFrom, OverflowingFrom, WrappingFrom};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use std::fmt::Debug;

#[test]
pub fn test_overflowing_from() {
    fn test_single<T: Copy + Debug + Eq + OverflowingFrom<T>>(n: T) {
        assert_eq!(T::overflowing_from(n), (n, false));
    }
    test_single(0u8);
    test_single(5u64);
    test_single(1000u32);
    test_single(123u8);
    test_single(-123i16);
    test_single(i64::MIN);
    test_single(usize::MAX);

    fn test_double<T, U: Copy + Debug + Eq + OverflowingFrom<T>>(n_in: T, n_out: (U, bool)) {
        assert_eq!(U::overflowing_from(n_in), n_out);
    }
    test_double(0u8, (0u16, false));
    test_double(1000u16, (1000i32, false));
    test_double(-5i16, (-5i8, false));
    test_double(255u8, (255u64, false));

    test_double(-1i8, (u32::MAX, true));
    test_double(u32::MAX, (u16::MAX, true));
    test_double(i32::MIN, (0x80000000u32, true));
    test_double(i32::MIN, (0u16, true));
    test_double(i32::MIN, (0i16, true));
    test_double(-5i32, (0xfffffffbu32, true));
    test_double(3000000000u32, (-1294967296i32, true));
    test_double(-1000i16, (24i8, true));
}

fn overflowing_from_helper_primitive_int_unsigned<
    T: ConvertibleFrom<U> + OverflowingFrom<U> + WrappingFrom<U> + PrimitiveInt,
    U: PrimitiveUnsigned,
>() {
    unsigned_gen::<U>().test_properties(|u| {
        let result = T::overflowing_from(u);
        assert_eq!(result, (T::wrapping_from(u), !T::convertible_from(u)));
    });
}

fn overflowing_from_helper_primitive_int_signed<
    T: ConvertibleFrom<U> + OverflowingFrom<U> + WrappingFrom<U> + PrimitiveInt,
    U: PrimitiveSigned,
>() {
    signed_gen::<U>().test_properties(|i| {
        let result = T::overflowing_from(i);
        assert_eq!(result, (T::wrapping_from(i), !T::convertible_from(i)));
    });
}

#[test]
fn overflowing_from_properties() {
    apply_fn_to_primitive_ints_and_unsigneds!(overflowing_from_helper_primitive_int_unsigned);
    apply_fn_to_primitive_ints_and_signeds!(overflowing_from_helper_primitive_int_signed);
}
