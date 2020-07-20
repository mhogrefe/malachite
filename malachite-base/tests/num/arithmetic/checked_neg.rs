use malachite_base::num::basic::signeds::PrimitiveSigned;

fn checked_neg_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.checked_neg(), out);
    };
    test(T::ZERO, Some(T::ZERO));
    test(T::ONE, Some(T::NEGATIVE_ONE));
    test(T::exact_from(100), Some(T::exact_from(-100)));
    test(T::NEGATIVE_ONE, Some(T::ONE));
    test(T::exact_from(-100), Some(T::exact_from(100)));
    test(T::MIN, None);
}

#[test]
fn test_checked_neg() {
    apply_fn_to_signeds!(checked_neg_helper);
}
