use std::str::FromStr;

use malachite_base::num::arithmetic::traits::DivisibleByPowerOfTwo;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use common::test_properties;
use malachite_test::common::natural_to_rug_integer;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned_var_1, unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned, pairs_of_natural_and_small_unsigned_var_1,
    pairs_of_natural_and_small_unsigned_var_2,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_divisible_by_power_of_two() {
    let test = |limbs: &[Limb], pow: u64, out: bool| {
        assert_eq!(limbs_divisible_by_power_of_two(limbs, pow), out);
    };
    test(&[1], 0, true);
    test(&[1], 1, false);
    test(&[2], 0, true);
    test(&[2], 1, true);
    test(&[2], 2, false);
    test(&[3], 1, false);
    test(&[122, 456], 1, true);
    test(&[0, 0, 1], 64, true);
    test(&[0, 0, 1], 65, false);
    test(&[0, 0, 1], 100, false);
    test(&[3_567_587_328, 232], 11, true);
    test(&[3_567_587_328, 232], 12, true);
    test(&[3_567_587_328, 232], 13, false);
}

#[test]
fn test_divisible_by_power_of_two() {
    let test = |n, pow, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().divisible_by_power_of_two(pow),
            out
        );
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .is_divisible_2pow(u32::checked_from(pow).unwrap()),
            out
        );
    };
    test("0", 0, true);
    test("0", 10, true);
    test("0", 100, true);
    test("123", 0, true);
    test("123", 1, false);
    test("1000000000000", 0, true);
    test("1000000000000", 12, true);
    test("1000000000000", 13, false);
    test("4294967295", 0, true);
    test("4294967295", 1, false);
    test("4294967296", 0, true);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("18446744073709551615", 0, true);
    test("18446744073709551615", 1, false);
    test("18446744073709551616", 0, true);
    test("18446744073709551616", 64, true);
    test("18446744073709551616", 65, false);
}

#[test]
fn limbs_divisible_by_power_of_two_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, pow)| {
            assert_eq!(
                limbs_divisible_by_power_of_two(limbs, pow),
                Natural::from_limbs_asc(limbs).divisible_by_power_of_two(pow),
            );
        },
    );
}

#[test]
fn divisible_by_power_of_two_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref x, pow)| {
        let divisible = x.divisible_by_power_of_two(pow);
        assert_eq!(
            natural_to_rug_integer(x).is_divisible_2pow(u32::checked_from(pow).unwrap()),
            divisible
        );
        if *x != 0 as Limb {
            assert_eq!(x.trailing_zeros().unwrap() >= pow, divisible);
        }
        assert_eq!((-x).divisible_by_power_of_two(pow), divisible);
        assert!((x << pow).divisible_by_power_of_two(pow));
        assert_eq!(x >> pow << pow == *x, divisible);
    });

    test_properties(
        pairs_of_natural_and_small_unsigned_var_1,
        |&(ref x, pow)| {
            assert!(x.divisible_by_power_of_two(pow));
            assert!(natural_to_rug_integer(x).is_divisible_2pow(u32::checked_from(pow).unwrap()));
            if *x != 0 as Limb {
                assert!(x.trailing_zeros().unwrap() >= pow);
            }
            assert!((-x).divisible_by_power_of_two(pow));
            assert_eq!(x >> pow << pow, *x);
        },
    );

    test_properties(
        pairs_of_natural_and_small_unsigned_var_2,
        |&(ref x, pow)| {
            assert!(!x.divisible_by_power_of_two(pow));
            assert!(!natural_to_rug_integer(x).is_divisible_2pow(u32::checked_from(pow).unwrap()));
            if *x != 0 as Limb {
                assert!(x.trailing_zeros().unwrap() < pow);
            }
            assert!(!(-x).divisible_by_power_of_two(pow));
            assert_ne!(x >> pow << pow, *x);
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(x, pow)| {
            assert_eq!(
                x.divisible_by_power_of_two(pow),
                Natural::from(x).divisible_by_power_of_two(pow)
            );
        },
    );

    test_properties(naturals, |x| {
        assert!(x.divisible_by_power_of_two(0));
    });

    test_properties(unsigneds, |&pow| {
        assert!(Natural::ZERO.divisible_by_power_of_two(pow));
    });
}
