// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{
    primitive_float_gen, signed_gen_var_10, unsigned_gen_var_21,
};

#[test]
fn test_abs_squared() {
    fn test_u<T: PrimitiveUnsigned>(x: T, out: T) {
        assert_eq!(x.abs_squared(), out);
    }
    test_u::<u8>(0, 0);
    test_u::<u8>(1, 1);
    test_u::<u8>(15, 225);
    test_u::<u32>(1000, 1000000);

    fn test_i<T: PrimitiveSigned>(x: T, out: T) {
        assert_eq!(x.abs_squared(), out);
    }
    test_i::<i8>(0, 0);
    test_i::<i8>(11, 121);
    test_i::<i8>(-11, 121);
    test_i::<i32>(-1000, 1000000);

    fn test_f<T: PrimitiveFloat>(x: T, out: T) {
        assert_eq!(NiceFloat(x.abs_squared()), NiceFloat(out));
    }
    test_f::<f32>(0.0, 0.0);
    test_f::<f32>(-0.0, 0.0);
    test_f::<f32>(1.5, 2.25);
    test_f::<f32>(-1.5, 2.25);
    test_f::<f64>(f64::NAN, f64::NAN);
    test_f::<f64>(f64::INFINITY, f64::INFINITY);
    test_f::<f64>(f64::NEGATIVE_INFINITY, f64::INFINITY);
}

fn abs_squared_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen_var_21::<T>().test_properties(|x| {
        assert_eq!(x.abs_squared(), x.square());
    });
}

fn abs_squared_properties_helper_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() {
    signed_gen_var_10::<U, S>().test_properties(|x| {
        let abs_squared = x.abs_squared();
        assert_eq!(abs_squared, x.square());
        if x != S::MIN {
            assert_eq!((-x).abs_squared(), abs_squared);
        }
        assert!(abs_squared >= S::ZERO);
    });
}

fn abs_squared_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let abs_squared = x.abs_squared();
        assert_eq!(NiceFloat(abs_squared), NiceFloat(x.square()));
        assert_eq!(NiceFloat((-x).abs_squared()), NiceFloat(abs_squared));
        assert!(x.is_nan() || abs_squared >= T::ZERO);
    });
}

#[test]
fn abs_squared_properties() {
    apply_fn_to_unsigneds!(abs_squared_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(abs_squared_properties_helper_signed);
    apply_fn_to_primitive_floats!(abs_squared_properties_helper_primitive_float);
}
