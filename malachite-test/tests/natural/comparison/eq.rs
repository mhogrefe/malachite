use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{natural_to_biguint, natural_to_rug_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};

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

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(Natural::from(x) == Natural::from(y), x == y);
    });
}
