use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_two_is_reduced_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.mod_power_of_two_is_reduced(pow), out);
    };

    test(T::ZERO, 5, true);
    test(T::exact_from(100), 5, false);
    test(T::exact_from(100), 8, true);
    test(T::MAX, T::WIDTH - 1, false);
    test(T::MAX, T::WIDTH, true);
}

#[test]
fn test_mod_power_of_two_is_reduced() {
    apply_fn_to_unsigneds!(mod_power_of_two_is_reduced_helper);
}
