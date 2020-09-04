use malachite_base::num::basic::integers::PrimitiveInt;

fn not_assign_helper<T: PrimitiveInt>() {
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
    apply_fn_to_primitive_ints!(not_assign_helper);
}
