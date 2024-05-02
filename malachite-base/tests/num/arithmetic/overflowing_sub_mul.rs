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
use malachite_base::test_util::generators::{
    signed_pair_gen, signed_triple_gen, unsigned_pair_gen_var_27, unsigned_triple_gen_var_19,
};

#[test]
fn test_overflowing_sub_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, out: T, overflow: bool) {
        assert_eq!(x.overflowing_sub_mul(y, z), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_sub_mul_assign(y, z), overflow);
        assert_eq!(x, out);
    }
    test::<u8>(100, 3, 7, 79, false);
    test::<u32>(60, 5, 10, 10, false);
    test::<u64>(1000000, 456, 789, 640216, false);
    test::<i32>(123, -456, 789, 359907, false);
    test::<i128>(-123, 456, 789, -359907, false);
    test::<i8>(127, 2, 100, -73, false);
    test::<i8>(-127, -2, 100, 73, false);
    test::<i8>(-128, 1, 0, -128, false);

    test::<u8>(2, 10, 5, 208, true);
    test::<i8>(-127, 2, 100, -71, true);
    test::<i8>(-127, 1, 100, 29, true);
    test::<i8>(127, -1, 100, -29, true);
    test::<i8>(127, -10, 100, 103, true);
}

fn overflowing_sub_mul_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        let (result, overflow) = x.overflowing_sub_mul(y, z);

        let mut x_alt = x;
        assert_eq!(x_alt.overflowing_sub_mul_assign(y, z), overflow);
        assert_eq!(x_alt, result);

        assert_eq!(x.overflowing_sub_mul(z, y), (result, overflow));
        assert_eq!(result.overflowing_add_mul(y, z), (x, overflow));
        assert_eq!(x.wrapping_sub_mul(y, z), result);
        assert_eq!(x.checked_sub_mul(y, z).is_none(), overflow);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(a, b)| {
        assert_eq!(a.overflowing_sub_mul(T::ZERO, b), (a, false));
        assert_eq!(a.overflowing_sub_mul(T::ONE, b), a.overflowing_sub(b));
        assert_eq!(a.overflowing_sub_mul(b, T::ZERO), (a, false));
        assert_eq!(a.overflowing_sub_mul(b, T::ONE), a.overflowing_sub(b));
    });
}

fn overflowing_sub_mul_properties_helper_signed<T: PrimitiveSigned>() {
    signed_triple_gen::<T>().test_properties(|(x, y, z)| {
        let (result, overflow) = x.overflowing_sub_mul(y, z);

        let mut x_alt = x;
        assert_eq!(x_alt.overflowing_sub_mul_assign(y, z), overflow);
        assert_eq!(x_alt, result);

        assert_eq!(x.overflowing_sub_mul(z, y), (result, overflow));
        assert_eq!(result.overflowing_add_mul(y, z), (x, overflow));
        assert_eq!(x.wrapping_sub_mul(y, z), result);
        assert_eq!(x.checked_sub_mul(y, z).is_none(), overflow);
    });

    signed_pair_gen::<T>().test_properties(|(a, b)| {
        assert_eq!(a.overflowing_sub_mul(T::ZERO, b), (a, false));
        assert_eq!(a.overflowing_sub_mul(T::ONE, b), a.overflowing_sub(b));
        assert_eq!(a.overflowing_sub_mul(b, T::ZERO), (a, false));
        assert_eq!(a.overflowing_sub_mul(b, T::ONE), a.overflowing_sub(b));
    });
}

#[test]
fn overflowing_sub_mul_properties() {
    apply_fn_to_unsigneds!(overflowing_sub_mul_properties_helper_unsigned);
    apply_fn_to_signeds!(overflowing_sub_mul_properties_helper_signed);
}
