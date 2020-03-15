use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, ModPowerOfTwoIsReduced};
use malachite_nz::natural::Natural;

use malachite_test::common::test_properties;
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn test_mod_power_of_two_is_reduced() {
    let test = |u, log_base, out| {
        assert_eq!(
            Natural::from_str(u)
                .unwrap()
                .mod_power_of_two_is_reduced(log_base),
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
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, log_base)| {
        assert_eq!(
            n.mod_power_of_two_is_reduced(log_base),
            n.mod_power_of_two(log_base) == *n
        );
    });
}
