use malachite_base::num::arithmetic::traits::{
    Abs, DivisibleByPowerOf2, PowerOf2, RoundToMultiple, RoundToMultipleOfPowerOf2,
    RoundToMultipleOfPowerOf2Assign, ShrRound,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_2::{
    limbs_round_to_multiple_of_power_of_2, limbs_round_to_multiple_of_power_of_2_down,
    limbs_round_to_multiple_of_power_of_2_down_in_place,
    limbs_round_to_multiple_of_power_of_2_in_place, limbs_round_to_multiple_of_power_of_2_nearest,
    limbs_round_to_multiple_of_power_of_2_nearest_in_place,
    limbs_round_to_multiple_of_power_of_2_up, limbs_round_to_multiple_of_power_of_2_up_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_rounding_mode, pairs_of_unsigned_vec_and_small_unsigned,
    pairs_of_unsigned_vec_and_small_unsigned_var_1,
    triples_of_unsigned_small_u64_and_rounding_mode_var_2,
    triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_rounding_mode, pairs_of_natural_and_small_unsigned,
    pairs_of_natural_and_small_unsigned_var_2, pairs_of_positive_natural_and_small_unsigned,
    triples_of_natural_small_unsigned_and_rounding_mode_var_1,
};

#[test]
fn limbs_round_to_multiple_of_power_of_2_down_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_round_to_multiple_of_power_of_2_down(
                    limbs, pow
                )),
                Natural::from_limbs_asc(limbs) >> pow << pow
            );
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_up_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_round_to_multiple_of_power_of_2_up(limbs, pow)),
                Natural::from_limbs_asc(limbs)
                    .round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
            );
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_nearest_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_round_to_multiple_of_power_of_2_nearest(
                    limbs, pow
                )),
                Natural::from_limbs_asc(limbs)
                    .round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest)
            );
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_properties() {
    test_properties(
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
        |&(ref limbs, pow, rm)| {
            let n = Natural::from_limbs_asc(limbs);
            if let Some(result_limbs) = limbs_round_to_multiple_of_power_of_2(limbs, pow, rm) {
                let m = (&n).round_to_multiple_of_power_of_2(pow, rm);
                assert_eq!(Natural::from_owned_limbs_asc(result_limbs), m);
                if rm == RoundingMode::Exact {
                    assert_eq!(m, n);
                }
            } else {
                assert_eq!(rm, RoundingMode::Exact);
                assert!(!n.divisible_by_power_of_2(pow));
            }
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_down_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_round_to_multiple_of_power_of_2_down_in_place(&mut limbs, pow);
            let n = Natural::from_limbs_asc(&old_limbs) >> pow << pow;
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_up_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, pow)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_round_to_multiple_of_power_of_2_up_in_place(&mut limbs, pow);
            let n = Natural::from_limbs_asc(&old_limbs)
                .round_to_multiple_of_power_of_2(pow, RoundingMode::Up);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_nearest_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_round_to_multiple_of_power_of_2_nearest_in_place(&mut limbs, pow);
            let n = Natural::from_limbs_asc(&old_limbs)
                .round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_in_place_properties() {
    test_properties(
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
        |&(ref limbs, pow, rm)| {
            let n = Natural::from_limbs_asc(limbs);
            let mut limbs = limbs.to_vec();
            if limbs_round_to_multiple_of_power_of_2_in_place(&mut limbs, pow, rm) {
                let m = (&n).round_to_multiple_of_power_of_2(pow, rm);
                assert_eq!(Natural::from_owned_limbs_asc(limbs), m);
                if rm == RoundingMode::Exact {
                    assert_eq!(m, n);
                }
            } else {
                assert_eq!(rm, RoundingMode::Exact);
                assert!(!n.divisible_by_power_of_2(pow));
            }
        },
    );
}

#[test]
fn round_to_multiple_of_power_of_2_properties() {
    test_properties(
        triples_of_natural_small_unsigned_and_rounding_mode_var_1,
        |&(ref n, pow, rm)| {
            let r = n.round_to_multiple_of_power_of_2(pow, rm);
            assert!(r.is_valid());

            let r_alt = n.clone().round_to_multiple_of_power_of_2(pow, rm);
            assert!(r_alt.is_valid());
            assert_eq!(r_alt, r);

            let mut mut_n = n.clone();
            mut_n.round_to_multiple_of_power_of_2_assign(pow, rm);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, r);

            assert!(r.divisible_by_power_of_2(pow));
            assert_eq!(n.shr_round(pow, rm) << pow, r);
            assert!((Integer::from(&r) - Integer::from(n)).abs() <= Natural::power_of_2(pow));
            assert_eq!(n.round_to_multiple(Natural::power_of_2(pow), rm), r);
            match rm {
                RoundingMode::Floor | RoundingMode::Down => assert!(r <= *n),
                RoundingMode::Ceiling | RoundingMode::Up => assert!(r >= *n),
                RoundingMode::Exact => assert_eq!(r, *n),
                RoundingMode::Nearest => {
                    let k = Natural::power_of_2(pow);
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
        pairs_of_natural_and_small_unsigned::<u64>,
        |&(ref n, pow)| {
            let shifted = n << pow;
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Down),
                shifted
            );
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
                shifted
            );
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
                shifted
            );
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
                shifted
            );
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest),
                shifted
            );
            assert_eq!(
                (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Exact),
                shifted
            );
        },
    );

    test_properties(
        pairs_of_natural_and_small_unsigned_var_2,
        |&(ref n, pow)| {
            let down = n.round_to_multiple_of_power_of_2(pow, RoundingMode::Down);
            let up = &down + Natural::power_of_2(pow);
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
                up
            );
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
                down
            );
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
                up
            );
            let nearest = n.round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        },
    );

    test_properties(
        pairs_of_positive_natural_and_small_unsigned::<u64>,
        |&(ref n, pow)| {
            if let Some(shift) = pow.checked_add(n.significant_bits()) {
                assert_eq!(
                    (&n).round_to_multiple_of_power_of_2(shift, RoundingMode::Down),
                    0
                );
                assert_eq!(
                    (&n).round_to_multiple_of_power_of_2(shift, RoundingMode::Floor),
                    0
                );
                if let Some(extra_shift) = shift.checked_add(1) {
                    assert_eq!(
                        n.round_to_multiple_of_power_of_2(extra_shift, RoundingMode::Nearest),
                        0
                    );
                }
            }
        },
    );

    test_properties(pairs_of_natural_and_rounding_mode, |&(ref n, rm)| {
        assert_eq!(n.round_to_multiple_of_power_of_2(0, rm), *n);
    });

    test_properties(pairs_of_unsigned_and_rounding_mode, |&(pow, rm)| {
        assert_eq!(Natural::ZERO.round_to_multiple_of_power_of_2(pow, rm), 0);
    });

    test_properties(
        triples_of_unsigned_small_u64_and_rounding_mode_var_2::<Limb>,
        |&(n, pow, rm)| {
            assert_eq!(
                n.round_to_multiple_of_power_of_2(pow, rm),
                Natural::from(n).round_to_multiple_of_power_of_2(pow, rm)
            );
        },
    );
}
