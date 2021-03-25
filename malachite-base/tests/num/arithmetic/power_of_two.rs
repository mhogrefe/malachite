use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use std::panic::catch_unwind;

fn power_of_two_primitive_helper<T: PrimitiveInt>() {
    let test = |pow, out| {
        assert_eq!(T::power_of_two(pow), out);
    };
    test(0, T::ONE);
    test(1, T::TWO);
    test(2, T::exact_from(4));
    test(3, T::exact_from(8));
}

fn power_of_two_unsigned_helper<T: PrimitiveUnsigned>() {
    let test = |pow, out| {
        assert_eq!(T::power_of_two(pow), out);
    };
    test(T::WIDTH - 1, T::ONE << (T::WIDTH - 1));
}

#[test]
fn test_power_of_two() {
    apply_fn_to_primitive_ints!(power_of_two_primitive_helper);
    apply_fn_to_unsigneds!(power_of_two_unsigned_helper);
}

fn power_of_two_unsigned_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::power_of_two(T::WIDTH));
}

fn power_of_two_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::power_of_two(T::WIDTH - 1));
}

#[test]
fn power_of_two_fail() {
    apply_fn_to_unsigneds!(power_of_two_unsigned_fail_helper);
    apply_fn_to_signeds!(power_of_two_signed_fail_helper);
}
