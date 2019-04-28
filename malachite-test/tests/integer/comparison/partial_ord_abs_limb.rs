use common::test_properties;
use malachite_base::num::traits::{Abs, PartialOrdAbs};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::integer::{
    pairs_of_integer_and_unsigned, triples_of_integer_unsigned_and_integer,
    triples_of_unsigned_integer_and_unsigned,
};
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_limb_abs() {
    let test = |u, v: Limb, out| {
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
fn partial_cmp_limb_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            let cmp = n.partial_cmp_abs(&u);
            assert_eq!(n.partial_cmp_abs(&Integer::from(u)), cmp);
            assert_eq!(n.abs().partial_cmp(&u), cmp);

            let cmp_rev = cmp.map(|o| o.reverse());
            assert_eq!(PartialOrdAbs::partial_cmp_abs(&u, n), cmp_rev);
            assert_eq!(Integer::from(u).partial_cmp_abs(n), cmp_rev);
            assert_eq!(u.partial_cmp(&n.abs()), cmp_rev);
        },
    );

    test_properties(
        triples_of_integer_unsigned_and_integer,
        |&(ref n, u, ref m): &(Integer, Limb, Integer)| {
            if n.lt_abs(&u) && PartialOrdAbs::lt_abs(&u, m) {
                assert!(n.lt_abs(m));
            } else if n.gt_abs(&u) && PartialOrdAbs::gt_abs(&u, m) {
                assert!(n.gt_abs(m));
            }
        },
    );

    test_properties(
        triples_of_unsigned_integer_and_unsigned,
        |&(u, ref n, v): &(Limb, Integer, Limb)| {
            if PartialOrdAbs::lt_abs(&u, n) && n.lt_abs(&v) {
                assert!(u < v);
            } else if PartialOrdAbs::gt_abs(&u, n) && n.gt_abs(&v) {
                assert!(u > v);
            }
        },
    );

    test_properties(pairs_of_natural_and_unsigned::<Limb>, |&(ref x, y)| {
        assert_eq!(Integer::from(x).partial_cmp_abs(&y), x.partial_cmp(&y));
        assert_eq!(x.partial_cmp_abs(&Integer::from(y)), x.partial_cmp(&y));
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(Integer::from(x).partial_cmp_abs(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp_abs(&Integer::from(y)), Some(x.cmp(&y)));
    });
}
