// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{
    integer_gen, integer_integer_primitive_float_triple_gen, integer_primitive_float_pair_gen,
    integer_primitive_float_primitive_float_triple_gen,
};
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_partial_cmp_abs_primitive_float() {
    let test = |u, v: f32, out: Option<Ordering>| {
        let u = Integer::from_str(u).unwrap();

        let out_rev = out.map(Ordering::reverse);
        assert_eq!(u.partial_cmp_abs(&v), out);
        assert_eq!((&u).abs().partial_cmp_abs(&v.abs()), out);
        assert_eq!(v.partial_cmp_abs(&u), out_rev);

        let v = f64::from(v);
        assert_eq!(u.partial_cmp_abs(&v), out);
        assert_eq!(v.partial_cmp_abs(&u), out_rev);
    };
    test("5", f32::NAN, None);
    test("5", f32::INFINITY, Some(Less));
    test("5", f32::NEGATIVE_INFINITY, Some(Less));
    test("-5", f32::NAN, None);
    test("-5", f32::INFINITY, Some(Less));
    test("-5", f32::NEGATIVE_INFINITY, Some(Less));

    test("0", 0.0, Some(Equal));
    test("0", -0.0, Some(Equal));
    test("0", 5.0, Some(Less));
    test("0", -5.0, Some(Less));
    test("123", 123.0, Some(Equal));
    test("123", 5.0, Some(Greater));
    test("123", -123.0, Some(Equal));
    test("-123", 123.0, Some(Equal));
    test("-123", 5.0, Some(Greater));
    test("-123", -123.0, Some(Equal));
    test("1000000000000", 123.0, Some(Greater));
    test("-1000000000000", 123.0, Some(Greater));

    test("1208925819614629174706175", 1.2089258e24, Some(Less));
    test("1208925819614629174706176", 1.2089258e24, Some(Equal));
    test("1208925819614629174706177", 1.2089258e24, Some(Greater));
    test("1208925819614629174706175", -1.2089258e24, Some(Less));
    test("1208925819614629174706176", -1.2089258e24, Some(Equal));
    test("1208925819614629174706177", -1.2089258e24, Some(Greater));
    test("-1208925819614629174706175", 1.2089258e24, Some(Less));
    test("-1208925819614629174706176", 1.2089258e24, Some(Equal));
    test("-1208925819614629174706177", 1.2089258e24, Some(Greater));
    test("-1208925819614629174706175", -1.2089258e24, Some(Less));
    test("-1208925819614629174706176", -1.2089258e24, Some(Equal));
    test("-1208925819614629174706177", -1.2089258e24, Some(Greater));
}

fn partial_cmp_abs_primitive_float_properties_helper<T: PartialOrdAbs<Integer> + PrimitiveFloat>()
where
    Integer: PartialOrd<T> + PartialOrdAbs<T>,
{
    integer_primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let cmp_abs = x.partial_cmp_abs(&y);
        let cmp_abs_rev = cmp_abs.map(Ordering::reverse);
        assert_eq!((&x).abs().partial_cmp_abs(&y.abs()), cmp_abs);
        assert_eq!(y.partial_cmp_abs(&x), cmp_abs_rev);

        assert_eq!((&x).abs().partial_cmp(&y.abs()), cmp_abs);
        assert_eq!(x.partial_cmp_abs(&-y), cmp_abs);
    });

    integer_integer_primitive_float_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n.lt_abs(&u) && u.lt_abs(&m) {
            assert_eq!(n.cmp_abs(&m), Less);
        } else if n.gt_abs(&u) && u.gt_abs(&m) {
            assert_eq!(n.cmp_abs(&m), Greater);
        }
    });

    integer_primitive_float_primitive_float_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u.lt_abs(&n) && n.lt_abs(&v) {
            assert!(u.lt_abs(&v));
        } else if u.gt_abs(&n) && n.gt_abs(&v) {
            assert!(u.gt_abs(&v));
        }
    });

    integer_gen().test_properties(|x| {
        assert!(x.ge_abs(&T::ZERO));
        assert!(x.lt_abs(&T::NEGATIVE_INFINITY));
        assert!(x.lt_abs(&T::INFINITY));
    });
}

#[test]
fn partial_cmp_abs_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_cmp_abs_primitive_float_properties_helper);
}
