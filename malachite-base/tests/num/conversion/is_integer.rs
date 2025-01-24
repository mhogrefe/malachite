// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};

#[test]
pub fn test_is_integer() {
    fn test_u<T: PrimitiveUnsigned>(u: T, is_integer: bool) {
        assert_eq!(u.is_integer(), is_integer);
    }
    test_u::<u8>(0, true);
    test_u::<u8>(1, true);
    test_u::<u8>(100, true);

    fn test_i<T: PrimitiveSigned>(i: T, is_integer: bool) {
        assert_eq!(i.is_integer(), is_integer);
    }
    test_i::<i8>(0, true);
    test_i::<i8>(1, true);
    test_i::<i8>(100, true);
    test_i::<i8>(-1, true);
    test_i::<i8>(-100, true);

    fn test_f<T: PrimitiveFloat>(f: T, is_integer: bool) {
        assert_eq!(f.is_integer(), is_integer);
    }
    test_f::<f32>(0.0, true);
    test_f::<f32>(1.0, true);
    test_f::<f32>(100.0, true);
    test_f::<f32>(-1.0, true);
    test_f::<f32>(-100.0, true);

    test_f::<f32>(0.1, false);
    test_f::<f32>(100.1, false);
    test_f::<f32>(-0.1, false);
    test_f::<f32>(-100.1, false);
    test_f::<f32>(f32::NAN, false);
    test_f::<f32>(f32::INFINITY, false);
    test_f::<f32>(f32::NEGATIVE_INFINITY, false);
}

fn is_integer_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|u| {
        assert!(u.is_integer());
    });
}

fn is_integer_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|i| {
        assert!(i.is_integer());
    });
}

fn is_integer_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|f| {
        assert_eq!(
            f.is_integer(),
            f.is_finite() && (f == T::ZERO || f.integer_exponent() >= 0)
        );
    });
}

#[test]
fn is_integer_properties() {
    apply_fn_to_unsigneds!(is_integer_unsigned);
    apply_fn_to_signeds!(is_integer_signed);
    apply_fn_to_primitive_floats!(is_integer_primitive_float);
}
