// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_primitive_float_pair_gen, natural_primitive_float_pair_gen,
};
use rug;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_partial_eq() {
    let test = |u, v: f32, out| {
        assert_eq!(Integer::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);

        let v = f64::from(v);
        assert_eq!(Integer::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("5", f32::NAN, false);
    test("5", f32::INFINITY, false);
    test("5", f32::NEGATIVE_INFINITY, false);
    test("-5", f32::NAN, false);
    test("-5", f32::INFINITY, false);
    test("-5", f32::NEGATIVE_INFINITY, false);

    test("0", 0.0, true);
    test("0", -0.0, true);
    test("0", 5.0, false);
    test("0", -5.0, false);
    test("123", 123.0, true);
    test("123", 5.0, false);
    test("123", -123.0, false);
    test("-123", 123.0, false);
    test("-123", 5.0, false);
    test("-123", -123.0, true);
    test("1000000000000", 123.0, false);
    test("-1000000000000", 123.0, false);

    test("1208925819614629174706175", 1.2089258e24, false);
    test("1208925819614629174706176", 1.2089258e24, true);
    test("1208925819614629174706177", 1.2089258e24, false);
    test("1208925819614629174706175", -1.2089258e24, false);
    test("1208925819614629174706176", -1.2089258e24, false);
    test("1208925819614629174706177", -1.2089258e24, false);
    test("-1208925819614629174706175", 1.2089258e24, false);
    test("-1208925819614629174706176", 1.2089258e24, false);
    test("-1208925819614629174706177", 1.2089258e24, false);
    test("-1208925819614629174706175", -1.2089258e24, false);
    test("-1208925819614629174706176", -1.2089258e24, true);
    test("-1208925819614629174706177", -1.2089258e24, false);
}

#[allow(clippy::cmp_owned, clippy::trait_duplication_in_bounds)]
fn partial_eq_primitive_float_properties_helper<
    T: PartialEq<Integer> + PartialEq<Natural> + PartialEq<rug::Integer> + PrimitiveFloat,
>()
where
    Integer: PartialEq<T> + PartialOrd<T>,
    Natural: PartialEq<T>,
    rug::Integer: PartialEq<T>,
{
    integer_primitive_float_pair_gen::<T>().test_properties(|(n, f)| {
        let eq = n == f;
        assert_eq!(rug::Integer::from(&n) == f, eq);
        assert_eq!(f == n, eq);
        assert_eq!(f == rug::Integer::from(&n), eq);
        assert_eq!(n.partial_cmp(&f) == Some(Equal), eq);
        if eq {
            assert!(f.is_integer());
        }
    });

    natural_primitive_float_pair_gen::<T>().test_properties(|(n, f)| {
        let i_n: Integer = From::from(&n);
        let eq = n == f;
        assert_eq!(i_n == f, eq);
        assert_eq!(f == n, eq);
        assert_eq!(f == i_n, eq);
    });
}

#[test]
fn partial_eq_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_eq_primitive_float_properties_helper);
}
