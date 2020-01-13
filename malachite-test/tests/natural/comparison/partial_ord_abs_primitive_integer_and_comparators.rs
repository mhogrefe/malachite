use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_nz::natural::Natural;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_natural_signeds, pairs_of_unsigneds};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_natural_signed, pairs_of_natural_and_signed,
    pairs_of_natural_and_unsigned, triples_of_natural_signed_and_natural,
    triples_of_natural_unsigned_and_natural, triples_of_signed_natural_and_signed,
    triples_of_unsigned_natural_and_unsigned,
};

#[test]
fn test_partial_cmp_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp_abs(&v), out);

        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
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
fn test_partial_cmp_u64() {
    let test = |u, v: u64, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp_abs(&v), out);

        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", 1000000000000, Some(Ordering::Equal));
    test("1000000000000", 1000000000001, Some(Ordering::Less));
}

#[test]
fn test_partial_cmp_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp_abs(&v), out);

        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("0", -5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", -123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", -124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("123", -122, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
}

#[test]
fn test_partial_cmp_i64() {
    let test = |u, v: i64, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp_abs(&v), out);

        assert_eq!(
            v.partial_cmp_abs(&Natural::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("0", -5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", -123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", -124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("123", -122, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("1000000000000", 1000000000000, Some(Ordering::Equal));
    test("1000000000000", -1000000000000, Some(Ordering::Equal));
    test("1000000000000", 1000000000001, Some(Ordering::Less));
    test("1000000000000", -1000000000001, Some(Ordering::Less));
}

fn partial_cmp_abs_primitive_integer_properties_helper_unsigned<
    T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand,
>()
where
    Natural: From<T> + PartialOrdAbs<T>,
{
    test_properties(pairs_of_natural_and_unsigned::<T>, |&(ref n, u)| {
        let cmp = n.partial_cmp_abs(&u);
        assert_eq!(Some(n.cmp(&Natural::from(u))), cmp);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(u.partial_cmp_abs(n), cmp_rev);
        assert_eq!(Some(Natural::from(u).cmp(n)), cmp_rev);
    });

    test_properties(
        triples_of_natural_unsigned_and_natural::<T>,
        |&(ref n, u, ref m): &(Natural, T, Natural)| {
            if n.lt_abs(&u) && u.lt_abs(m) {
                assert_eq!(n.cmp(m), Ordering::Less);
            } else if n.gt_abs(&u) && u.gt_abs(m) {
                assert_eq!(n.cmp(m), Ordering::Greater);
            }
        },
    );

    test_properties(
        triples_of_unsigned_natural_and_unsigned::<T>,
        |&(u, ref n, v)| {
            if u.lt_abs(n) && n.lt_abs(&v) {
                assert!(u.lt_abs(&v));
            } else if u.gt_abs(n) && n.gt_abs(&v) {
                assert!(u.gt_abs(&v));
            }
        },
    );

    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        assert_eq!(Natural::from(x).partial_cmp_abs(&y), Some(x.cmp_abs(&y)));
        assert_eq!(x.partial_cmp_abs(&Natural::from(y)), Some(x.cmp_abs(&y)));
    });
}

fn partial_cmp_abs_primitive_integer_properties_helper_signed<
    T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand,
>()
where
    Natural: ExactFrom<T> + PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_natural_and_signed::<T>, |&(ref n, i)| {
        let cmp = n.partial_cmp_abs(&i);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(i.partial_cmp_abs(n), cmp_rev);
    });

    test_properties(pairs_of_natural_and_natural_signed::<T>, |&(ref n, i)| {
        let cmp = n.partial_cmp_abs(&i);
        assert_eq!(Some(n.cmp(&Natural::exact_from(i))), cmp);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(Some(Natural::exact_from(i).cmp(n)), cmp_rev);
    });

    test_properties(
        triples_of_natural_signed_and_natural::<T>,
        |&(ref n, i, ref m): &(Natural, T, Natural)| {
            if n.lt_abs(&i) && i.lt_abs(m) {
                assert_eq!(n.cmp(m), Ordering::Less);
            } else if n.gt_abs(&i) && i.gt_abs(m) {
                assert_eq!(n.cmp(m), Ordering::Greater);
            }
        },
    );

    test_properties(
        triples_of_signed_natural_and_signed::<T>,
        |&(i, ref n, j)| {
            if i.lt_abs(n) && n.lt_abs(&j) {
                assert!(i.lt_abs(&j));
            } else if i.gt_abs(n) && n.gt_abs(&j) {
                assert!(i.gt_abs(&j));
            }
        },
    );

    test_properties(pairs_of_natural_signeds::<T>, |&(x, y)| {
        assert_eq!(Natural::exact_from(x).partial_cmp_abs(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp_abs(&Natural::exact_from(y)), Some(x.cmp(&y)));
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
