use malachite_base::num::arithmetic::traits::{DivRound, ShrRound, ShrRoundAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};

use malachite_test::common::test_properties;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::{
    pairs_of_negative_signed_not_min_and_small_unsigned,
    pairs_of_positive_signed_and_small_unsigned, pairs_of_signed_and_small_unsigned,
    pairs_of_unsigned_and_rounding_mode, triples_of_signed_small_unsigned_and_rounding_mode_var_1,
    unsigneds,
};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_rounding_mode, pairs_of_integer_and_small_unsigned,
    pairs_of_integer_and_small_unsigned_var_2,
    triples_of_integer_small_unsigned_and_rounding_mode_var_1,
    triples_of_integer_small_unsigned_and_small_unsigned,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_unsigned, triples_of_natural_small_unsigned_and_rounding_mode_var_1,
};

macro_rules! tests_and_properties {
    (
        $t:ident,
        $shr_u_properties:ident,
        $shr_round_u_properties:ident,
        $n:ident,
        $u:ident,
        $shifted:ident,
        $shl_library_comparison_properties:expr
    ) => {
        #[test]
        fn $shr_u_properties() {
            test_properties(
                pairs_of_integer_and_small_unsigned::<$t>,
                |&(ref $n, $u)| {
                    let mut mut_n = $n.clone();
                    mut_n >>= $u;
                    assert!(mut_n.is_valid());
                    let $shifted = mut_n;

                    let shifted_alt = $n >> $u;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);
                    let shifted_alt = $n.clone() >> $u;
                    assert!(shifted_alt.is_valid());
                    assert_eq!(shifted_alt, $shifted);

                    assert!($shifted.le_abs($n));
                    assert_eq!($n.shr_round($u, RoundingMode::Floor), $shifted);

                    if $u < $t::wrapping_from(<$t as PrimitiveUnsigned>::SignedOfEqualWidth::MAX) {
                        let u = <$t as PrimitiveUnsigned>::SignedOfEqualWidth::wrapping_from($u);
                        assert_eq!($n >> u, $shifted);
                        assert_eq!($n << -u, $shifted);
                    }

                    $shl_library_comparison_properties
                },
            );

            test_properties(
                pairs_of_signed_and_small_unsigned::<SignedLimb, $t>,
                |&(i, j)| {
                    if let Some(sum) = j.checked_add($t::exact_from(SignedLimb::WIDTH)) {
                        let shifted = Integer::from(i) >> sum;
                        if i >= 0 {
                            assert_eq!(shifted, 0);
                        } else {
                            assert_eq!(shifted, -1);
                        }
                    }

                    if j < $t::exact_from(SignedLimb::WIDTH) {
                        assert_eq!(i >> j, Integer::from(i) >> j);
                    }
                },
            );

            test_properties(
                triples_of_integer_small_unsigned_and_small_unsigned::<$t, $t>,
                |&(ref n, u, v)| {
                    if let Some(sum) = u.checked_add(v) {
                        assert_eq!(n >> u >> v, n >> sum);
                    }
                },
            );

            test_properties(integers, |n| {
                assert_eq!(n >> $t::ZERO, *n);
            });

            test_properties(unsigneds::<$t>, |&u| {
                assert_eq!(Integer::ZERO >> u, 0);
            });

            test_properties(pairs_of_natural_and_small_unsigned::<$t>, |&(ref n, u)| {
                assert_eq!(n >> u, Integer::from(n) >> u);
            });
        }

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

            test_properties(
                pairs_of_positive_signed_and_small_unsigned::<SignedLimb, $t>,
                |&(i, u)| {
                    if let Some(sum) = u.checked_add($t::exact_from(Limb::WIDTH - 1)) {
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Down), 0);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Floor), 0);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Up), 1);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Ceiling), 1);
                        if let Some(sum) = sum.checked_add(1) {
                            assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Nearest), 0);
                        }
                    }
                },
            );

            test_properties(
                pairs_of_negative_signed_not_min_and_small_unsigned::<SignedLimb, $t>,
                |&(i, u)| {
                    if let Some(sum) = u.checked_add($t::exact_from(Limb::WIDTH - 1)) {
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Down), 0);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Floor), -1);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Up), -1);
                        assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Ceiling), 0);
                        if let Some(sum) = sum.checked_add(1) {
                            assert_eq!(Integer::from(i).shr_round(sum, RoundingMode::Nearest), 0);
                        }
                    }
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
tests_and_properties!(
    u8,
    shr_u8_properties,
    shr_round_u8_properties,
    n,
    u,
    shifted,
    {}
);
tests_and_properties!(
    u16,
    shr_u16_properties,
    shr_round_u16_properties,
    n,
    u,
    shifted,
    {}
);
tests_and_properties!(
    u32,
    shr_u32_properties,
    shr_round_u32_properties,
    n,
    u,
    shifted,
    {
        let mut rug_n = integer_to_rug_integer(n);
        rug_n >>= u;
        assert_eq!(rug_integer_to_integer(&rug_n), shifted);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(n) >> u)),
            shifted
        );

        assert_eq!(
            bigint_to_integer(&(&integer_to_bigint(n) >> usize::exact_from(u))),
            shifted
        );
        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(n) >> usize::exact_from(u))),
            shifted
        );
    }
);
tests_and_properties!(
    u64,
    shr_u64_properties,
    shr_round_u64_properties,
    n,
    u,
    shifted,
    {}
);
tests_and_properties!(
    usize,
    shr_usize_properties,
    shr_round_usize_properties,
    n,
    u,
    shifted,
    {}
);
