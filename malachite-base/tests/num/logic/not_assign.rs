use malachite_base::num::basic::integers::PrimitiveInteger;

fn not_assign_helper<T: PrimitiveInteger>() {
    let test = |n: T| {
        let mut x = n;
        x.not_assign();
        assert_eq!(x, !n);
    };
    test(T::ZERO);
    test(T::ONE);
    test(T::exact_from(2));
    test(T::exact_from(3));
    test(T::exact_from(4));
    test(T::exact_from(5));
    test(T::exact_from(100));
    test(T::exact_from(63));
    test(T::exact_from(64));
    test(T::MIN);
    test(T::MAX);
}

#[test]
fn test_not_assign() {
    not_assign_helper::<u8>();
    not_assign_helper::<u16>();
    not_assign_helper::<u32>();
    not_assign_helper::<u64>();
    not_assign_helper::<u128>();
    not_assign_helper::<usize>();
    not_assign_helper::<i8>();
    not_assign_helper::<i16>();
    not_assign_helper::<i32>();
    not_assign_helper::<i64>();
    not_assign_helper::<i128>();
    not_assign_helper::<isize>();
}
