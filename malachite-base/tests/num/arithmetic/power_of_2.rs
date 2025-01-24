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
use malachite_base::test_util::generators::{
    signed_gen_var_11, unsigned_gen_var_15, unsigned_gen_var_16,
};
use std::panic::catch_unwind;

fn power_of_2_primitive_int_helper<T: PrimitiveInt>() {
    let test = |pow, out| {
        assert_eq!(T::power_of_2(pow), out);
    };
    test(0, T::ONE);
    test(1, T::TWO);
    test(2, T::exact_from(4));
    test(3, T::exact_from(8));
}

fn power_of_2_unsigned_helper<T: PrimitiveUnsigned>() {
    let test = |pow, out| {
        assert_eq!(T::power_of_2(pow), out);
    };
    test(T::WIDTH - 1, T::ONE << (T::WIDTH - 1));
}

fn power_of_2_primitive_float_helper<T: PrimitiveFloat>() {
    let test = |pow, out| {
        assert_eq!(T::power_of_2(pow), out);
    };
    test(0, T::ONE);
    test(1, T::TWO);
    test(-1, T::from(0.5f32));
    test(2, T::from(4.0f32));
    test(-2, T::from(0.25f32));
    test(T::MIN_EXPONENT, T::MIN_POSITIVE_SUBNORMAL);
}

#[test]
fn test_power_of_2() {
    apply_fn_to_primitive_ints!(power_of_2_primitive_int_helper);
    apply_fn_to_unsigneds!(power_of_2_unsigned_helper);
    apply_fn_to_primitive_floats!(power_of_2_primitive_float_helper);
}

fn power_of_2_unsigned_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::power_of_2(T::WIDTH));
}

fn power_of_2_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::power_of_2(T::WIDTH - 1));
}

fn power_of_2_primitive_float_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::power_of_2(T::MAX_EXPONENT + 1));
    assert_panic!(T::power_of_2(T::MIN_EXPONENT - 1));
    assert_panic!(T::power_of_2(10000));
    assert_panic!(T::power_of_2(-10000));
}

#[test]
fn power_of_2_fail() {
    apply_fn_to_unsigneds!(power_of_2_unsigned_fail_helper);
    apply_fn_to_signeds!(power_of_2_signed_fail_helper);
    apply_fn_to_primitive_floats!(power_of_2_primitive_float_fail_helper);
}

fn power_of_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen_var_15::<T>().test_properties(|pow| {
        let mut n = T::power_of_2(pow);
        assert_eq!(n.checked_log_base_2(), Some(pow));
        assert!(n.is_power_of_2());
        n.clear_bit(pow);
        assert_eq!(n, T::ZERO);
    });
}

fn power_of_2_properties_helper_signed<U: TryFrom<S> + PrimitiveUnsigned, S: PrimitiveSigned>() {
    unsigned_gen_var_16::<S>().test_properties(|pow| {
        let mut n = S::power_of_2(pow);
        assert_eq!(U::exact_from(n), U::power_of_2(pow));
        n.clear_bit(pow);
        assert_eq!(n, S::ZERO);
    });
}

fn power_of_2_properties_helper_primitive_float<T: PrimitiveFloat>() {
    signed_gen_var_11::<T>().test_properties(|pow| {
        let n = T::power_of_2(pow);
        assert!(n > T::ZERO);
        assert!(n.is_power_of_2());
    });
}

#[test]
fn power_of_2_properties() {
    apply_fn_to_unsigneds!(power_of_2_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(power_of_2_properties_helper_signed);
    apply_fn_to_primitive_floats!(power_of_2_properties_helper_primitive_float);
}
