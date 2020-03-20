use std::cmp::Ordering;

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;

fn primitive_sign_helper<T: PrimitiveInteger>() {
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
    primitive_sign_helper::<u8>();
    primitive_sign_helper::<u16>();
    primitive_sign_helper::<u32>();
    primitive_sign_helper::<u64>();
    primitive_sign_helper::<u128>();
    primitive_sign_helper::<usize>();
    primitive_sign_helper::<i8>();
    primitive_sign_helper::<i16>();
    primitive_sign_helper::<i32>();
    primitive_sign_helper::<i64>();
    primitive_sign_helper::<i128>();
    primitive_sign_helper::<isize>();

    signed_sign_helper::<i8>();
    signed_sign_helper::<i16>();
    signed_sign_helper::<i32>();
    signed_sign_helper::<i64>();
    signed_sign_helper::<i128>();
    signed_sign_helper::<isize>();
}
