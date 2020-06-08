use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoIsReduced, ModPowerOfTwoNeg, ModPowerOfTwoNegAssign,
};

use malachite_nz::natural::Natural;

#[test]
fn test_mod_power_of_two_neg() {
    let test = |u, pow, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_is_reduced(pow));
        let n = Natural::from_str(u).unwrap().mod_power_of_two_neg(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert!(n.mod_power_of_two_is_reduced(pow));

        let n = (&Natural::from_str(u).unwrap()).mod_power_of_two_neg(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_neg_assign(pow);
        assert_eq!(n.to_string(), out);
    };
    test("0", 5, "0");
    test("10", 4, "6");
    test("100", 8, "156");
    test("1", 32, "4294967295");
    test("100", 100, "1267650600228229401496703205276");
    test("1267650600228229401496703205276", 100, "100");
}
