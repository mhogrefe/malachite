use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{integer_to_bigint, integer_to_rug_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use malachite_test::inputs::natural::pairs_of_naturals;

#[test]
fn eq_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let eq = x == y;
        assert_eq!(integer_to_bigint(x) == integer_to_bigint(y), eq);
        assert_eq!(integer_to_rug_integer(x) == integer_to_rug_integer(y), eq);
        assert_eq!(y == x, eq);
    });

    test_properties(integers, |x| {
        assert_eq!(x, x);
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        if x == y && x == z {
            assert_eq!(x, z);
        }
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x) == Integer::from(y), x == y);
        assert_eq!(Integer::from(x) == *y, x == y);
        assert_eq!(*x == Integer::from(y), x == y);
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(Integer::from(x) == Integer::from(y), x == y);
    });
}
