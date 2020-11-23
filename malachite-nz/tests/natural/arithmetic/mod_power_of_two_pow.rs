use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoIsReduced, ModPowerOfTwoPow, ModPowerOfTwoPowAssign,
};

use malachite_nz::natural::Natural;

#[test]
fn test_mod_power_of_two_pow() {
    let test = |u, exp, pow, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_is_reduced(pow));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_pow_assign(Natural::from_str(exp).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_two_is_reduced(pow));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_pow_assign(&Natural::from_str(exp).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_pow(Natural::from_str(exp).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_power_of_two_pow(Natural::from_str(exp).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_pow(&Natural::from_str(exp).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_power_of_two_pow(&Natural::from_str(exp).unwrap(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 10, "1");
    test("0", "1", 10, "0");
    test("2", "10", 8, "0");
    test("3", "10", 8, "169");
    test("10", "1000", 30, "0");
    test("11", "1000", 30, "289109473");
    test("3", "1000000", 100, "1176684907284103408190379631873");
    test(
        "123456789",
        "1000000000",
        100,
        "1180978940853570377595087681537",
    );
}
