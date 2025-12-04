// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{integer_pair_gen, natural_pair_gen};
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    rational_rational_signed_triple_gen, rational_rational_unsigned_triple_gen,
    rational_signed_pair_gen, rational_signed_signed_triple_gen, rational_unsigned_pair_gen,
    rational_unsigned_unsigned_triple_gen,
};
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_partial_cmp_abs_u32() {
    let test = |s, v: u32, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(u.partial_cmp_abs(&v), cmp);
        assert_eq!((&u).abs().partial_cmp(&v), cmp);
        assert_eq!(v.partial_cmp_abs(&u), cmp.map(Ordering::reverse));
        assert_eq!(lt, u.lt_abs(&v));
        assert_eq!(gt, u.gt_abs(&v));
        assert_eq!(le, u.le_abs(&v));
        assert_eq!(ge, u.ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&u));
        assert_eq!(gt, v.lt_abs(&u));
        assert_eq!(le, v.ge_abs(&u));
        assert_eq!(ge, v.le_abs(&u));
    };
    test("0", 0, Some(Equal), false, false, true, true);
    test("0", 5, Some(Less), true, false, true, false);
    test("123", 123, Some(Equal), false, false, true, true);
    test("-123", 123, Some(Equal), false, false, true, true);
    test("123", 124, Some(Less), true, false, true, false);
    test("-123", 124, Some(Less), true, false, true, false);
    test("123", 122, Some(Greater), false, true, false, true);
    test("-123", 122, Some(Greater), false, true, false, true);
    test(
        "1000000000000",
        123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-1000000000000",
        123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );

    test("99/100", 1, Some(Less), true, false, true, false);
    test("101/100", 1, Some(Greater), false, true, false, true);
    test("22/7", 3, Some(Greater), false, true, false, true);
    test("22/7", 4, Some(Less), true, false, true, false);
    test("-99/100", 1, Some(Less), true, false, true, false);
    test("-101/100", 1, Some(Greater), false, true, false, true);
    test("-22/7", 3, Some(Greater), false, true, false, true);
    test("-22/7", 4, Some(Less), true, false, true, false);
}

#[test]
fn test_partial_cmp_abs_u64() {
    let test = |u, v: u64, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        let u = Rational::from_str(u).unwrap();
        assert_eq!(u.partial_cmp_abs(&v), cmp);
        assert_eq!((&u).abs().partial_cmp(&v), cmp);
        assert_eq!(v.partial_cmp_abs(&u), cmp.map(Ordering::reverse));
        assert_eq!(lt, u.lt_abs(&v));
        assert_eq!(gt, u.gt_abs(&v));
        assert_eq!(le, u.le_abs(&v));
        assert_eq!(ge, u.ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&u));
        assert_eq!(gt, v.lt_abs(&u));
        assert_eq!(le, v.ge_abs(&u));
        assert_eq!(ge, v.le_abs(&u));
    };
    test("0", 0, Some(Equal), false, false, true, true);
    test("0", 5, Some(Less), true, false, true, false);
    test("123", 123, Some(Equal), false, false, true, true);
    test("-123", 123, Some(Equal), false, false, true, true);
    test("123", 124, Some(Less), true, false, true, false);
    test("-123", 124, Some(Less), true, false, true, false);
    test("123", 122, Some(Greater), false, true, false, true);
    test("-123", 122, Some(Greater), false, true, false, true);
    test(
        "1000000000000",
        123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-1000000000000",
        123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        1000000000000,
        Some(Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "-1000000000000",
        1000000000000,
        Some(Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "1000000000000",
        1000000000001,
        Some(Less),
        true,
        false,
        true,
        false,
    );
    test(
        "-1000000000000",
        1000000000001,
        Some(Less),
        true,
        false,
        true,
        false,
    );

    test("99/100", 1, Some(Less), true, false, true, false);
    test("101/100", 1, Some(Greater), false, true, false, true);
    test("22/7", 3, Some(Greater), false, true, false, true);
    test("22/7", 4, Some(Less), true, false, true, false);
    test("-99/100", 1, Some(Less), true, false, true, false);
    test("-101/100", 1, Some(Greater), false, true, false, true);
    test("-22/7", 3, Some(Greater), false, true, false, true);
    test("-22/7", 4, Some(Less), true, false, true, false);
}

#[test]
fn test_partial_cmp_abs_i32() {
    let test = |u, v: i32, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        let u = Rational::from_str(u).unwrap();
        assert_eq!(u.partial_cmp_abs(&v), cmp);
        assert_eq!((&u).abs().partial_cmp(&v.abs()), cmp);
        assert_eq!(v.partial_cmp_abs(&u), cmp.map(Ordering::reverse));
        assert_eq!(lt, u.lt_abs(&v));
        assert_eq!(gt, u.gt_abs(&v));
        assert_eq!(le, u.le_abs(&v));
        assert_eq!(ge, u.ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&u));
        assert_eq!(gt, v.lt_abs(&u));
        assert_eq!(le, v.ge_abs(&u));
        assert_eq!(ge, v.le_abs(&u));
    };
    test("0", 0, Some(Equal), false, false, true, true);
    test("0", 5, Some(Less), true, false, true, false);
    test("0", -5, Some(Less), true, false, true, false);
    test("123", 123, Some(Equal), false, false, true, true);
    test("123", -123, Some(Equal), false, false, true, true);
    test("-123", 123, Some(Equal), false, false, true, true);
    test("-123", -123, Some(Equal), false, false, true, true);
    test("123", 124, Some(Less), true, false, true, false);
    test("123", -124, Some(Less), true, false, true, false);
    test("-123", 124, Some(Less), true, false, true, false);
    test("-123", -124, Some(Less), true, false, true, false);
    test("123", 122, Some(Greater), false, true, false, true);
    test("123", -122, Some(Greater), false, true, false, true);
    test("-123", 122, Some(Greater), false, true, false, true);
    test("-123", -122, Some(Greater), false, true, false, true);
    test(
        "1000000000000",
        123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        -123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-1000000000000",
        123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-1000000000000",
        -123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );

    test("99/100", 1, Some(Less), true, false, true, false);
    test("101/100", 1, Some(Greater), false, true, false, true);
    test("22/7", 3, Some(Greater), false, true, false, true);
    test("22/7", 4, Some(Less), true, false, true, false);
    test("-99/100", -1, Some(Less), true, false, true, false);
    test("-101/100", -1, Some(Greater), false, true, false, true);
    test("-22/7", -3, Some(Greater), false, true, false, true);
    test("-22/7", -4, Some(Less), true, false, true, false);
}

#[test]
fn test_partial_cmp_abs_i64() {
    let test = |u, v: i64, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        let u = Rational::from_str(u).unwrap();
        assert_eq!(u.partial_cmp_abs(&v), cmp);
        assert_eq!((&u).abs().partial_cmp(&v.abs()), cmp);
        assert_eq!(v.partial_cmp_abs(&u), cmp.map(Ordering::reverse));
        assert_eq!(lt, u.lt_abs(&v));
        assert_eq!(gt, u.gt_abs(&v));
        assert_eq!(le, u.le_abs(&v));
        assert_eq!(ge, u.ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&u));
        assert_eq!(gt, v.lt_abs(&u));
        assert_eq!(le, v.ge_abs(&u));
        assert_eq!(ge, v.le_abs(&u));
    };
    test("0", 0, Some(Equal), false, false, true, true);
    test("0", 5, Some(Less), true, false, true, false);
    test("0", -5, Some(Less), true, false, true, false);
    test("123", 123, Some(Equal), false, false, true, true);
    test("123", -123, Some(Equal), false, false, true, true);
    test("-123", 123, Some(Equal), false, false, true, true);
    test("-123", -123, Some(Equal), false, false, true, true);
    test("123", 124, Some(Less), true, false, true, false);
    test("123", -124, Some(Less), true, false, true, false);
    test("-123", 124, Some(Less), true, false, true, false);
    test("-123", -124, Some(Less), true, false, true, false);
    test("123", 122, Some(Greater), false, true, false, true);
    test("123", -122, Some(Greater), false, true, false, true);
    test("-123", 122, Some(Greater), false, true, false, true);
    test("-123", -122, Some(Greater), false, true, false, true);
    test(
        "1000000000000",
        123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        -123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-1000000000000",
        123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-1000000000000",
        -123,
        Some(Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        1000000000000,
        Some(Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "1000000000000",
        -1000000000000,
        Some(Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "-1000000000000",
        1000000000000,
        Some(Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "-1000000000000",
        -1000000000000,
        Some(Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "1000000000000",
        1000000000001,
        Some(Less),
        true,
        false,
        true,
        false,
    );
    test(
        "1000000000000",
        -1000000000001,
        Some(Less),
        true,
        false,
        true,
        false,
    );
    test(
        "-1000000000000",
        1000000000001,
        Some(Less),
        true,
        false,
        true,
        false,
    );
    test(
        "-1000000000000",
        -1000000000001,
        Some(Less),
        true,
        false,
        true,
        false,
    );

    test("99/100", 1, Some(Less), true, false, true, false);
    test("101/100", 1, Some(Greater), false, true, false, true);
    test("22/7", 3, Some(Greater), false, true, false, true);
    test("22/7", 4, Some(Less), true, false, true, false);
    test("-99/100", -1, Some(Less), true, false, true, false);
    test("-101/100", -1, Some(Greater), false, true, false, true);
    test("-22/7", -3, Some(Greater), false, true, false, true);
    test("-22/7", -4, Some(Less), true, false, true, false);
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_abs_primitive_int_properties_helper_unsigned<
    T: PartialOrdAbs<Rational> + PartialOrd<Rational> + PrimitiveUnsigned,
>()
where
    Rational: From<T>
        + for<'a> From<&'a Natural>
        + PartialOrdAbs<T>
        + PartialOrd<T>
        + PartialOrdAbs<Natural>,
{
    rational_unsigned_pair_gen::<T>().test_properties(|(x, y)| {
        let cmp = x.partial_cmp_abs(&y);
        assert_eq!(Some(x.cmp_abs(&Rational::from(y))), cmp);
        assert_eq!((&x).abs().partial_cmp(&y), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(y.partial_cmp_abs(&x), cmp_rev);
        assert_eq!(Some(Rational::from(y).cmp_abs(&x)), cmp_rev);
    });

    rational_rational_unsigned_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n.lt_abs(&u) && u.lt_abs(&m) {
            assert_eq!(n.cmp_abs(&m), Less);
        } else if n.gt_abs(&u) && u.gt_abs(&m) {
            assert_eq!(n.cmp_abs(&m), Greater);
        }
    });

    rational_unsigned_unsigned_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u.lt_abs(&n) && n.lt_abs(&v) {
            assert!(u < v);
        } else if u.gt_abs(&n) && n.gt_abs(&v) {
            assert!(u > v);
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            PartialOrdAbs::<Natural>::partial_cmp_abs(&Rational::from(&x), &y),
            Some(x.cmp(&y))
        );
        assert_eq!(x.partial_cmp_abs(&Rational::from(&y)), Some(x.cmp(&y)));
    });
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_abs_primitive_int_properties_helper_signed<
    T: PartialOrdAbs<Rational> + PartialOrd<rug::Rational> + PrimitiveSigned,
>()
where
    Rational: From<T>
        + for<'a> From<&'a Integer>
        + PartialOrdAbs<T>
        + PartialOrd<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PartialOrd<Rational>,
{
    rational_signed_pair_gen::<T>().test_properties(|(x, y)| {
        let cmp = x.partial_cmp_abs(&y);
        assert_eq!(Some(x.cmp_abs(&Rational::from(y))), cmp);
        assert_eq!((&x).abs().partial_cmp(&y.unsigned_abs()), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(y.partial_cmp_abs(&x), cmp_rev);
        assert_eq!(Some(Rational::from(y).cmp_abs(&x)), cmp_rev);
    });

    rational_rational_signed_triple_gen::<T>().test_properties(|(n, m, i)| {
        if n.lt_abs(&i) && i.lt_abs(&m) {
            assert_eq!(n.cmp_abs(&m), Less);
        } else if n.gt_abs(&i) && i.gt_abs(&m) {
            assert_eq!(n.cmp_abs(&m), Greater);
        }
    });

    rational_signed_signed_triple_gen::<T>().test_properties(|(n, i, j)| {
        if i.lt_abs(&n) && n.lt_abs(&j) {
            assert!(i.lt_abs(&j));
        } else if i.gt_abs(&n) && n.gt_abs(&j) {
            assert!(i.gt_abs(&j));
        }
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            PartialOrdAbs::<Integer>::partial_cmp_abs(&Rational::from(&x), &y),
            Some(x.cmp_abs(&y))
        );
        assert_eq!(x.partial_cmp_abs(&Rational::from(&y)), Some(x.cmp_abs(&y)));
    });
}

#[test]
fn partial_cmp_abs_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_cmp_abs_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_cmp_abs_primitive_int_properties_helper_signed);
}
