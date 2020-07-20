use malachite_base::num::basic::signeds::PrimitiveSigned;

fn saturating_neg_assign_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.saturating_neg(), out);

        let mut n = n;
        n.saturating_neg_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::NEGATIVE_ONE);
    test(T::exact_from(100), T::exact_from(-100));
    test(T::MAX, T::MIN + T::ONE);
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
    test(T::MIN, T::MAX);
}

#[test]
fn test_saturating_neg_assign() {
    apply_fn_to_signeds!(saturating_neg_assign_helper);
}
