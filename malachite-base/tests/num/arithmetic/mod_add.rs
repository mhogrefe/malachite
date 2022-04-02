use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_16, unsigned_quadruple_gen_var_4, unsigned_triple_gen_var_12,
};

fn mod_add_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, m, out| {
        assert_eq!(x.mod_add(y, m), out);

        let mut x = x;
        x.mod_add_assign(y, m);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    test(
        T::ZERO,
        T::exact_from(3),
        T::exact_from(5),
        T::exact_from(3),
    );
    test(
        T::exact_from(7),
        T::exact_from(5),
        T::exact_from(10),
        T::TWO,
    );
    test(
        T::exact_from(100),
        T::exact_from(100),
        T::exact_from(123),
        T::exact_from(77),
    );
    test(T::MAX - T::ONE, T::ONE, T::MAX, T::ZERO);
    test(T::MAX - T::ONE, T::MAX - T::ONE, T::MAX, T::MAX - T::TWO);
}

#[test]
fn test_mod_add() {
    apply_fn_to_unsigneds!(mod_add_helper);
}

fn mod_add_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_12::<T>().test_properties(|(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let sum = x.mod_add(y, m);
        assert!(sum.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_add_assign(y, m);
        assert_eq!(x_alt, sum);

        assert_eq!(sum.mod_sub(y, m), x);
        assert_eq!(sum.mod_sub(x, m), y);
        assert_eq!(y.mod_add(x, m), sum);
        assert_eq!(x.mod_sub(y.mod_neg(m), m), sum);
    });

    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        assert_eq!(x.mod_add(T::ZERO, m), x);
        assert_eq!(T::ZERO.mod_add(x, m), x);
        assert_eq!(x.mod_add(x.mod_neg(m), m), T::ZERO);
    });

    unsigned_quadruple_gen_var_4::<T>().test_properties(|(x, y, z, m)| {
        assert_eq!(x.mod_add(y, m).mod_add(z, m), x.mod_add(y.mod_add(z, m), m));
    });
}

#[test]
fn mod_add_properties() {
    apply_fn_to_unsigneds!(mod_add_properties_helper);
}
