use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_nz_test_util::integer::arithmetic::divisible_by::num_divisible_by;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_integer_var_1,
    pairs_of_integer_and_nonzero_integer_var_2, pairs_of_integers,
};
use malachite_test::inputs::natural::pairs_of_naturals;

#[test]
fn divisible_by_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(x.divisible_by(y.clone()), divisible);
        assert_eq!(x.clone().divisible_by(y), divisible);
        assert_eq!(x.clone().divisible_by(y.clone()), divisible);

        assert_eq!(*x == 0 || *y != 0 && x % y == 0, divisible);
        assert_eq!((-x).divisible_by(y), divisible);
        assert_eq!(x.divisible_by(-y), divisible);
        assert_eq!(
            num_divisible_by(&integer_to_bigint(x), &integer_to_bigint(y)),
            divisible
        );
        assert_eq!(
            integer_to_rug_integer(x).is_divisible(&integer_to_rug_integer(y)),
            divisible
        );
    });

    test_properties(
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            assert!(x.divisible_by(y));
            assert!(*x == 0 || *y != 0 && x % y == 0);
            assert!(num_divisible_by(
                &integer_to_bigint(x),
                &integer_to_bigint(y)
            ));
            assert!(integer_to_rug_integer(x).is_divisible(&integer_to_rug_integer(y)));
        },
    );

    test_properties(
        pairs_of_integer_and_nonzero_integer_var_2,
        |&(ref x, ref y)| {
            assert!(!x.divisible_by(y));
            assert!(*x != 0 && (*y == 0 || x % y != 0));
            assert!(!num_divisible_by(
                &integer_to_bigint(x),
                &integer_to_bigint(y)
            ));
            assert!(!integer_to_rug_integer(x).is_divisible(&integer_to_rug_integer(y)));
        },
    );

    test_properties(integers, |n| {
        assert!(n.divisible_by(Integer::ONE));
        assert!(n.divisible_by(Integer::NEGATIVE_ONE));
    });

    test_properties(nonzero_integers, |n| {
        assert!(!n.divisible_by(Integer::ZERO));
        assert!(Integer::ZERO.divisible_by(n));
        if *n > 1 {
            assert!(!Integer::ONE.divisible_by(n));
        }
        assert!(n.divisible_by(n));
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(
            Integer::from(x).divisible_by(Integer::from(y)),
            x.divisible_by(y)
        );
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(
            Integer::from(x).divisible_by(Integer::from(y)),
            x.divisible_by(y)
        );
    });
}
