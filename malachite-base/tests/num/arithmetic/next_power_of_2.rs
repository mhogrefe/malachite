use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::unsigned_gen_var_14;

fn next_power_of_2_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, out| {
        assert_eq!(n.next_power_of_2(), out);

        let mut n = n;
        n.next_power_of_2_assign();
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
fn test_next_power_of_2() {
    apply_fn_to_unsigneds!(next_power_of_2_helper);
}

fn next_power_of_2_assign_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_14::<T>().test_properties(|n| {
        let p = n.next_power_of_2();
        assert!(p >= n);
        assert!(p >> 1 <= n);
        assert!(p.is_power_of_2());

        let mut n = n;
        n.next_power_of_2_assign();
        assert_eq!(n, p);
    });
}

#[test]
fn next_power_of_2_assign_properties() {
    apply_fn_to_unsigneds!(next_power_of_2_assign_properties_helper);
}
