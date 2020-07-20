use malachite_base::num::basic::signeds::PrimitiveSigned;

fn neg_assign_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        let mut n = n;
        n.neg_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::NEGATIVE_ONE);
    test(T::exact_from(100), T::exact_from(-100));
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
}

#[test]
fn test_neg_assign() {
    apply_fn_to_signeds!(neg_assign_helper);
}
