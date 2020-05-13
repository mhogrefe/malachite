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
        let cmp = x.partial_cmp(y);
        assert_eq!(x.cmp(&y.into()), cmp.unwrap());
        assert_eq!(
            integer_to_rug_integer(x).partial_cmp(&natural_to_rug_integer(y)),
            cmp
        );
        assert_eq!(y.partial_cmp(x), cmp.map(|o| o.reverse()));
    });

    test_properties(
        triples_of_integer_natural_and_integer,
        |&(ref x, ref y, ref z)| {
            if x < y && y < z {
                assert!(x < z);
            } else if x > y && y > z {
                assert!(x > z);
            }
        },
    );

    test_properties(
        triples_of_natural_integer_and_natural,
        |&(ref x, ref y, ref z)| {
            if x < y && y < z {
                assert!(x < z);
            } else if x > y && y > z {
                assert!(x > z);
            }
        },
    );

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x).partial_cmp(y), Some(x.cmp(y)));
        assert_eq!(x.partial_cmp(&Integer::from(y)), Some(x.cmp(y)));
    });
}
