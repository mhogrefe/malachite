use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;
use malachite_test::inputs::natural::naturals;

#[test]
fn neg_properties() {
    test_properties(integers, |x| {
        let negative = -x;
        assert!(negative.is_valid());
        assert!(negative.is_valid());

        let negative_alt = -x.clone();
        assert!(negative_alt.is_valid());
        assert_eq!(negative_alt, negative);

        assert_eq!(bigint_to_integer(&(-integer_to_bigint(x))), negative);
        assert_eq!(
            rug_integer_to_integer(&(-integer_to_rug_integer(x))),
            negative
        );

        assert_eq!(negative == *x, *x == 0);
        assert_eq!(-&negative, *x);
        assert_eq!(x + negative, 0);
    });

    test_properties(signeds::<SignedLimb>, |&x| {
        assert_eq!(Integer::from(-i64::from(x)), -Integer::from(x));
    });

    test_properties(naturals, |x| {
        assert_eq!(-x, -Integer::from(x));
    });
}
