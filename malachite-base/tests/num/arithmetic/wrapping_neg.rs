use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn unsigned_wrapping_neg_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, out| {
        assert_eq!(n.wrapping_neg(), out);

        let mut n = n;
        n.wrapping_neg_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::MAX);
    test(T::exact_from(100), T::MAX - T::exact_from(100) + T::ONE);
    test(T::MAX, T::ONE);
}

fn signed_wrapping_neg_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.wrapping_neg(), out);

        let mut n = n;
        n.wrapping_neg_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::NEGATIVE_ONE);
    test(T::exact_from(100), T::exact_from(-100));
    test(T::MAX, T::MIN + T::ONE);
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
    test(T::MIN, T::MIN);
}

#[test]
fn test_wrapping_neg() {
    apply_fn_to_unsigneds!(unsigned_wrapping_neg_helper);
    apply_fn_to_signeds!(signed_wrapping_neg_helper);
}
