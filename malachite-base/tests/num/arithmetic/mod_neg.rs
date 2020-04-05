use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

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
    mod_neg_helper::<u8>();
    mod_neg_helper::<u16>();
    mod_neg_helper::<u32>();
    mod_neg_helper::<u64>();
    mod_neg_helper::<u128>();
    mod_neg_helper::<usize>();
}
