use malachite_base::num::arithmetic::mod_pow::_simple_binary_mod_pow;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::num::arithmetic::mod_pow::_naive_mod_pow;

fn mod_pow_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, exp: u64, m, out| {
        assert_eq!(_naive_mod_pow(x, exp, m), out);
        assert_eq!(_simple_binary_mod_pow(x, exp, m), out);

        assert_eq!(x.mod_pow(exp, m), out);

        let mut mut_x = x;
        mut_x.mod_pow_assign(exp, m);
        assert_eq!(mut_x, out);

        let data = T::precompute_mod_pow_data(&m);
        assert_eq!(x.mod_pow_precomputed(exp, m, &data), out);

        let mut mut_x = x;
        mut_x.mod_pow_precomputed_assign(exp, m, &data);
        assert_eq!(mut_x, out);
    };
    test(T::ZERO, 0, T::ONE, T::ZERO);
    test(T::ZERO, 0, T::exact_from(10), T::ONE);
    test(T::ZERO, 1, T::exact_from(10), T::ZERO);

    test(T::TWO, 10, T::exact_from(10), T::exact_from(4));
    if T::WIDTH > u8::WIDTH {
        test(T::exact_from(4), 13, T::exact_from(497), T::exact_from(445));
        test(
            T::exact_from(10),
            1000,
            T::exact_from(30),
            T::exact_from(10),
        );
        test(T::TWO, 340, T::exact_from(341), T::ONE);
        test(T::exact_from(5), 216, T::exact_from(217), T::ONE);
    }
    if T::WIDTH > u16::WIDTH {
        test(
            T::TWO,
            1000000,
            T::exact_from(1000000000),
            T::exact_from(747109376),
        );
    }
}

#[test]
fn test_mod_pow() {
    apply_fn_to_unsigneds!(mod_pow_helper);
}
