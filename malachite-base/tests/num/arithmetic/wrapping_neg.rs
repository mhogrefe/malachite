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
    unsigned_wrapping_neg_helper::<u8>();
    unsigned_wrapping_neg_helper::<u16>();
    unsigned_wrapping_neg_helper::<u32>();
    unsigned_wrapping_neg_helper::<u64>();
    unsigned_wrapping_neg_helper::<u128>();
    unsigned_wrapping_neg_helper::<usize>();

    signed_wrapping_neg_helper::<i8>();
    signed_wrapping_neg_helper::<i16>();
    signed_wrapping_neg_helper::<i32>();
    signed_wrapping_neg_helper::<i64>();
    signed_wrapping_neg_helper::<i128>();
    signed_wrapping_neg_helper::<isize>();
}
