use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn next_power_of_two_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, out| {
        assert_eq!(n.next_power_of_two(), out);

        let mut n = n;
        n.next_power_of_two_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ONE);
    test(T::ONE, T::ONE);
    test(T::exact_from(7), T::exact_from(8));
    test(T::exact_from(8), T::exact_from(8));
    test(T::exact_from(10), T::exact_from(16));
    test((T::MAX >> 1u64) + T::ONE, (T::MAX >> 1u64) + T::ONE);
    test(
        (T::MAX >> 1u64) - T::exact_from(10),
        (T::MAX >> 1u64) + T::ONE,
    );
}

#[test]
fn test_next_power_of_two() {
    apply_fn_to_unsigneds!(next_power_of_two_helper);
}
