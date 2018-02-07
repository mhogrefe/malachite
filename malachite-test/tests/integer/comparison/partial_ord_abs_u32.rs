use common::test_properties;
use malachite_base::num::PartialOrdAbs;
use malachite_nz::integer::Integer;
use malachite_test::inputs::integer::{pairs_of_integer_and_unsigned,
                                      triples_of_integer_unsigned_and_integer,
                                      triples_of_unsigned_integer_and_unsigned};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_u32_abs() {
    let test = |u, v: u32, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp_abs(&v), out);

        assert_eq!(
            PartialOrdAbs::partial_cmp_abs(&v, &Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Equal));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Greater));
    test("3000000000", 3_000_000_000, Some(Ordering::Equal));
    test("3000000000", 3_000_000_001, Some(Ordering::Less));
    test("3000000000", 2_999_999_999, Some(Ordering::Greater));
}

#[test]
fn partial_cmp_u32_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            let cmp = n.partial_cmp_abs(&u);
            assert_eq!(n.partial_cmp_abs(&Integer::from(u)), cmp);
            assert_eq!(n.abs_ref().partial_cmp(&u), cmp);

            let cmp_rev = cmp.map(|o| o.reverse());
            assert_eq!(PartialOrdAbs::partial_cmp_abs(&u, n), cmp_rev);
            assert_eq!(Integer::from(u).partial_cmp_abs(n), cmp_rev);
            assert_eq!(u.partial_cmp(&n.abs_ref()), cmp_rev);
        },
    );

    test_properties(
        triples_of_integer_unsigned_and_integer,
        |&(ref n, u, ref m): &(Integer, u32, Integer)| {
            if n.lt_abs(&u) && PartialOrdAbs::lt_abs(&u, m) {
                assert!(n.lt_abs(m));
            } else if n.gt_abs(&u) && PartialOrdAbs::gt_abs(&u, m) {
                assert!(n.gt_abs(m));
            }
        },
    );

    test_properties(
        triples_of_unsigned_integer_and_unsigned,
        |&(u, ref n, v): &(u32, Integer, u32)| {
            if PartialOrdAbs::lt_abs(&u, n) && n.lt_abs(&v) {
                assert!(u < v);
            } else if PartialOrdAbs::gt_abs(&u, n) && n.gt_abs(&v) {
                assert!(u > v);
            }
        },
    );
}
