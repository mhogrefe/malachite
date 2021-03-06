use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModPowerOf2, ModPowerOf2IsReduced, PowerOf2,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigned_and_small_unsigned;
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn mod_power_of_2_is_reduced_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, pow)| {
        let is_reduced = n.mod_power_of_2_is_reduced(pow);
        assert_eq!(is_reduced, n.mod_power_of_2(pow) == *n);
        assert_eq!(is_reduced, n.mod_is_reduced(&Natural::power_of_2(pow)));
    });

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(n, pow)| {
            assert_eq!(
                n.mod_power_of_2_is_reduced(pow),
                Natural::from(n).mod_power_of_2_is_reduced(pow)
            );
        },
    );
}
