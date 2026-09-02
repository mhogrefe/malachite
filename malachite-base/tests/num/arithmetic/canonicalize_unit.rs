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
use malachite_base::test_util::generators::{
    primitive_float_gen, signed_gen, signed_gen_var_1, unsigned_gen,
};

#[test]
fn test_canonicalize_unit() {
    fn test_u<T: PrimitiveUnsigned>(x: T) {
        assert_eq!(x.canonicalize_unit(), x);
        let mut y = x;
        y.canonicalize_unit_assign();
        assert_eq!(y, x);
    }
    test_u::<u8>(0);
    test_u::<u8>(123);
    test_u::<u64>(1000000000000);

    fn test_i<T: PrimitiveSigned>(x: T, out: T) {
        assert_eq!(x.canonicalize_unit(), out);
        let mut y = x;
        y.canonicalize_unit_assign();
        assert_eq!(y, out);
    }
    test_i::<i8>(0, 0);
    test_i::<i8>(123, 123);
    test_i::<i8>(-123, 123);

    fn test_f<T: PrimitiveFloat>(x: T, out: T) {
        assert_eq!(NiceFloat(x.canonicalize_unit()), NiceFloat(out));
        let mut y = x;
        y.canonicalize_unit_assign();
        assert_eq!(NiceFloat(y), NiceFloat(out));
    }
    test_f::<f32>(0.0, 0.0);
    test_f::<f32>(-0.0, 0.0);
    test_f::<f32>(-1.5, 1.5);
    test_f::<f32>(1.5, 1.5);
    test_f::<f64>(f64::NAN, f64::NAN);
    test_f::<f64>(f64::NEG_INFINITY, f64::INFINITY);
}

fn canonicalize_unit_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.canonicalize_unit(), x);
        let mut y = x;
        y.canonicalize_unit_assign();
        assert_eq!(y, x);
    });
}

fn canonicalize_unit_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen_var_1::<T>().test_properties(|x| {
        let y = x.canonicalize_unit();
        assert_eq!(y, x.abs());
        let mut x_alt = x;
        x_alt.canonicalize_unit_assign();
        assert_eq!(x_alt, y);
        assert!(y >= T::ZERO);
        assert_eq!(y.canonicalize_unit(), y);
        assert_eq!(y.canonical_unit_i_pow(), 0);
        assert_eq!((-x).canonicalize_unit(), y);
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.canonical_unit_i_pow() == 0, x >= T::ZERO);
    });
}

fn canonicalize_unit_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let y = x.canonicalize_unit();
        assert_eq!(NiceFloat(y), NiceFloat(x.abs()));
        let mut x_alt = x;
        x_alt.canonicalize_unit_assign();
        assert_eq!(NiceFloat(x_alt), NiceFloat(y));
        assert!(!y.is_sign_negative());
        assert_eq!(NiceFloat(y.canonicalize_unit()), NiceFloat(y));
        assert_eq!(y.canonical_unit_i_pow(), 0);
        assert_eq!(NiceFloat((-x).canonicalize_unit()), NiceFloat(y));
    });
}

#[test]
fn canonicalize_unit_properties() {
    apply_fn_to_unsigneds!(canonicalize_unit_properties_helper_unsigned);
    apply_fn_to_signeds!(canonicalize_unit_properties_helper_signed);
    apply_fn_to_primitive_floats!(canonicalize_unit_properties_helper_primitive_float);
}
