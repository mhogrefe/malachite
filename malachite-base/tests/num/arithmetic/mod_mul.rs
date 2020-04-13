use malachite_base::num::arithmetic::mod_mul::{test_invert_u32_table, test_invert_u64_table};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_test_invert_u32_table() {
    test_invert_u32_table();
}

#[test]
fn test_test_invert_u64_table() {
    test_invert_u64_table();
}

fn mod_mul_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, m, out| {
        assert_eq!(x.mod_mul(y, m), out);

        let mut mut_x = x;
        mut_x.mod_mul_assign(y, m);
        assert_eq!(mut_x, out);

        let data = T::precompute_mod_mul_data(m);
        assert_eq!(x.mod_mul_precomputed(y, m, &data), out);

        let mut mut_x = x;
        mut_x.mod_mul_precomputed_assign(y, m, &data);
        assert_eq!(mut_x, out);
    };
    test(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    test(T::TWO, T::exact_from(3), T::exact_from(7), T::exact_from(6));
    test(
        T::exact_from(7),
        T::exact_from(3),
        T::exact_from(10),
        T::ONE,
    );
    test(
        T::exact_from(100),
        T::exact_from(100),
        T::exact_from(123),
        T::exact_from(37),
    );
    test(T::MAX - T::ONE, T::MAX - T::ONE, T::MAX, T::ONE);
}

#[test]
fn test_mod_mul() {
    mod_mul_helper::<u8>();
    mod_mul_helper::<u16>();
    mod_mul_helper::<u32>();
    mod_mul_helper::<u64>();
    mod_mul_helper::<u128>();
    mod_mul_helper::<usize>();
}
