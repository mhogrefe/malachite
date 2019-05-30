use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOfTwo, EqModPowerOfTwo, ModPowerOfTwo,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::arithmetic::eq_limb_mod_power_of_two::limbs_eq_mod_power_of_two_neg_limb;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use common::test_properties;
use malachite_test::common::integer_to_rug_integer;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, triples_of_unsigned_unsigned_and_small_unsigned,
    triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_small_unsigned, pairs_of_integer_and_unsigned,
    triples_of_integer_limb_and_small_unsigned_var_2,
    triples_of_integer_unsigned_and_small_unsigned,
    triples_of_integer_unsigned_and_small_unsigned_var_1,
};
use malachite_test::inputs::natural::triples_of_natural_unsigned_and_small_unsigned;
use malachite_test::natural::arithmetic::eq_limb_mod_power_of_two::rug_eq_limb_mod_power_of_two;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_power_of_two_neg_limb() {
    let test = |limbs, limb, pow, out| {
        assert_eq!(limbs_eq_mod_power_of_two_neg_limb(limbs, limb, pow), out);
    };
    let width = u64::from(Limb::WIDTH);
    test(&[1, 1], 3, 0, true);
    test(&[1, 1], 3, 1, true);
    test(&[1, 1], 3, 2, true);
    test(&[1, 1], 3, 3, false);
    test(&[1, 1], Limb::MAX, 0, true);
    test(&[1, 1], Limb::MAX, 1, true);
    test(&[1, 1], Limb::MAX, width, true);
    test(&[1, 1], Limb::MAX, width + 1, true);
    test(&[1, 2], Limb::MAX, width + 1, false);
    test(&[1, Limb::MAX, Limb::MAX], Limb::MAX, width + 1, true);
    test(&[1, Limb::MAX, Limb::MAX], Limb::MAX, 2 * width, true);
    test(&[1, Limb::MAX, Limb::MAX], Limb::MAX, 3 * width - 1, true);
    test(&[1, Limb::MAX, Limb::MAX], Limb::MAX, 3 * width, false);
}

#[test]
fn test_eq_limb_mod_power_of_two() {
    let test = |n, u: Limb, pow, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().eq_mod_power_of_two(u, pow),
            out
        );
        assert_eq!(
            u.eq_mod_power_of_two(&Integer::from_str(n).unwrap(), pow),
            out
        );
        assert_eq!(
            rug_eq_limb_mod_power_of_two(&rug::Integer::from_str(n).unwrap(), u, pow),
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
    test("-1", Limb::MAX, 0, true);
    test("-1", Limb::MAX, 1, true);
    test("-1", Limb::MAX, u64::from(Limb::WIDTH), true);
    test("-1", Limb::MAX, u64::from(Limb::WIDTH) + 1, false);
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
                (-Natural::from_limbs_asc(limbs)).eq_mod_power_of_two(limb, pow)
            );
        },
    );
}

#[test]
fn eq_limb_mod_power_of_two_properties() {
    test_properties(
        triples_of_integer_unsigned_and_small_unsigned::<Limb, u64>,
        |&(ref n, u, pow)| {
            let eq_mod_power_of_two = n.eq_mod_power_of_two(u, pow);
            assert_eq!(u.eq_mod_power_of_two(n, pow), eq_mod_power_of_two);
            assert_eq!(
                rug_eq_limb_mod_power_of_two(&integer_to_rug_integer(n), u, pow),
                eq_mod_power_of_two
            );
            assert_eq!(
                n.mod_power_of_two(pow) == u.mod_power_of_two(pow),
                eq_mod_power_of_two,
            );
        },
    );

    test_properties(
        triples_of_integer_unsigned_and_small_unsigned_var_1::<Limb, u64>,
        |&(ref n, u, pow)| {
            assert!(n.eq_mod_power_of_two(u, pow));
            assert!(u.eq_mod_power_of_two(n, pow));
            assert!(rug_eq_limb_mod_power_of_two(
                &integer_to_rug_integer(n),
                u,
                pow,
            ));
            assert_eq!(n.mod_power_of_two(pow), u.mod_power_of_two(pow),);
        },
    );

    test_properties(
        triples_of_integer_limb_and_small_unsigned_var_2::<u64>,
        |&(ref n, u, pow)| {
            assert!(!n.eq_mod_power_of_two(u, pow));
            assert!(!u.eq_mod_power_of_two(n, pow));
            assert!(!rug_eq_limb_mod_power_of_two(
                &integer_to_rug_integer(n),
                u,
                pow,
            ));
            assert_ne!(n.mod_power_of_two(pow), u.mod_power_of_two(pow),);
        },
    );

    test_properties(pairs_of_integer_and_unsigned::<Limb>, |&(ref n, u)| {
        assert!(n.eq_mod_power_of_two(u, 0));
        assert!(u.eq_mod_power_of_two(n, 0));
    });

    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, pow)| {
        assert_eq!(
            n.eq_mod_power_of_two(0 as Limb, pow),
            n.divisible_by_power_of_two(pow),
        );
        assert_eq!(
            (0 as Limb).eq_mod_power_of_two(n, pow),
            n.divisible_by_power_of_two(pow),
        );
    });

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(u, pow)| {
            assert_eq!(
                Integer::ZERO.eq_mod_power_of_two(u, pow),
                u.divisible_by_power_of_two(pow)
            );
            assert_eq!(
                u.eq_mod_power_of_two(&Integer::ZERO, pow),
                u.divisible_by_power_of_two(pow)
            );
        },
    );

    test_properties(
        triples_of_unsigned_unsigned_and_small_unsigned::<Limb, u64>,
        |&(x, y, pow)| {
            let equal = x.eq_mod_power_of_two(y, pow);
            assert_eq!(equal, Integer::from(x).eq_mod_power_of_two(y, pow));
            assert_eq!(equal, x.eq_mod_power_of_two(&Integer::from(y), pow));
        },
    );

    test_properties(
        triples_of_natural_unsigned_and_small_unsigned::<Limb, u64>,
        |&(ref n, u, pow)| {
            assert_eq!(
                n.eq_mod_power_of_two(u, pow),
                Integer::from(n).eq_mod_power_of_two(u, pow)
            );
        },
    );
}
