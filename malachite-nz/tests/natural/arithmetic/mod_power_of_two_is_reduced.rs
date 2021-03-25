use malachite_base::num::arithmetic::traits::ModPowerOfTwoIsReduced;
use malachite_nz::natural::Natural;
use std::str::FromStr;

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
