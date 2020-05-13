use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_nz::integer::Integer;

use malachite_test::common::test_properties;
use malachite_test::common::{integer_to_rug_integer, natural_to_rug_integer};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_natural, triples_of_integer_natural_and_integer,
    triples_of_natural_integer_and_natural,
};
use malachite_test::inputs::natural::pairs_of_naturals;

#[test]
fn partial_cmp_integer_natural_properties() {
    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        let cmp = x.partial_cmp_abs(y);
        assert_eq!(x.cmp_abs(&y.into()), cmp.unwrap());
        assert_eq!(
            Some(integer_to_rug_integer(x).cmp_abs(&natural_to_rug_integer(y))),
            cmp
        );
        assert_eq!(y.partial_cmp_abs(x), cmp.map(|o| o.reverse()));
    });

    test_properties(
        triples_of_integer_natural_and_integer,
        |&(ref x, ref y, ref z)| {
            if x.lt_abs(y) && y.lt_abs(z) {
                assert!(x.lt_abs(z));
            } else if x.gt_abs(y) && y.gt_abs(z) {
                assert!(x.gt_abs(z));
            }
        },
    );

    test_properties(
        triples_of_natural_integer_and_natural,
        |&(ref x, ref y, ref z)| {
            if x.lt_abs(y) && y.lt_abs(z) {
                assert!(x < z);
            } else if x.gt_abs(y) && y.gt_abs(z) {
                assert!(x > z);
            }
        },
    );

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x).partial_cmp_abs(y), Some(x.cmp(y)));
        assert_eq!(x.partial_cmp_abs(&Integer::from(y)), Some(x.cmp(y)));
    });
}
