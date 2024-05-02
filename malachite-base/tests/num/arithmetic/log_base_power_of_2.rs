// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::log_base_power_of_2::ceiling_log_base_power_of_2_naive;
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{
    primitive_float_gen_var_18, primitive_float_unsigned_pair_gen_var_3, unsigned_gen_var_1,
    unsigned_gen_var_11, unsigned_pair_gen_var_21,
};
use std::panic::catch_unwind;

fn floor_log_base_power_of_2_helper_unsigned<T: PrimitiveUnsigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.floor_log_base_power_of_2(pow), out);
    };
    test(T::ONE, 1, 0);
    test(T::ONE, 5, 0);
    test(T::TWO, 1, 1);
    test(T::TWO, 2, 0);
    test(T::exact_from(3), 1, 1);
    test(T::exact_from(3), 2, 0);
    test(T::exact_from(4), 1, 2);
    test(T::exact_from(4), 2, 1);
    test(T::exact_from(4), 3, 0);
    test(T::exact_from(5), 1, 2);
    test(T::exact_from(5), 2, 1);
    test(T::exact_from(5), 3, 0);
    test(T::exact_from(100), 1, 6);
    test(T::exact_from(100), 2, 3);
    test(T::exact_from(100), 3, 2);
    test(T::exact_from(100), 4, 1);
    test(T::exact_from(100), 7, 0);
    test(T::exact_from(128), 1, 7);
    test(T::exact_from(128), 2, 3);
    test(T::exact_from(128), 3, 2);
    test(T::exact_from(128), 4, 1);
    test(T::exact_from(128), 7, 1);
    test(T::MAX, 1, T::WIDTH - 1);
}

fn floor_log_base_power_of_2_helper_primitive_float<T: PrimitiveFloat>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.floor_log_base_power_of_2(pow), out);
    };
    test(T::ONE, 1, 0);
    test(T::ONE, 5, 0);
    test(T::TWO, 1, 1);
    test(T::TWO, 2, 0);
    test(T::from(3.0f32), 1, 1);
    test(T::from(3.0f32), 2, 0);
    test(T::from(4.0f32), 1, 2);
    test(T::from(4.0f32), 2, 1);
    test(T::from(4.0f32), 3, 0);
    test(T::from(5.0f32), 1, 2);
    test(T::from(5.0f32), 2, 1);
    test(T::from(5.0f32), 3, 0);
    test(T::from(100.0f32), 1, 6);
    test(T::from(100.0f32), 2, 3);
    test(T::from(100.0f32), 3, 2);
    test(T::from(100.0f32), 4, 1);
    test(T::from(100.0f32), 7, 0);
    test(T::from(128.0f32), 1, 7);
    test(T::from(128.0f32), 2, 3);
    test(T::from(128.0f32), 3, 2);
    test(T::from(128.0f32), 4, 1);
    test(T::from(128.0f32), 7, 1);
    test(T::from(0.1f32), 1, -4);
    test(T::from(0.1f32), 2, -2);
    test(T::from(0.1f32), 3, -2);
    test(T::from(0.1f32), 4, -1);
    test(T::MIN_POSITIVE_SUBNORMAL, 1, T::MIN_EXPONENT);
    test(T::MAX_FINITE, 1, T::MAX_EXPONENT);
}

#[test]
fn test_floor_log_base_power_of_2() {
    apply_fn_to_unsigneds!(floor_log_base_power_of_2_helper_unsigned);
    apply_fn_to_primitive_floats!(floor_log_base_power_of_2_helper_primitive_float);
}

fn floor_log_base_power_of_2_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.floor_log_base_power_of_2(2));
    assert_panic!(T::TWO.floor_log_base_power_of_2(0));
}

fn floor_log_base_power_of_2_fail_helper_primitive_float<T: PrimitiveFloat>() {
    assert_panic!(T::ZERO.floor_log_base_power_of_2(2));
    assert_panic!(T::NEGATIVE_ONE.floor_log_base_power_of_2(2));
    assert_panic!(T::NEGATIVE_INFINITY.floor_log_base_power_of_2(2));
    assert_panic!(T::INFINITY.floor_log_base_power_of_2(2));
    assert_panic!(T::NAN.floor_log_base_power_of_2(2));

    assert_panic!(T::TWO.floor_log_base_power_of_2(0));
}

#[test]
fn floor_log_base_power_of_2_fail() {
    apply_fn_to_unsigneds!(floor_log_base_power_of_2_fail_helper_unsigned);
    apply_fn_to_primitive_floats!(floor_log_base_power_of_2_fail_helper_primitive_float);
}

fn ceiling_log_base_power_of_2_helper_unsigned<T: PrimitiveUnsigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.ceiling_log_base_power_of_2(pow), out);
    };
    test(T::ONE, 1, 0);
    test(T::ONE, 5, 0);
    test(T::TWO, 1, 1);
    test(T::TWO, 2, 1);
    test(T::exact_from(3), 1, 2);
    test(T::exact_from(3), 2, 1);
    test(T::exact_from(4), 1, 2);
    test(T::exact_from(4), 2, 1);
    test(T::exact_from(4), 3, 1);
    test(T::exact_from(5), 1, 3);
    test(T::exact_from(5), 2, 2);
    test(T::exact_from(5), 3, 1);
    test(T::exact_from(100), 1, 7);
    test(T::exact_from(100), 2, 4);
    test(T::exact_from(100), 3, 3);
    test(T::exact_from(100), 4, 2);
    test(T::exact_from(100), 7, 1);
    test(T::exact_from(128), 1, 7);
    test(T::exact_from(128), 2, 4);
    test(T::exact_from(128), 3, 3);
    test(T::exact_from(128), 4, 2);
    test(T::exact_from(128), 7, 1);
    test(T::MAX, 1, T::WIDTH);
}

fn ceiling_log_base_power_of_2_helper_primitive_float<T: PrimitiveFloat>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.ceiling_log_base_power_of_2(pow), out);
    };
    test(T::ONE, 1, 0);
    test(T::ONE, 5, 0);
    test(T::TWO, 1, 1);
    test(T::TWO, 2, 1);
    test(T::from(3.0f32), 1, 2);
    test(T::from(3.0f32), 2, 1);
    test(T::from(4.0f32), 1, 2);
    test(T::from(4.0f32), 2, 1);
    test(T::from(4.0f32), 3, 1);
    test(T::from(5.0f32), 1, 3);
    test(T::from(5.0f32), 2, 2);
    test(T::from(5.0f32), 3, 1);
    test(T::from(100.0f32), 1, 7);
    test(T::from(100.0f32), 2, 4);
    test(T::from(100.0f32), 3, 3);
    test(T::from(100.0f32), 4, 2);
    test(T::from(100.0f32), 7, 1);
    test(T::from(128.0f32), 1, 7);
    test(T::from(128.0f32), 2, 4);
    test(T::from(128.0f32), 3, 3);
    test(T::from(128.0f32), 4, 2);
    test(T::from(128.0f32), 7, 1);
    test(T::from(0.1f32), 1, -3);
    test(T::from(0.1f32), 2, -1);
    test(T::from(0.1f32), 3, -1);
    test(T::from(0.1f32), 4, 0);
    test(T::MIN_POSITIVE_SUBNORMAL, 1, T::MIN_EXPONENT);
    test(T::MAX_FINITE, 1, T::MAX_EXPONENT + 1);
}

#[test]
fn test_ceiling_log_base_power_of_2() {
    apply_fn_to_unsigneds!(ceiling_log_base_power_of_2_helper_unsigned);
    apply_fn_to_primitive_floats!(ceiling_log_base_power_of_2_helper_primitive_float);
}

fn ceiling_log_base_power_of_2_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.ceiling_log_base_power_of_2(2));
    assert_panic!(T::TWO.ceiling_log_base_power_of_2(0));
}

fn ceiling_log_base_power_of_2_fail_helper_primitive_float<T: PrimitiveFloat>() {
    assert_panic!(T::ZERO.ceiling_log_base_power_of_2(2));
    assert_panic!(T::NEGATIVE_ONE.ceiling_log_base_power_of_2(2));
    assert_panic!(T::NEGATIVE_INFINITY.ceiling_log_base_power_of_2(2));
    assert_panic!(T::INFINITY.ceiling_log_base_power_of_2(2));
    assert_panic!(T::NAN.ceiling_log_base_power_of_2(2));

    assert_panic!(T::TWO.ceiling_log_base_power_of_2(0));
}

#[test]
fn ceiling_log_base_power_of_2_fail() {
    apply_fn_to_unsigneds!(ceiling_log_base_power_of_2_fail_helper_unsigned);
    apply_fn_to_primitive_floats!(ceiling_log_base_power_of_2_fail_helper_primitive_float);
}

fn checked_log_base_power_of_2_helper_unsigned<T: PrimitiveUnsigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.checked_log_base_power_of_2(pow), out);
    };
    test(T::ONE, 1, Some(0));
    test(T::ONE, 5, Some(0));
    test(T::TWO, 1, Some(1));
    test(T::TWO, 2, None);
    test(T::exact_from(3), 1, None);
    test(T::exact_from(3), 2, None);
    test(T::exact_from(4), 1, Some(2));
    test(T::exact_from(4), 2, Some(1));
    test(T::exact_from(4), 3, None);
    test(T::exact_from(5), 1, None);
    test(T::exact_from(5), 2, None);
    test(T::exact_from(5), 3, None);
    test(T::exact_from(100), 1, None);
    test(T::exact_from(100), 2, None);
    test(T::exact_from(100), 3, None);
    test(T::exact_from(100), 4, None);
    test(T::exact_from(100), 7, None);
    test(T::exact_from(128), 1, Some(7));
    test(T::exact_from(128), 2, None);
    test(T::exact_from(128), 3, None);
    test(T::exact_from(128), 4, None);
    test(T::exact_from(128), 7, Some(1));
    test(T::MAX, 1, None);
}

fn checked_log_base_power_of_2_helper_primitive_float<T: PrimitiveFloat>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.checked_log_base_power_of_2(pow), out);
    };
    test(T::ONE, 1, Some(0));
    test(T::ONE, 5, Some(0));
    test(T::TWO, 1, Some(1));
    test(T::TWO, 2, None);
    test(T::from(3.0f32), 1, None);
    test(T::from(3.0f32), 2, None);
    test(T::from(4.0f32), 1, Some(2));
    test(T::from(4.0f32), 2, Some(1));
    test(T::from(4.0f32), 3, None);
    test(T::from(5.0f32), 1, None);
    test(T::from(5.0f32), 2, None);
    test(T::from(5.0f32), 3, None);
    test(T::from(100.0f32), 1, None);
    test(T::from(100.0f32), 2, None);
    test(T::from(100.0f32), 3, None);
    test(T::from(100.0f32), 4, None);
    test(T::from(100.0f32), 7, None);
    test(T::from(128.0f32), 1, Some(7));
    test(T::from(128.0f32), 2, None);
    test(T::from(128.0f32), 3, None);
    test(T::from(128.0f32), 4, None);
    test(T::from(128.0f32), 7, Some(1));
    test(T::from(0.1f32), 1, None);
    test(T::from(0.1f32), 2, None);
    test(T::from(0.1f32), 3, None);
    test(T::from(0.1f32), 4, None);
    test(T::MIN_POSITIVE_SUBNORMAL, 1, Some(T::MIN_EXPONENT));
    test(T::MAX_FINITE, 1, None);
}

#[test]
fn test_checked_log_base_power_of_2() {
    apply_fn_to_unsigneds!(checked_log_base_power_of_2_helper_unsigned);
    apply_fn_to_primitive_floats!(checked_log_base_power_of_2_helper_primitive_float);
}

fn checked_log_base_power_of_2_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.checked_log_base_power_of_2(2));
    assert_panic!(T::TWO.checked_log_base_power_of_2(0));
}

fn checked_log_base_power_of_2_fail_helper_primitive_float<T: PrimitiveFloat>() {
    assert_panic!(T::ZERO.checked_log_base_power_of_2(2));
    assert_panic!(T::NEGATIVE_ONE.checked_log_base_power_of_2(2));
    assert_panic!(T::NEGATIVE_INFINITY.checked_log_base_power_of_2(2));
    assert_panic!(T::INFINITY.checked_log_base_power_of_2(2));
    assert_panic!(T::NAN.checked_log_base_power_of_2(2));

    assert_panic!(T::TWO.checked_log_base_power_of_2(0));
}

#[test]
fn checked_log_base_power_of_2_fail() {
    apply_fn_to_unsigneds!(checked_log_base_power_of_2_fail_helper_unsigned);
    apply_fn_to_primitive_floats!(checked_log_base_power_of_2_fail_helper_primitive_float);
}

fn floor_log_base_power_of_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_21::<T, u64>().test_properties(|(n, pow)| {
        let floor_log = n.floor_log_base_power_of_2(pow);
        assert!(floor_log < T::WIDTH);
        assert_eq!(floor_log == 0, n.significant_bits() - 1 < pow);
        if pow < T::WIDTH {
            assert_eq!(n.floor_log_base(T::power_of_2(pow)), floor_log);
        }

        let product = floor_log * pow;
        if product < T::WIDTH {
            assert!(T::power_of_2(product) <= n);
        }
        let product_2 = product + pow;
        if product_2 < T::WIDTH {
            assert!(T::power_of_2(product_2) > n);
        }

        let ceiling_log = n.ceiling_log_base_power_of_2(pow);
        if n.is_power_of_2() && (n.significant_bits() - 1).divisible_by(pow) {
            assert_eq!(ceiling_log, floor_log);
        } else {
            assert_eq!(ceiling_log, floor_log + 1);
        }
    });

    unsigned_gen_var_1::<T>().test_properties(|n| {
        assert_eq!(n.floor_log_base_power_of_2(1), n.floor_log_base_2());
        assert_eq!(n.floor_log_base_power_of_2(T::WIDTH), 0);
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(T::ONE.floor_log_base_power_of_2(pow), 0);
        if pow < T::WIDTH {
            assert_eq!(T::power_of_2(pow).floor_log_base_power_of_2(pow), 1);
        }
    });
}

fn floor_log_base_power_of_2_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_unsigned_pair_gen_var_3::<T, u64>().test_properties(|(n, pow)| {
        let floor_log = n.floor_log_base_power_of_2(pow);
        assert!(floor_log <= T::MAX_EXPONENT);
        assert!(floor_log >= T::MIN_EXPONENT);
        let i_pow = i64::exact_from(pow);
        if i_pow >= T::MIN_EXPONENT && i_pow <= T::MAX_EXPONENT {
            assert_eq!(floor_log == 0, n >= T::ONE && n < T::power_of_2(i_pow));
        }
        let product = floor_log * i_pow;
        if product >= T::MIN_EXPONENT && product <= T::MAX_EXPONENT {
            assert!(T::power_of_2(product) <= n);
        }
        let product_2 = product + i_pow;
        if product_2 >= T::MIN_EXPONENT && product_2 <= T::MAX_EXPONENT {
            assert!(T::power_of_2(product_2) > n);
        }

        let ceiling_log = n.ceiling_log_base_power_of_2(pow);
        if n.is_power_of_2() && n.floor_log_base_2().divisible_by(i_pow) {
            assert_eq!(ceiling_log, floor_log);
        } else {
            assert_eq!(ceiling_log, floor_log + 1);
        }
    });

    primitive_float_gen_var_18::<T>().test_properties(|n| {
        assert_eq!(n.floor_log_base_power_of_2(1), n.floor_log_base_2());
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(T::ONE.floor_log_base_power_of_2(pow), 0);
    });
}

#[test]
fn floor_log_base_power_of_2_properties_unsigned() {
    apply_fn_to_unsigneds!(floor_log_base_power_of_2_properties_helper_unsigned);
    apply_fn_to_primitive_floats!(floor_log_base_power_of_2_properties_helper_primitive_float);
}

fn ceiling_log_base_power_of_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_21::<T, u64>().test_properties(|(n, pow)| {
        let ceiling_log = n.ceiling_log_base_power_of_2(pow);
        assert!(ceiling_log <= T::WIDTH);
        assert_eq!(ceiling_log, ceiling_log_base_power_of_2_naive(n, pow));
        assert_eq!(ceiling_log == 0, n == T::ONE);
        if pow < T::WIDTH {
            assert_eq!(n.ceiling_log_base(T::power_of_2(pow)), ceiling_log);
        }

        let product = ceiling_log * pow;
        if product < T::WIDTH {
            assert!(T::power_of_2(product) >= n);
        }
        if product != 0 {
            assert!(T::power_of_2(product - pow) < n);
        }

        let floor_log = n.floor_log_base_power_of_2(pow);
        if n.is_power_of_2() && (n.significant_bits() - 1).divisible_by(pow) {
            assert_eq!(floor_log, ceiling_log);
        } else {
            assert_eq!(floor_log, ceiling_log - 1);
        }
    });

    unsigned_gen_var_1::<T>().test_properties(|n| {
        assert_eq!(n.ceiling_log_base_power_of_2(1), n.ceiling_log_base_2());
        assert_eq!(
            n.ceiling_log_base_power_of_2(T::WIDTH),
            u64::from(n != T::ONE)
        );
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(T::ONE.ceiling_log_base_power_of_2(pow), 0);
        if pow < T::WIDTH {
            assert_eq!(T::power_of_2(pow).ceiling_log_base_power_of_2(pow), 1);
        }
    });
}

fn ceiling_log_base_power_of_2_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_unsigned_pair_gen_var_3::<T, u64>().test_properties(|(n, pow)| {
        let ceiling_log = n.ceiling_log_base_power_of_2(pow);
        assert!(ceiling_log <= T::MAX_EXPONENT + 1);
        assert!(ceiling_log >= T::MIN_EXPONENT);
        let i_pow = i64::exact_from(pow);
        if i_pow >= T::MIN_EXPONENT && i_pow <= T::MAX_EXPONENT {
            assert_eq!(
                ceiling_log == 0,
                n > T::ONE / T::power_of_2(i_pow) && n <= T::ONE
            );
        }
        let product = ceiling_log * i_pow;
        if product >= T::MIN_EXPONENT && product <= T::MAX_EXPONENT {
            assert!(T::power_of_2(product) >= n);
        }
        let product_2 = product - i_pow;
        if product_2 >= T::MIN_EXPONENT && product_2 <= T::MAX_EXPONENT {
            assert!(T::power_of_2(product_2) < n);
        }

        let floor_log = n.floor_log_base_power_of_2(pow);
        if n.is_power_of_2() && n.floor_log_base_2().divisible_by(i_pow) {
            assert_eq!(floor_log, ceiling_log);
        } else {
            assert_eq!(floor_log, ceiling_log - 1);
        }
    });

    primitive_float_gen_var_18::<T>().test_properties(|n| {
        assert_eq!(n.ceiling_log_base_power_of_2(1), n.ceiling_log_base_2());
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(T::ONE.ceiling_log_base_power_of_2(pow), 0);
    });
}

#[test]
fn ceiling_log_base_power_of_2_properties() {
    apply_fn_to_unsigneds!(ceiling_log_base_power_of_2_properties_helper_unsigned);
    apply_fn_to_primitive_floats!(ceiling_log_base_power_of_2_properties_helper_primitive_float);
}

fn checked_log_base_power_of_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_21::<T, u64>().test_properties(|(n, pow)| {
        let checked_log = n.checked_log_base_power_of_2(pow);
        assert_eq!(
            checked_log.is_some(),
            n.is_power_of_2() && (n.significant_bits() - 1).divisible_by(pow)
        );
        if pow < T::WIDTH {
            assert_eq!(n.checked_log_base(T::power_of_2(pow)), checked_log);
        }
        if let Some(log) = checked_log {
            assert_eq!(T::power_of_2(log * pow), n);
            assert!(log <= T::WIDTH);
            assert_eq!(log == 0, n == T::ONE);
            assert_eq!(n.floor_log_base_power_of_2(pow), log);
            assert_eq!(n.ceiling_log_base_power_of_2(pow), log);
        }
    });

    unsigned_gen_var_1::<T>().test_properties(|n| {
        assert_eq!(n.checked_log_base_power_of_2(1), n.checked_log_base_2());
        assert_eq!(
            n.checked_log_base_power_of_2(T::WIDTH),
            if n == T::ONE { Some(0) } else { None }
        );
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(T::ONE.checked_log_base_power_of_2(pow), Some(0));
        if pow < T::WIDTH {
            assert_eq!(T::power_of_2(pow).checked_log_base_power_of_2(pow), Some(1));
        }
    });
}

fn checked_log_base_power_of_2_properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_unsigned_pair_gen_var_3::<T, u64>().test_properties(|(n, pow)| {
        let checked_log = n.checked_log_base_power_of_2(pow);
        let i_pow = i64::exact_from(pow);
        assert_eq!(
            checked_log.is_some(),
            n.is_power_of_2() && n.checked_log_base_2().unwrap().divisible_by(i_pow)
        );
        if let Some(log) = checked_log {
            assert_eq!(T::power_of_2(log * i_pow), n);
            assert!(log <= T::MAX_EXPONENT);
            assert!(log >= T::MIN_EXPONENT);
            assert_eq!(log == 0, n == T::ONE);
            assert_eq!(n.floor_log_base_power_of_2(pow), log);
            assert_eq!(n.ceiling_log_base_power_of_2(pow), log);
        }
    });

    primitive_float_gen_var_18::<T>().test_properties(|n| {
        assert_eq!(n.checked_log_base_power_of_2(1), n.checked_log_base_2());
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(T::ONE.checked_log_base_power_of_2(pow), Some(0));
        let i_pow = i64::exact_from(pow);
        if i_pow >= T::MIN_EXPONENT && i_pow <= T::MAX_EXPONENT {
            assert_eq!(
                T::power_of_2(i_pow).checked_log_base_power_of_2(pow),
                Some(1)
            );
        }
    });
}

#[test]
fn checked_log_base_power_of_2_properties() {
    apply_fn_to_unsigneds!(checked_log_base_power_of_2_properties_helper_unsigned);
    apply_fn_to_primitive_floats!(checked_log_base_power_of_2_properties_helper_primitive_float);
}
