use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2ShlAssign,
};
use malachite_nz::natural::Natural;
use std::str::FromStr;

macro_rules! test_mod_power_of_2_shl_unsigned {
    ($t:ident) => {
        let test = |s, v: $t, pow, out| {
            let u = Natural::from_str(s).unwrap();

            let mut n = u.clone();
            assert!(n.mod_power_of_2_is_reduced(pow));
            n.mod_power_of_2_shl_assign(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);
            assert!(n.mod_power_of_2_is_reduced(pow));

            let n = u.clone().mod_power_of_2_shl(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&u).mod_power_of_2_shl(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            assert_eq!((u << v).mod_power_of_2(pow).to_string(), out);
        };
        test("0", 10, 0, "0");
        test("0", 10, 8, "0");
        test("123", 5, 8, "96");
        test("123", 100, 80, "0");
    };
}

macro_rules! test_mod_power_of_2_shl_signed {
    ($t:ident) => {
        let test = |s, v: $t, pow, out| {
            let u = Natural::from_str(s).unwrap();

            let mut n = u.clone();
            assert!(n.mod_power_of_2_is_reduced(pow));
            n.mod_power_of_2_shl_assign(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);
            assert!(n.mod_power_of_2_is_reduced(pow));

            let n = u.clone().mod_power_of_2_shl(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&u).mod_power_of_2_shl(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            assert_eq!((u << v).mod_power_of_2(pow).to_string(), out);
        };
        test("0", 10, 0, "0");
        test("0", 10, 8, "0");
        test("123", 5, 8, "96");
        test("123", 100, 80, "0");
        test("123", -2, 8, "30");
        test("123", -10, 8, "0");
    };
}

#[test]
fn test_mod_power_of_2_shl() {
    apply_to_unsigneds!(test_mod_power_of_2_shl_unsigned);
    apply_to_signeds!(test_mod_power_of_2_shl_signed);
}
