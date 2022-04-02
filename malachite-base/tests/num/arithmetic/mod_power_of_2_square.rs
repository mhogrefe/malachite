use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_gen_var_9, unsigned_pair_gen_var_17, unsigned_triple_gen_var_11,
};

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

fn mod_power_of_2_square_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_17::<T>().test_properties(|(x, pow)| {
        assert!(x.mod_power_of_2_is_reduced(pow));
        let square = x.mod_power_of_2_square(pow);
        assert!(square.mod_power_of_2_is_reduced(pow));

        let mut x_alt = x;
        x_alt.mod_power_of_2_square_assign(pow);
        assert_eq!(x_alt, square);

        assert_eq!(x.mod_power_of_2_pow(2, pow), x.mod_power_of_2_mul(x, pow));
        assert_eq!(x.mod_power_of_2_neg(pow).mod_power_of_2_square(pow), square);
    });

    unsigned_gen_var_9::<T>().test_properties(|pow| {
        assert_eq!(T::ZERO.mod_power_of_2_square(pow), T::ZERO);
        if pow != 0 {
            assert_eq!(T::ONE.mod_power_of_2_square(pow), T::ONE);
        }
    });

    unsigned_triple_gen_var_11::<T>().test_properties(|(x, y, pow)| {
        assert_eq!(
            x.mod_power_of_2_mul(y, pow).mod_power_of_2_square(pow),
            x.mod_power_of_2_square(pow)
                .mod_power_of_2_mul(y.mod_power_of_2_square(pow), pow)
        );
    });
}

#[test]
fn mod_power_of_2_square_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_square_properties_helper);
}
