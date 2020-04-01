use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_two_add_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, pow, out| {
        assert_eq!(x.mod_power_of_two_add(y, pow), out);

        let mut x = x;
        x.mod_power_of_two_add_assign(y, pow);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, 0, T::ZERO);
    test(T::ZERO, T::ONE, 1, T::ONE);
    test(T::ONE, T::ONE, 1, T::ZERO);
    test(T::ZERO, T::TWO, 5, T::TWO);
    test(T::exact_from(10), T::exact_from(14), 4, T::exact_from(8));
    test(T::exact_from(100), T::exact_from(200), 8, T::exact_from(44));
    test(T::MAX, T::ONE, T::WIDTH, T::ZERO);
    test(T::MAX, T::MAX, T::WIDTH, T::MAX - T::ONE);
}

#[test]
fn test_mod_power_of_two_add() {
    mod_power_of_two_add_helper::<u8>();
    mod_power_of_two_add_helper::<u16>();
    mod_power_of_two_add_helper::<u32>();
    mod_power_of_two_add_helper::<u64>();
    mod_power_of_two_add_helper::<u128>();
    mod_power_of_two_add_helper::<usize>();
}
