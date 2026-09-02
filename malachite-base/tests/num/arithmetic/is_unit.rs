// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};

#[test]
fn test_is_unit() {
    fn test_u<T: PrimitiveUnsigned>(x: T, out: bool) {
        assert_eq!(x.is_unit(), out);
    }
    test_u::<u8>(0, false);
    test_u::<u8>(1, true);
    test_u::<u8>(2, false);
    test_u::<u64>(1000000000000, false);

    fn test_i<T: PrimitiveSigned>(x: T, out: bool) {
        assert_eq!(x.is_unit(), out);
    }
    test_i::<i8>(0, false);
    test_i::<i8>(1, true);
    test_i::<i8>(-1, true);
    test_i::<i8>(2, false);
    test_i::<i8>(-2, false);
    test_i::<i8>(i8::MIN, false);

    fn test_f<T: PrimitiveFloat>(x: T, out: bool) {
        assert_eq!(x.is_unit(), out);
    }
    test_f::<f32>(0.0, false);
    test_f::<f32>(-0.0, false);
    test_f::<f32>(1.0, true);
    test_f::<f32>(-1.5, true);
    test_f::<f32>(f32::MIN_POSITIVE_SUBNORMAL, true);
    test_f::<f64>(f64::NAN, false);
    test_f::<f64>(f64::INFINITY, false);
    test_f::<f64>(f64::NEG_INFINITY, false);
}

fn is_unit_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.is_unit(), x == T::ONE);
    });
}

fn is_unit_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        let is_unit = x.is_unit();
        assert_eq!(is_unit, x == T::ONE || x == T::NEGATIVE_ONE);
        if let Some(neg_x) = x.checked_neg() {
            assert_eq!(neg_x.is_unit(), is_unit);
        }
    });
}

fn is_unit_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let is_unit = x.is_unit();
        assert_eq!(is_unit, x.is_finite() && x != T::ZERO);
        assert_eq!((-x).is_unit(), is_unit);
    });
}

#[test]
fn is_unit_properties() {
    apply_fn_to_unsigneds!(is_unit_properties_helper_unsigned);
    apply_fn_to_signeds!(is_unit_properties_helper_signed);
    apply_fn_to_primitive_floats!(is_unit_properties_helper_primitive_float);
}
