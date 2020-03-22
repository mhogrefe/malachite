use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_two_neg_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.mod_power_of_two_neg(pow), out);

        let mut n = n;
        n.mod_power_of_two_neg_assign(pow);
        assert_eq!(n, out);
    };
    test(T::ZERO, 5, T::ZERO);
    test(T::exact_from(10), 4, T::exact_from(6));
    test(T::exact_from(100), 8, T::exact_from(156));
    test(T::ONE, T::WIDTH, T::MAX);
    test(T::MAX, T::WIDTH, T::ONE);
}

#[test]
fn test_mod_power_of_two_neg() {
    mod_power_of_two_neg_helper::<u8>();
    mod_power_of_two_neg_helper::<u16>();
    mod_power_of_two_neg_helper::<u32>();
    mod_power_of_two_neg_helper::<u64>();
    mod_power_of_two_neg_helper::<u128>();
    mod_power_of_two_neg_helper::<usize>();
}
