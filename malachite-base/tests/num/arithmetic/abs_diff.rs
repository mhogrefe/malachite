// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsDiff, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{
    signed_gen, signed_pair_gen, unsigned_gen, unsigned_pair_gen_var_27,
};
use std::cmp::{max, min};

#[test]
fn test_abs_diff() {
    fn test_unsigned<T: PrimitiveUnsigned>(x: T, y: T, out: T) {
        assert_eq!(x.abs_diff(y), out);

        let mut x = x;
        x.abs_diff_assign(y);
        assert_eq!(x, out);
    }
    test_unsigned::<u8>(1, 100, 99);
    test_unsigned::<u8>(100, 1, 99);
    test_unsigned::<u16>(10, 10, 0);
    test_unsigned::<u32>(0, u32::MAX, u32::MAX);
    test_unsigned::<u32>(u32::MAX, 0, u32::MAX);
    test_unsigned::<u32>(u32::MAX, u32::MAX, 0);

    fn test_signed<T: PrimitiveSigned + AbsDiff<Output = U>, U: PrimitiveUnsigned>(
        x: T,
        y: T,
        out: U,
    ) {
        assert_eq!(x.abs_diff(y), out);
    }
    test_signed::<i8, _>(1, 100, 99);
    test_signed::<i8, _>(1, -100, 101);
    test_signed::<i8, _>(-1, 100, 101);
    test_signed::<i8, _>(-1, -100, 99);
    test_signed::<i8, _>(100, 1, 99);
    test_signed::<i8, _>(100, -1, 101);
    test_signed::<i8, _>(-100, 1, 101);
    test_signed::<i8, _>(-100, -1, 99);
    test_signed::<i16, _>(10, 10, 0);
    test_signed::<i16, _>(10, -10, 20);
    test_signed::<i16, _>(-10, 10, 20);
    test_signed::<i16, _>(-10, -10, 0);
    test_signed::<i32, _>(0, i32::MAX, u32::exact_from(i32::MAX));
    test_signed::<i32, _>(0, -i32::MAX, u32::exact_from(i32::MAX));
    test_signed::<i32, _>(i32::MAX, 0, u32::exact_from(i32::MAX));
    test_signed::<i32, _>(-i32::MAX, 0, u32::exact_from(i32::MAX));
    test_signed::<i32, _>(i32::MAX, i32::MAX, 0);
    test_signed::<i32, _>(i32::MAX, -i32::MAX, u32::MAX - 1);
    test_signed::<i32, _>(-i32::MAX, i32::MAX, u32::MAX - 1);
    test_signed::<i32, _>(-i32::MAX, -i32::MAX, 0);

    test_signed::<i64, _>(i64::MIN, i64::MAX, u64::MAX);
    test_signed::<i64, _>(i64::MIN, -i64::MAX, 1);
}

fn abs_diff_properties_unsigned_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let diff = x.abs_diff(y);
        let mut mut_x = x;
        mut_x.abs_diff_assign(y);
        assert_eq!(mut_x, diff);

        assert_eq!(y.abs_diff(x), diff);
        assert_eq!(max(x, y) - min(x, y), diff);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.abs_diff(T::ZERO), x);
        assert_eq!(T::ZERO.abs_diff(x), x);
        assert_eq!(x.abs_diff(x), T::ZERO);
    });
}

fn abs_diff_properties_signed_helper<
    U: PrimitiveUnsigned,
    T: AbsDiff<Output = U> + UnsignedAbs<Output = U> + PrimitiveSigned,
>() {
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let diff = x.abs_diff(y);
        assert_eq!(y.abs_diff(x), diff);
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.abs_diff(T::ZERO), x.unsigned_abs());
        assert_eq!(T::ZERO.abs_diff(x), x.unsigned_abs());
        assert_eq!(x.abs_diff(x), U::ZERO);
    });
}

#[test]
fn abs_diff_properties() {
    apply_fn_to_unsigneds!(abs_diff_properties_unsigned_helper);
    apply_fn_to_unsigned_signed_pairs!(abs_diff_properties_signed_helper);
}
