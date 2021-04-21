use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_2_sub_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, pow, out| {
        assert_eq!(x.mod_power_of_2_sub(y, pow), out);

        let mut x = x;
        x.mod_power_of_2_sub_assign(y, pow);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, 0, T::ZERO);
    test(T::ZERO, T::ONE, 1, T::ONE);
    test(T::ONE, T::ONE, 1, T::ZERO);
    test(T::exact_from(5), T::TWO, 5, T::exact_from(3));
    test(T::exact_from(10), T::exact_from(14), 4, T::exact_from(12));
    test(
        T::exact_from(100),
        T::exact_from(200),
        8,
        T::exact_from(156),
    );
    test(T::ZERO, T::ONE, T::WIDTH, T::MAX);
    test(T::ONE, T::MAX, T::WIDTH, T::TWO);
}

#[test]
fn test_mod_power_of_2_sub() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_helper);
}
