use malachite_base::num::arithmetic::log_base_power_of_2::_ceiling_log_base_power_of_2_naive;
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{
    unsigned_gen_var_1, unsigned_gen_var_11, unsigned_pair_gen_var_21,
};
use std::panic::catch_unwind;

fn floor_log_base_power_of_2_helper<T: PrimitiveUnsigned>() {
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

#[test]
fn test_floor_log_base_power_of_2() {
    apply_fn_to_unsigneds!(floor_log_base_power_of_2_helper);
}

fn floor_log_base_power_of_2_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.floor_log_base_power_of_2(2));
    assert_panic!(T::TWO.floor_log_base_power_of_2(0));
}

#[test]
fn floor_log_base_power_of_2_fail() {
    apply_fn_to_unsigneds!(floor_log_base_power_of_2_fail_helper);
}

fn ceiling_log_base_power_of_2_helper<T: PrimitiveUnsigned>() {
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

#[test]
fn test_ceiling_log_base_power_of_2() {
    apply_fn_to_unsigneds!(ceiling_log_base_power_of_2_helper);
}

fn ceiling_log_base_power_of_2_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.ceiling_log_base_power_of_2(2));
    assert_panic!(T::TWO.ceiling_log_base_power_of_2(0));
}

#[test]
fn ceiling_log_base_power_of_2_fail() {
    apply_fn_to_unsigneds!(ceiling_log_base_power_of_2_fail_helper);
}

fn checked_log_base_power_of_2_helper<T: PrimitiveUnsigned>() {
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

#[test]
fn test_checked_log_base_power_of_2() {
    apply_fn_to_unsigneds!(checked_log_base_power_of_2_helper);
}

fn checked_log_base_power_of_2_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.checked_log_base_power_of_2(2));
    assert_panic!(T::TWO.checked_log_base_power_of_2(0));
}

#[test]
fn checked_log_base_power_of_2_fail() {
    apply_fn_to_unsigneds!(checked_log_base_power_of_2_fail_helper);
}

fn floor_log_base_power_of_2_properties_helper<T: PrimitiveUnsigned>() {
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

#[test]
fn floor_log_base_power_of_2_properties() {
    apply_fn_to_unsigneds!(floor_log_base_power_of_2_properties_helper);
}

fn ceiling_log_base_power_of_2_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_21::<T, u64>().test_properties(|(n, pow)| {
        let ceiling_log = n.ceiling_log_base_power_of_2(pow);
        assert!(ceiling_log <= T::WIDTH);
        assert_eq!(ceiling_log, _ceiling_log_base_power_of_2_naive(n, pow));
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
            u64::iverson(n != T::ONE)
        );
    });

    unsigned_gen_var_11().test_properties(|pow| {
        assert_eq!(T::ONE.ceiling_log_base_power_of_2(pow), 0);
        if pow < T::WIDTH {
            assert_eq!(T::power_of_2(pow).ceiling_log_base_power_of_2(pow), 1);
        }
    });
}

#[test]
fn ceiling_log_base_power_of_2_properties() {
    apply_fn_to_unsigneds!(ceiling_log_base_power_of_2_properties_helper);
}

fn checked_log_base_power_of_2_properties_helper<T: PrimitiveUnsigned>() {
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

#[test]
fn checked_log_base_power_of_2_properties() {
    apply_fn_to_unsigneds!(checked_log_base_power_of_2_properties_helper);
}
