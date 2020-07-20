use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn unsigned_overflowing_neg_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, out, overflow| {
        assert_eq!(n.overflowing_neg(), (out, overflow));

        let mut n = n;
        assert_eq!(n.overflowing_neg_assign(), overflow);
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO, false);
    test(T::ONE, T::MAX, true);
    test(
        T::exact_from(100),
        T::MAX - T::exact_from(100) + T::ONE,
        true,
    );
    test(T::MAX, T::ONE, true);
}

fn signed_overflowing_neg_helper<T: PrimitiveSigned>() {
    let test = |n: T, out, overflow| {
        assert_eq!(n.overflowing_neg(), (out, overflow));

        let mut n = n;
        assert_eq!(n.overflowing_neg_assign(), overflow);
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO, false);
    test(T::ONE, T::NEGATIVE_ONE, false);
    test(T::exact_from(100), T::exact_from(-100), false);
    test(T::MAX, T::MIN + T::ONE, false);
    test(T::NEGATIVE_ONE, T::ONE, false);
    test(T::exact_from(-100), T::exact_from(100), false);
    test(T::MIN, T::MIN, true);
}

#[test]
fn test_overflowing_neg() {
    apply_fn_to_unsigneds!(unsigned_overflowing_neg_helper);
    apply_fn_to_signeds!(signed_overflowing_neg_helper);
}
