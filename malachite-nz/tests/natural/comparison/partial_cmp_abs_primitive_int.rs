// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_pair_gen_var_7, unsigned_pair_gen_var_27};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_natural_signed_triple_gen, natural_natural_unsigned_triple_gen,
    natural_signed_pair_gen, natural_signed_pair_gen_var_1, natural_signed_signed_triple_gen,
    natural_unsigned_pair_gen, natural_unsigned_unsigned_triple_gen,
};
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

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
    test("0", 0, Some(Equal), false, false, true, true);
    test("0", 5, Some(Less), true, false, true, false);
    test("123", 123, Some(Equal), false, false, true, true);
    test("123", 124, Some(Less), true, false, true, false);
    test("123", 122, Some(Greater), false, true, false, true);
    test(
        "1000000000000",
        123,
        Some(Greater),
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
    test("0", 0, Some(Equal), false, false, true, true);
    test("0", 5, Some(Less), true, false, true, false);
    test("123", 123, Some(Equal), false, false, true, true);
    test("123", 124, Some(Less), true, false, true, false);
    test("123", 122, Some(Greater), false, true, false, true);
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
    test("0", 0, Some(Equal), false, false, true, true);
    test("0", 5, Some(Less), true, false, true, false);
    test("0", -5, Some(Less), true, false, true, false);
    test("123", 123, Some(Equal), false, false, true, true);
    test("123", -123, Some(Equal), false, false, true, true);
    test("123", 124, Some(Less), true, false, true, false);
    test("123", -124, Some(Less), true, false, true, false);
    test("123", 122, Some(Greater), false, true, false, true);
    test("123", -122, Some(Greater), false, true, false, true);
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
    test("0", 0, Some(Equal), false, false, true, true);
    test("0", 5, Some(Less), true, false, true, false);
    test("0", -5, Some(Less), true, false, true, false);
    test("123", 123, Some(Equal), false, false, true, true);
    test("123", -123, Some(Equal), false, false, true, true);
    test("123", 124, Some(Less), true, false, true, false);
    test("123", -124, Some(Less), true, false, true, false);
    test("123", 122, Some(Greater), false, true, false, true);
    test("123", -122, Some(Greater), false, true, false, true);
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
}

fn partial_cmp_abs_primitive_int_properties_helper_unsigned<
    T: PartialOrdAbs<Natural> + PrimitiveUnsigned,
>()
where
    Natural: From<T> + PartialOrdAbs<T>,
{
    natural_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let cmp = n.partial_cmp_abs(&u);
        assert_eq!(Some(n.cmp(&Natural::from(u))), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(u.partial_cmp_abs(&n), cmp_rev);
        assert_eq!(Some(Natural::from(u).cmp(&n)), cmp_rev);
    });

    natural_natural_unsigned_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n.lt_abs(&u) && u.lt_abs(&m) {
            assert_eq!(n.cmp(&m), Less);
        } else if n.gt_abs(&u) && u.gt_abs(&m) {
            assert_eq!(n.cmp(&m), Greater);
        }
    });

    natural_unsigned_unsigned_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u.lt_abs(&n) && n.lt_abs(&v) {
            assert!(u.lt_abs(&v));
        } else if u.gt_abs(&n) && n.gt_abs(&v) {
            assert!(u.gt_abs(&v));
        }
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Natural::from(x).partial_cmp_abs(&y), Some(x.cmp_abs(&y)));
        assert_eq!(x.partial_cmp_abs(&Natural::from(y)), Some(x.cmp_abs(&y)));
    });
}

fn partial_cmp_abs_primitive_int_properties_helper_signed<
    T: PartialOrdAbs<Natural> + PartialOrd<rug::Integer> + PrimitiveSigned,
>()
where
    Integer: From<T>,
    Natural: TryFrom<T> + PartialOrdAbs<T>,
{
    natural_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let cmp = n.partial_cmp_abs(&i);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(i.partial_cmp_abs(&n), cmp_rev);
        if i != T::MIN {
            assert_eq!(n.partial_cmp_abs(&-i), cmp);
        }
    });

    natural_signed_pair_gen_var_1::<T>().test_properties(|(n, i)| {
        let cmp = n.partial_cmp_abs(&i);
        assert_eq!(Some(n.cmp(&Natural::exact_from(i))), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(Some(Natural::exact_from(i).cmp(&n)), cmp_rev);
    });

    natural_natural_signed_triple_gen::<T>().test_properties(|(n, m, i)| {
        if n.lt_abs(&i) && i.lt_abs(&m) {
            assert_eq!(n.cmp(&m), Less);
        } else if n.gt_abs(&i) && i.gt_abs(&m) {
            assert_eq!(n.cmp(&m), Greater);
        }
    });

    natural_signed_signed_triple_gen::<T>().test_properties(|(n, i, j)| {
        if i.lt_abs(&n) && n.lt_abs(&j) {
            assert!(i.lt_abs(&j));
        } else if i.gt_abs(&n) && n.gt_abs(&j) {
            assert!(i.gt_abs(&j));
        }
    });

    signed_pair_gen_var_7::<T>().test_properties(|(x, y)| {
        assert_eq!(Natural::exact_from(x).partial_cmp_abs(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp_abs(&Natural::exact_from(y)), Some(x.cmp(&y)));
    });
}

#[test]
fn partial_cmp_abs_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_cmp_abs_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_cmp_abs_primitive_int_properties_helper_signed);
}
