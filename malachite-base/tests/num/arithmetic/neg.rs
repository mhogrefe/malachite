use malachite_base::num::basic::signeds::PrimitiveSigned;

fn neg_assign_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        let mut n = n;
        n.neg_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::NEGATIVE_ONE);
    test(T::exact_from(100), T::exact_from(-100));
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
}

#[test]
fn test_neg_assign() {
    neg_assign_helper::<i8>();
    neg_assign_helper::<i16>();
    neg_assign_helper::<i32>();
    neg_assign_helper::<i64>();
    neg_assign_helper::<i128>();
    neg_assign_helper::<isize>();
}
