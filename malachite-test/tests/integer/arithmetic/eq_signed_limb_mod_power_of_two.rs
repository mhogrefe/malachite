use std::str::FromStr;

use malachite_base::comparison::{Max, Min};
use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOfTwo, EqModPowerOfTwo, ModPowerOfTwo,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use rug;

use common::test_properties;
use malachite_test::common::integer_to_rug_integer;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, triples_of_signed_signed_and_small_unsigned,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_signed, pairs_of_integer_and_small_unsigned,
    triples_of_integer_signed_and_small_unsigned,
    triples_of_integer_signed_and_small_unsigned_var_1,
    triples_of_integer_signed_limb_and_small_unsigned_var_2,
};
use malachite_test::integer::arithmetic::eq_signed_limb_mod_power_of_two::*;

#[test]
fn test_eq_signed_limb_mod_power_of_two() {
    let test = |n, i: SignedLimb, pow, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().eq_mod_power_of_two(i, pow),
            out
        );
        assert_eq!(
            i.eq_mod_power_of_two(&Integer::from_str(n).unwrap(), pow),
            out
        );
        assert_eq!(
            rug_eq_signed_limb_mod_power_of_two(&rug::Integer::from_str(n).unwrap(), i, pow),
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
    test("-1", SignedLimb::MAX, 0, true);
    test("-1", SignedLimb::MAX, 1, true);
    test("-1", SignedLimb::MAX, u64::from(Limb::WIDTH) - 1, true);
    test("-1", SignedLimb::MAX, u64::from(Limb::WIDTH), false);
    test("-13", 11, 0, true);
    test("-13", 11, 1, true);
    test("-13", 11, 2, true);
    test("-13", 11, 3, true);
    test("-13", 11, 4, false);
    test("-999999999999", 1, 12, true);
    test("-999999999999", 1, 13, false);
    test("0", -256, 8, true);
    test("0", -256, 9, false);
    test("-13", -21, 0, true);
    test("-13", -21, 1, true);
    test("-13", -21, 2, true);
    test("-13", -21, 3, true);
    test("-13", -21, 4, false);
    test("-13", -21, 100, false);
    test("-1000000000001", -1, 12, true);
    test("-1000000000001", -1, 13, false);
    test("3", -5, 0, true);
    test("3", -5, 1, true);
    test("3", -5, 2, true);
    test("3", -5, 3, true);
    test("3", -5, 4, false);
    test("0", SignedLimb::MIN, 0, true);
    test("0", SignedLimb::MIN, 1, true);
    test("0", SignedLimb::MIN, u64::from(Limb::WIDTH) - 1, true);
    test("0", SignedLimb::MIN, u64::from(Limb::WIDTH), false);
    test("13", -11, 0, true);
    test("13", -11, 1, true);
    test("13", -11, 2, true);
    test("13", -11, 3, true);
    test("13", -11, 4, false);
    test("999999999999", -1, 12, true);
    test("999999999999", -1, 13, false);
}

#[test]
fn eq_signed_limb_mod_power_of_two_properties() {
    test_properties(
        triples_of_integer_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(ref n, i, pow)| {
            let eq_mod_power_of_two = n.eq_mod_power_of_two(i, pow);
            assert_eq!(i.eq_mod_power_of_two(n, pow), eq_mod_power_of_two);
            assert_eq!(
                rug_eq_signed_limb_mod_power_of_two(&integer_to_rug_integer(n), i, pow),
                eq_mod_power_of_two
            );
            assert_eq!(
                n.mod_power_of_two(pow) == Integer::from(i).mod_power_of_two(pow),
                eq_mod_power_of_two,
            );
        },
    );

    test_properties(
        triples_of_integer_signed_and_small_unsigned_var_1::<SignedLimb, u64>,
        |&(ref n, i, pow)| {
            assert!(n.eq_mod_power_of_two(i, pow));
            assert!(i.eq_mod_power_of_two(n, pow));
            assert!(rug_eq_signed_limb_mod_power_of_two(
                &integer_to_rug_integer(n),
                i,
                pow,
            ));
            assert_eq!(
                n.mod_power_of_two(pow),
                Integer::from(i).mod_power_of_two(pow),
            );
        },
    );

    test_properties(
        triples_of_integer_signed_limb_and_small_unsigned_var_2::<u64>,
        |&(ref n, i, pow)| {
            assert!(!n.eq_mod_power_of_two(i, pow));
            assert!(!i.eq_mod_power_of_two(n, pow));
            assert!(!rug_eq_signed_limb_mod_power_of_two(
                &integer_to_rug_integer(n),
                i,
                pow,
            ));
            assert_ne!(
                n.mod_power_of_two(pow),
                Integer::from(i).mod_power_of_two(pow),
            );
        },
    );

    test_properties(pairs_of_integer_and_signed::<SignedLimb>, |&(ref n, i)| {
        assert!(n.eq_mod_power_of_two(i, 0));
        assert!(i.eq_mod_power_of_two(n, 0));
    });

    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, pow)| {
        assert_eq!(
            n.eq_mod_power_of_two(0 as SignedLimb, pow),
            n.divisible_by_power_of_two(pow),
        );
        assert_eq!(
            (0 as SignedLimb).eq_mod_power_of_two(n, pow),
            n.divisible_by_power_of_two(pow),
        );
    });

    test_properties(
        pairs_of_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(i, pow)| {
            assert_eq!(
                Integer::ZERO.eq_mod_power_of_two(i, pow),
                i.divisible_by_power_of_two(pow)
            );
            assert_eq!(
                i.eq_mod_power_of_two(&Integer::ZERO, pow),
                i.divisible_by_power_of_two(pow)
            );
        },
    );

    test_properties(
        triples_of_signed_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(x, y, pow)| {
            let equal = x.eq_mod_power_of_two(y, pow);
            assert_eq!(equal, Integer::from(x).eq_mod_power_of_two(y, pow));
            assert_eq!(
                equal,
                EqModPowerOfTwo::eq_mod_power_of_two(x, &Integer::from(y), pow)
            );
        },
    );
}
