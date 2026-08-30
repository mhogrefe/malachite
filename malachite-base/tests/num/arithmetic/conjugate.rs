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
fn test_conjugate() {
    fn test_u<T: PrimitiveUnsigned>(x: T) {
        assert_eq!(x.conjugate(), x);
        let mut y = x;
        y.conjugate_assign();
        assert_eq!(y, x);
    }
    test_u::<u8>(0);
    test_u::<u8>(123);
    test_u::<u64>(1000000000000);

    fn test_i<T: PrimitiveSigned>(x: T) {
        assert_eq!(x.conjugate(), x);
        let mut y = x;
        y.conjugate_assign();
        assert_eq!(y, x);
    }
    test_i::<i8>(0);
    test_i::<i8>(123);
    test_i::<i8>(-123);

    fn test_f<T: PrimitiveFloat>(x: T) {
        assert_eq!(NiceFloat(x.conjugate()), NiceFloat(x));
        let mut y = x;
        y.conjugate_assign();
        assert_eq!(NiceFloat(y), NiceFloat(x));
    }
    test_f::<f32>(0.0);
    test_f::<f32>(-0.0);
    test_f::<f32>(-1.5);
    test_f::<f64>(f64::NAN);
    test_f::<f64>(f64::INFINITY);
}

fn conjugate_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.conjugate(), x);
        let mut y = x;
        y.conjugate_assign();
        assert_eq!(y, x);
    });
}

fn conjugate_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.conjugate(), x);
        let mut y = x;
        y.conjugate_assign();
        assert_eq!(y, x);
    });
}

fn conjugate_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        assert_eq!(NiceFloat(x.conjugate()), NiceFloat(x));
        let mut y = x;
        y.conjugate_assign();
        assert_eq!(NiceFloat(y), NiceFloat(x));
    });
}

#[test]
fn conjugate_properties() {
    apply_fn_to_unsigneds!(conjugate_properties_helper_unsigned);
    apply_fn_to_signeds!(conjugate_properties_helper_signed);
    apply_fn_to_primitive_floats!(conjugate_properties_helper_primitive_float);
}
