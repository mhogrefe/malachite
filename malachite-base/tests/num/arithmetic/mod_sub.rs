use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_sub_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, modulus, out| {
        assert_eq!(x.mod_sub(y, modulus), out);

        let mut x = x;
        x.mod_sub_assign(y, modulus);
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
    mod_sub_helper::<u8>();
    mod_sub_helper::<u16>();
    mod_sub_helper::<u32>();
    mod_sub_helper::<u64>();
    mod_sub_helper::<u128>();
    mod_sub_helper::<usize>();
}
