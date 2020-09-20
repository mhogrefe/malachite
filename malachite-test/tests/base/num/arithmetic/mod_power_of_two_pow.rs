use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_2, pairs_of_unsigned_and_small_u64_var_3,
    quadruples_of_unsigneds_var_5, quadruples_of_unsigneds_var_6,
    triples_of_unsigned_unsigned_and_small_u64_var_2,
};

fn mod_power_of_two_pow_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties_no_special(
        triples_of_unsigned_unsigned_and_small_u64_var_2::<T>,
        |&(x, exp, pow)| {
            assert!(x.mod_power_of_two_is_reduced(pow));
            let power = x.mod_power_of_two_pow(exp, pow);
            assert!(power.mod_power_of_two_is_reduced(pow));

            let mut x_alt = x;
            x_alt.mod_power_of_two_pow_assign(exp, pow);
            assert_eq!(x_alt, power);
            if exp.even() {
                assert_eq!(
                    x.mod_power_of_two_neg(pow).mod_power_of_two_pow(exp, pow),
                    power
                );
            } else {
                assert_eq!(
                    x.mod_power_of_two_neg(pow).mod_power_of_two_pow(exp, pow),
                    power.mod_power_of_two_neg(pow)
                );
            }
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_u64_var_3::<u64, T>,
        |&(exp, pow)| {
            assert_eq!(
                T::ZERO.mod_power_of_two_pow(exp, pow),
                if exp == 0 && pow != 0 {
                    T::ONE
                } else {
                    T::ZERO
                }
            );
            if pow != 0 {
                assert_eq!(T::ONE.mod_power_of_two_pow(exp, pow), T::ONE);
            }
        },
    );

    test_properties_no_special(pairs_of_unsigned_and_small_u64_var_2::<T>, |&(x, pow)| {
        assert_eq!(x.mod_power_of_two_pow(0, pow), T::iverson(pow != 0));
        assert_eq!(x.mod_power_of_two_pow(1, pow), x);
        assert_eq!(
            x.mod_power_of_two_pow(2, pow),
            x.mod_power_of_two_mul(x, pow)
        );
    });

    test_properties_no_special(quadruples_of_unsigneds_var_5::<T>, |&(x, y, exp, pow)| {
        assert_eq!(
            x.mod_power_of_two_mul(y, pow)
                .mod_power_of_two_pow(exp, pow),
            x.mod_power_of_two_pow(exp, pow)
                .mod_power_of_two_mul(y.mod_power_of_two_pow(exp, pow), pow)
        );
    });

    test_properties_no_special(quadruples_of_unsigneds_var_6::<T>, |&(x, e, f, pow)| {
        if let Some(sum) = e.checked_add(f) {
            assert_eq!(
                x.mod_power_of_two_pow(sum, pow),
                x.mod_power_of_two_pow(e, pow)
                    .mod_power_of_two_mul(x.mod_power_of_two_pow(f, pow), pow)
            );
        }
        if let Some(product) = e.checked_mul(f) {
            assert_eq!(
                x.mod_power_of_two_pow(product, pow),
                x.mod_power_of_two_pow(e, pow).mod_power_of_two_pow(f, pow)
            );
        }
    });
}

#[test]
fn mod_power_of_two_pow_properties() {
    mod_power_of_two_pow_properties_helper::<u8>();
    mod_power_of_two_pow_properties_helper::<u16>();
    mod_power_of_two_pow_properties_helper::<u32>();
    mod_power_of_two_pow_properties_helper::<u64>();
    mod_power_of_two_pow_properties_helper::<usize>();
}
