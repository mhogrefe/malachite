use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_integer_natural() {
    let test = |s, t, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        let u = Integer::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert_eq!(u.partial_cmp_abs(&v), cmp);
        assert_eq!(v.partial_cmp_abs(&u).map(Ordering::reverse), cmp);
        assert_eq!(lt, u.lt_abs(&v));
        assert_eq!(gt, u.gt_abs(&v));
        assert_eq!(le, u.le_abs(&v));
        assert_eq!(ge, u.ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&u));
        assert_eq!(gt, v.lt_abs(&u));
        assert_eq!(le, v.ge_abs(&u));
        assert_eq!(ge, v.le_abs(&u));
    };
    test("0", "0", Some(Ordering::Equal), false, false, true, true);
    test("0", "5", Some(Ordering::Less), true, false, true, false);
    test(
        "123",
        "123",
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test("123", "124", Some(Ordering::Less), true, false, true, false);
    test(
        "123",
        "122",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        "123",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "123",
        "1000000000000",
        Some(Ordering::Less),
        true,
        false,
        true,
        false,
    );
    test(
        "1000000000000",
        "1000000000000",
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "-1000000000000",
        "1000000000000",
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "-1000000000000",
        "0",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
}
