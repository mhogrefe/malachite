use malachite_base::num::arithmetic::traits::{
    Abs, DivRound, DivisibleBy, Parity, RoundToMultiple, RoundToMultipleAssign,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::triples_of_signed_signed_and_rounding_mode_var_1;
use malachite_test::inputs::integer::{
    pairs_of_integer_and_nonzero_integer, pairs_of_integer_and_nonzero_integer_var_2,
    pairs_of_integer_and_rounding_mode, triples_of_integer_integer_and_rounding_mode_var_2,
};
use malachite_test::inputs::natural::triples_of_natural_natural_and_rounding_mode_var_2;

#[test]
fn round_to_multiple_properties() {
    test_properties(
        triples_of_integer_integer_and_rounding_mode_var_2,
        |&(ref x, ref y, rm): &(Integer, Integer, RoundingMode)| {
            let mut mut_n = x.clone();
            mut_n.round_to_multiple_assign(y, rm);
            assert!(mut_n.is_valid());
            let r = mut_n;

            let mut mut_n = x.clone();
            mut_n.round_to_multiple_assign(y.clone(), rm);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, r);

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

            assert_eq!(-(-x).round_to_multiple(y, -rm), r);
            assert_eq!(x.round_to_multiple(-y, rm), r);
            assert!((&r).divisible_by(y));
            if *y == 0 {
                assert_eq!(r, 0);
            } else {
                assert!((&r - x).le_abs(y));
                match rm {
                    RoundingMode::Floor => assert!(r <= *x),
                    RoundingMode::Ceiling => assert!(r >= *x),
                    RoundingMode::Down => assert!(r.le_abs(x)),
                    RoundingMode::Up => assert!(r.ge_abs(x)),
                    RoundingMode::Exact => assert_eq!(r, *x),
                    RoundingMode::Nearest => {
                        let closest;
                        let second_closest;
                        if r <= *x {
                            closest = x - &r;
                            second_closest = &r + y.abs() - x;
                        } else {
                            closest = &r - x;
                            second_closest = x + y.abs() - &r;
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

    test_properties(pairs_of_integer_and_nonzero_integer, |&(ref x, ref y)| {
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
        pairs_of_integer_and_nonzero_integer_var_2,
        |&(ref x, ref y)| {
            let down = x.round_to_multiple(y, RoundingMode::Down);
            let up = if *x >= 0 {
                &down + y.abs()
            } else {
                &down - y.abs()
            };
            let floor = x.round_to_multiple(y, RoundingMode::Floor);
            let ceiling = &floor + y.abs();
            assert_eq!(x.round_to_multiple(y, RoundingMode::Up), up);
            assert_eq!(x.round_to_multiple(y, RoundingMode::Ceiling), ceiling);
            let nearest = x.round_to_multiple(y, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        },
    );

    test_properties(pairs_of_integer_and_rounding_mode, |&(ref x, rm)| {
        assert_eq!(x.round_to_multiple(Integer::ONE, rm), *x);
        assert_eq!(x.round_to_multiple(Integer::NEGATIVE_ONE, rm), *x);
        assert_eq!(Integer::ZERO.round_to_multiple(x, rm), 0);
        assert_eq!(x.round_to_multiple(x, rm), *x);
        assert_eq!(x.round_to_multiple(-x, rm), *x);
        assert_eq!((-x).round_to_multiple(x, rm), -x);
    });

    test_properties(
        triples_of_natural_natural_and_rounding_mode_var_2,
        |&(ref x, ref y, rm)| {
            assert_eq!(
                Integer::from(x).round_to_multiple(Integer::from(y), rm),
                x.round_to_multiple(y, rm)
            );
        },
    );

    test_properties(
        triples_of_signed_signed_and_rounding_mode_var_1::<SignedLimb>,
        |&(x, y, rm)| {
            assert_eq!(
                Integer::from(x).round_to_multiple(Integer::from(y), rm),
                x.round_to_multiple(y, rm)
            );
        },
    );
}
