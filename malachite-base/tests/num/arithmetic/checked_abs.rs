use malachite_base::num::basic::signeds::PrimitiveSigned;

fn checked_abs_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.checked_abs(), out);
    };
    test(T::ZERO, Some(T::ZERO));
    test(T::ONE, Some(T::ONE));
    test(T::exact_from(100), Some(T::exact_from(100)));
    test(T::NEGATIVE_ONE, Some(T::ONE));
    test(T::exact_from(-100), Some(T::exact_from(100)));
    test(T::MIN, None);
}

#[test]
fn test_checked_abs() {
    apply_fn_to_signeds!(checked_abs_helper);
}
