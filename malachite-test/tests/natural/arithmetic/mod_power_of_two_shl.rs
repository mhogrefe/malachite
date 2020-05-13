use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    IsPowerOfTwo, ModPowerOfTwo, ModPowerOfTwoIsReduced, ModPowerOfTwoNeg, ModPowerOfTwoShl,
    ModPowerOfTwoShlAssign, ModPowerOfTwoShr,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_small_signed_and_small_unsigned, pairs_of_small_unsigneds,
    triples_of_unsigned_small_signed_and_small_unsigned_var_1,
    triples_of_unsigned_small_unsigned_and_small_unsigned_var_3,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_u64_var_1, triples_of_natural_small_signed_and_u64_var_1,
    triples_of_natural_small_unsigned_and_u64_var_1,
};

macro_rules! tests_and_properties_unsigned {
    ($t:ident, $test_mod_power_of_two_shl_u:ident, $mod_power_of_two_shl_u_properties:ident) => {
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

        #[test]
        fn $mod_power_of_two_shl_u_properties() {
            test_properties(
                triples_of_natural_small_unsigned_and_u64_var_1::<$t>,
                |&(ref n, u, pow)| {
                    assert!(n.mod_power_of_two_is_reduced(pow));
                    let mut mut_n = n.clone();
                    mut_n.mod_power_of_two_shl_assign(u, pow);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;
                    assert!(shifted.mod_power_of_two_is_reduced(pow));

                    let shifted_alt = n.mod_power_of_two_shl(u, pow);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().mod_power_of_two_shl(u, pow);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    assert_eq!((n << u).mod_power_of_two(pow), shifted);
                    assert_eq!(
                        n.mod_power_of_two_neg(pow).mod_power_of_two_shl(u, pow),
                        n.mod_power_of_two_shl(u, pow).mod_power_of_two_neg(pow)
                    );
                },
            );

            test_properties(pairs_of_natural_and_u64_var_1, |&(ref n, pow)| {
                assert_eq!(n.mod_power_of_two_shl($t::ZERO, pow), *n);
            });

            test_properties_no_special(pairs_of_small_unsigneds::<$t, u64>, |&(u, pow)| {
                assert_eq!(Natural::ZERO.mod_power_of_two_shl(u, pow), 0);
                if pow != 0 {
                    let shifted = Natural::ONE.mod_power_of_two_shl(u, pow);
                    assert!(shifted == 0 || shifted.is_power_of_two());
                }
            });

            test_properties_no_special(
                triples_of_unsigned_small_unsigned_and_small_unsigned_var_3::<Limb, $t>,
                |&(n, u, pow)| {
                    assert_eq!(
                        Natural::from(n).mod_power_of_two_shl(u, pow),
                        n.mod_power_of_two_shl(u, pow)
                    );
                },
            );
        }
    };
}
tests_and_properties_unsigned!(
    u8,
    test_mod_power_of_two_shl_u8,
    mod_power_of_two_shl_u8_properties
);
tests_and_properties_unsigned!(
    u16,
    test_mod_power_of_two_shl_u16,
    mod_power_of_two_shl_u16_properties
);
tests_and_properties_unsigned!(
    u32,
    test_mod_power_of_two_shl_u32,
    mod_power_of_two_shl_u32_properties
);
tests_and_properties_unsigned!(
    u64,
    test_mod_power_of_two_shl_u64,
    mod_power_of_two_shl_u64_properties
);
tests_and_properties_unsigned!(
    usize,
    test_mod_power_of_two_shl_usize,
    mod_power_of_two_shl_usize_properties
);

macro_rules! tests_and_properties_signed {
    ($t:ident, $test_mod_power_of_two_shl_i:ident, $mod_power_of_two_shl_i_properties:ident) => {
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

        #[test]
        fn $mod_power_of_two_shl_i_properties() {
            test_properties(
                triples_of_natural_small_signed_and_u64_var_1::<$t>,
                |&(ref n, i, pow)| {
                    assert!(n.mod_power_of_two_is_reduced(pow));
                    let mut mut_n = n.clone();
                    mut_n.mod_power_of_two_shl_assign(i, pow);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;
                    assert!(shifted.mod_power_of_two_is_reduced(pow));

                    let shifted_alt = n.mod_power_of_two_shl(i, pow);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);
                    let shifted_alt = n.clone().mod_power_of_two_shl(i, pow);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    assert_eq!((n << i).mod_power_of_two(pow), shifted);

                    if i != $t::MIN {
                        assert_eq!(n.mod_power_of_two_shr(-i, pow), shifted);
                    }
                },
            );

            test_properties(pairs_of_natural_and_u64_var_1, |&(ref n, pow)| {
                assert_eq!(n.mod_power_of_two_shl($t::ZERO, pow), *n);
            });

            test_properties_no_special(
                pairs_of_small_signed_and_small_unsigned::<$t, u64>,
                |&(i, pow)| {
                    assert_eq!(Natural::ZERO.mod_power_of_two_shl(i, pow), 0);
                    if pow != 0 {
                        let shifted = Natural::ONE.mod_power_of_two_shl(i, pow);
                        assert!(shifted == 0 || shifted.is_power_of_two());
                    }
                },
            );

            test_properties_no_special(
                triples_of_unsigned_small_signed_and_small_unsigned_var_1::<Limb, $t>,
                |&(n, i, pow)| {
                    assert_eq!(
                        Natural::from(n).mod_power_of_two_shl(i, pow),
                        n.mod_power_of_two_shl(i, pow)
                    );
                },
            );
        }
    };
}
tests_and_properties_signed!(
    i8,
    test_mod_power_of_two_shl_i8,
    mod_power_of_two_shl_i8_properties
);
tests_and_properties_signed!(
    i16,
    test_mod_power_of_two_shl_i16,
    mod_power_of_two_shl_i16_properties
);
tests_and_properties_signed!(
    i32,
    test_mod_power_of_two_shl_i32,
    mod_power_of_two_shl_i32_properties
);
tests_and_properties_signed!(
    i64,
    test_mod_power_of_two_shl_i64,
    mod_power_of_two_shl_i64_properties
);
tests_and_properties_signed!(
    isize,
    test_mod_power_of_two_shl_isize,
    mod_power_of_two_shl_isize_properties
);
