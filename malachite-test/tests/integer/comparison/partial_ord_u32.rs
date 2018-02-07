use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_test::inputs::integer::{pairs_of_integer_and_unsigned,
                                      triples_of_integer_unsigned_and_integer,
                                      triples_of_unsigned_integer_and_unsigned};
use malachite_test::integer::comparison::partial_ord_u32::num_partial_cmp_u32;
use num::BigInt;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(num_partial_cmp_u32(&BigInt::from_str(u).unwrap(), v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
        assert_eq!(
            v.partial_cmp(&rug::Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("3000000000", 3_000_000_000, Some(Ordering::Equal));
    test("3000000000", 3_000_000_001, Some(Ordering::Less));
    test("3000000000", 2_999_999_999, Some(Ordering::Greater));
}

#[test]
fn partial_cmp_u32_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            let cmp = n.partial_cmp(&u);
            assert_eq!(num_partial_cmp_u32(&integer_to_bigint(n), u), cmp);
            assert_eq!(integer_to_rug_integer(n).partial_cmp(&u), cmp);
            assert_eq!(n.partial_cmp(&Integer::from(u)), cmp);

            let cmp_rev = cmp.map(|o| o.reverse());
            assert_eq!(u.partial_cmp(n), cmp_rev);
            assert_eq!(u.partial_cmp(&integer_to_rug_integer(n)), cmp_rev);
            assert_eq!(Integer::from(u).partial_cmp(n), cmp_rev);
        },
    );

    test_properties(
        triples_of_integer_unsigned_and_integer,
        |&(ref n, u, ref m): &(Integer, u32, Integer)| {
            if *n < u && u < *m {
                assert!(n < m);
            } else if *n > u && u > *m {
                assert!(n > m);
            }
        },
    );

    test_properties(
        triples_of_unsigned_integer_and_unsigned,
        |&(u, ref n, v): &(u32, Integer, u32)| {
            if u < *n && *n < v {
                assert!(u < v);
            } else if u > *n && *n > v {
                assert!(u > v);
            }
        },
    );
}
