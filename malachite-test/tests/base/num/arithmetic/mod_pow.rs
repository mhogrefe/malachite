use malachite_base::num::arithmetic::mod_pow::_simple_binary_mod_pow;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::num::arithmetic::mod_pow::_naive_mod_pow;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_positive_unsigned, pairs_of_unsigneds_var_5,
    quadruples_of_unsigneds_var_3, quadruples_of_unsigneds_var_4,
    triples_of_unsigned_small_unsigned_and_unsigned_var_1,
    triples_of_unsigned_unsigned_and_unsigned_var_1,
};

fn mod_pow_properties_helper_helper<T: PrimitiveUnsigned, F: Fn(T, u64, T) -> T>(
    x: T,
    exp: u64,
    m: T,
    f: F,
) {
    assert!(x.mod_is_reduced(&m));
    let power = x.mod_pow(exp, m);
    assert!(power.mod_is_reduced(&m));

    let mut x_alt = x;
    x_alt.mod_pow_assign(exp, m);
    assert_eq!(x_alt, power);

    let data = T::precompute_mod_pow_data(&m);

    assert_eq!(x.mod_pow_precomputed(exp, m, &data), power);

    let mut x_alt = x;
    x_alt.mod_pow_precomputed_assign(exp, m, &data);
    assert_eq!(x_alt, power);

    assert_eq!(f(x, exp, m), power);
    if exp.even() {
        assert_eq!(x.mod_neg(m).mod_pow(exp, m), power);
    } else {
        assert_eq!(x.mod_neg(m).mod_pow(exp, m), power.mod_neg(m));
    }
}

fn mod_pow_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(
        triples_of_unsigned_unsigned_and_unsigned_var_1::<T, u64>,
        |&(x, exp, m)| {
            mod_pow_properties_helper_helper(x, exp, m, _simple_binary_mod_pow);
        },
    );

    test_properties(
        triples_of_unsigned_small_unsigned_and_unsigned_var_1::<T, u64>,
        |&(x, exp, m)| {
            mod_pow_properties_helper_helper(x, exp, m, _naive_mod_pow);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<u64, T>,
        |&(exp, m)| {
            assert_eq!(
                T::ZERO.mod_pow(exp, m),
                if exp == 0 && m != T::ONE {
                    T::ONE
                } else {
                    T::ZERO
                }
            );
            if m != T::ONE {
                assert_eq!(T::ONE.mod_pow(exp, m), T::ONE);
            }
        },
    );

    test_properties(pairs_of_unsigneds_var_5::<T>, |&(x, m)| {
        assert_eq!(x.mod_pow(0, m), T::iverson(m != T::ONE));
        assert_eq!(x.mod_pow(1, m), x);
        assert_eq!(x.mod_pow(2, m), x.mod_mul(x, m));
    });

    test_properties(
        quadruples_of_unsigneds_var_3::<T, u64>,
        |&(x, y, exp, m)| {
            assert_eq!(
                x.mod_mul(y, m).mod_pow(exp, m),
                x.mod_pow(exp, m).mod_mul(y.mod_pow(exp, m), m)
            );
        },
    );

    test_properties(quadruples_of_unsigneds_var_4::<T, u64>, |&(x, e, f, m)| {
        if let Some(sum) = e.checked_add(f) {
            assert_eq!(
                x.mod_pow(sum, m),
                x.mod_pow(e, m).mod_mul(x.mod_pow(f, m), m)
            );
        }
        if let Some(product) = e.checked_mul(f) {
            assert_eq!(x.mod_pow(product, m), x.mod_pow(e, m).mod_pow(f, m));
        }
    });
}

#[test]
fn mod_pow_properties() {
    mod_pow_properties_helper::<u8>();
    mod_pow_properties_helper::<u16>();
    mod_pow_properties_helper::<u32>();
    mod_pow_properties_helper::<u64>();
    mod_pow_properties_helper::<usize>();
}
