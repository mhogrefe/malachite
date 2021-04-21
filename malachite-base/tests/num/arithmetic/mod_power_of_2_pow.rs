use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_2_pow_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, exp: u64, pow: u64, out| {
        assert_eq!(x.mod_power_of_2_pow(exp, pow), out);

        let mut mut_x = x;
        mut_x.mod_power_of_2_pow_assign(exp, pow);
        assert_eq!(mut_x, out);
    };
    test(T::ZERO, 0, 0, T::ZERO);
    test(T::ZERO, 0, 3, T::ONE);
    test(T::ZERO, 1, 3, T::ZERO);

    test(T::TWO, 2, 3, T::exact_from(4));
    test(T::exact_from(5), 13, 3, T::exact_from(5));
    test(T::exact_from(7), 1000, 6, T::ONE);
    test(T::exact_from(101), 1000000, 8, T::ONE);
}

#[test]
fn test_mod_power_of_2_pow() {
    apply_fn_to_unsigneds!(mod_power_of_2_pow_helper);
}
