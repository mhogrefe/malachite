use std::str::FromStr;

use malachite_base::num::traits::{DivisibleByPowerOfTwo, EqModPowerOfTwo, ModPowerOfTwo, Zero};
use malachite_nz::natural::arithmetic::eq_limb_mod_power_of_two::limbs_eq_limb_mod_power_of_two;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use common::test_properties;
use malachite_test::common::natural_to_rug_integer;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, triples_of_unsigned_unsigned_and_small_unsigned,
    triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_unsigned, pairs_of_natural_and_unsigned,
    triples_of_natural_limb_and_small_unsigned_var_2,
    triples_of_natural_unsigned_and_small_unsigned,
    triples_of_natural_unsigned_and_small_unsigned_var_1,
};
use malachite_test::natural::arithmetic::eq_limb_mod_power_of_two::rug_eq_limb_mod_power_of_two;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_limb_mod_power_of_two() {
    let test = |limbs, limb, pow, out| {
        assert_eq!(limbs_eq_limb_mod_power_of_two(limbs, limb, pow), out);
    };
    test(&[0b1111011, 0b111001000], 0b101011, 4, true);
    test(&[0b1111011, 0b111001000], 0b101011, 5, false);
    test(&[0b1111011, 0b111001000], 0b1111011, 35, true);
    test(&[0b1111011, 0b111001000], 0b1111011, 36, false);
    test(&[0b1111011, 0b111001000], 0b1111011, 100, false);
}

#[test]
fn test_eq_limb_mod_power_of_two() {
    let test = |n, u: Limb, pow, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().eq_mod_power_of_two(u, pow),
            out
        );
        assert_eq!(
            u.eq_mod_power_of_two(&Natural::from_str(n).unwrap(), pow),
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
    test("4294967295", 4294967295, 32, true);
}

#[test]
fn limbs_eq_limb_mod_power_of_two_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
        |&(ref limbs, limb, pow)| {
            assert_eq!(
                limbs_eq_limb_mod_power_of_two(limbs, limb, pow),
                Natural::from_limbs_asc(limbs).eq_mod_power_of_two(limb, pow)
            );
        },
    );
}

#[test]
fn eq_limb_mod_power_of_two_properties() {
    test_properties(
        triples_of_natural_unsigned_and_small_unsigned::<Limb, u64>,
        |&(ref n, u, pow)| {
            let eq_mod_power_of_two = n.eq_mod_power_of_two(u, pow);
            assert_eq!(u.eq_mod_power_of_two(n, pow), eq_mod_power_of_two);
            assert_eq!(
                rug_eq_limb_mod_power_of_two(&natural_to_rug_integer(n), u, pow),
                eq_mod_power_of_two
            );
            assert_eq!(
                n.mod_power_of_two(pow) == u.mod_power_of_two(pow),
                eq_mod_power_of_two
            );
        },
    );

    test_properties(
        triples_of_natural_unsigned_and_small_unsigned_var_1::<Limb, u64>,
        |&(ref n, u, pow)| {
            assert!(n.eq_mod_power_of_two(u, pow));
            assert!(u.eq_mod_power_of_two(n, pow));
            assert!(rug_eq_limb_mod_power_of_two(
                &natural_to_rug_integer(n),
                u,
                pow,
            ));
            assert_eq!(n.mod_power_of_two(pow), u.mod_power_of_two(pow),);
        },
    );

    test_properties(
        triples_of_natural_limb_and_small_unsigned_var_2::<u64>,
        |&(ref n, u, pow)| {
            assert!(!n.eq_mod_power_of_two(u, pow));
            assert!(!u.eq_mod_power_of_two(n, pow));
            assert!(!rug_eq_limb_mod_power_of_two(
                &natural_to_rug_integer(n),
                u,
                pow,
            ));
            assert_ne!(n.mod_power_of_two(pow), u.mod_power_of_two(pow),);
        },
    );

    test_properties(pairs_of_natural_and_unsigned::<Limb>, |&(ref n, u)| {
        assert!(n.eq_mod_power_of_two(u, 0));
        assert!(u.eq_mod_power_of_two(n, 0));
    });

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, pow)| {
        assert_eq!(
            n.eq_mod_power_of_two(0 as Limb, pow),
            n.divisible_by_power_of_two(pow)
        );
        assert_eq!(
            (0 as Limb).eq_mod_power_of_two(n, pow),
            n.divisible_by_power_of_two(pow)
        );
    });

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(u, pow)| {
            assert_eq!(
                Natural::ZERO.eq_mod_power_of_two(u, pow),
                u.divisible_by_power_of_two(pow)
            );
            assert_eq!(
                u.eq_mod_power_of_two(&Natural::ZERO, pow),
                u.divisible_by_power_of_two(pow)
            );
        },
    );

    test_properties(
        triples_of_unsigned_unsigned_and_small_unsigned::<Limb, u64>,
        |&(x, y, pow)| {
            let eq_mod_power_of_two = x.eq_mod_power_of_two(y, pow);
            assert_eq!(
                Natural::from(x).eq_mod_power_of_two(y, pow),
                eq_mod_power_of_two
            );
            assert_eq!(
                x.eq_mod_power_of_two(&Natural::from(y), pow),
                eq_mod_power_of_two
            );
        },
    );
}
