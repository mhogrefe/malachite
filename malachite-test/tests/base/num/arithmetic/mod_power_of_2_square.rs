use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_2, triples_of_unsigned_unsigned_and_small_u64_var_1,
    unsigneds_var_3,
};

fn mod_power_of_2_square_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties_no_special(pairs_of_unsigned_and_small_u64_var_2::<T>, |&(x, pow)| {
        assert!(x.mod_power_of_2_is_reduced(pow));
        let square = x.mod_power_of_2_square(pow);
        assert!(square.mod_power_of_2_is_reduced(pow));

        let mut x_alt = x;
        x_alt.mod_power_of_2_square_assign(pow);
        assert_eq!(x_alt, square);

        assert_eq!(x.mod_power_of_2_pow(2, pow), x.mod_power_of_2_mul(x, pow));
        assert_eq!(x.mod_power_of_2_neg(pow).mod_power_of_2_square(pow), square);
    });

    test_properties_no_special(unsigneds_var_3::<T>, |&pow| {
        assert_eq!(T::ZERO.mod_power_of_2_square(pow), T::ZERO);
        if pow != 0 {
            assert_eq!(T::ONE.mod_power_of_2_square(pow), T::ONE);
        }
    });

    test_properties_no_special(
        triples_of_unsigned_unsigned_and_small_u64_var_1::<T>,
        |&(x, y, pow)| {
            assert_eq!(
                x.mod_power_of_2_mul(y, pow).mod_power_of_2_square(pow),
                x.mod_power_of_2_square(pow)
                    .mod_power_of_2_mul(y.mod_power_of_2_square(pow), pow)
            );
        },
    );
}

#[test]
fn mod_power_of_2_square_properties() {
    mod_power_of_2_square_properties_helper::<u8>();
    mod_power_of_2_square_properties_helper::<u16>();
    mod_power_of_2_square_properties_helper::<u32>();
    mod_power_of_2_square_properties_helper::<u64>();
    mod_power_of_2_square_properties_helper::<usize>();
}
