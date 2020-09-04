use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShr, DivRound, DivisibleByPowerOfTwo, ShlRound, ShrRound, ShrRoundAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::arithmetic::shr_round::{
    limbs_shr_exact, limbs_shr_round, limbs_shr_round_nearest, limbs_shr_round_up,
    limbs_vec_shr_exact_in_place, limbs_vec_shr_round_in_place,
    limbs_vec_shr_round_nearest_in_place, limbs_vec_shr_round_up_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_positive_unsigned_and_small_unsigned, pairs_of_signed_and_rounding_mode,
    pairs_of_unsigned_and_rounding_mode, pairs_of_unsigned_vec_and_small_unsigned,
    pairs_of_unsigned_vec_and_small_unsigned_var_1,
    triples_of_unsigned_small_signed_and_rounding_mode_var_1,
    triples_of_unsigned_small_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_rounding_mode, pairs_of_natural_and_small_unsigned,
    pairs_of_natural_and_small_unsigned_var_2,
    triples_of_natural_small_signed_and_rounding_mode_var_2,
    triples_of_natural_small_unsigned_and_rounding_mode_var_1,
};

#[test]
fn limbs_shr_round_up_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, bits)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_shr_round_up(limbs, bits)),
                Natural::from_limbs_asc(limbs).shr_round(bits, RoundingMode::Up),
            );
        },
    );
}

#[test]
fn limbs_shr_round_nearest_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, bits)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_shr_round_nearest(limbs, bits)),
                Natural::from_limbs_asc(limbs).shr_round(bits, RoundingMode::Nearest),
            );
        },
    );
}

#[test]
fn limbs_shr_exact_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, bits)| {
            let n = Natural::from_limbs_asc(limbs);
            if let Some(result_limbs) = limbs_shr_exact(limbs, bits) {
                let m = (&n).shr_round(bits, RoundingMode::Exact);
                assert_eq!(Natural::from_owned_limbs_asc(result_limbs), m);
                assert_eq!(m << bits, n);
            } else {
                assert!(!n.divisible_by_power_of_two(bits));
            }
        },
    );
}

#[test]
fn limbs_shr_round_properties() {
    test_properties(
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
        |&(ref limbs, bits, rm)| {
            let n = Natural::from_limbs_asc(limbs);
            if let Some(result_limbs) = limbs_shr_round(limbs, bits, rm) {
                let m = (&n).shr_round(bits, rm);
                assert_eq!(Natural::from_owned_limbs_asc(result_limbs), m);
                if rm == RoundingMode::Exact {
                    assert_eq!(m << bits, n);
                }
            } else {
                assert_eq!(rm, RoundingMode::Exact);
                assert!(!n.divisible_by_power_of_two(bits));
            }
        },
    );
}

#[test]
fn limbs_vec_shr_round_up_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, bits)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_shr_round_up_in_place(&mut limbs, bits);
            let n = Natural::from_limbs_asc(&old_limbs).shr_round(bits, RoundingMode::Up);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn limbs_vec_shr_round_nearest_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, bits)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_shr_round_nearest_in_place(&mut limbs, bits);
            let n = Natural::from_limbs_asc(&old_limbs).shr_round(bits, RoundingMode::Nearest);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn limbs_vec_shr_exact_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, bits)| {
            let n = Natural::from_limbs_asc(limbs);
            let mut limbs = limbs.to_vec();
            if limbs_vec_shr_exact_in_place(&mut limbs, bits) {
                let m = (&n).shr_round(bits, RoundingMode::Exact);
                assert_eq!(Natural::from_owned_limbs_asc(limbs), m);
                assert_eq!(m << bits, n);
            } else {
                assert!(!n.divisible_by_power_of_two(bits));
            }
        },
    );
}

#[test]
fn limbs_vec_shr_round_in_place_properties() {
    test_properties(
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
        |&(ref limbs, bits, rm)| {
            let n = Natural::from_limbs_asc(limbs);
            let mut limbs = limbs.to_vec();
            if limbs_vec_shr_round_in_place(&mut limbs, bits, rm) {
                let m = (&n).shr_round(bits, rm);
                assert_eq!(Natural::from_owned_limbs_asc(limbs), m);
                if rm == RoundingMode::Exact {
                    assert_eq!(m << bits, n);
                }
            } else {
                assert_eq!(rm, RoundingMode::Exact);
                assert!(!n.divisible_by_power_of_two(bits));
            }
        },
    );
}

macro_rules! tests_and_properties_unsigned {
    (
        $t:ident,
        $shr_round_u_properties:ident
    ) => {
        #[test]
        fn $shr_round_u_properties() {
            test_properties(
                triples_of_natural_small_unsigned_and_rounding_mode_var_1::<$t>,
                |&(ref n, u, rm)| {
                    let mut mut_n = n.clone();
                    mut_n.shr_round_assign(u, rm);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;

                    let shifted_alt = n.shr_round(u, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    let shifted_alt = n.clone().shr_round(u, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    assert!(shifted <= *n);
                    assert_eq!(n.div_round(Natural::ONE << u, rm), shifted);
                },
            );

            test_properties(pairs_of_natural_and_small_unsigned::<$t>, |&(ref n, u)| {
                let left_shifted = n << u;
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Down), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Up), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Floor), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Ceiling), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Nearest), *n);
                assert_eq!((&left_shifted).shr_round(u, RoundingMode::Exact), *n);
            });

            // TODO test using Rationals
            test_properties(
                pairs_of_natural_and_small_unsigned_var_2::<$t>,
                |&(ref n, u)| {
                    let down = n.shr_round(u, RoundingMode::Down);
                    let up = &down + Natural::ONE;
                    assert_eq!(n.shr_round(u, RoundingMode::Up), up);
                    assert_eq!(n.shr_round(u, RoundingMode::Floor), down);
                    assert_eq!(n.shr_round(u, RoundingMode::Ceiling), up);
                    let nearest = n.shr_round(u, RoundingMode::Nearest);
                    assert!(nearest == down || nearest == up);
                },
            );

            test_properties(
                pairs_of_positive_unsigned_and_small_unsigned::<Limb, $t>,
                |&(u, v)| {
                    if let Some(shift) = v.checked_add($t::exact_from(Limb::WIDTH)) {
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Down), 0);
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Floor), 0);
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Up), 1);
                        assert_eq!(Natural::from(u).shr_round(shift, RoundingMode::Ceiling), 1);
                        if let Some(extra_shift) = shift.checked_add(1) {
                            assert_eq!(
                                Natural::from(u).shr_round(extra_shift, RoundingMode::Nearest),
                                0
                            );
                        }
                    }
                },
            );

            #[allow(unknown_lints, identity_op)]
            test_properties(pairs_of_natural_and_rounding_mode, |&(ref n, rm)| {
                assert_eq!(n.shr_round($t::ZERO, rm), *n);
            });

            test_properties(pairs_of_unsigned_and_rounding_mode::<$t>, |&(u, rm)| {
                assert_eq!(Natural::ZERO.shr_round(u, rm), 0);
            });

            test_properties(
                triples_of_unsigned_small_unsigned_and_rounding_mode_var_1::<Limb, $t>,
                |&(n, u, rm)| {
                    assert_eq!(n.shr_round(u, rm), Natural::from(n).shr_round(u, rm));
                },
            );
        }
    };
}
tests_and_properties_unsigned!(u8, shr_round_u8_properties);
tests_and_properties_unsigned!(u16, shr_round_u16_properties);
tests_and_properties_unsigned!(u32, shr_round_u32_properties);
tests_and_properties_unsigned!(u64, shr_round_u64_properties);
tests_and_properties_unsigned!(usize, shr_round_usize_properties);

macro_rules! tests_and_properties_signed {
    (
        $t:ident,
        $shr_round_i_properties:ident
    ) => {
        #[test]
        fn $shr_round_i_properties() {
            test_properties(
                triples_of_natural_small_signed_and_rounding_mode_var_2::<$t>,
                |&(ref n, i, rm)| {
                    let mut mut_n = n.clone();
                    mut_n.shr_round_assign(i, rm);
                    assert!(mut_n.is_valid());
                    let shifted = mut_n;

                    let shifted_alt = n.shr_round(i, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    let shifted_alt = n.clone().shr_round(i, rm);
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, shifted);

                    if i != $t::MIN {
                        assert_eq!(n.shl_round(-i, rm), shifted);
                    }
                },
            );

            #[allow(unknown_lints, identity_op)]
            test_properties(pairs_of_natural_and_rounding_mode, |&(ref n, rm)| {
                assert_eq!(n.shr_round($t::ZERO, rm), *n);
            });

            test_properties(pairs_of_signed_and_rounding_mode::<$t>, |&(i, rm)| {
                assert_eq!(Natural::ZERO.shr_round(i, rm), 0);
            });

            test_properties(
                triples_of_unsigned_small_signed_and_rounding_mode_var_1::<Limb, $t>,
                |&(n, i, rm)| {
                    if n.arithmetic_checked_shr(i).is_some() {
                        assert_eq!(n.shr_round(i, rm), Natural::from(n).shr_round(i, rm));
                    }
                },
            );
        }
    };
}
tests_and_properties_signed!(i8, shr_round_i8_properties);
tests_and_properties_signed!(i16, shr_round_i16_properties);
tests_and_properties_signed!(i32, shr_round_i32_properties);
tests_and_properties_signed!(i64, shr_round_i64_properties);
tests_and_properties_signed!(isize, shr_round_isize_properties);
