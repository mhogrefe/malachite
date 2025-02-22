// Copyright © 2025 Mikhail Hogrefe
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
fn test_wrapping_add_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, out: T) {
        assert_eq!(x.wrapping_add_mul(y, z), out);

        let mut x = x;
        x.wrapping_add_mul_assign(y, z);
        assert_eq!(x, out);
    }
    test::<u8>(2, 3, 7, 23);
    test::<u32>(7, 5, 10, 57);
    test::<u64>(123, 456, 789, 359907);
    test::<i32>(123, -456, 789, -359661);
    test::<i128>(-123, 456, 789, 359661);
    test::<i8>(127, -2, 100, -73);
    test::<i8>(-127, 2, 100, 73);
    test::<i8>(-128, 1, 0, -128);

    test::<u8>(2, 20, 20, 146);
    test::<i8>(-127, -2, 100, -71);
    test::<i8>(127, 1, 100, -29);
    test::<i8>(-127, -1, 100, 29);
    test::<i8>(-127, -10, 100, -103);
}

fn wrapping_add_mul_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        let result = x.wrapping_add_mul(y, z);

        let mut x_alt = x;
        x_alt.wrapping_add_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.wrapping_add_mul(z, y), result);
        assert_eq!(result.wrapping_sub_mul(y, z), x);
        assert_eq!(x.overflowing_add_mul(y, z).0, result);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(a, b)| {
        assert_eq!(a.wrapping_add_mul(T::ZERO, b), a);
        assert_eq!(a.wrapping_add_mul(T::ONE, b), a.wrapping_add(b));
        assert_eq!(T::ZERO.wrapping_add_mul(a, b), a.wrapping_mul(b));
        assert_eq!(a.wrapping_add_mul(b, T::ZERO), a);
        assert_eq!(a.wrapping_add_mul(b, T::ONE), a.wrapping_add(b));
    });
}

fn wrapping_add_mul_properties_helper_signed<T: PrimitiveSigned>() {
    signed_triple_gen::<T>().test_properties(|(x, y, z)| {
        let result = x.wrapping_add_mul(y, z);

        let mut x_alt = x;
        x_alt.wrapping_add_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.wrapping_add_mul(z, y), result);
        assert_eq!(result.wrapping_sub_mul(y, z), x);
        assert_eq!(x.overflowing_add_mul(y, z).0, result);
    });

    signed_pair_gen::<T>().test_properties(|(a, b)| {
        assert_eq!(a.wrapping_add_mul(T::ZERO, b), a);
        assert_eq!(a.wrapping_add_mul(T::ONE, b), a.wrapping_add(b));
        assert_eq!(T::ZERO.wrapping_add_mul(a, b), a.wrapping_mul(b));
        assert_eq!(a.wrapping_add_mul(b, T::ZERO), a);
        assert_eq!(a.wrapping_add_mul(b, T::ONE), a.wrapping_add(b));
    });
}

#[test]
fn wrapping_add_mul_properties() {
    apply_fn_to_unsigneds!(wrapping_add_mul_properties_helper_unsigned);
    apply_fn_to_signeds!(wrapping_add_mul_properties_helper_signed);
}
