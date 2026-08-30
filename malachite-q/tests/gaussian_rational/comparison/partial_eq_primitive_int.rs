// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{
    gaussian_rational_signed_pair_gen, gaussian_rational_unsigned_pair_gen,
};
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |s, v: u32, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(x == v, out);
        assert_eq!(v == x, out);
    };
    test("0", 0, true);
    test("123", 123, true);
    test("-123", 123, false);
    test("22/7", 3, false);
    test("123+i", 123, false);
    test("i", 0, false);
}

#[test]
fn test_partial_eq_u64() {
    let test = |s, v: u64, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(x == v, out);
        assert_eq!(v == x, out);
    };
    test("0", 0, true);
    test("1000000000000", 1000000000000, true);
    test("-1000000000000", 1000000000000, false);
    test("1000000000000+i", 1000000000000, false);
}

#[test]
fn test_partial_eq_i32() {
    let test = |s, v: i32, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(x == v, out);
        assert_eq!(v == x, out);
    };
    test("0", 0, true);
    test("-123", -123, true);
    test("-123", 123, false);
    test("-22/7", -3, false);
    test("-123+i", -123, false);
}

#[test]
fn test_partial_eq_i64() {
    let test = |s, v: i64, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(x == v, out);
        assert_eq!(v == x, out);
    };
    test("0", 0, true);
    test("-1000000000000", -1000000000000, true);
    test("-1000000000000", 1000000000000, false);
    test("-1000000000000+i", -1000000000000, false);
}

// Extra refs necessary for type inference
#[allow(clippy::cmp_owned, clippy::op_ref, clippy::trait_duplication_in_bounds)]
fn partial_eq_primitive_int_properties_helper_unsigned<
    T: PartialEq<GaussianRational> + PrimitiveUnsigned,
>()
where
    GaussianRational: From<T> + PartialEq<T>,
{
    gaussian_rational_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let eq = n == u;
        assert_eq!(u == n, eq);
        assert_eq!(&n == &GaussianRational::from(u), eq);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(GaussianRational::from(x) == y, x == y);
        assert_eq!(x == GaussianRational::from(y), x == y);
    });
}

// Extra refs necessary for type inference
#[allow(clippy::cmp_owned, clippy::op_ref, clippy::trait_duplication_in_bounds)]
fn partial_eq_primitive_int_properties_helper_signed<
    T: PartialEq<GaussianRational> + PrimitiveSigned,
>()
where
    GaussianRational: From<T> + PartialEq<T>,
{
    gaussian_rational_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let eq = n == i;
        assert_eq!(i == n, eq);
        assert_eq!(&n == &GaussianRational::from(i), eq);
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(GaussianRational::from(x) == y, x == y);
        assert_eq!(x == GaussianRational::from(y), x == y);
    });
}

#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_eq_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_eq_primitive_int_properties_helper_signed);
}
