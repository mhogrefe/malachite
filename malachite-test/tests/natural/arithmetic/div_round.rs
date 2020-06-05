use malachite_base::num::arithmetic::traits::{CeilingDivNegMod, DivRound, DivRoundAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use num::Integer;
use rug::ops::DivRounding;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::triples_of_unsigned_positive_unsigned_and_rounding_mode_var_1;
use malachite_test::inputs::natural::{
    pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural_var_2,
    pairs_of_natural_and_rounding_mode, pairs_of_positive_natural_and_rounding_mode,
    triples_of_natural_positive_natural_and_rounding_mode_var_1,
};

#[test]
fn div_round_properties() {
    test_properties(
        triples_of_natural_positive_natural_and_rounding_mode_var_1,
        |&(ref x, ref y, rm)| {
            let mut mut_x = x.clone();
            mut_x.div_round_assign(y, rm);
            assert!(mut_x.is_valid());
            let q = mut_x;

            let mut mut_x = x.clone();
            mut_x.div_round_assign(y.clone(), rm);
            assert!(mut_x.is_valid());
            assert_eq!(mut_x, q);

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

            assert!(q <= *x);
        },
    );

    test_properties(pairs_of_natural_and_positive_natural, |&(ref x, ref y)| {
        let left_multiplied = x * y;
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Down), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Up), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Floor), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Ceiling), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Nearest), *x);
        assert_eq!((&left_multiplied).div_round(y, RoundingMode::Exact), *x);

        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(x).div_trunc(natural_to_rug_integer(y))),
            x.div_round(y, RoundingMode::Down)
        );
        assert_eq!(
            biguint_to_natural(&natural_to_biguint(x).div_floor(&natural_to_biguint(y))),
            x.div_round(y, RoundingMode::Floor)
        );
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(x).div_floor(natural_to_rug_integer(y))),
            x.div_round(y, RoundingMode::Floor)
        );
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(x).div_ceil(natural_to_rug_integer(y))),
            x.div_round(y, RoundingMode::Ceiling)
        );
        assert_eq!(
            x.ceiling_div_neg_mod(y).0,
            x.div_round(y, RoundingMode::Ceiling)
        );
    });

    // TODO test using Rationals
    test_properties(
        pairs_of_natural_and_positive_natural_var_2,
        |&(ref x, ref y)| {
            let down = x.div_round(y, RoundingMode::Down);
            let up = &down + Natural::ONE;
            assert_eq!(x.div_round(y, RoundingMode::Up), up);
            assert_eq!(x.div_round(y, RoundingMode::Floor), down);
            assert_eq!(x.div_round(y, RoundingMode::Ceiling), up);
            let nearest = x.div_round(y, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        },
    );

    test_properties(pairs_of_natural_and_rounding_mode, |&(ref x, rm)| {
        assert_eq!(x.div_round(Natural::ONE, rm), *x);
    });

    test_properties(
        pairs_of_positive_natural_and_rounding_mode,
        |&(ref x, rm)| {
            assert_eq!(Natural::ZERO.div_round(x, rm), 0);
            assert_eq!(x.div_round(x, rm), 1);
        },
    );

    test_properties(
        triples_of_unsigned_positive_unsigned_and_rounding_mode_var_1::<Limb>,
        |&(x, y, rm)| {
            assert_eq!(
                Natural::from(x).div_round(Natural::from(y), rm),
                x.div_round(y, rm)
            );
        },
    );
}
