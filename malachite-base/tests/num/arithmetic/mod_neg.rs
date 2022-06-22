use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_gen_var_1, unsigned_gen_var_6, unsigned_pair_gen_var_16,
};

fn mod_neg_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, m, out| {
        assert_eq!(n.mod_neg(m), out);

        let mut n = n;
        n.mod_neg_assign(m);
        assert_eq!(n, out);
    };

    test(T::ZERO, T::exact_from(5), T::ZERO);
    test(T::exact_from(7), T::exact_from(10), T::exact_from(3));
    test(T::exact_from(100), T::exact_from(101), T::ONE);
    test(T::MAX - T::ONE, T::MAX, T::ONE);
    test(T::ONE, T::MAX, T::MAX - T::ONE);
}

#[test]
fn test_mod_neg() {
    apply_fn_to_unsigneds!(mod_neg_helper);
}

fn mod_neg_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_16::<T>().test_properties(|(n, m)| {
        assert!(n.mod_is_reduced(&m));
        let neg = n.mod_neg(m);
        assert!(neg.mod_is_reduced(&m));

        let mut n_alt = n;
        n_alt.mod_neg_assign(m);
        assert_eq!(n_alt, neg);

        assert_eq!(neg.mod_neg(m), n);
        assert_eq!(n.mod_add(neg, m), T::ZERO);
        assert_eq!(n == neg, n == T::ZERO || m.even() && n == m >> 1);
    });

    unsigned_gen_var_1::<T>().test_properties(|m| {
        assert_eq!(T::ZERO.mod_neg(m), T::ZERO);
    });

    unsigned_gen_var_6::<T>().test_properties(|m| {
        assert_eq!(T::ONE.mod_neg(m), m - T::ONE);
        assert_eq!((m - T::ONE).mod_neg(m), T::ONE);
    });
}

#[test]
fn mod_neg_properties() {
    apply_fn_to_unsigneds!(mod_neg_properties_helper);
}
