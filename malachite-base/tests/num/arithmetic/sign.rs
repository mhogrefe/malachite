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
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};
use std::cmp::Ordering::*;

fn sign_helper_primitive_int<T: PrimitiveInt>() {
    let test = |n: T, out| {
        assert_eq!(n.sign(), out);
    };
    test(T::ZERO, Equal);
    test(T::ONE, Greater);
    test(T::exact_from(100), Greater);
    test(T::MAX, Greater);
}

fn sign_helper_signed<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.sign(), out);
    };
    test(T::NEGATIVE_ONE, Less);
    test(T::exact_from(-100), Less);
    test(T::MIN, Less);
}

fn sign_helper_primitive_float<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(n.sign(), out);
    };
    test(T::ZERO, Greater);
    test(T::NEGATIVE_ZERO, Less);
    test(T::ONE, Greater);
    test(T::NEGATIVE_ONE, Less);
    test(T::INFINITY, Greater);
    test(T::NEGATIVE_INFINITY, Less);
    test(T::NAN, Equal);
}

#[test]
fn test_sign() {
    apply_fn_to_primitive_ints!(sign_helper_primitive_int);
    apply_fn_to_signeds!(sign_helper_signed);
    apply_fn_to_primitive_floats!(sign_helper_primitive_float);
}

fn sign_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let sign = n.sign();
        assert_ne!(sign, Less);
        assert_eq!(n.partial_cmp(&T::ZERO), Some(sign));
    });
}

fn sign_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|n| {
        let sign = n.sign();
        assert_eq!(n.partial_cmp(&T::ZERO), Some(sign));
        if n != T::MIN {
            assert_eq!((-n).sign(), sign.reverse());
        }
    });
}

fn sign_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|f| {
        let sign = f.sign();
        if !f.is_nan() {
            assert_eq!((-f).sign(), sign.reverse());
        }
    });
}

#[test]
fn sign_properties() {
    apply_fn_to_unsigneds!(sign_properties_helper_unsigned);
    apply_fn_to_signeds!(sign_properties_helper_signed);
    apply_fn_to_primitive_floats!(sign_properties_helper_primitive_float);
}
