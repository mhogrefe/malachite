use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{
    pairs_of_integer_and_signed, triples_of_integer_signed_and_integer,
    triples_of_signed_integer_and_signed,
};

#[test]
fn test_partial_ord_signed_limb_abs() {
    let test = |u, v: SignedLimb, out| {
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
    test("-123", -123, Some(Ordering::Equal));
    test("-123", -122, Some(Ordering::Greater));
    test("-123", -124, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", -123, Some(Ordering::Greater));
}

#[test]
fn partial_cmp_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, SignedLimb)| {
            let cmp = n.partial_cmp_abs(&i);
            assert_eq!(n.partial_cmp_abs(&Integer::from(i)), cmp);
            assert_eq!(n.abs().partial_cmp(&(i.wrapping_abs() as Limb)), cmp);
            assert_eq!(
                PartialOrdAbs::partial_cmp_abs(&i, n),
                cmp.map(|o| o.reverse())
            );
        },
    );

    test_properties(
        triples_of_integer_signed_and_integer,
        |&(ref n, i, ref m): &(Integer, SignedLimb, Integer)| {
            if n.lt_abs(&i) && PartialOrdAbs::lt_abs(&i, m) {
                assert!(n.lt_abs(m));
            } else if n.gt_abs(&i) && PartialOrdAbs::gt_abs(&i, m) {
                assert!(n.gt_abs(m));
            }
        },
    );

    test_properties(
        triples_of_signed_integer_and_signed,
        |&(i, ref n, j): &(SignedLimb, Integer, SignedLimb)| {
            if PartialOrdAbs::lt_abs(&i, n) && n.lt_abs(&j) {
                assert!((i.wrapping_abs() as Limb) < (j.wrapping_abs() as Limb));
            } else if PartialOrdAbs::gt_abs(&i, n) && n.gt_abs(&j) {
                assert!((i.wrapping_abs() as Limb) > (j.wrapping_abs() as Limb));
            }
        },
    );

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(Integer::from(x).partial_cmp_abs(&y), Some(x.cmp_abs(&y)));
        assert_eq!(x.partial_cmp_abs(&Integer::from(y)), Some(x.cmp_abs(&y)));
    });
}
