use std::cmp::Ordering;

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;

fn primitive_sign_helper<T: PrimitiveInt>() {
    let test = |n: T, out| {
        assert_eq!(n.sign(), out);
    };
    test(T::ZERO, Ordering::Equal);
    test(T::ONE, Ordering::Greater);
    test(T::exact_from(100), Ordering::Greater);
    test(T::MAX, Ordering::Greater);
}

fn signed_sign_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.sign(), out);
    };
    test(T::NEGATIVE_ONE, Ordering::Less);
    test(T::exact_from(-100), Ordering::Less);
    test(T::MIN, Ordering::Less);
}

#[test]
fn test_sign() {
    apply_fn_to_primitive_ints!(primitive_sign_helper);
    apply_fn_to_signeds!(signed_sign_helper);
}
