use std::panic::catch_unwind;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_is_reduced_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, m, out| {
        assert_eq!(n.mod_is_reduced(&m), out);
    };
    test(T::ZERO, T::exact_from(5), true);
    test(T::exact_from(100), T::exact_from(100), false);
    test(T::exact_from(100), T::exact_from(101), true);
    test(T::MAX - T::ONE, T::MAX - T::ONE, false);
    test(T::MAX - T::ONE, T::MAX, true);
    test(T::MAX, T::MAX, false);
}

#[test]
fn test_mod_is_reduced() {
    apply_fn_to_unsigneds!(mod_is_reduced_helper);
}

fn mod_is_reduced_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_is_reduced(&T::ZERO));
}

#[test]
fn mod_is_reduced_fail() {
    apply_fn_to_unsigneds!(mod_is_reduced_fail_helper);
}
