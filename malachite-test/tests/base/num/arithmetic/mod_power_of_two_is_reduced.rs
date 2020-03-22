use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigned_and_small_unsigned;

fn mod_power_of_two_is_reduced_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        pairs_of_unsigned_and_small_unsigned::<T, u64>,
        |&(n, pow)| {
            assert_eq!(
                n.mod_power_of_two_is_reduced(pow),
                n.mod_power_of_two(pow) == n
            );
        },
    );
}

#[test]
fn mod_power_of_two_is_reduced_properties() {
    mod_power_of_two_is_reduced_properties_helper::<u8>();
    mod_power_of_two_is_reduced_properties_helper::<u16>();
    mod_power_of_two_is_reduced_properties_helper::<u32>();
    mod_power_of_two_is_reduced_properties_helper::<u64>();
    mod_power_of_two_is_reduced_properties_helper::<usize>();
}
