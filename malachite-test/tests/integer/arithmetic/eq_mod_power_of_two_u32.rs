use common::test_properties;
use malachite_base::num::{DivisibleByPowerOfTwo, EqModPowerOfTwo, ModPowerOfTwo, Zero};
use malachite_nz::integer::arithmetic::eq_mod_power_of_two_u32::limbs_eq_mod_power_of_two_neg_limb;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_small_unsigned, triples_of_integer_unsigned_and_small_unsigned,
};
use std::str::FromStr;
use std::u32;

#[test]
fn test_limbs_eq_mod_power_of_two_neg_limb() {
    let test = |limbs, limb, pow, out| {
        assert_eq!(limbs_eq_mod_power_of_two_neg_limb(limbs, limb, pow), out);
    };
    test(&[1, 1], 3, 0, true);
    test(&[1, 1], 3, 1, true);
    test(&[1, 1], 3, 2, true);
    test(&[1, 1], 3, 3, false);
    test(&[1, 1], u32::MAX, 0, true);
    test(&[1, 1], u32::MAX, 1, true);
    test(&[1, 1], u32::MAX, 32, true);
    test(&[1, 1], u32::MAX, 33, true);
    test(&[1, 2], u32::MAX, 33, false);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 33, true);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 64, true);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 95, true);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 96, false);
}

#[test]
fn test_eq_mod_power_of_two_u32() {
    let test = |n, u: u32, pow, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().eq_mod_power_of_two(&u, pow),
            out
        );
    };
    test("0", 256, 8, true);
    test("0", 256, 9, false);
    test("13", 21, 0, true);
    test("13", 21, 1, true);
    test("13", 21, 2, true);
    test("13", 21, 3, true);
    test("13", 21, 4, false);
    test("13", 21, 100, false);
    test("1000000000001", 1, 12, true);
    test("1000000000001", 1, 13, false);
    test("-3", 5, 0, true);
    test("-3", 5, 1, true);
    test("-3", 5, 2, true);
    test("-3", 5, 3, true);
    test("-3", 5, 4, false);
    test("-1", u32::MAX, 0, true);
    test("-1", u32::MAX, 1, true);
    test("-1", u32::MAX, 32, true);
    test("-1", u32::MAX, 33, false);
    test("-13", 11, 0, true);
    test("-13", 11, 1, true);
    test("-13", 11, 2, true);
    test("-13", 11, 3, true);
    test("-13", 11, 4, false);
    test("-999999999999", 1, 12, true);
    test("-999999999999", 1, 13, false);
    test("-18446744073709551616", 0, 33, true);
    test("-18446744073709551616", 0, 64, true);
    test("-18446744073709551616", 0, 65, false);
}

#[test]
fn limbs_eq_mod_power_of_two_neg_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
        |&(ref limbs, limb, pow)| {
            assert_eq!(
                limbs_eq_mod_power_of_two_neg_limb(limbs, limb, pow),
                (-Natural::from_limbs_asc(limbs)).eq_mod_power_of_two(&limb, pow)
            );
        },
    );
}

#[test]
fn eq_mod_power_of_two_u32_properties() {
    test_properties(
        triples_of_integer_unsigned_and_small_unsigned::<u32, u64>,
        |&(ref n, u, pow)| {
            let eq_mod_power_of_two = n.eq_mod_power_of_two(&u, pow);
            assert_eq!(
                n.mod_power_of_two(pow) == u.mod_power_of_two(pow),
                eq_mod_power_of_two,
            );
        },
    );

    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, pow)| {
        assert_eq!(
            n.eq_mod_power_of_two(&0u32, pow),
            n.divisible_by_power_of_two(pow),
        );
    });

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<u32, u64>,
        |&(u, pow)| {
            assert_eq!(
                Integer::ZERO.eq_mod_power_of_two(&u, pow),
                u.divisible_by_power_of_two(pow)
            );
        },
    );
}
