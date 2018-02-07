use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_test::inputs::integer::{pairs_of_integer_and_signed,
                                      triples_of_integer_signed_and_integer,
                                      triples_of_signed_integer_and_signed};
use malachite_test::integer::comparison::partial_ord_i32::num_partial_cmp_i32;
use num::BigInt;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(num_partial_cmp_i32(&BigInt::from_str(u).unwrap(), v), out);
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
    test("-123", -123, Some(Ordering::Equal));
    test("-123", -122, Some(Ordering::Less));
    test("-123", -124, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("-1000000000000", -123, Some(Ordering::Less));
}

#[test]
fn partial_cmp_i32_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, i32)| {
            let cmp = n.partial_cmp(&i);
            assert_eq!(num_partial_cmp_i32(&integer_to_bigint(n), i), cmp);
            assert_eq!(integer_to_rug_integer(n).partial_cmp(&i), cmp);
            assert_eq!(n.partial_cmp(&Integer::from(i)), cmp);

            let cmp_rev = cmp.map(|o| o.reverse());
            assert_eq!(i.partial_cmp(n), cmp_rev);
            assert_eq!(i.partial_cmp(&integer_to_rug_integer(n)), cmp_rev);
            assert_eq!(Integer::from(i).partial_cmp(n), cmp_rev);
        },
    );

    test_properties(
        triples_of_integer_signed_and_integer,
        |&(ref n, i, ref m): &(Integer, i32, Integer)| {
            if *n < i && i < *m {
                assert!(n < m);
            } else if *n > i && i > *m {
                assert!(n > m);
            }
        },
    );

    test_properties(
        triples_of_signed_integer_and_signed,
        |&(i, ref n, j): &(i32, Integer, i32)| {
            if i < *n && *n < j {
                assert!(i < j);
            } else if i > *n && *n > j {
                assert!(i > j);
            }
        },
    );
}
