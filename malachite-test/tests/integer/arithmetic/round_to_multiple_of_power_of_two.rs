use malachite_base::num::arithmetic::traits::{
    Abs, DivisibleByPowerOfTwo, PowerOfTwo, RoundToMultiple, RoundToMultipleOfPowerOfTwo,
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
            let r = n.round_to_multiple_of_power_of_two(pow, rm);
            assert!(r.is_valid());

            let r_alt = n.clone().round_to_multiple_of_power_of_two(pow, rm);
            assert!(r_alt.is_valid());
            assert_eq!(r_alt, r);

            let mut mut_n = n.clone();
            mut_n.round_to_multiple_of_power_of_two_assign(pow, rm);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, r);

            assert!(r.divisible_by_power_of_two(pow));
            assert_eq!(n.shr_round(pow, rm) << pow, r);
            assert_eq!(-(-n).round_to_multiple_of_power_of_two(pow, -rm), r);
            assert!((&r - n).abs() <= Integer::power_of_two(pow));
            assert_eq!(n.round_to_multiple(Integer::power_of_two(pow), rm), r);
            match rm {
                RoundingMode::Floor => assert!(r <= *n),
                RoundingMode::Ceiling => assert!(r >= *n),
                RoundingMode::Down => assert!(r.le_abs(n)),
                RoundingMode::Up => assert!(r.ge_abs(n)),
                RoundingMode::Exact => assert_eq!(r, *n),
                RoundingMode::Nearest => {
                    let k = Integer::power_of_two(pow);
                    let closest;
                    let second_closest;
                    if r <= *n {
                        closest = n - &r;
                        second_closest = &r + k - n;
                    } else {
                        closest = &r - n;
                        second_closest = n + k - &r;
                    }
                    assert!(closest <= second_closest);
                    if closest == second_closest {
                        assert!(!r.get_bit(pow));
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
