use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoIsReduced, ModPowerOfTwoShl, ModPowerOfTwoShlAssign,
};

use malachite_nz::natural::Natural;

macro_rules! tests_unsigned {
    ($t:ident, $test_mod_power_of_two_shl_u:ident) => {
        #[test]
        fn $test_mod_power_of_two_shl_u() {
            let test = |u, v: $t, pow, out| {
                let mut n = Natural::from_str(u).unwrap();
                assert!(n.mod_power_of_two_is_reduced(pow));
                n.mod_power_of_two_shl_assign(v, pow);
                assert!(n.is_valid());
                assert_eq!(n.to_string(), out);
                assert!(n.mod_power_of_two_is_reduced(pow));

                let n = Natural::from_str(u).unwrap().mod_power_of_two_shl(v, pow);
                assert!(n.is_valid());

                let n = (&Natural::from_str(u).unwrap()).mod_power_of_two_shl(v, pow);
                assert!(n.is_valid());
                assert_eq!(n.to_string(), out);

                assert_eq!(
                    (Natural::from_str(u).unwrap() << v)
                        .mod_power_of_two(pow)
                        .to_string(),
                    out
                );
            };
            test("0", 10, 0, "0");
            test("0", 10, 8, "0");
            test("123", 5, 8, "96");
            test("123", 100, 80, "0");
        }
    };
}
tests_unsigned!(u8, test_mod_power_of_two_shl_u8);
tests_unsigned!(u16, test_mod_power_of_two_shl_u16);
tests_unsigned!(u32, test_mod_power_of_two_shl_u32);
tests_unsigned!(u64, test_mod_power_of_two_shl_u64);
tests_unsigned!(usize, test_mod_power_of_two_shl_usize);

macro_rules! tests_signed {
    ($t:ident, $test_mod_power_of_two_shl_i:ident) => {
        #[test]
        fn $test_mod_power_of_two_shl_i() {
            let test = |u, v: $t, pow, out| {
                let mut n = Natural::from_str(u).unwrap();
                assert!(n.mod_power_of_two_is_reduced(pow));
                n.mod_power_of_two_shl_assign(v, pow);
                assert!(n.is_valid());
                assert_eq!(n.to_string(), out);
                assert!(n.mod_power_of_two_is_reduced(pow));

                let n = Natural::from_str(u).unwrap().mod_power_of_two_shl(v, pow);
                assert!(n.is_valid());

                let n = (&Natural::from_str(u).unwrap()).mod_power_of_two_shl(v, pow);
                assert!(n.is_valid());
                assert_eq!(n.to_string(), out);

                assert_eq!(
                    (Natural::from_str(u).unwrap() << v)
                        .mod_power_of_two(pow)
                        .to_string(),
                    out
                );
            };
            test("0", 10, 0, "0");
            test("0", 10, 8, "0");
            test("123", 5, 8, "96");
            test("123", 100, 80, "0");
            test("123", -2, 8, "30");
            test("123", -10, 8, "0");
        }
    };
}
tests_signed!(i8, test_mod_power_of_two_shl_i8);
tests_signed!(i16, test_mod_power_of_two_shl_i16);
tests_signed!(i32, test_mod_power_of_two_shl_i32);
tests_signed!(i64, test_mod_power_of_two_shl_i64);
tests_signed!(isize, test_mod_power_of_two_shl_isize);
