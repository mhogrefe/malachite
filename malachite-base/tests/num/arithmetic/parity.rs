use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;

fn even_primitive_helper<T: PrimitiveInt>() {
    let test = |n: T, out| {
        assert_eq!(n.even(), out);
    };
    test(T::ZERO, true);
    test(T::ONE, false);
    test(T::TWO, true);
    test(T::exact_from(123), false);
    test(T::exact_from(124), true);
    test(T::MAX, false);
}

fn even_signed_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.even(), out);
    };
    test(T::NEGATIVE_ONE, false);
    test(T::exact_from(-123), false);
    test(T::exact_from(-124), true);
    test(T::MIN, true);
}

#[test]
fn test_even() {
    apply_fn_to_primitive_ints!(even_primitive_helper);
    apply_fn_to_signeds!(even_signed_helper);
}

fn odd_primitive_helper<T: PrimitiveInt>() {
    let test = |n: T, out| {
        assert_eq!(n.odd(), out);
    };
    test(T::ZERO, false);
    test(T::ONE, true);
    test(T::TWO, false);
    test(T::exact_from(123), true);
    test(T::exact_from(124), false);
    test(T::MAX, true);
}

fn odd_signed_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.odd(), out);
    };
    test(T::NEGATIVE_ONE, true);
    test(T::exact_from(-123), true);
    test(T::exact_from(-124), false);
    test(T::MIN, false);
}

#[test]
fn test_odd() {
    apply_fn_to_primitive_ints!(odd_primitive_helper);
    apply_fn_to_signeds!(odd_signed_helper);
}
