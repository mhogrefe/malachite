use common::{test_eq_helper, test_properties};
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};
use num::BigUint;
use rug;

#[test]
fn test_eq() {
    let strings = vec!["0", "1", "2", "123", "1000000000000"];
    test_eq_helper::<Natural>(&strings);
    test_eq_helper::<BigUint>(&strings);
    test_eq_helper::<rug::Integer>(&strings);
}

#[test]
fn eq_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let eq = *x == *y;
        assert_eq!(natural_to_biguint(x) == natural_to_biguint(y), eq);
        assert_eq!(natural_to_rug_integer(x) == natural_to_rug_integer(y), eq);
        assert_eq!(*y == *x, eq);
    });

    test_properties(naturals, |x| {
        assert_eq!(*x, *x);
    });

    test_properties(triples_of_naturals, |&(ref x, ref y, ref z)| {
        if *x == *y && *x == *z {
            assert_eq!(*x, *z);
        }
    });
}
