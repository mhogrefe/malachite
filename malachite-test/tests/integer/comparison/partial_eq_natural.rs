use malachite_nz::integer::Integer;
use malachite_nz_test_util::common::{integer_to_rug_integer, natural_to_rug_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::integer::pairs_of_integer_and_natural;
use malachite_test::inputs::natural::pairs_of_naturals;

#[test]
fn partial_eq_natural_properties() {
    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        let eq = x == y;
        assert_eq!(y == x, eq);
        assert_eq!(*x == Integer::from(y), eq);
        assert_eq!(integer_to_rug_integer(x) == natural_to_rug_integer(y), eq);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x) == *y, x == y);
        assert_eq!(*x == Integer::from(y), x == y);
    });
}
