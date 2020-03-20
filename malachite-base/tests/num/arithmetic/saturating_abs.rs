use malachite_base::num::basic::signeds::PrimitiveSigned;

fn saturating_abs_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.saturating_abs(), out);

        let mut n = n;
        n.saturating_abs_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::ONE);
    test(T::exact_from(100), T::exact_from(100));
    test(T::MAX, T::MAX);
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
    test(T::MIN, T::MAX);
}

#[test]
fn test_saturating_abs() {
    saturating_abs_helper::<i8>();
    saturating_abs_helper::<i16>();
    saturating_abs_helper::<i32>();
    saturating_abs_helper::<i64>();
    saturating_abs_helper::<i128>();
    saturating_abs_helper::<isize>();
}
