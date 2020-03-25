use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModPowerOfTwo, ModPowerOfTwoIsReduced, PowerOfTwo,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigned_and_small_unsigned;
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn test_mod_power_of_two_is_reduced() {
    let test = |u, pow, out| {
        assert_eq!(
            Natural::from_str(u)
                .unwrap()
                .mod_power_of_two_is_reduced(pow),
            out
        );
    };

    test("0", 5, true);
    test("100", 5, false);
    test("100", 8, true);
    test("1000000000000", 39, false);
    test("1000000000000", 40, true);
}

#[test]
fn mod_power_of_two_is_reduced_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, pow)| {
        let is_reduced = n.mod_power_of_two_is_reduced(pow);
        assert_eq!(is_reduced, n.mod_power_of_two(pow) == *n);
        assert_eq!(is_reduced, n.mod_is_reduced(&Natural::power_of_two(pow)));
    });

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(n, pow)| {
            assert_eq!(
                n.mod_power_of_two_is_reduced(pow),
                Natural::from(n).mod_power_of_two_is_reduced(pow)
            );
        },
    );
}
