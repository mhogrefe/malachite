use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_2, quadruples_of_three_unsigneds_and_small_u64_var_1,
    triples_of_unsigned_unsigned_and_small_u64_var_1,
};

fn mod_power_of_two_add_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties_no_special(
        triples_of_unsigned_unsigned_and_small_u64_var_1::<T>,
        |&(x, y, pow)| {
            assert!(x.mod_power_of_two_is_reduced(pow));
            assert!(y.mod_power_of_two_is_reduced(pow));
            let sum = x.mod_power_of_two_add(y, pow);
            assert!(sum.mod_power_of_two_is_reduced(pow));

            let mut x_alt = x;
            x_alt.mod_power_of_two_add_assign(y, pow);
            assert_eq!(x_alt, sum);

            assert_eq!(sum.mod_power_of_two_sub(y, pow), x);
            assert_eq!(sum.mod_power_of_two_sub(x, pow), y);
            assert_eq!(y.mod_power_of_two_add(x, pow), sum);
            assert_eq!(
                x.mod_power_of_two_sub(y.mod_power_of_two_neg(pow), pow),
                sum
            );
        },
    );

    test_properties_no_special(pairs_of_unsigned_and_small_u64_var_2::<T>, |&(x, pow)| {
        assert_eq!(x.mod_power_of_two_add(T::ZERO, pow), x);
        assert_eq!(T::ZERO.mod_power_of_two_add(x, pow), x);
        assert_eq!(
            x.mod_power_of_two_add(x.mod_power_of_two_neg(pow), pow),
            T::ZERO
        );
    });

    test_properties_no_special(
        quadruples_of_three_unsigneds_and_small_u64_var_1::<T>,
        |&(x, y, z, pow)| {
            assert_eq!(
                x.mod_power_of_two_add(y, pow).mod_power_of_two_add(z, pow),
                x.mod_power_of_two_add(y.mod_power_of_two_add(z, pow), pow)
            );
        },
    );
}

#[test]
fn mod_power_of_two_add_properties() {
    mod_power_of_two_add_properties_helper::<u8>();
    mod_power_of_two_add_properties_helper::<u16>();
    mod_power_of_two_add_properties_helper::<u32>();
    mod_power_of_two_add_properties_helper::<u64>();
    mod_power_of_two_add_properties_helper::<usize>();
}
