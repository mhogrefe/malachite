use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base_test_util::generators::signed_gen_var_1;

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

fn neg_assign_properties_helper<T: PrimitiveSigned>() {
    signed_gen_var_1::<T>().test_properties(|n| {
        let mut neg = n;
        neg.neg_assign();
        assert_eq!(neg, -n);
        assert_eq!(-neg, n);
        assert_eq!(neg == n, n == T::ZERO);
        assert_eq!(n + neg, T::ZERO);
    });
}

#[test]
fn neg_assign_properties() {
    apply_fn_to_signeds!(neg_assign_properties_helper);
}
