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
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{
    primitive_float_triple_gen, signed_pair_gen, signed_triple_gen_var_2, unsigned_pair_gen_var_27,
    unsigned_triple_gen_var_2,
};

#[test]
fn test_sub_mul() {
    fn test_i<T: PrimitiveInt>(x: T, y: T, z: T, out: T) {
        assert_eq!(x.sub_mul(y, z), out);

        let mut x = x;
        x.sub_mul_assign(y, z);
        assert_eq!(x, out);
    }
    test_i::<u8>(100, 3, 7, 79);
    test_i::<u32>(60, 5, 10, 10);
    test_i::<u64>(1000000, 456, 789, 640216);
    test_i::<i32>(123, -456, 789, 359907);
    test_i::<i128>(-123, 456, 789, -359907);
    test_i::<i8>(127, 2, 100, -73);
    test_i::<i8>(-127, -2, 100, 73);
    test_i::<i8>(-128, 1, 0, -128);

    fn test_f<T: PrimitiveFloat>(x: T, y: T, z: T, out: T) {
        assert_eq!(NiceFloat(x.sub_mul(y, z)), NiceFloat(out));

        let mut x = x;
        x.sub_mul_assign(y, z);
        assert_eq!(NiceFloat(x), NiceFloat(out));
    }
    test_f::<f32>(1.0, 2.0, 3.0, -5.0);
    test_f::<f32>(1.0, f32::INFINITY, 2.0, f32::NEGATIVE_INFINITY);
    test_f::<f32>(f32::NAN, 1.0, 2.0, f32::NAN);
}

fn sub_mul_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_2::<T>().test_properties(|(x, y, z)| {
        let result = x.sub_mul(y, z);

        let mut x_alt = x;
        x_alt.sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.sub_mul(z, y), result);
        assert_eq!(result.add_mul(y, z), x);
        assert_eq!(x.checked_sub_mul(y, z), Some(result));
        assert_eq!(x.saturating_sub_mul(y, z), result);
        assert_eq!(x.wrapping_sub_mul(y, z), result);
        assert_eq!(x.overflowing_sub_mul(y, z), (result, false));
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(a, b)| {
        assert_eq!(a.sub_mul(T::ZERO, b), a);
        assert_eq!(a.sub_mul(b, T::ZERO), a);
    });
}

fn sub_mul_properties_helper_signed<T: PrimitiveSigned>() {
    signed_triple_gen_var_2::<T>().test_properties(|(x, y, z)| {
        let result = x.sub_mul(y, z);

        let mut x_alt = x;
        x_alt.sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.sub_mul(z, y), result);
        assert_eq!(result.add_mul(y, z), x);
        assert_eq!(x.checked_sub_mul(y, z), Some(result));
        assert_eq!(x.saturating_sub_mul(y, z), result);
        assert_eq!(x.wrapping_sub_mul(y, z), result);
        assert_eq!(x.overflowing_sub_mul(y, z), (result, false));
    });

    signed_pair_gen::<T>().test_properties(|(a, b)| {
        assert_eq!(a.sub_mul(T::ZERO, b), a);
        assert_eq!(a.sub_mul(b, T::ZERO), a);
    });
}

fn sub_mul_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_triple_gen::<T>().test_properties(|(x, y, z)| {
        let result = x.sub_mul(y, z);

        let mut x_alt = x;
        x_alt.sub_mul_assign(y, z);
        assert_eq!(NiceFloat(x_alt), NiceFloat(result));

        assert_eq!(NiceFloat(x.sub_mul(z, y)), NiceFloat(result));
    });
}

#[test]
fn sub_mul_properties() {
    apply_fn_to_unsigneds!(sub_mul_properties_helper_unsigned);
    apply_fn_to_signeds!(sub_mul_properties_helper_signed);
    apply_fn_to_primitive_floats!(sub_mul_properties_helper_primitive_float);
}
