use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{unsigned_pair_gen_var_16, unsigned_triple_gen_var_12};

fn mod_sub_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, m, out| {
        assert_eq!(x.mod_sub(y, m), out);

        let mut x = x;
        x.mod_sub_assign(y, m);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    test(T::exact_from(4), T::exact_from(3), T::exact_from(5), T::ONE);
    test(
        T::exact_from(7),
        T::exact_from(9),
        T::exact_from(10),
        T::exact_from(8),
    );
    test(
        T::exact_from(100),
        T::exact_from(120),
        T::exact_from(123),
        T::exact_from(103),
    );
    test(T::ZERO, T::ONE, T::MAX, T::MAX - T::ONE);
    test(T::MAX - T::TWO, T::MAX - T::ONE, T::MAX, T::MAX - T::ONE);
}

#[test]
fn test_mod_sub() {
    apply_fn_to_unsigneds!(mod_sub_helper);
}

fn mod_sub_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_12::<T>().test_properties(|(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let diff = x.mod_sub(y, m);
        assert!(diff.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_sub_assign(y, m);
        assert_eq!(x_alt, diff);

        assert_eq!(diff.mod_add(y, m), x);
        assert_eq!(diff.mod_sub(x, m), y.mod_neg(m));
        assert_eq!(y.mod_sub(x, m), diff.mod_neg(m));
        assert_eq!(x.mod_add(y.mod_neg(m), m), diff);
    });

    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        assert_eq!(x.mod_sub(T::ZERO, m), x);
        assert_eq!(T::ZERO.mod_sub(x, m), x.mod_neg(m));
        assert_eq!(x.mod_sub(x, m), T::ZERO);
    });
}

#[test]
fn mod_sub_properties() {
    apply_fn_to_unsigneds!(mod_sub_properties_helper);
}
