// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{
    primitive_float_pair_gen, primitive_float_signed_pair_gen, signed_unsigned_pair_gen_var_15,
    unsigned_pair_gen_var_29,
};

#[test]
fn test_pow() {
    fn test_primitive_int<T: PrimitiveInt>(x: T, y: u64, out: T) {
        assert_eq!(x.pow(y), out);

        let mut x = x;
        x.pow_assign(y);
        assert_eq!(x, out);
    }
    test_primitive_int::<u8>(0, 0, 1);
    test_primitive_int::<u64>(123, 0, 1);
    test_primitive_int::<u64>(123, 1, 123);
    test_primitive_int::<u16>(0, 123, 0);
    test_primitive_int::<u16>(1, 123, 1);
    test_primitive_int::<i16>(-1, 123, -1);
    test_primitive_int::<i16>(-1, 124, 1);
    test_primitive_int::<u8>(3, 3, 27);
    test_primitive_int::<i32>(-10, 9, -1000000000);
    test_primitive_int::<i32>(-10, 8, 100000000);

    fn test_i64_primitive_float<T: PrimitiveFloat>(x: T, y: i64, out: T) {
        assert_eq!(NiceFloat(x.pow(y)), NiceFloat(out));

        let mut x = x;
        x.pow_assign(y);
        assert_eq!(NiceFloat(x), NiceFloat(out));
    }
    test_i64_primitive_float::<f32>(2.0, 5, 32.0);

    fn test_primitive_float_primitive_float<T: PrimitiveFloat>(x: T, y: T, out: T) {
        assert_eq!(NiceFloat(x.pow(y)), NiceFloat(out));

        let mut x = x;
        x.pow_assign(y);
        assert_eq!(NiceFloat(x), NiceFloat(out));
    }
    test_primitive_float_primitive_float::<f32>(2.0, 5.0, 32.0);
}

fn pow_assign_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_29::<T>().test_properties(|(x, y)| {
        let mut power = x;
        power.pow_assign(y);
        assert_eq!(power, x.pow(y));
        if x > T::ONE {
            assert_eq!(power.checked_log_base(x), Some(y));
        }
        if y != 0 {
            assert_eq!(power.checked_root(y), Some(x));
        }
    });
}

fn pow_assign_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_15::<T>().test_properties(|(x, y)| {
        let mut power = x;
        power.pow_assign(y);
        assert_eq!(power, x.pow(y));
        if y != 0 {
            assert_eq!(
                power.checked_root(y),
                Some(if y.even() { x.abs() } else { x })
            );
        }
    });
}

fn pow_assign_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_signed_pair_gen::<T, i64>().test_properties(|(x, y)| {
        let mut power = x;
        power.pow_assign(y);
        assert_eq!(NiceFloat(power), NiceFloat(x.pow(y)));
    });

    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let mut power = x;
        power.pow_assign(y);
        assert_eq!(NiceFloat(power), NiceFloat(x.pow(y)));
    });
}

#[test]
fn pow_assign_properties() {
    apply_fn_to_unsigneds!(pow_assign_properties_helper_unsigned);
    apply_fn_to_signeds!(pow_assign_properties_helper_signed);
    apply_fn_to_primitive_floats!(pow_assign_properties_helper_primitive_float);
}
