// Copyright © 2026 Mikhail Hogrefe
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
pub fn test_is_real() {
    fn test_u<T: PrimitiveUnsigned>(u: T, is_real: bool) {
        assert_eq!(u.is_real(), is_real);
    }
    test_u::<u8>(0, true);
    test_u::<u8>(1, true);
    test_u::<u8>(100, true);

    fn test_i<T: PrimitiveSigned>(i: T, is_real: bool) {
        assert_eq!(i.is_real(), is_real);
    }
    test_i::<i8>(0, true);
    test_i::<i8>(1, true);
    test_i::<i8>(-1, true);
    test_i::<i8>(-100, true);

    fn test_f<T: PrimitiveFloat>(f: T, is_real: bool) {
        assert_eq!(f.is_real(), is_real);
    }
    test_f::<f32>(0.0, true);
    test_f::<f32>(1.0, true);
    test_f::<f32>(-100.0, true);
    test_f::<f32>(0.1, true);
    test_f::<f32>(-100.1, true);

    test_f::<f32>(f32::NAN, false);
    test_f::<f32>(f32::INFINITY, false);
    test_f::<f32>(f32::NEGATIVE_INFINITY, false);
}

fn is_real_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|u| {
        assert!(u.is_real());
    });
}

fn is_real_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|i| {
        assert!(i.is_real());
    });
}

fn is_real_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|f| {
        assert_eq!(f.is_real(), f.is_finite());
    });
}

#[test]
fn is_real_properties() {
    apply_fn_to_unsigneds!(is_real_unsigned);
    apply_fn_to_signeds!(is_real_signed);
    apply_fn_to_primitive_floats!(is_real_primitive_float);
}
