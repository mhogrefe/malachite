use malachite_base::num::arithmetic::traits::{DivRound, DivRoundAssign};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use num::Integer as NumInteger;
use rug::ops::DivRounding;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::triples_of_signed_nonzero_signed_and_rounding_mode_var_1;
use malachite_test::inputs::integer::{
    pairs_of_integer_and_nonzero_integer, pairs_of_integer_and_nonzero_integer_var_2,
    pairs_of_integer_and_rounding_mode, pairs_of_nonzero_integer_and_rounding_mode,
    triples_of_integer_nonzero_integer_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::triples_of_natural_positive_natural_and_rounding_mode_var_1;

#[test]
fn div_round_properties() {
    test_properties(
        triples_of_integer_nonzero_integer_and_rounding_mode_var_1,
        |&(ref x, ref y, rm): &(Integer, Integer, RoundingMode)| {
            let mut mut_n = x.clone();
            mut_n.div_round_assign(y, rm);
            assert!(mut_n.is_valid());
            let q = mut_n;

            let mut mut_n = x.clone();
            mut_n.div_round_assign(y.clone(), rm);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, q);

            let q_alt = x.div_round(y, rm);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.div_round(y.clone(), rm);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.clone().div_round(y, rm);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.clone().div_round(y.clone(), rm);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            assert!(q.le_abs(x));
            assert_eq!(-(-x).div_round(y, -rm), q);
            assert_eq!(-x.div_round(-y, -rm), q);
        },
    );

    test_properties(pairs_of_integer_and_nonzero_integer, |&(ref x, ref y)| {
        let left_multiplied = x * y;
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Down), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Up), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Floor), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Ceiling), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Nearest), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Exact), *x);

        assert_eq!(
            rug_integer_to_integer(&integer_to_rug_integer(x).div_trunc(integer_to_rug_integer(y))),
            x.div_round(y, RoundingMode::Down)
        );
        assert_eq!(
            bigint_to_integer(&integer_to_bigint(x).div_floor(&integer_to_bigint(y))),
            x.div_round(y, RoundingMode::Floor)
        );
        {
            assert_eq!(
                rug_integer_to_integer(
                    &integer_to_rug_integer(x).div_floor(integer_to_rug_integer(y))
                ),
                x.div_round(y, RoundingMode::Floor)
            );
            assert_eq!(
                rug_integer_to_integer(
                    &integer_to_rug_integer(x).div_ceil(integer_to_rug_integer(y))
                ),
                x.div_round(y, RoundingMode::Ceiling)
            );
        }
    });

    // TODO test using Rationals
    test_properties(
        pairs_of_integer_and_nonzero_integer_var_2,
        |&(ref x, ref y)| {
            let down = x.div_round(y, RoundingMode::Down);
            let up = if (*x >= 0) == (*y >= 0) {
                &down + Integer::ONE
            } else {
                &down - Integer::ONE
            };
            let floor = x.div_round(y, RoundingMode::Floor);
            let ceiling = &floor + Integer::ONE;
            assert_eq!(x.div_round(y, RoundingMode::Up), up);
            assert_eq!(x.div_round(y, RoundingMode::Ceiling), ceiling);
            let nearest = x.div_round(y, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        },
    );

    test_properties(pairs_of_integer_and_rounding_mode, |&(ref x, rm)| {
        assert_eq!(x.div_round(Integer::ONE, rm), *x);
        assert_eq!(x.div_round(Integer::NEGATIVE_ONE, rm), -x);
    });

    test_properties(
        pairs_of_nonzero_integer_and_rounding_mode,
        |&(ref x, rm)| {
            assert_eq!(Integer::ZERO.div_round(x, rm), 0);
            assert_eq!(x.div_round(x, rm), 1);
            assert_eq!(x.div_round(-x, rm), -1);
            assert_eq!((-x).div_round(x, rm), -1);
        },
    );

    test_properties(
        triples_of_natural_positive_natural_and_rounding_mode_var_1,
        |&(ref x, ref y, rm)| {
            assert_eq!(
                Integer::from(x).div_round(Integer::from(y), rm),
                x.div_round(y, rm)
            );
        },
    );

    test_properties(
        triples_of_signed_nonzero_signed_and_rounding_mode_var_1::<SignedLimb>,
        |&(x, y, rm)| {
            assert_eq!(
                Integer::from(x).div_round(Integer::from(y), rm),
                x.div_round(y, rm)
            );
        },
    );
}
