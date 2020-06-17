use std::cmp::Ordering;

use malachite_base::num::comparison::traits::OrdAbs;

#[test]
pub fn test_cmp_abs_partial_cmp_abs_and_comparators() {
    fn test<T: Copy + OrdAbs>(x: T, y: T, cmp: Ordering, lt: bool, gt: bool, le: bool, ge: bool) {
        assert_eq!(x.cmp_abs(&y), cmp);
        assert_eq!(x.partial_cmp_abs(&y), Some(cmp));
        assert_eq!(lt, x.lt_abs(&y));
        assert_eq!(gt, x.gt_abs(&y));
        assert_eq!(le, x.le_abs(&y));
        assert_eq!(ge, x.ge_abs(&y));
    };
    test(123u16, 123u16, Ordering::Equal, false, false, true, true);
    test(123u16, 456u16, Ordering::Less, true, false, true, false);
    test(456u16, 123u16, Ordering::Greater, false, true, false, true);

    test(123i64, 123i64, Ordering::Equal, false, false, true, true);
    test(123i64, 456i64, Ordering::Less, true, false, true, false);
    test(456i64, 123i64, Ordering::Greater, false, true, false, true);

    test(123i64, -123i64, Ordering::Equal, false, false, true, true);
    test(123i64, -456i64, Ordering::Less, true, false, true, false);
    test(456i64, -123i64, Ordering::Greater, false, true, false, true);

    test(-123i64, 123i64, Ordering::Equal, false, false, true, true);
    test(-123i64, 456i64, Ordering::Less, true, false, true, false);
    test(-456i64, 123i64, Ordering::Greater, false, true, false, true);

    test(-123i64, -123i64, Ordering::Equal, false, false, true, true);
    test(-123i64, -456i64, Ordering::Less, true, false, true, false);
    test(
        -456i64,
        -123i64,
        Ordering::Greater,
        false,
        true,
        false,
        true,
    );
}
