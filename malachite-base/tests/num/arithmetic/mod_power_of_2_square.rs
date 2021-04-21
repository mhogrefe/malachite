use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_2_square_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, pow: u64, out| {
        assert_eq!(x.mod_power_of_2_square(pow), out);

        let mut mut_x = x;
        mut_x.mod_power_of_2_square_assign(pow);
        assert_eq!(mut_x, out);
    };
    test(T::ZERO, 0, T::ZERO);
    test(T::ZERO, 2, T::ZERO);
    test(T::ONE, 2, T::ONE);
    test(T::TWO, 2, T::ZERO);
    test(T::TWO, 3, T::exact_from(4));
    test(T::exact_from(5), 3, T::ONE);
    test(T::exact_from(100), 8, T::exact_from(16));
}

#[test]
fn test_mod_power_of_2_square() {
    apply_fn_to_unsigneds!(mod_power_of_2_square_helper);
}
