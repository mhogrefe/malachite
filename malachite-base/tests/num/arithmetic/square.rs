// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{
    primitive_float_gen, signed_gen_var_10, unsigned_gen_var_21,
};

#[test]
fn test_square() {
    fn test_i<T: PrimitiveInt>(x: T, out: T) {
        assert_eq!(x.square(), out);

        let mut x = x;
        x.square_assign();
        assert_eq!(x, out);
    }
    test_i::<u8>(0, 0);
    test_i::<i16>(1, 1);
    test_i::<u32>(2, 4);
    test_i::<i64>(3, 9);
    test_i::<u128>(10, 100);
    test_i::<isize>(123, 15129);
    test_i::<u32>(1000, 1000000);

    test_i::<i16>(-1, 1);
    test_i::<i32>(-2, 4);
    test_i::<i64>(-3, 9);
    test_i::<i128>(-10, 100);
    test_i::<isize>(-123, 15129);
    test_i::<i32>(-1000, 1000000);

    fn test_f<T: PrimitiveFloat>(x: T, out: T) {
        assert_eq!(NiceFloat(x.square()), NiceFloat(out));

        let mut x = x;
        x.square_assign();
        assert_eq!(NiceFloat(x), NiceFloat(out));
    }
    test_f::<f32>(f32::NAN, f32::NAN);
    test_f::<f32>(f32::INFINITY, f32::INFINITY);
    test_f::<f32>(f32::NEGATIVE_INFINITY, f32::INFINITY);
    test_f::<f32>(0.0, 0.0);
    test_f::<f32>(-0.0, 0.0);
    test_f::<f32>(1.0, 1.0);
    test_f::<f32>(-1.0, 1.0);
    test_f::<f32>(0.5, 0.25);
    test_f::<f32>(-0.5, 0.25);
    test_f::<f32>(core::f32::consts::SQRT_2, 1.9999999);
    test_f::<f32>(-core::f32::consts::SQRT_2, 1.9999999);
    test_f::<f32>(core::f32::consts::PI, 9.869605);
    test_f::<f32>(-core::f32::consts::PI, 9.869605);

    test_f::<f64>(f64::NAN, f64::NAN);
    test_f::<f64>(f64::INFINITY, f64::INFINITY);
    test_f::<f64>(f64::NEGATIVE_INFINITY, f64::INFINITY);
    test_f::<f64>(0.0, 0.0);
    test_f::<f64>(-0.0, 0.0);
    test_f::<f64>(1.0, 1.0);
    test_f::<f64>(-1.0, 1.0);
    test_f::<f64>(0.5, 0.25);
    test_f::<f64>(-0.5, 0.25);
    test_f::<f64>(core::f64::consts::SQRT_2, 2.0000000000000004);
    test_f::<f64>(-core::f64::consts::SQRT_2, 2.0000000000000004);
    test_f::<f64>(core::f64::consts::PI, 9.869604401089358);
    test_f::<f64>(-core::f64::consts::PI, 9.869604401089358);
}

fn square_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen_var_21::<T>().test_properties(|x| {
        let mut square = x;
        square.square_assign();
        assert_eq!(square, x.square());
        assert_eq!(square, x.pow(2));
        assert_eq!(square.checked_sqrt(), Some(x));
        if x > T::ONE {
            assert_eq!(square.checked_log_base(x), Some(2));
        }
    });
}

fn square_properties_helper_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() {
    signed_gen_var_10::<U, S>().test_properties(|x| {
        let mut square = x;
        square.square_assign();
        assert_eq!(square, x.square());
        assert_eq!(square, x.pow(2));
        if x != S::MIN {
            assert_eq!((-x).square(), square);
        }
        assert_eq!(
            U::wrapping_from(square).checked_sqrt().unwrap(),
            x.unsigned_abs()
        );
    });
}

fn square_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let mut square = x;
        square.square_assign();
        assert_eq!(NiceFloat(square), NiceFloat(x.square()));
        assert_eq!(NiceFloat(square), NiceFloat(x.pow(2)));
        assert_eq!(NiceFloat((-x).square()), NiceFloat(square));
    });
}

#[test]
fn square_properties() {
    apply_fn_to_unsigneds!(square_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(square_properties_helper_signed);
    apply_fn_to_primitive_floats!(square_properties_helper_primitive_float);
}
