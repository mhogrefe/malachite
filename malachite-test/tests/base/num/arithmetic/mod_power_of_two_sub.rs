use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_2, triples_of_unsigned_unsigned_and_small_u64_var_1,
};

fn mod_power_of_two_sub_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties_no_special(
        triples_of_unsigned_unsigned_and_small_u64_var_1::<T>,
        |&(x, y, pow)| {
            assert!(x.mod_power_of_two_is_reduced(pow));
            assert!(y.mod_power_of_two_is_reduced(pow));
            let difference = x.mod_power_of_two_sub(y, pow);
            assert!(difference.mod_power_of_two_is_reduced(pow));

            let mut x_alt = x;
            x_alt.mod_power_of_two_sub_assign(y, pow);
            assert_eq!(x_alt, difference);

            assert_eq!(difference.mod_power_of_two_add(y, pow), x);
            assert_eq!(
                difference.mod_power_of_two_sub(x, pow),
                y.mod_power_of_two_neg(pow)
            );
            assert_eq!(
                y.mod_power_of_two_sub(x, pow),
                difference.mod_power_of_two_neg(pow)
            );
            assert_eq!(
                x.mod_power_of_two_add(y.mod_power_of_two_neg(pow), pow),
                difference
            );
        },
    );

    test_properties_no_special(pairs_of_unsigned_and_small_u64_var_2::<T>, |&(x, pow)| {
        assert_eq!(x.mod_power_of_two_sub(T::ZERO, pow), x);
        assert_eq!(
            T::ZERO.mod_power_of_two_sub(x, pow),
            x.mod_power_of_two_neg(pow)
        );
        assert_eq!(x.mod_power_of_two_sub(x, pow), T::ZERO);
    });
}

#[test]
fn mod_power_of_two_sub_properties() {
    mod_power_of_two_sub_properties_helper::<u8>();
    mod_power_of_two_sub_properties_helper::<u16>();
    mod_power_of_two_sub_properties_helper::<u32>();
    mod_power_of_two_sub_properties_helper::<u64>();
    mod_power_of_two_sub_properties_helper::<usize>();
}
