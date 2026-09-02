// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen};

fn is_power_of_2_helper<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(n.is_power_of_2(), out);
    };
    test(T::ZERO, false);
    test(T::NEGATIVE_ZERO, false);
    test(T::INFINITY, false);
    test(T::NEGATIVE_INFINITY, false);
    test(T::NAN, false);
    test(T::NEGATIVE_ONE, false);
    test(T::from(1.5f32), false);
    test(T::from(-1.5f32), false);

    test(T::ONE, true);
    test(T::TWO, true);
    test(T::from(4.0f32), true);
    test(T::from(0.5f32), true);
    test(T::from(0.25f32), true);
}

fn is_power_of_2_helper_signed<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.is_power_of_2(), out);
    };
    test(T::ZERO, false);
    test(T::ONE, true);
    test(T::TWO, true);
    test(T::from(3), false);
    test(T::from(4), true);
    test(T::from(64), true);
    test(T::from(100), false);
    test(T::NEGATIVE_ONE, false);
    test(T::from(-2), false);
    test(T::from(-64), false);
    test(T::MAX, false);
    test(T::MIN, false);
}

#[test]
fn test_is_power_of_2() {
    apply_fn_to_primitive_floats!(is_power_of_2_helper);
    apply_fn_to_signeds!(is_power_of_2_helper_signed);
}

fn is_power_of_2_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|f| {
        if f.is_power_of_2() {
            assert_eq!(f.precision(), 1);
            assert_eq!(T::power_of_2(f.checked_log_base_2().unwrap()), f);
        }
    });
}

fn is_power_of_2_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|i| {
        let is_power = i.is_power_of_2();
        assert_eq!(i > T::ZERO && i.count_ones() == 1, is_power);
        if is_power {
            assert_eq!(T::power_of_2(i.trailing_zeros()), i);
        }
        if let Some(neg_i) = i.checked_neg() {
            assert!(!neg_i.is_power_of_2() || !is_power);
        }
    });
}

#[test]
fn is_power_of_2_properties() {
    apply_fn_to_primitive_floats!(is_power_of_2_properties_helper);
    apply_fn_to_signeds!(is_power_of_2_properties_helper_signed);
}
