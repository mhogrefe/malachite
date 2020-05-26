use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;

fn even_primitive_helper<T: PrimitiveInteger>() {
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
    even_primitive_helper::<u8>();
    even_primitive_helper::<u16>();
    even_primitive_helper::<u32>();
    even_primitive_helper::<u64>();
    even_primitive_helper::<u128>();
    even_primitive_helper::<usize>();
    even_primitive_helper::<i8>();
    even_primitive_helper::<i16>();
    even_primitive_helper::<i32>();
    even_primitive_helper::<i64>();
    even_primitive_helper::<i128>();
    even_primitive_helper::<isize>();

    even_signed_helper::<i8>();
    even_signed_helper::<i16>();
    even_signed_helper::<i32>();
    even_signed_helper::<i64>();
    even_signed_helper::<i128>();
    even_signed_helper::<isize>();
}

fn odd_primitive_helper<T: PrimitiveInteger>() {
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
    odd_primitive_helper::<u8>();
    odd_primitive_helper::<u16>();
    odd_primitive_helper::<u32>();
    odd_primitive_helper::<u64>();
    odd_primitive_helper::<u128>();
    odd_primitive_helper::<usize>();
    odd_primitive_helper::<i8>();
    odd_primitive_helper::<i16>();
    odd_primitive_helper::<i32>();
    odd_primitive_helper::<i64>();
    odd_primitive_helper::<i128>();
    odd_primitive_helper::<isize>();

    odd_signed_helper::<i8>();
    odd_signed_helper::<i16>();
    odd_signed_helper::<i32>();
    odd_signed_helper::<i64>();
    odd_signed_helper::<i128>();
    odd_signed_helper::<isize>();
}
