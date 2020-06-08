use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOfTwo, PowerOfTwo, RoundToMultipleOfPowerOfTwo,
    RoundToMultipleOfPowerOfTwoAssign, ShrRound,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_rounding_mode, triples_of_signed_small_u64_and_rounding_mode_var_2,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_rounding_mode, pairs_of_integer_and_small_unsigned,
    pairs_of_integer_and_small_unsigned_var_2,
    triples_of_integer_small_unsigned_and_rounding_mode_var_1,
};

#[test]
fn round_to_multiple_of_power_of_two_properties() {
    test_properties(
        triples_of_integer_small_unsigned_and_rounding_mode_var_1,
        |&(ref n, pow, rm)| {
            let rounded = n.round_to_multiple_of_power_of_two(pow, rm);
            assert!(rounded.is_valid());

            let rounded_alt = n.clone().round_to_multiple_of_power_of_two(pow, rm);
            assert!(rounded_alt.is_valid());
            assert_eq!(rounded_alt, rounded);

            let mut mut_n = n.clone();
            mut_n.round_to_multiple_of_power_of_two_assign(pow, rm);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, rounded);

            assert!(rounded.divisible_by_power_of_two(pow));
            assert_eq!(n.shr_round(pow, rm) << pow, rounded);
            assert_eq!(-(-n).round_to_multiple_of_power_of_two(pow, -rm), rounded);
            match rm {
                RoundingMode::Floor => assert!(rounded <= *n),
                RoundingMode::Ceiling => assert!(rounded >= *n),
                RoundingMode::Down => assert!(rounded.le_abs(n)),
                RoundingMode::Up => assert!(rounded.ge_abs(n)),
                RoundingMode::Exact => assert_eq!(rounded, *n),
                RoundingMode::Nearest => {
                    let k = Integer::power_of_two(pow);
                    let closest;
                    let second_closest;
                    if rounded <= *n {
                        closest = n - &rounded;
                        second_closest = &rounded + k - n;
                    } else {
                        closest = &rounded - n;
                        second_closest = n + k - &rounded;
                    }
                    assert!(closest <= second_closest);
                    if closest == second_closest {
                        assert!(!rounded.get_bit(pow));
                    }
                }
            }
        },
    );

    test_properties(
        pairs_of_integer_and_small_unsigned::<u64>,
        |&(ref n, pow)| {
            let shifted = n << pow;
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_two(pow, RoundingMode::Down),
                shifted
            );
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_two(pow, RoundingMode::Up),
                shifted
            );
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_two(pow, RoundingMode::Floor),
                shifted
            );
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_two(pow, RoundingMode::Ceiling),
                shifted
            );
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_two(pow, RoundingMode::Nearest),
                shifted
            );
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_two(pow, RoundingMode::Exact),
                shifted
            );
        },
    );

    test_properties(
        pairs_of_integer_and_small_unsigned_var_2,
        |&(ref n, pow)| {
            let floor = n.round_to_multiple_of_power_of_two(pow, RoundingMode::Floor);
            let ceiling = &floor + Integer::power_of_two(pow);
            assert_eq!(
                (&n).round_to_multiple_of_power_of_two(pow, RoundingMode::Ceiling),
                ceiling
            );
            if *n >= 0 {
                assert_eq!(
                    (&n).round_to_multiple_of_power_of_two(pow, RoundingMode::Up),
                    ceiling
                );
                assert_eq!(
                    (&n).round_to_multiple_of_power_of_two(pow, RoundingMode::Down),
                    floor
                );
            } else {
                assert_eq!(
                    (&n).round_to_multiple_of_power_of_two(pow, RoundingMode::Up),
                    floor
                );
                assert_eq!(
                    (&n).round_to_multiple_of_power_of_two(pow, RoundingMode::Down),
                    ceiling
                );
            }
            let nearest = n.round_to_multiple_of_power_of_two(pow, RoundingMode::Nearest);
            assert!(nearest == ceiling || nearest == floor);
        },
    );

    test_properties(pairs_of_integer_and_rounding_mode, |&(ref n, rm)| {
        assert_eq!(n.round_to_multiple_of_power_of_two(0, rm), *n);
    });

    test_properties(pairs_of_unsigned_and_rounding_mode, |&(pow, rm)| {
        assert_eq!(Integer::ZERO.round_to_multiple_of_power_of_two(pow, rm), 0);
    });

    test_properties(
        triples_of_signed_small_u64_and_rounding_mode_var_2::<SignedLimb>,
        |&(n, pow, rm)| {
            assert_eq!(
                n.round_to_multiple_of_power_of_two(pow, rm),
                Integer::from(n).round_to_multiple_of_power_of_two(pow, rm)
            );
        },
    );
}
