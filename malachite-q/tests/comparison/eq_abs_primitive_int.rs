// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_pair_gen_var_7, unsigned_pair_gen_var_27};
use malachite_q::test_util::generators::{rational_signed_pair_gen, rational_unsigned_pair_gen};
use malachite_q::Rational;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_eq_abs_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Rational::from_str(u).unwrap().eq_abs(&v), out);
        assert_eq!(!Rational::from_str(u).unwrap().ne_abs(&v), out);
        assert_eq!(v.eq_abs(&Rational::from_str(u).unwrap()), out);
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
    let test = |u, v: u64, out| {
        assert_eq!(Rational::from_str(u).unwrap().eq_abs(&v), out);
        assert_eq!(!Rational::from_str(u).unwrap().ne_abs(&v), out);
        assert_eq!(v.eq_abs(&Rational::from_str(u).unwrap()), out);
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
    let test = |u, v: i32, out| {
        assert_eq!(Rational::from_str(u).unwrap().eq_abs(&v), out);
        assert_eq!(!Rational::from_str(u).unwrap().ne_abs(&v), out);
        assert_eq!(v.eq_abs(&Rational::from_str(u).unwrap()), out);
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
    let test = |u, v: i64, out| {
        assert_eq!(Rational::from_str(u).unwrap().eq_abs(&v), out);
        assert_eq!(!Rational::from_str(u).unwrap().ne_abs(&v), out);
        assert_eq!(v.eq_abs(&Rational::from_str(u).unwrap()), out);
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
    Rational: EqAbs<T> + From<T> + PartialEq<T> + PartialOrdAbs<T>,
{
    rational_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let eq = n.eq_abs(&u);
        assert_ne!(n.ne_abs(&u), eq);

        assert_eq!(u.eq_abs(&n), eq);
        assert_eq!(n.partial_cmp_abs(&u) == Some(Equal), eq);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Rational::from(x).eq_abs(&y), x == y);
        assert_eq!(x.eq_abs(&Rational::from(y)), x == y);
    });
}

fn eq_abs_primitive_int_properties_helper_signed<T: EqAbs<Rational> + PrimitiveSigned>()
where
    Rational: EqAbs<T> + From<T> + TryFrom<T> + PartialOrdAbs<T>,
{
    rational_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let eq = n.eq_abs(&i);
        assert_ne!(n.ne_abs(&i), eq);
        assert_eq!(
            <Rational as EqAbs<Rational>>::eq_abs(&n, &Rational::from(i)),
            eq
        );
        assert_eq!(i.eq_abs(&n), eq);
        assert_eq!(
            <Rational as EqAbs<Rational>>::eq_abs(&Rational::from(i), &n),
            eq
        );
        assert_eq!(n.partial_cmp_abs(&i) == Some(Equal), eq);
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
