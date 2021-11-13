use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2NegAssign,
};
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_mod_power_of_2_neg() {
    let test = |s, pow, out| {
        let u = Natural::from_str(s).unwrap();

        assert!(u.mod_power_of_2_is_reduced(pow));
        let n = u.clone().mod_power_of_2_neg(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert!(n.mod_power_of_2_is_reduced(pow));

        let n = (&u).mod_power_of_2_neg(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u;
        n.mod_power_of_2_neg_assign(pow);
        assert_eq!(n.to_string(), out);
    };
    test("0", 5, "0");
    test("10", 4, "6");
    test("100", 8, "156");
    test("1", 32, "4294967295");
    test("100", 100, "1267650600228229401496703205276");
    test("1267650600228229401496703205276", 100, "100");
}
