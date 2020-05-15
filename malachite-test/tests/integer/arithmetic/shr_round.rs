use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShr, DivRound, ShrRound, ShrRoundAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_rounding_mode, pairs_of_unsigned_and_rounding_mode,
    triples_of_signed_small_signed_and_rounding_mode_var_1,
    triples_of_signed_small_unsigned_and_rounding_mode_var_1,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_rounding_mode, pairs_of_integer_and_small_unsigned,
    pairs_of_integer_and_small_unsigned_var_2,
    triples_of_integer_small_signed_and_rounding_mode_var_2,
    triples_of_integer_small_unsigned_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::{
    triples_of_natural_small_signed_and_rounding_mode_var_2,
    triples_of_natural_small_unsigned_and_rounding_mode_var_1,
};

macro_rules! properties_unsigned {
    (
        $t:ident,
        $shr_round_u_properties:ident
    ) => {
        #[test]
        fn $shr_round_u_properties() {
            test_properties(
                triples_of_integer_small_unsigned_and_rounding_mode_var_1::<$t>,
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

                    assert!(n.shr_round(u, rm).le_abs(n));
                    assert_eq!(-(-n).shr_round(u, -rm), shifted);
                    assert_eq!(n.div_round(Integer::ONE << u, rm), shifted);
                },
            );

            test_properties(pairs_of_integer_and_small_unsigned::<$t>, |&(ref n, u)| {
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
                pairs_of_integer_and_small_unsigned_var_2::<$t>,
                |&(ref n, u)| {
                    let floor = n.shr_round(u, RoundingMode::Floor);
                    let ceiling = &floor + Integer::ONE;
                    assert_eq!(n.shr_round(u, RoundingMode::Ceiling), ceiling);
                    if *n >= 0 {
                        assert_eq!(n.shr_round(u, RoundingMode::Up), ceiling);
                        assert_eq!(n.shr_round(u, RoundingMode::Down), floor);
                    } else {
                        assert_eq!(n.shr_round(u, RoundingMode::Up), floor);
                        assert_eq!(n.shr_round(u, RoundingMode::Down), ceiling);
                    }
                    let nearest = n.shr_round(u, RoundingMode::Nearest);
                    assert!(nearest == floor || nearest == ceiling);
                },
            );

            test_properties(pairs_of_integer_and_rounding_mode, |&(ref n, rm)| {
                assert_eq!(n.shr_round($t::ZERO, rm), *n);
            });

            test_properties(pairs_of_unsigned_and_rounding_mode::<$t>, |&(u, rm)| {
                assert_eq!(Integer::ZERO.shr_round(u, rm), 0);
            });

            test_properties(
                triples_of_natural_small_unsigned_and_rounding_mode_var_1::<$t>,
                |&(ref n, u, rm)| {
                    assert_eq!(n.shr_round(u, rm), Integer::from(n).shr_round(u, rm));
                },
            );

            test_properties(
                triples_of_signed_small_unsigned_and_rounding_mode_var_1::<SignedLimb, $t>,
                |&(n, u, rm)| {
                    assert_eq!(n.shr_round(u, rm), Integer::from(n).shr_round(u, rm));
                },
            );
        }
    };
}
properties_unsigned!(u8, shr_round_u8_properties);
properties_unsigned!(u16, shr_round_u16_properties);
properties_unsigned!(u32, shr_round_u32_properties);
properties_unsigned!(u64, shr_round_u64_properties);
properties_unsigned!(usize, shr_round_usize_properties);

macro_rules! properties_signed {
    (
        $t:ident,
        $shr_round_i_properties:ident
    ) => {
        #[test]
        fn $shr_round_i_properties() {
            test_properties(
                triples_of_integer_small_signed_and_rounding_mode_var_2::<$t>,
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

                    assert_eq!(-(-n).shr_round(i, -rm), shifted);
                },
            );

            test_properties(pairs_of_integer_and_rounding_mode, |&(ref n, rm)| {
                assert_eq!(n.shr_round($t::ZERO, rm), *n);
            });

            test_properties(pairs_of_signed_and_rounding_mode::<$t>, |&(i, rm)| {
                assert_eq!(Integer::ZERO.shr_round(i, rm), 0);
            });

            test_properties(
                triples_of_natural_small_signed_and_rounding_mode_var_2::<$t>,
                |&(ref n, i, rm)| {
                    assert_eq!(n.shr_round(i, rm), Integer::from(n).shr_round(i, rm));
                },
            );

            test_properties(
                triples_of_signed_small_signed_and_rounding_mode_var_1::<SignedLimb, $t>,
                |&(n, i, rm)| {
                    if n.arithmetic_checked_shr(i).is_some() {
                        assert_eq!(n.shr_round(i, rm), Integer::from(n).shr_round(i, rm));
                    }
                },
            );
        }
    };
}
properties_signed!(i8, shr_round_i8_properties);
properties_signed!(i16, shr_round_i16_properties);
properties_signed!(i32, shr_round_i32_properties);
properties_signed!(i64, shr_round_i64_properties);
properties_signed!(isize, shr_round_isize_properties);
