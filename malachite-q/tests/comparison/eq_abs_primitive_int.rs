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
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_pair_gen_var_7, unsigned_pair_gen_var_27};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_signed_pair_gen, rational_unsigned_pair_gen};
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_eq_abs_u32() {
    let test = |u, v: u32, eq: bool| {
        let u = Rational::from_str(u).unwrap();
        assert_eq!(u.eq_abs(&v), eq);
        assert_eq!(u.ne_abs(&v), !eq);
        assert_eq!((&u).abs() == v, eq);
        assert_eq!(v.eq_abs(&u), eq);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 123, false);
    test("-123", 123, true);
    test("-123", 5, false);
    test("-1000000000000", 123, false);
    test("22/7", 123, false);
    test("-22/7", 123, false);
}

#[test]
fn test_eq_abs_u64() {
    let test = |u, v: u64, eq: bool| {
        let u = Rational::from_str(u).unwrap();
        assert_eq!(u.eq_abs(&v), eq);
        assert_eq!(u.ne_abs(&v), !eq);
        assert_eq!((&u).abs() == v, eq);
        assert_eq!(v.eq_abs(&u), eq);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 1000000000000, true);
    test("1000000000000", 1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
    test("-123", 123, true);
    test("-123", 5, false);
    test("-1000000000000", 1000000000000, true);
    test("-1000000000000", 1000000000001, false);
    test("-1000000000000000000000000", 1000000000000, false);
    test("22/7", 1000000000000, false);
    test("-22/7", 1000000000000, false);
}

#[test]
fn test_eq_abs_i32() {
    let test = |u, v: i32, eq: bool| {
        let u = Rational::from_str(u).unwrap();
        assert_eq!(u.eq_abs(&v), eq);
        assert_eq!(u.ne_abs(&v), !eq);
        assert_eq!((&u).abs() == v.unsigned_abs(), eq);
        assert_eq!(v.eq_abs(&u), eq);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", -123, true);
    test("123", 5, false);
    test("1000000000000", 123, false);
    test("-123", 123, true);
    test("-123", -123, true);
    test("-123", 5, false);
    test("-1000000000000", 123, false);
    test("22/7", 123, false);
    test("-22/7", 123, false);
}

#[test]
fn test_eq_abs_i64() {
    let test = |u, v: i64, eq: bool| {
        let u = Rational::from_str(u).unwrap();
        assert_eq!(u.eq_abs(&v), eq);
        assert_eq!(u.ne_abs(&v), !eq);
        assert_eq!((&u).abs() == v.unsigned_abs(), eq);
        assert_eq!(v.eq_abs(&u), eq);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", -123, true);
    test("123", 5, false);
    test("1000000000000", 1000000000000, true);
    test("1000000000000", 1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
    test("-123", 123, true);
    test("-123", -123, true);
    test("-123", 5, false);
    test("-1000000000000", 1000000000000, true);
    test("-1000000000000", 1000000000001, false);
    test("-1000000000000000000000000", 1000000000000, false);
    test("22/7", 1000000000000, false);
    test("-22/7", 1000000000000, false);
}

// Extra refs necessary for type inference
#[allow(clippy::cmp_owned, clippy::op_ref, clippy::trait_duplication_in_bounds)]
fn eq_abs_primitive_int_properties_helper_unsigned<
    T: EqAbs<Rational> + PartialEq<Rational> + PrimitiveUnsigned,
>()
where
    Rational: EqAbs<T> + PartialEq<T> + From<T> + PartialEq<T> + PartialOrdAbs<T>,
{
    rational_unsigned_pair_gen::<T>().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        assert_eq!(x.ne_abs(&y), !eq);
        assert_eq!((&x).abs().eq_abs(&y), eq);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!(x.partial_cmp_abs(&y) == Some(Equal), eq);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Rational::from(x).eq_abs(&y), x == y);
        assert_eq!(x.eq_abs(&Rational::from(y)), x == y);
    });
}

fn eq_abs_primitive_int_properties_helper_signed<T: EqAbs<Rational> + PrimitiveSigned>()
where
    Rational:
        EqAbs<T> + PartialEq<<T as UnsignedAbs>::Output> + From<T> + TryFrom<T> + PartialOrdAbs<T>,
{
    rational_signed_pair_gen::<T>().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        assert_eq!(x.ne_abs(&y), !eq);
        assert_eq!((&x).abs() == y.unsigned_abs(), eq);
        assert_eq!(
            <Rational as EqAbs<Rational>>::eq_abs(&x, &Rational::from(y)),
            eq
        );
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!(
            <Rational as EqAbs<Rational>>::eq_abs(&Rational::from(y), &x),
            eq
        );
        assert_eq!(x.partial_cmp_abs(&y) == Some(Equal), eq);
    });

    signed_pair_gen_var_7::<T>().test_properties(|(x, y)| {
        assert_eq!(Rational::exact_from(x).eq_abs(&y), x.eq_abs(&y));
        assert_eq!(x.eq_abs(&Rational::exact_from(y)), x.eq_abs(&y));
    });
}

#[test]
fn eq_abs_primitive_int_properties() {
    apply_fn_to_unsigneds!(eq_abs_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(eq_abs_primitive_int_properties_helper_signed);
}
