use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_2, quadruples_of_three_unsigneds_and_small_u64_var_1,
    triples_of_unsigned_unsigned_and_small_u64_var_1,
};

fn mod_power_of_two_mul_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties_no_special(
        triples_of_unsigned_unsigned_and_small_u64_var_1::<T>,
        |&(x, y, pow)| {
            assert!(x.mod_power_of_two_is_reduced(pow));
            assert!(y.mod_power_of_two_is_reduced(pow));
            let product = x.mod_power_of_two_mul(y, pow);
            assert!(product.mod_power_of_two_is_reduced(pow));

            let mut x_alt = x;
            x_alt.mod_power_of_two_mul_assign(y, pow);
            assert_eq!(x_alt, product);

            assert_eq!(y.mod_power_of_two_mul(x, pow), product);
        },
    );

    test_properties_no_special(pairs_of_unsigned_and_small_u64_var_2::<T>, |&(x, pow)| {
        assert_eq!(x.mod_power_of_two_mul(T::ZERO, pow), T::ZERO);
        assert_eq!(T::ZERO.mod_power_of_two_mul(x, pow), T::ZERO);
        assert_eq!(x.mod_power_of_two_mul(T::ONE, pow), x);
        assert_eq!(T::ONE.mod_power_of_two_mul(x, pow), x);
    });

    test_properties_no_special(
        quadruples_of_three_unsigneds_and_small_u64_var_1::<T>,
        |&(x, y, z, pow)| {
            assert_eq!(
                x.mod_power_of_two_mul(y, pow).mod_power_of_two_mul(z, pow),
                x.mod_power_of_two_mul(y.mod_power_of_two_mul(z, pow), pow)
            );
        },
    );
}

#[test]
fn mod_power_of_two_mul_properties() {
    mod_power_of_two_mul_properties_helper::<u8>();
    mod_power_of_two_mul_properties_helper::<u16>();
    mod_power_of_two_mul_properties_helper::<u32>();
    mod_power_of_two_mul_properties_helper::<u64>();
    mod_power_of_two_mul_properties_helper::<usize>();
}
