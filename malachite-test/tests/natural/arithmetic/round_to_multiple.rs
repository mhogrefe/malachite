use malachite_base::num::arithmetic::traits::{
    Abs, DivRound, DivisibleBy, Parity, RoundToMultiple, RoundToMultipleAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::triples_of_unsigned_unsigned_and_rounding_mode_var_1;
use malachite_test::inputs::natural::{
    pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural_var_2,
    pairs_of_natural_and_rounding_mode, triples_of_natural_natural_and_rounding_mode_var_2,
};

#[test]
fn round_to_multiple_properties() {
    test_properties(
        triples_of_natural_natural_and_rounding_mode_var_2,
        |&(ref x, ref y, rm)| {
            let mut mut_x = x.clone();
            mut_x.round_to_multiple_assign(y, rm);
            assert!(mut_x.is_valid());
            let r = mut_x;

            let mut mut_x = x.clone();
            mut_x.round_to_multiple_assign(y.clone(), rm);
            assert!(mut_x.is_valid());
            assert_eq!(mut_x, r);

            let r_alt = x.round_to_multiple(y, rm);
            assert!(r_alt.is_valid());
            assert_eq!(r_alt, r);

            let r_alt = x.round_to_multiple(y.clone(), rm);
            assert!(r_alt.is_valid());
            assert_eq!(r_alt, r);

            let r_alt = x.clone().round_to_multiple(y, rm);
            assert!(r_alt.is_valid());
            assert_eq!(r_alt, r);

            let r_alt = x.clone().round_to_multiple(y.clone(), rm);
            assert!(r_alt.is_valid());
            assert_eq!(r_alt, r);

            assert!((&r).divisible_by(y));
            if *y == 0 {
                assert_eq!(r, 0);
            } else {
                assert!((Integer::from(&r) - Integer::from(x)).abs() <= *y);
                match rm {
                    RoundingMode::Floor | RoundingMode::Down => assert!(r <= *x),
                    RoundingMode::Ceiling | RoundingMode::Up => assert!(r >= *x),
                    RoundingMode::Exact => assert_eq!(r, *x),
                    RoundingMode::Nearest => {
                        let closest;
                        let second_closest;
                        if r <= *x {
                            closest = x - &r;
                            second_closest = &r + y - x;
                        } else {
                            closest = &r - x;
                            second_closest = x + y - &r;
                        }
                        assert!(closest <= second_closest);
                        if closest == second_closest {
                            assert!(r.div_round(y, RoundingMode::Exact).even());
                        }
                    }
                }
            }
        },
    );

    test_properties(pairs_of_natural_and_positive_natural, |&(ref x, ref y)| {
        let product = x * y;
        assert_eq!((&product).round_to_multiple(y, RoundingMode::Down), product);
        assert_eq!((&product).round_to_multiple(y, RoundingMode::Up), product);
        assert_eq!(
            (&product).round_to_multiple(y, RoundingMode::Floor),
            product
        );
        assert_eq!(
            (&product).round_to_multiple(y, RoundingMode::Ceiling),
            product
        );
        assert_eq!(
            (&product).round_to_multiple(y, RoundingMode::Nearest),
            product
        );
        assert_eq!(
            (&product).round_to_multiple(y, RoundingMode::Exact),
            product
        );
    });

    test_properties(
        pairs_of_natural_and_positive_natural_var_2,
        |&(ref x, ref y)| {
            let down = x.round_to_multiple(y, RoundingMode::Down);
            let up = &down + y;
            assert_eq!(x.round_to_multiple(y, RoundingMode::Up), up);
            assert_eq!(x.round_to_multiple(y, RoundingMode::Floor), down);
            assert_eq!(x.round_to_multiple(y, RoundingMode::Ceiling), up);
            let nearest = x.round_to_multiple(y, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        },
    );

    test_properties(pairs_of_natural_and_rounding_mode, |&(ref x, rm)| {
        assert_eq!(x.round_to_multiple(Natural::ONE, rm), *x);
        assert_eq!(Natural::ZERO.round_to_multiple(x, rm), 0);
        assert_eq!(x.round_to_multiple(x, rm), *x);
    });

    test_properties(
        triples_of_unsigned_unsigned_and_rounding_mode_var_1::<Limb>,
        |&(x, y, rm)| {
            assert_eq!(
                Natural::from(x).round_to_multiple(Natural::from(y), rm),
                x.round_to_multiple(y, rm)
            );
        },
    );
}
