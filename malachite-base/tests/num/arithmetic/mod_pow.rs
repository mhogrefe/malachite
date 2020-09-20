use malachite_base_test_util::num::arithmetic::mod_pow::_naive_mod_pow;

use malachite_base::num::arithmetic::mod_pow::_simple_binary_mod_pow;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

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
    }
    if T::WIDTH > u16::WIDTH {
        test(
            T::TWO,
            1_000_000,
            T::exact_from(1_000_000_000),
            T::exact_from(747_109_376),
        );
    }
}

#[test]
fn test_mod_pow() {
    apply_fn_to_unsigneds!(mod_pow_helper);
}

fn mod_square_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, m, out| {
        assert_eq!(x.mod_mul(x, m), out);
        assert_eq!(x.mod_square(m), out);

        let mut mut_x = x;
        mut_x.mod_square_assign(m);
        assert_eq!(mut_x, out);

        let data = T::precompute_mod_pow_data(&m);
        assert_eq!(x.mod_square_precomputed(m, &data), out);

        let mut mut_x = x;
        mut_x.mod_square_precomputed_assign(m, &data);
        assert_eq!(mut_x, out);
    };
    test(T::ZERO, T::ONE, T::ZERO);
    test(T::ONE, T::exact_from(10), T::ONE);
    test(T::TWO, T::exact_from(10), T::exact_from(4));
    if T::WIDTH > u8::WIDTH {
        test(T::exact_from(100), T::exact_from(497), T::exact_from(60));
        test(T::exact_from(200), T::exact_from(497), T::exact_from(240));
        test(T::exact_from(300), T::exact_from(497), T::exact_from(43));
    }
}

#[test]
fn test_mod_square() {
    apply_fn_to_unsigneds!(mod_square_helper);
}
