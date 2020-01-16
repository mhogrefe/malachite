use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_nz::integer::Integer;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_signeds, pairs_of_unsigneds};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_signed, pairs_of_integer_and_unsigned,
    triples_of_integer_signed_and_integer, triples_of_integer_unsigned_and_integer,
    triples_of_signed_integer_and_signed, triples_of_unsigned_integer_and_unsigned,
};

#[test]
fn test_partial_cmp_abs_u32() {
    let test = |u, v: u32, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp_abs(&v), cmp);
        assert_eq!(
            v.partial_cmp_abs(&Integer::from_str(u).unwrap()),
            cmp.map(|o| o.reverse())
        );
        assert_eq!(lt, Integer::from_str(u).unwrap().lt_abs(&v));
        assert_eq!(gt, Integer::from_str(u).unwrap().gt_abs(&v));
        assert_eq!(le, Integer::from_str(u).unwrap().le_abs(&v));
        assert_eq!(ge, Integer::from_str(u).unwrap().ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(gt, v.lt_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(le, v.ge_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(ge, v.le_abs(&Integer::from_str(u).unwrap()));
    };
    test("0", 0, Some(Ordering::Equal), false, false, true, true);
    test("0", 5, Some(Ordering::Less), true, false, true, false);
    test("123", 123, Some(Ordering::Equal), false, false, true, true);
    test("-123", 123, Some(Ordering::Equal), false, false, true, true);
    test("123", 124, Some(Ordering::Less), true, false, true, false);
    test("-123", 124, Some(Ordering::Less), true, false, true, false);
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
        "-123",
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
        "-1000000000000",
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
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp_abs(&v), cmp);
        assert_eq!(
            v.partial_cmp_abs(&Integer::from_str(u).unwrap()),
            cmp.map(|o| o.reverse())
        );
        assert_eq!(lt, Integer::from_str(u).unwrap().lt_abs(&v));
        assert_eq!(gt, Integer::from_str(u).unwrap().gt_abs(&v));
        assert_eq!(le, Integer::from_str(u).unwrap().le_abs(&v));
        assert_eq!(ge, Integer::from_str(u).unwrap().ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(gt, v.lt_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(le, v.ge_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(ge, v.le_abs(&Integer::from_str(u).unwrap()));
    };
    test("0", 0, Some(Ordering::Equal), false, false, true, true);
    test("0", 5, Some(Ordering::Less), true, false, true, false);
    test("123", 123, Some(Ordering::Equal), false, false, true, true);
    test("-123", 123, Some(Ordering::Equal), false, false, true, true);
    test("123", 124, Some(Ordering::Less), true, false, true, false);
    test("-123", 124, Some(Ordering::Less), true, false, true, false);
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
        "-123",
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
        "-1000000000000",
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
        "-1000000000000",
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
    test(
        "-1000000000000",
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
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp_abs(&v), cmp);
        assert_eq!(
            v.partial_cmp_abs(&Integer::from_str(u).unwrap()),
            cmp.map(|o| o.reverse())
        );
        assert_eq!(lt, Integer::from_str(u).unwrap().lt_abs(&v));
        assert_eq!(gt, Integer::from_str(u).unwrap().gt_abs(&v));
        assert_eq!(le, Integer::from_str(u).unwrap().le_abs(&v));
        assert_eq!(ge, Integer::from_str(u).unwrap().ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(gt, v.lt_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(le, v.ge_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(ge, v.le_abs(&Integer::from_str(u).unwrap()));
    };
    test("0", 0, Some(Ordering::Equal), false, false, true, true);
    test("0", 5, Some(Ordering::Less), true, false, true, false);
    test("0", -5, Some(Ordering::Less), true, false, true, false);
    test("123", 123, Some(Ordering::Equal), false, false, true, true);
    test("123", -123, Some(Ordering::Equal), false, false, true, true);
    test("-123", 123, Some(Ordering::Equal), false, false, true, true);
    test(
        "-123",
        -123,
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test("123", 124, Some(Ordering::Less), true, false, true, false);
    test("123", -124, Some(Ordering::Less), true, false, true, false);
    test("-123", 124, Some(Ordering::Less), true, false, true, false);
    test("-123", -124, Some(Ordering::Less), true, false, true, false);
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
        "-123",
        122,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-123",
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
        "-1000000000000",
        123,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-1000000000000",
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
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp_abs(&v), cmp);
        assert_eq!(
            v.partial_cmp_abs(&Integer::from_str(u).unwrap()),
            cmp.map(|o| o.reverse())
        );
        assert_eq!(lt, Integer::from_str(u).unwrap().lt_abs(&v));
        assert_eq!(gt, Integer::from_str(u).unwrap().gt_abs(&v));
        assert_eq!(le, Integer::from_str(u).unwrap().le_abs(&v));
        assert_eq!(ge, Integer::from_str(u).unwrap().ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(gt, v.lt_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(le, v.ge_abs(&Integer::from_str(u).unwrap()));
        assert_eq!(ge, v.le_abs(&Integer::from_str(u).unwrap()));
    };
    test("0", 0, Some(Ordering::Equal), false, false, true, true);
    test("0", 5, Some(Ordering::Less), true, false, true, false);
    test("0", -5, Some(Ordering::Less), true, false, true, false);
    test("123", 123, Some(Ordering::Equal), false, false, true, true);
    test("123", -123, Some(Ordering::Equal), false, false, true, true);
    test("-123", 123, Some(Ordering::Equal), false, false, true, true);
    test(
        "-123",
        -123,
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test("123", 124, Some(Ordering::Less), true, false, true, false);
    test("123", -124, Some(Ordering::Less), true, false, true, false);
    test("-123", 124, Some(Ordering::Less), true, false, true, false);
    test("-123", -124, Some(Ordering::Less), true, false, true, false);
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
        "-123",
        122,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-123",
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
        "-1000000000000",
        123,
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-1000000000000",
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
        "-1000000000000",
        1000000000000,
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "-1000000000000",
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
    test(
        "-1000000000000",
        1000000000001,
        Some(Ordering::Less),
        true,
        false,
        true,
        false,
    );
    test(
        "-1000000000000",
        -1000000000001,
        Some(Ordering::Less),
        true,
        false,
        true,
        false,
    );
}

fn partial_cmp_abs_primitive_integer_properties_helper_unsigned<
    T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand,
>()
where
    Integer: From<T> + PartialOrdAbs<T>,
{
    test_properties(pairs_of_integer_and_unsigned::<T>, |&(ref n, u)| {
        let cmp = n.partial_cmp_abs(&u);
        assert_eq!(Some(n.cmp_abs(&Integer::from(u))), cmp);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(u.partial_cmp_abs(n), cmp_rev);
        assert_eq!(Some(Integer::from(u).cmp_abs(n)), cmp_rev);
    });

    test_properties(
        triples_of_integer_unsigned_and_integer::<T>,
        |&(ref n, u, ref m): &(Integer, T, Integer)| {
            if n.lt_abs(&u) && u.lt_abs(m) {
                assert_eq!(n.cmp_abs(m), Ordering::Less);
            } else if n.gt_abs(&u) && u.gt_abs(m) {
                assert_eq!(n.cmp_abs(m), Ordering::Greater);
            }
        },
    );

    test_properties(
        triples_of_unsigned_integer_and_unsigned::<T>,
        |&(u, ref n, v)| {
            if u.lt_abs(n) && n.lt_abs(&v) {
                assert!(u.lt_abs(&v));
            } else if u.gt_abs(n) && n.gt_abs(&v) {
                assert!(u.gt_abs(&v));
            }
        },
    );

    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        assert_eq!(Integer::from(x).partial_cmp_abs(&y), Some(x.cmp_abs(&y)));
        assert_eq!(x.partial_cmp_abs(&Integer::from(y)), Some(x.cmp_abs(&y)));
    });
}

fn partial_cmp_abs_primitive_integer_properties_helper_signed<
    T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand,
>()
where
    Integer: From<T> + PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_integer_and_signed::<T>, |&(ref n, i)| {
        let cmp = n.partial_cmp_abs(&i);
        assert_eq!(Some(n.cmp_abs(&Integer::from(i))), cmp);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(i.partial_cmp_abs(n), cmp_rev);
        assert_eq!(Some(Integer::from(i).cmp_abs(n)), cmp_rev);
    });

    test_properties(
        triples_of_integer_signed_and_integer::<T>,
        |&(ref n, i, ref m): &(Integer, T, Integer)| {
            if n.lt_abs(&i) && i.lt_abs(m) {
                assert_eq!(n.cmp_abs(m), Ordering::Less);
            } else if n.gt_abs(&i) && i.gt_abs(m) {
                assert_eq!(n.cmp_abs(m), Ordering::Greater);
            }
        },
    );

    test_properties(
        triples_of_signed_integer_and_signed::<T>,
        |&(i, ref n, j)| {
            if i.lt_abs(n) && n.lt_abs(&j) {
                assert!(i.lt_abs(&j));
            } else if i.gt_abs(n) && n.gt_abs(&j) {
                assert!(i.gt_abs(&j));
            }
        },
    );

    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        assert_eq!(Integer::from(x).partial_cmp_abs(&y), Some(x.cmp_abs(&y)));
        assert_eq!(x.partial_cmp_abs(&Integer::from(y)), Some(x.cmp_abs(&y)));
    });
}

#[test]
fn partial_cmp_abs_primitive_integer_properties() {
    partial_cmp_abs_primitive_integer_properties_helper_unsigned::<u8>();
    partial_cmp_abs_primitive_integer_properties_helper_unsigned::<u16>();
    partial_cmp_abs_primitive_integer_properties_helper_unsigned::<u32>();
    partial_cmp_abs_primitive_integer_properties_helper_unsigned::<u64>();
    partial_cmp_abs_primitive_integer_properties_helper_unsigned::<usize>();
    partial_cmp_abs_primitive_integer_properties_helper_signed::<i8>();
    partial_cmp_abs_primitive_integer_properties_helper_signed::<i16>();
    partial_cmp_abs_primitive_integer_properties_helper_signed::<i32>();
    partial_cmp_abs_primitive_integer_properties_helper_signed::<i64>();
    partial_cmp_abs_primitive_integer_properties_helper_signed::<isize>();
}
