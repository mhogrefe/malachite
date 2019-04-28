use std::cmp::Ordering;
use std::str::FromStr;

use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use common::test_properties;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::natural::{
    pairs_of_natural_and_unsigned, triples_of_natural_unsigned_and_natural,
    triples_of_unsigned_natural_and_unsigned,
};
use malachite_test::natural::comparison::partial_ord_limb::num_partial_cmp_limb;

#[test]
fn test_partial_ord_limb() {
    let test = |u, v: Limb, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(num_partial_cmp_limb(&BigUint::from_str(u).unwrap(), v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Natural::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
}

#[test]
fn partial_cmp_limb_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, Limb)| {
            let cmp = n.partial_cmp(&u);
            assert_eq!(num_partial_cmp_limb(&natural_to_biguint(n), u), cmp);
            assert_eq!(natural_to_rug_integer(n).partial_cmp(&u), cmp);
            assert_eq!(n.partial_cmp(&Natural::from(u)), cmp);

            let cmp_rev = cmp.map(|o| o.reverse());
            assert_eq!(u.partial_cmp(n), cmp_rev);
            assert_eq!(u.partial_cmp(&natural_to_rug_integer(n)), cmp_rev);
            assert_eq!(Natural::from(u).partial_cmp(n), cmp_rev);
        },
    );

    test_properties(
        triples_of_natural_unsigned_and_natural,
        |&(ref n, u, ref m): &(Natural, Limb, Natural)| {
            if *n < u && u < *m {
                assert!(*n < *m);
            } else if *n > u && u > *m {
                assert!(*n > *m);
            }
        },
    );

    test_properties(
        triples_of_unsigned_natural_and_unsigned,
        |&(u, ref n, v): &(Limb, Natural, Limb)| {
            if u < *n && *n < v {
                assert!(u < v);
            } else if u > *n && *n > v {
                assert!(u > v);
            }
        },
    );

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(Natural::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Natural::from(y)), Some(x.cmp(&y)));
    });
}
