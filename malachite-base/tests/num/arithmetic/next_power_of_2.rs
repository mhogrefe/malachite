// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{primitive_float_gen_var_19, unsigned_gen_var_14};
use std::panic::catch_unwind;

fn next_power_of_2_helper_unsigned<T: PrimitiveUnsigned>() {
    let test = |n: T, out| {
        assert_eq!(n.next_power_of_2(), out);

        let mut n = n;
        n.next_power_of_2_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ONE);
    test(T::ONE, T::ONE);
    test(T::exact_from(7), T::exact_from(8));
    test(T::exact_from(8), T::exact_from(8));
    test(T::exact_from(10), T::exact_from(16));
    test((T::MAX >> 1u64) + T::ONE, (T::MAX >> 1u64) + T::ONE);
    test(
        (T::MAX >> 1u64) - T::exact_from(10),
        (T::MAX >> 1u64) + T::ONE,
    );
}

fn next_power_of_2_helper_primitive_float<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(NiceFloat(n.next_power_of_2()), NiceFloat(out));

        let mut n = n;
        n.next_power_of_2_assign();
        assert_eq!(NiceFloat(n), NiceFloat(out));
    };
    test(T::ZERO, T::MIN_POSITIVE_SUBNORMAL);
    test(T::ONE, T::ONE);
    test(T::from(7.0f32), T::from(8.0f32));
    test(T::from(8.0f32), T::from(8.0f32));
    test(T::from(10.0f32), T::from(16.0f32));
    test(T::from(0.1f32), T::from(0.125f32));
    test(T::from(0.01f32), T::from(0.015625f32));
    test(
        T::power_of_2(T::MAX_EXPONENT),
        T::power_of_2(T::MAX_EXPONENT),
    );
}

#[test]
fn test_next_power_of_2() {
    apply_fn_to_unsigneds!(next_power_of_2_helper_unsigned);
    apply_fn_to_primitive_floats!(next_power_of_2_helper_primitive_float);
}

fn next_power_of_2_fail_helper_primitive_float<T: PrimitiveFloat>() {
    assert_panic!(T::NEGATIVE_ZERO.next_power_of_2());
    assert_panic!(T::INFINITY.next_power_of_2());
    assert_panic!(T::NEGATIVE_INFINITY.next_power_of_2());
    assert_panic!(T::NAN.next_power_of_2());
    assert_panic!(T::NEGATIVE_ONE.next_power_of_2());
    assert_panic!(T::MAX_FINITE.next_power_of_2());

    let test = |x: T| {
        let mut x = x;
        x.next_power_of_2_assign();
    };
    assert_panic!(test(T::NEGATIVE_ZERO));
    assert_panic!(test(T::INFINITY));
    assert_panic!(test(T::NEGATIVE_INFINITY));
    assert_panic!(test(T::NAN));
    assert_panic!(test(T::NEGATIVE_ONE));
    assert_panic!(test(T::MAX_FINITE));
}

#[test]
fn next_power_of_2_fail() {
    apply_fn_to_primitive_floats!(next_power_of_2_fail_helper_primitive_float);
}

fn next_power_of_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen_var_14::<T>().test_properties(|n| {
        let p = n.next_power_of_2();
        assert!(p >= n);
        assert!(p >> 1 <= n);
        assert!(p.is_power_of_2());

        let mut n = n;
        n.next_power_of_2_assign();
        assert_eq!(n, p);
    });
}

fn next_power_of_2_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen_var_19::<T>().test_properties(|x| {
        let p = x.next_power_of_2();
        assert!(p >= x);
        assert!(p / T::TWO <= x);
        assert!(p.is_power_of_2());

        let mut x = x;
        x.next_power_of_2_assign();
        assert_eq!(x, p);
    });
}

#[test]
fn next_power_of_2_properties() {
    apply_fn_to_unsigneds!(next_power_of_2_properties_helper_unsigned);
    apply_fn_to_primitive_floats!(next_power_of_2_properties_helper_primitive_float);
}
