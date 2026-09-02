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
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};

#[test]
fn test_canonical_unit_i_pow() {
    fn test_u<T: PrimitiveUnsigned>(x: T, out: u64) {
        assert_eq!(x.canonical_unit_i_pow(), out);
    }
    test_u::<u8>(0, 0);
    test_u::<u8>(123, 0);
    test_u::<u64>(1000000000000, 0);

    fn test_i<T: PrimitiveSigned>(x: T, out: u64) {
        assert_eq!(x.canonical_unit_i_pow(), out);
    }
    test_i::<i8>(0, 0);
    test_i::<i8>(123, 0);
    test_i::<i8>(-123, 2);
    test_i::<i8>(i8::MIN, 2);

    fn test_f<T: PrimitiveFloat>(x: T, out: u64) {
        assert_eq!(x.canonical_unit_i_pow(), out);
    }
    test_f::<f32>(0.0, 0);
    test_f::<f32>(-0.0, 2);
    test_f::<f32>(1.5, 0);
    test_f::<f32>(-1.5, 2);
    test_f::<f64>(f64::NAN, 0);
    test_f::<f64>(f64::INFINITY, 0);
    test_f::<f64>(f64::NEG_INFINITY, 2);
}

fn canonical_unit_i_pow_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.canonical_unit_i_pow(), 0);
    });
}

fn canonical_unit_i_pow_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        let k = x.canonical_unit_i_pow();
        assert!(k == 0 || k == 2);
        assert_eq!(k == 2, x < T::ZERO);
        if let Some(neg_x) = x.checked_neg() {
            if x != T::ZERO {
                assert_eq!(neg_x.canonical_unit_i_pow(), 2 - k);
            }
            assert_eq!(x.canonicalize_unit(), if k == 0 { x } else { neg_x });
        }
    });
}

fn canonical_unit_i_pow_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let k = x.canonical_unit_i_pow();
        assert!(k == 0 || k == 2);
        assert_eq!(k == 2, x.is_sign_negative() && !x.is_nan());
        assert_eq!(
            NiceFloat(x.canonicalize_unit()),
            NiceFloat(if k == 0 { x } else { -x })
        );
    });
}

#[test]
fn canonical_unit_i_pow_properties() {
    apply_fn_to_unsigneds!(canonical_unit_i_pow_properties_helper_unsigned);
    apply_fn_to_signeds!(canonical_unit_i_pow_properties_helper_signed);
    apply_fn_to_primitive_floats!(canonical_unit_i_pow_properties_helper_primitive_float);
}
