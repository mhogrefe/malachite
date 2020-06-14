use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, DivRound};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{integer_to_rug_integer, rug_integer_to_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_signeds_var_4;
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_integer_var_1,
};
use malachite_test::inputs::natural::pairs_of_natural_and_positive_natural_var_1;

#[test]
fn div_exact_properties() {
    test_properties(
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            let mut mut_x = x.clone();
            mut_x.div_exact_assign(y);
            assert!(mut_x.is_valid());
            let q = mut_x;

            let mut mut_x = x.clone();
            mut_x.div_exact_assign(y.clone());
            assert!(mut_x.is_valid());
            assert_eq!(mut_x, q);

            let q_alt = x.div_exact(y);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.div_exact(y.clone());
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.clone().div_exact(y);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.clone().div_exact(y.clone());
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            let q_alt = x.div_round(y, RoundingMode::Exact);
            assert_eq!(q_alt, q);

            assert_eq!(
                rug_integer_to_integer(
                    &integer_to_rug_integer(x).div_exact(&integer_to_rug_integer(y))
                ),
                q
            );

            assert_eq!(&q * y, *x);
            assert_eq!((-x).div_exact(y), -&q);
            assert_eq!(x.div_exact(-y), -q);
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.div_exact(Integer::ONE), *n);
        assert_eq!(n.div_exact(Integer::NEGATIVE_ONE), -n);
    });

    test_properties(nonzero_integers, |n| {
        assert_eq!(Integer::ZERO.div_exact(n), 0);
        assert_eq!(n.div_exact(n), 1);
    });

    test_properties(
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            assert_eq!(Integer::from(x).div_exact(Integer::from(y)), x.div_exact(y));
        },
    );

    test_properties(pairs_of_signeds_var_4::<SignedLimb>, |&(x, y)| {
        assert_eq!(Integer::from(x).div_exact(Integer::from(y)), x.div_exact(y));
    });
}
