use malachite_base::num::basic::signeds::PrimitiveSigned;

fn overflowing_abs_helper<T: PrimitiveSigned>() {
    let test = |n: T, out, overflow| {
        assert_eq!(n.overflowing_abs(), (out, overflow));

        let mut n = n;
        assert_eq!(n.overflowing_abs_assign(), overflow);
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO, false);
    test(T::ONE, T::ONE, false);
    test(T::exact_from(100), T::exact_from(100), false);
    test(T::MAX, T::MAX, false);
    test(T::NEGATIVE_ONE, T::ONE, false);
    test(T::exact_from(-100), T::exact_from(100), false);
    test(T::MIN, T::MIN, true);
}

#[test]
fn test_overflowing_abs() {
    overflowing_abs_helper::<i8>();
    overflowing_abs_helper::<i16>();
    overflowing_abs_helper::<i32>();
    overflowing_abs_helper::<i64>();
    overflowing_abs_helper::<i128>();
    overflowing_abs_helper::<isize>();
}
