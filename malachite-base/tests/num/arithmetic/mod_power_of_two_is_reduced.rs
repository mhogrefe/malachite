use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_two_is_reduced_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, log_base, out| {
        assert_eq!(n.mod_power_of_two_is_reduced(log_base), out);
    };

    test(T::ZERO, 5, true);
    test(T::exact_from(100), 5, false);
    test(T::exact_from(100), 8, true);
    test(T::MAX, T::WIDTH - 1, false);
    test(T::MAX, T::WIDTH, true);
}

#[test]
fn test_mod_power_of_two_is_reduced() {
    mod_power_of_two_is_reduced_helper::<u8>();
    mod_power_of_two_is_reduced_helper::<u16>();
    mod_power_of_two_is_reduced_helper::<u32>();
    mod_power_of_two_is_reduced_helper::<u64>();
    mod_power_of_two_is_reduced_helper::<u128>();
    mod_power_of_two_is_reduced_helper::<usize>();
}
