use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::num::comparison::traits::PartialOrdAbs;

use malachite_nz::natural::Natural;

#[test]
fn test_partial_cmp_abs_u32() {
    let test = |u, v: u32, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp_abs(&v), cmp);
        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
            cmp.map(Ordering::reverse)
        );
        assert_eq!(lt, Natural::from_str(u).unwrap().lt_abs(&v));
        assert_eq!(gt, Natural::from_str(u).unwrap().gt_abs(&v));
        assert_eq!(le, Natural::from_str(u).unwrap().le_abs(&v));
        assert_eq!(ge, Natural::from_str(u).unwrap().ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(gt, v.lt_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(le, v.ge_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(ge, v.le_abs(&Natural::from_str(u).unwrap()));
    };
    test("0", 0, Some(Ordering::Equal), false, false, true, true);
    test("0", 5, Some(Ordering::Less), true, false, true, false);
    test("123", 123, Some(Ordering::Equal), false, false, true, true);
    test("123", 124, Some(Ordering::Less), true, false, true, false);
    test(
        "123",
        122,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        123,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
}

#[test]
fn test_partial_cmp_abs_u64() {
    let test = |u, v: u64, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp_abs(&v), cmp);
        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
            cmp.map(Ordering::reverse)
        );
        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
            cmp.map(Ordering::reverse)
        );
        assert_eq!(lt, Natural::from_str(u).unwrap().lt_abs(&v));
        assert_eq!(gt, Natural::from_str(u).unwrap().gt_abs(&v));
        assert_eq!(le, Natural::from_str(u).unwrap().le_abs(&v));
        assert_eq!(ge, Natural::from_str(u).unwrap().ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(gt, v.lt_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(le, v.ge_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(ge, v.le_abs(&Natural::from_str(u).unwrap()));
    };
    test("0", 0, Some(Ordering::Equal), false, false, true, true);
    test("0", 5, Some(Ordering::Less), true, false, true, false);
    test("123", 123, Some(Ordering::Equal), false, false, true, true);
    test("123", 124, Some(Ordering::Less), true, false, true, false);
    test(
        "123",
        122,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        123,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        1000000000000,
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "1000000000000",
        1000000000001,
        Some(Ordering::Less),
        true,
        false,
        true,
        false,
    );
}

#[test]
fn test_partial_cmp_abs_i32() {
    let test = |u, v: i32, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp_abs(&v), cmp);
        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
            cmp.map(Ordering::reverse)
        );
        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
            cmp.map(Ordering::reverse)
        );
        assert_eq!(lt, Natural::from_str(u).unwrap().lt_abs(&v));
        assert_eq!(gt, Natural::from_str(u).unwrap().gt_abs(&v));
        assert_eq!(le, Natural::from_str(u).unwrap().le_abs(&v));
        assert_eq!(ge, Natural::from_str(u).unwrap().ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(gt, v.lt_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(le, v.ge_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(ge, v.le_abs(&Natural::from_str(u).unwrap()));
    };
    test("0", 0, Some(Ordering::Equal), false, false, true, true);
    test("0", 5, Some(Ordering::Less), true, false, true, false);
    test("0", -5, Some(Ordering::Less), true, false, true, false);
    test("123", 123, Some(Ordering::Equal), false, false, true, true);
    test("123", -123, Some(Ordering::Equal), false, false, true, true);
    test("123", 124, Some(Ordering::Less), true, false, true, false);
    test("123", -124, Some(Ordering::Less), true, false, true, false);
    test(
        "123",
        122,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "123",
        -122,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        123,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        -123,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
}

#[test]
fn test_partial_cmp_abs_i64() {
    let test = |u, v: i64, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp_abs(&v), cmp);
        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
            cmp.map(Ordering::reverse)
        );
        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
            cmp.map(Ordering::reverse)
        );
        assert_eq!(lt, Natural::from_str(u).unwrap().lt_abs(&v));
        assert_eq!(gt, Natural::from_str(u).unwrap().gt_abs(&v));
        assert_eq!(le, Natural::from_str(u).unwrap().le_abs(&v));
        assert_eq!(ge, Natural::from_str(u).unwrap().ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(gt, v.lt_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(le, v.ge_abs(&Natural::from_str(u).unwrap()));
        assert_eq!(ge, v.le_abs(&Natural::from_str(u).unwrap()));
    };
    test("0", 0, Some(Ordering::Equal), false, false, true, true);
    test("0", 5, Some(Ordering::Less), true, false, true, false);
    test("0", -5, Some(Ordering::Less), true, false, true, false);
    test("123", 123, Some(Ordering::Equal), false, false, true, true);
    test("123", -123, Some(Ordering::Equal), false, false, true, true);
    test("123", 124, Some(Ordering::Less), true, false, true, false);
    test("123", -124, Some(Ordering::Less), true, false, true, false);
    test(
        "123",
        122,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "123",
        -122,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        123,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        -123,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        1000000000000,
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "1000000000000",
        -1000000000000,
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "1000000000000",
        1000000000001,
        Some(Ordering::Less),
        true,
        false,
        true,
        false,
    );
    test(
        "1000000000000",
        -1000000000001,
        Some(Ordering::Less),
        true,
        false,
        true,
        false,
    );
}
