use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::unsigned_gen_var_1;
use std::panic::catch_unwind;

fn floor_log_base_2_helper<T: PrimitiveUnsigned>() {
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

#[test]
fn test_floor_log_base_2() {
    apply_fn_to_unsigneds!(floor_log_base_2_helper);
}

fn floor_log_base_2_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.floor_log_base_2());
}

#[test]
fn floor_log_base_2_fail() {
    apply_fn_to_unsigneds!(floor_log_base_2_fail_helper);
}

fn ceiling_log_base_2_helper<T: PrimitiveUnsigned>() {
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

#[test]
fn test_ceiling_log_base_2() {
    apply_fn_to_unsigneds!(ceiling_log_base_2_helper);
}

fn ceiling_log_base_2_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.ceiling_log_base_2());
}

#[test]
fn ceiling_log_base_2_fail() {
    apply_fn_to_unsigneds!(ceiling_log_base_2_fail_helper);
}

fn floor_log_base_2_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_1::<T>().test_properties(|n| {
        let floor_log_base_2 = n.floor_log_base_2();
        assert_eq!(floor_log_base_2, n.significant_bits() - 1);
        assert!(floor_log_base_2 < T::WIDTH);
        assert_eq!(floor_log_base_2 == 0, n == T::ONE);
    });
}

#[test]
fn floor_log_base_2_properties() {
    apply_fn_to_unsigneds!(floor_log_base_2_properties_helper);
}

fn ceiling_log_base_2_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_1::<T>().test_properties(|n| {
        let ceiling_log_base_2 = n.ceiling_log_base_2();
        assert!(ceiling_log_base_2 <= T::WIDTH);
        assert_eq!(ceiling_log_base_2 == 0, n == T::ONE);
    });
}

#[test]
fn ceiling_log_base_2_properties() {
    apply_fn_to_unsigneds!(ceiling_log_base_2_properties_helper);
}
