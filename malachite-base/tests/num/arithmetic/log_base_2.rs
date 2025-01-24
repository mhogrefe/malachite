// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{primitive_float_gen_var_18, unsigned_gen_var_1};
use std::panic::catch_unwind;

fn floor_log_base_2_helper_unsigned<T: PrimitiveUnsigned>() {
    let test = |n: T, out| {
        assert_eq!(n.floor_log_base_2(), out);
    };
    test(T::ONE, 0);
    test(T::TWO, 1);
    test(T::exact_from(3), 1);
    test(T::exact_from(4), 2);
    test(T::exact_from(5), 2);
    test(T::exact_from(100), 6);
    test(T::exact_from(128), 7);
    test(T::MAX, T::WIDTH - 1);
}

fn floor_log_base_2_helper_primitive_float<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(n.floor_log_base_2(), out);
    };
    test(T::ONE, 0);
    test(T::TWO, 1);
    test(T::from(3.0f32), 1);
    test(T::from(4.0f32), 2);
    test(T::from(5.0f32), 2);
    test(T::from(100.0f32), 6);
    test(T::from(128.0f32), 7);
    test(T::from(0.4f32), -2);
    test(T::from(0.5f32), -1);
    test(T::from(0.6f32), -1);
    test(T::MAX_FINITE, T::MAX_EXPONENT);
    test(T::MIN_POSITIVE_SUBNORMAL, T::MIN_EXPONENT);
}

#[test]
fn test_floor_log_base_2() {
    apply_fn_to_unsigneds!(floor_log_base_2_helper_unsigned);
    apply_fn_to_primitive_floats!(floor_log_base_2_helper_primitive_float);
}

fn floor_log_base_2_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.floor_log_base_2());
}

fn floor_log_base_2_fail_helper_primitive_float<T: PrimitiveFloat>() {
    assert_panic!(T::ZERO.floor_log_base_2());
    assert_panic!(T::NAN.floor_log_base_2());
    assert_panic!(T::INFINITY.floor_log_base_2());
    assert_panic!(T::NEGATIVE_INFINITY.floor_log_base_2());
}

#[test]
fn floor_log_base_2_fail() {
    apply_fn_to_unsigneds!(floor_log_base_2_fail_helper_unsigned);
    apply_fn_to_primitive_floats!(floor_log_base_2_fail_helper_primitive_float);
}

fn ceiling_log_base_2_helper_unsigned<T: PrimitiveUnsigned>() {
    let test = |n: T, out| {
        assert_eq!(n.ceiling_log_base_2(), out);
    };
    test(T::ONE, 0);
    test(T::TWO, 1);
    test(T::exact_from(3), 2);
    test(T::exact_from(4), 2);
    test(T::exact_from(5), 3);
    test(T::exact_from(100), 7);
    test(T::exact_from(128), 7);
    test(T::MAX, T::WIDTH);
}

fn ceiling_log_base_2_helper_primitive_float<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(n.ceiling_log_base_2(), out);
    };
    test(T::ONE, 0);
    test(T::TWO, 1);
    test(T::from(3.0f32), 2);
    test(T::from(4.0f32), 2);
    test(T::from(5.0f32), 3);
    test(T::from(100.0f32), 7);
    test(T::from(128.0f32), 7);
    test(T::from(0.4f32), -1);
    test(T::from(0.5f32), -1);
    test(T::from(0.6f32), 0);
    test(T::MAX_FINITE, T::MAX_EXPONENT + 1);
    test(T::MIN_POSITIVE_SUBNORMAL, T::MIN_EXPONENT);
}

#[test]
fn test_ceiling_log_base_2() {
    apply_fn_to_unsigneds!(ceiling_log_base_2_helper_unsigned);
    apply_fn_to_primitive_floats!(ceiling_log_base_2_helper_primitive_float);
}

fn ceiling_log_base_2_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.ceiling_log_base_2());
}

fn ceiling_log_base_2_fail_helper_primitive_float<T: PrimitiveFloat>() {
    assert_panic!(T::ZERO.ceiling_log_base_2());
    assert_panic!(T::NAN.ceiling_log_base_2());
    assert_panic!(T::INFINITY.ceiling_log_base_2());
    assert_panic!(T::NEGATIVE_INFINITY.ceiling_log_base_2());
}

#[test]
fn ceiling_log_base_2_fail() {
    apply_fn_to_unsigneds!(ceiling_log_base_2_fail_helper_unsigned);
    apply_fn_to_primitive_floats!(ceiling_log_base_2_fail_helper_primitive_float);
}

fn checked_log_base_2_helper_unsigned<T: PrimitiveUnsigned>() {
    let test = |n: T, out| {
        assert_eq!(n.checked_log_base_2(), out);
    };
    test(T::ONE, Some(0));
    test(T::TWO, Some(1));
    test(T::exact_from(3), None);
    test(T::exact_from(4), Some(2));
    test(T::exact_from(5), None);
    test(T::exact_from(100), None);
    test(T::exact_from(128), Some(7));
    test(T::MAX, None);
}

fn checked_log_base_2_helper_primitive_float<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(n.checked_log_base_2(), out);
    };
    test(T::ONE, Some(0));
    test(T::TWO, Some(1));
    test(T::from(3.0f32), None);
    test(T::from(4.0f32), Some(2));
    test(T::from(5.0f32), None);
    test(T::from(100.0f32), None);
    test(T::from(128.0f32), Some(7));
    test(T::from(0.4f32), None);
    test(T::from(0.5f32), Some(-1));
    test(T::from(0.6f32), None);
    test(T::MAX_FINITE, None);
    test(T::MIN_POSITIVE_SUBNORMAL, Some(T::MIN_EXPONENT));
}

#[test]
fn test_checked_log_base_2() {
    apply_fn_to_unsigneds!(checked_log_base_2_helper_unsigned);
    apply_fn_to_primitive_floats!(checked_log_base_2_helper_primitive_float);
}

fn checked_log_base_2_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.checked_log_base_2());
}

fn checked_log_base_2_fail_helper_primitive_float<T: PrimitiveFloat>() {
    assert_panic!(T::ZERO.checked_log_base_2());
    assert_panic!(T::NAN.checked_log_base_2());
    assert_panic!(T::INFINITY.checked_log_base_2());
    assert_panic!(T::NEGATIVE_INFINITY.checked_log_base_2());
}

#[test]
fn checked_log_base_2_fail() {
    apply_fn_to_unsigneds!(checked_log_base_2_fail_helper_unsigned);
    apply_fn_to_primitive_floats!(checked_log_base_2_fail_helper_primitive_float);
}

fn floor_log_base_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen_var_1::<T>().test_properties(|n| {
        let floor_log_base_2 = n.floor_log_base_2();
        assert_eq!(floor_log_base_2, n.significant_bits() - 1);
        assert!(floor_log_base_2 < T::WIDTH);
        assert_eq!(floor_log_base_2 == 0, n == T::ONE);
        assert!(T::power_of_2(floor_log_base_2) <= n);
        if floor_log_base_2 < T::WIDTH - 1 {
            assert!(T::power_of_2(floor_log_base_2 + 1) > n);
        }

        let ceiling_log_base_2 = n.ceiling_log_base_2();
        if n.is_power_of_2() {
            assert_eq!(ceiling_log_base_2, floor_log_base_2);
        } else {
            assert_eq!(ceiling_log_base_2, floor_log_base_2 + 1);
        }
    });
}

fn floor_log_base_2_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen_var_18::<T>().test_properties(|n| {
        let floor_log_base_2 = n.floor_log_base_2();
        assert!(floor_log_base_2 <= T::MAX_EXPONENT);
        assert!(floor_log_base_2 >= T::MIN_EXPONENT);
        assert_eq!(floor_log_base_2 == 0, n >= T::ONE && n < T::TWO);
        assert!(T::power_of_2(floor_log_base_2) <= n);
        if floor_log_base_2 < T::MAX_EXPONENT {
            assert!(T::power_of_2(floor_log_base_2 + 1) > n);
        }

        let ceiling_log_base_2 = n.ceiling_log_base_2();
        if n.is_power_of_2() {
            assert_eq!(ceiling_log_base_2, floor_log_base_2);
        } else {
            assert_eq!(ceiling_log_base_2, floor_log_base_2 + 1);
        }
    });
}

#[test]
fn floor_log_base_2_properties() {
    apply_fn_to_unsigneds!(floor_log_base_2_properties_helper_unsigned);
    apply_fn_to_primitive_floats!(floor_log_base_2_properties_helper_primitive_float);
}

fn ceiling_log_base_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen_var_1::<T>().test_properties(|n| {
        let ceiling_log_base_2 = n.ceiling_log_base_2();
        assert!(ceiling_log_base_2 <= T::WIDTH);
        assert_eq!(ceiling_log_base_2 == 0, n == T::ONE);
        if ceiling_log_base_2 < T::WIDTH {
            assert!(T::power_of_2(ceiling_log_base_2) >= n);
        }
        if ceiling_log_base_2 != 0 {
            assert!(T::power_of_2(ceiling_log_base_2 - 1) < n);
        }

        let floor_log_base_2 = n.floor_log_base_2();
        if n.is_power_of_2() {
            assert_eq!(floor_log_base_2, ceiling_log_base_2);
        } else {
            assert_eq!(floor_log_base_2, ceiling_log_base_2 - 1);
        }
    });
}

fn ceiling_log_base_2_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen_var_18::<T>().test_properties(|n| {
        let ceiling_log_base_2 = n.ceiling_log_base_2();
        assert!(ceiling_log_base_2 <= T::MAX_EXPONENT + 1);
        assert!(ceiling_log_base_2 >= T::MIN_EXPONENT);
        assert_eq!(ceiling_log_base_2 == 0, n > T::ONE / T::TWO && n <= T::ONE);
        if ceiling_log_base_2 < T::MAX_EXPONENT {
            assert!(T::power_of_2(ceiling_log_base_2) >= n);
        }
        if ceiling_log_base_2 > T::MIN_EXPONENT {
            assert!(T::power_of_2(ceiling_log_base_2 - 1) < n);
        }

        let floor_log_base_2 = n.floor_log_base_2();
        if n.is_power_of_2() {
            assert_eq!(floor_log_base_2, ceiling_log_base_2);
        } else {
            assert_eq!(floor_log_base_2, ceiling_log_base_2 - 1);
        }
    });
}

#[test]
fn ceiling_log_base_2_properties() {
    apply_fn_to_unsigneds!(ceiling_log_base_2_properties_helper_unsigned);
    apply_fn_to_primitive_floats!(ceiling_log_base_2_properties_helper_primitive_float);
}

fn checked_log_base_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen_var_1::<T>().test_properties(|n| {
        let checked_log_base_2 = n.checked_log_base_2();
        assert_eq!(checked_log_base_2.is_some(), n.is_power_of_2());
        if let Some(log_base_2) = checked_log_base_2 {
            assert_eq!(T::power_of_2(log_base_2), n);
            assert!(log_base_2 <= T::WIDTH);
            assert_eq!(log_base_2 == 0, n == T::ONE);
            assert_eq!(n.floor_log_base_2(), log_base_2);
            assert_eq!(n.ceiling_log_base_2(), log_base_2);
        }
    });
}

fn checked_log_base_2_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_gen_var_18::<T>().test_properties(|n| {
        let checked_log_base_2 = n.checked_log_base_2();
        assert_eq!(checked_log_base_2.is_some(), n.is_power_of_2());
        if let Some(log_base_2) = checked_log_base_2 {
            assert_eq!(T::power_of_2(log_base_2), n);
            assert!(log_base_2 <= T::MAX_EXPONENT);
            assert!(log_base_2 >= T::MIN_EXPONENT);
            assert_eq!(log_base_2 == 0, n == T::ONE);
            assert_eq!(n.floor_log_base_2(), log_base_2);
            assert_eq!(n.ceiling_log_base_2(), log_base_2);
        }
    });
}

#[test]
fn checked_log_base_2_properties() {
    apply_fn_to_unsigneds!(checked_log_base_2_properties_helper_unsigned);
    apply_fn_to_primitive_floats!(checked_log_base_2_properties_helper_primitive_float);
}
