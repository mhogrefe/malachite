use malachite_base::num::basic::signeds::PrimitiveSigned;

fn wrapping_abs_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.wrapping_abs(), out);

        let mut n = n;
        n.wrapping_abs_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::ONE);
    test(T::exact_from(100), T::exact_from(100));
    test(T::MAX, T::MAX);
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
    test(T::MIN, T::MIN);
}

#[test]
fn test_wrapping_neg() {
    wrapping_abs_helper::<i8>();
    wrapping_abs_helper::<i16>();
    wrapping_abs_helper::<i32>();
    wrapping_abs_helper::<i64>();
    wrapping_abs_helper::<i128>();
    wrapping_abs_helper::<isize>();
}
