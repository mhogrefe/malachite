use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_add_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, m, out| {
        assert_eq!(x.mod_add(y, m), out);

        let mut x = x;
        x.mod_add_assign(y, m);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    test(
        T::ZERO,
        T::exact_from(3),
        T::exact_from(5),
        T::exact_from(3),
    );
    test(
        T::exact_from(7),
        T::exact_from(5),
        T::exact_from(10),
        T::TWO,
    );
    test(
        T::exact_from(100),
        T::exact_from(100),
        T::exact_from(123),
        T::exact_from(77),
    );
    test(T::MAX - T::ONE, T::ONE, T::MAX, T::ZERO);
    test(T::MAX - T::ONE, T::MAX - T::ONE, T::MAX, T::MAX - T::TWO);
}

#[test]
fn test_mod_add() {
    apply_fn_to_unsigneds!(mod_add_helper);
}
