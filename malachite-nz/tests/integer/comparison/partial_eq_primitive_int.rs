// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{integer_signed_pair_gen, integer_unsigned_pair_gen};
use malachite_nz::test_util::integer::comparison::partial_eq_primitive_int::*;
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |s, v: u32, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(
            num_partial_eq_primitive(&BigInt::from_str(s).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Integer::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", 5, false);
    test("1000000000000", 123, false);
    test("-1000000000000", 123, false);
}

#[test]
fn test_partial_eq_u64() {
    let test = |s, v: u64, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(
            num_partial_eq_primitive(&BigInt::from_str(s).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Integer::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", 5, false);
    test("1000000000000", 1000000000000, true);
    test("-1000000000000", 1000000000000, false);
    test("1000000000000", 1000000000001, false);
    test("-1000000000000", 1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
    test("-1000000000000000000000000", 1000000000000, false);
}

#[test]
fn test_partial_eq_i32() {
    let test = |s, v: i32, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(rug::Integer::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Integer::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", -123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", -5, false);
    test("1000000000000", 123, false);
    test("-1000000000000", -123, false);
}

#[test]
fn test_partial_eq_i64() {
    let test = |s, v: i64, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(rug::Integer::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Integer::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", -123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", -5, false);
    test("1000000000000", 1000000000000, true);
    test("-1000000000000", -1000000000000, true);
    test("1000000000000", 1000000000001, false);
    test("-1000000000000", -1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
    test("-1000000000000000000000000", -1000000000000, false);
}

// Extra refs necessary for type inference
#[allow(clippy::cmp_owned, clippy::op_ref, clippy::trait_duplication_in_bounds)]
fn partial_eq_primitive_int_properties_helper_unsigned<
    T: PartialEq<Integer> + PartialEq<rug::Integer> + PrimitiveUnsigned,
>()
where
    BigInt: From<T>,
    Integer: From<T> + PartialEq<T>,
    rug::Integer: PartialEq<T>,
{
    integer_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let eq = n == u;
        assert_eq!(num_partial_eq_primitive(&From::from(&n), u), eq);
        assert_eq!(rug::Integer::from(&n) == u, eq);
        assert_eq!(&n == &Integer::from(u), eq);

        assert_eq!(u == n, eq);
        assert_eq!(u == rug::Integer::from(&n), eq);
        assert_eq!(&Integer::from(u) == &n, eq);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x) == y, x == y);
        assert_eq!(x == Integer::from(y), x == y);
    });
}

// Extra refs necessary for type inference
#[allow(clippy::cmp_owned, clippy::op_ref, clippy::trait_duplication_in_bounds)]
fn partial_eq_primitive_int_properties_helper_signed<
    T: PartialEq<Integer> + PartialEq<rug::Integer> + PrimitiveSigned,
>()
where
    Integer: From<T> + PartialEq<T>,
    rug::Integer: PartialEq<T>,
{
    integer_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let eq = n == i;
        assert_eq!(rug::Integer::from(&n) == i, eq);
        assert_eq!(&n == &Integer::from(i), eq);

        assert_eq!(i == n, eq);
        assert_eq!(i == rug::Integer::from(&n), eq);
        assert_eq!(&Integer::from(i) == &n, eq);
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x) == y, x == y);
        assert_eq!(x == Integer::from(y), x == y);
    });
}

#[test]
fn partial_eq_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_eq_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_eq_primitive_int_properties_helper_signed);
}
