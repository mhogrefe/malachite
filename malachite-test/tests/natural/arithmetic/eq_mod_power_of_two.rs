use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOfTwo, EqModPowerOfTwo, ModPowerOfTwo,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::natural::arithmetic::eq_mod_power_of_two::{
    limbs_eq_limb_mod_power_of_two, limbs_eq_mod_power_of_two,
};
use malachite_nz::natural::Natural;
use rug;

use common::test_properties;
use malachite_test::common::natural_to_rug_integer;
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
    triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_unsigned, pairs_of_naturals,
    quadruples_of_natural_natural_natural_and_small_unsigned,
    triples_of_natural_natural_and_small_unsigned,
    triples_of_natural_natural_and_small_unsigned_var_1,
    triples_of_natural_natural_and_small_unsigned_var_2,
};

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

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_power_of_two() {
    let test = |xs, ys, pow, out| {
        assert_eq!(limbs_eq_mod_power_of_two(xs, ys, pow), out);
    };
    test(&[0b111_1011, 0b1_1100_1000], &[0b10_1011], 4, true);
    test(&[0b111_1011, 0b1_1100_1000], &[0b10_1011], 5, false);
    test(&[0b111_1011, 0b1_1100_1000], &[0b111_1011], 35, true);
    test(&[0b111_1011, 0b1_1100_1000], &[0b111_1011], 36, false);
    test(&[0b111_1011, 0b1_1100_1000], &[0b111_1011], 100, false);
    test(
        &[0b111_1011, 0b1_1100_1000],
        &[0b111_1011, 0b1_1110_1000],
        37,
        true,
    );
    test(
        &[0b111_1011, 0b1_1100_1000],
        &[0b111_1011, 0b1_1110_1000],
        38,
        false,
    );
    test(
        &[0b111_1011, 0b1_1100_1000],
        &[0b111_1011, 0b1_1110_1000],
        100,
        false,
    );
}

#[test]
fn test_eq_mod_power_of_two() {
    let test = |x, y, pow, out| {
        assert_eq!(
            Natural::from_str(x)
                .unwrap()
                .eq_mod_power_of_two(&Natural::from_str(y).unwrap(), pow),
            out
        );
        assert_eq!(
            rug::Integer::from_str(x).unwrap().is_congruent_2pow(
                &rug::Integer::from_str(y).unwrap(),
                u32::checked_from(pow).unwrap(),
            ),
            out
        );
    };
    test("0", "256", 8, true);
    test("0", "256", 9, false);
    test("13", "21", 0, true);
    test("13", "21", 1, true);
    test("13", "21", 2, true);
    test("13", "21", 3, true);
    test("13", "21", 4, false);
    test("13", "21", 100, false);
    test("1000000000001", "1", 12, true);
    test("1000000000001", "1", 13, false);
    test("4294967295", "4294967295", 32, true);
    test("281474976710672", "844424930131984", 49, true);
    test("281474976710672", "844424930131984", 50, false);
}

#[test]
fn limbs_eq_limb_mod_power_of_two_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
        |&(ref limbs, limb, pow)| {
            assert_eq!(
                limbs_eq_limb_mod_power_of_two(limbs, limb, pow),
                Natural::from_limbs_asc(limbs).eq_mod_power_of_two(&Natural::from(limb), pow)
            );
        },
    );
}

#[test]
fn limbs_eq_mod_power_of_two_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1,
        |&(ref xs, ref ys, pow)| {
            assert_eq!(
                limbs_eq_mod_power_of_two(xs, ys, pow),
                Natural::from_limbs_asc(xs).eq_mod_power_of_two(&Natural::from_limbs_asc(ys), pow)
            );
        },
    );
}

#[test]
fn eq_mod_power_of_two_properties() {
    test_properties(
        triples_of_natural_natural_and_small_unsigned,
        |&(ref x, ref y, pow)| {
            let eq_mod_power_of_two = x.eq_mod_power_of_two(y, pow);
            assert_eq!(
                natural_to_rug_integer(x)
                    .is_congruent_2pow(&natural_to_rug_integer(y), u32::checked_from(pow).unwrap()),
                eq_mod_power_of_two
            );
            assert_eq!(y.eq_mod_power_of_two(x, pow), eq_mod_power_of_two);
            assert_eq!(
                x.mod_power_of_two(pow) == y.mod_power_of_two(pow),
                eq_mod_power_of_two
            );
        },
    );

    test_properties(
        triples_of_natural_natural_and_small_unsigned_var_1::<u64>,
        |&(ref x, ref y, pow)| {
            assert!(x.eq_mod_power_of_two(y, pow));
            assert!(natural_to_rug_integer(x)
                .is_congruent_2pow(&natural_to_rug_integer(y), u32::checked_from(pow).unwrap()));
            assert!(y.eq_mod_power_of_two(x, pow));
            assert_eq!(x.mod_power_of_two(pow), y.mod_power_of_two(pow));
        },
    );

    test_properties(
        triples_of_natural_natural_and_small_unsigned_var_2::<u64>,
        |&(ref x, ref y, pow)| {
            assert!(!x.eq_mod_power_of_two(y, pow));
            assert!(!natural_to_rug_integer(x)
                .is_congruent_2pow(&natural_to_rug_integer(y), u32::checked_from(pow).unwrap()));
            assert!(!y.eq_mod_power_of_two(x, pow));
            assert_ne!(x.mod_power_of_two(pow), y.mod_power_of_two(pow));
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, pow)| {
        assert!(n.eq_mod_power_of_two(n, pow));
        assert_eq!(
            n.eq_mod_power_of_two(&Natural::ZERO, pow),
            n.divisible_by_power_of_two(pow)
        );
        assert_eq!(
            Natural::ZERO.eq_mod_power_of_two(n, pow),
            n.divisible_by_power_of_two(pow)
        );
    });

    test_properties(
        quadruples_of_natural_natural_natural_and_small_unsigned,
        |&(ref x, ref y, ref z, pow)| {
            if x.eq_mod_power_of_two(y, pow) && y.eq_mod_power_of_two(z, pow) {
                assert!(x.eq_mod_power_of_two(z, pow));
            }
        },
    );

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert!(x.eq_mod_power_of_two(y, 0));
    });
}
