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
fn test_wrapping_sub_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, out: T) {
        assert_eq!(x.wrapping_sub_mul(y, z), out);

        let mut x = x;
        x.wrapping_sub_mul_assign(y, z);
        assert_eq!(x, out);
    }
    test::<u8>(100, 3, 7, 79);
    test::<u32>(60, 5, 10, 10);
    test::<u64>(1000000, 456, 789, 640216);
    test::<i32>(123, -456, 789, 359907);
    test::<i128>(-123, 456, 789, -359907);
    test::<i8>(127, 2, 100, -73);
    test::<i8>(-127, -2, 100, 73);
    test::<i8>(-128, 1, 0, -128);

    test::<u8>(2, 10, 5, 208);
    test::<i8>(-127, 2, 100, -71);
    test::<i8>(-127, 1, 100, 29);
    test::<i8>(127, -1, 100, -29);
    test::<i8>(127, -10, 100, 103);
}

fn wrapping_sub_mul_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        let result = x.wrapping_sub_mul(y, z);

        let mut x_alt = x;
        x_alt.wrapping_sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.wrapping_sub_mul(z, y), result);
        assert_eq!(result.wrapping_add_mul(y, z), x);
        assert_eq!(x.overflowing_sub_mul(y, z).0, result);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(a, b)| {
        assert_eq!(a.wrapping_sub_mul(T::ZERO, b), a);
        assert_eq!(a.wrapping_sub_mul(T::ONE, b), a.wrapping_sub(b));
        assert_eq!(
            T::ZERO.wrapping_sub_mul(a, b),
            a.wrapping_mul(b).wrapping_neg()
        );
        assert_eq!(a.wrapping_sub_mul(b, T::ZERO), a);
        assert_eq!(a.wrapping_sub_mul(b, T::ONE), a.wrapping_sub(b));
    });
}

fn wrapping_sub_mul_properties_helper_signed<T: PrimitiveSigned>() {
    signed_triple_gen::<T>().test_properties(|(x, y, z)| {
        let result = x.wrapping_sub_mul(y, z);

        let mut x_alt = x;
        x_alt.wrapping_sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.wrapping_sub_mul(z, y), result);
        assert_eq!(result.wrapping_add_mul(y, z), x);
        assert_eq!(x.overflowing_sub_mul(y, z).0, result);
    });

    signed_pair_gen::<T>().test_properties(|(a, b)| {
        assert_eq!(a.wrapping_sub_mul(T::ZERO, b), a);
        assert_eq!(a.wrapping_sub_mul(T::ONE, b), a.wrapping_sub(b));
        assert_eq!(
            T::ZERO.wrapping_sub_mul(a, b),
            a.wrapping_mul(b).wrapping_neg()
        );
        assert_eq!(a.wrapping_sub_mul(b, T::ZERO), a);
        assert_eq!(a.wrapping_sub_mul(b, T::ONE), a.wrapping_sub(b));
    });
}

#[test]
fn wrapping_sub_mul_properties() {
    apply_fn_to_unsigneds!(wrapping_sub_mul_properties_helper_unsigned);
    apply_fn_to_signeds!(wrapping_sub_mul_properties_helper_signed);
}
