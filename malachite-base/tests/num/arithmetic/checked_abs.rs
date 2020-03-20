use malachite_base::num::basic::signeds::PrimitiveSigned;

fn checked_abs_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.checked_abs(), out);
    };
    test(T::ZERO, Some(T::ZERO));
    test(T::ONE, Some(T::ONE));
    test(T::exact_from(100), Some(T::exact_from(100)));
    test(T::NEGATIVE_ONE, Some(T::ONE));
    test(T::exact_from(-100), Some(T::exact_from(100)));
    test(T::MIN, None);
}

#[test]
fn test_checked_abs() {
    checked_abs_helper::<i8>();
    checked_abs_helper::<i16>();
    checked_abs_helper::<i32>();
    checked_abs_helper::<i64>();
    checked_abs_helper::<i128>();
    checked_abs_helper::<isize>();
}
