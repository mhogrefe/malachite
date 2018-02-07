use common::test_properties;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::natural::{pairs_of_natural_and_unsigned,
                                      triples_of_natural_unsigned_and_natural,
                                      triples_of_unsigned_natural_and_unsigned};
use malachite_test::natural::comparison::partial_ord_u32::num_partial_cmp_u32;
use num::BigUint;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(num_partial_cmp_u32(&BigUint::from_str(u).unwrap(), v), out);
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
fn partial_cmp_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let cmp = n.partial_cmp(&u);
            assert_eq!(num_partial_cmp_u32(&natural_to_biguint(n), u), cmp);
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
        |&(ref n, u, ref m): &(Natural, u32, Natural)| {
            if *n < u && u < *m {
                assert!(*n < *m);
            } else if *n > u && u > *m {
                assert!(*n > *m);
            }
        },
    );

    test_properties(
        triples_of_unsigned_natural_and_unsigned,
        |&(u, ref n, v): &(u32, Natural, u32)| {
            if u < *n && *n < v {
                assert!(u < v);
            } else if u > *n && *n > v {
                assert!(u > v);
            }
        },
    );
}
