// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{Ceiling, Floor};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::OneHalf;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, IsInteger, RoundingFrom,
};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_nz::integer::Integer;
use malachite_q::conversion::primitive_int_from_rational::{
    SignedFromRationalError, UnsignedFromRationalError,
};
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_3, rational_rounding_mode_pair_gen_var_3,
};
use malachite_q::Rational;
use std::cmp::Ordering::*;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_u32_try_from_rational() {
    let test = |s, out: Result<u32, UnsignedFromRationalError>| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(u32::try_from(&u), out);
    };
    test("0", Ok(0));
    test("123", Ok(123));
    test("-123", Err(UnsignedFromRationalError));
    test("1000000000000", Err(UnsignedFromRationalError));
    test("-1000000000000", Err(UnsignedFromRationalError));
    test("22/7", Err(UnsignedFromRationalError));
    test("-22/7", Err(UnsignedFromRationalError));
}

#[test]
fn test_i32_try_from_rational() {
    let test = |s, out: Result<i32, SignedFromRationalError>| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(i32::try_from(&u), out);
    };
    test("0", Ok(0));
    test("123", Ok(123));
    test("-123", Ok(-123));
    test("1000000000000", Err(SignedFromRationalError));
    test("-1000000000000", Err(SignedFromRationalError));
    test("22/7", Err(SignedFromRationalError));
    test("-22/7", Err(SignedFromRationalError));
}

#[test]
fn test_u32_convertible_from_rational() {
    let test = |s, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(u32::convertible_from(&u), out);
    };
    test("0", true);
    test("123", true);
    test("-123", false);
    test("1000000000000", false);
    test("-1000000000000", false);
    test("22/7", false);
    test("-22/7", false);
}

#[test]
fn test_i32_convertible_from_rational() {
    let test = |s, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(i32::convertible_from(&u), out);
    };
    test("0", true);
    test("123", true);
    test("-123", true);
    test("1000000000000", false);
    test("-1000000000000", false);
    test("22/7", false);
    test("-22/7", false);
}

#[test]
fn test_u32_rounding_from_rational() {
    let test = |s, rm, out: u32, o_out| {
        let r = Rational::from_str(s).unwrap();
        let (u, o) = u32::rounding_from(&r, rm);
        assert_eq!(u, out);
        assert_eq!(o, o_out);
    };
    test("123", Floor, 123, Equal);
    test("123", Ceiling, 123, Equal);
    test("123", Down, 123, Equal);
    test("123", Up, 123, Equal);
    test("123", Nearest, 123, Equal);
    test("123", Exact, 123, Equal);

    test("22/7", Floor, 3, Less);
    test("22/7", Ceiling, 4, Greater);
    test("22/7", Down, 3, Less);
    test("22/7", Up, 4, Greater);
    test("22/7", Nearest, 3, Less);

    test("7/2", Floor, 3, Less);
    test("7/2", Ceiling, 4, Greater);
    test("7/2", Down, 3, Less);
    test("7/2", Up, 4, Greater);
    test("7/2", Nearest, 4, Greater);

    test("9/2", Floor, 4, Less);
    test("9/2", Ceiling, 5, Greater);
    test("9/2", Down, 4, Less);
    test("9/2", Up, 5, Greater);
    test("9/2", Nearest, 4, Less);

    test("-123", Ceiling, 0, Greater);
    test("-123", Down, 0, Greater);
    test("-123", Nearest, 0, Greater);

    test("1000000000000", Floor, u32::MAX, Less);
    test("1000000000000", Down, u32::MAX, Less);
    test("1000000000000", Nearest, u32::MAX, Less);
}

#[test]
fn test_i32_rounding_from_rational() {
    let test = |s, rm, out: i32, o_out| {
        let r = Rational::from_str(s).unwrap();
        let (i, o) = i32::rounding_from(&r, rm);
        assert_eq!(i, out);
        assert_eq!(o, o_out);
    };
    test("123", Floor, 123, Equal);
    test("123", Ceiling, 123, Equal);
    test("123", Down, 123, Equal);
    test("123", Up, 123, Equal);
    test("123", Nearest, 123, Equal);
    test("123", Exact, 123, Equal);

    test("22/7", Floor, 3, Less);
    test("22/7", Ceiling, 4, Greater);
    test("22/7", Down, 3, Less);
    test("22/7", Up, 4, Greater);
    test("22/7", Nearest, 3, Less);

    test("-22/7", Floor, -4, Less);
    test("-22/7", Ceiling, -3, Greater);
    test("-22/7", Down, -3, Greater);
    test("-22/7", Up, -4, Less);
    test("-22/7", Nearest, -3, Greater);

    test("7/2", Floor, 3, Less);
    test("7/2", Ceiling, 4, Greater);
    test("7/2", Down, 3, Less);
    test("7/2", Up, 4, Greater);
    test("7/2", Nearest, 4, Greater);

    test("9/2", Floor, 4, Less);
    test("9/2", Ceiling, 5, Greater);
    test("9/2", Down, 4, Less);
    test("9/2", Up, 5, Greater);
    test("9/2", Nearest, 4, Less);

    test("-1000000000000", Ceiling, i32::MIN, Greater);
    test("-1000000000000", Down, i32::MIN, Greater);
    test("-1000000000000", Nearest, i32::MIN, Greater);

    test("1000000000000", Floor, i32::MAX, Less);
    test("1000000000000", Down, i32::MAX, Less);
    test("1000000000000", Nearest, i32::MAX, Less);
}

#[test]
fn rounding_from_rational_fail() {
    let x = Rational::from_str("22/7").unwrap();
    assert_panic!(u32::rounding_from(&x, Exact));

    let x = Rational::from_str("-123").unwrap();
    assert_panic!(u32::rounding_from(&x, Floor));
    assert_panic!(u32::rounding_from(&x, Up));
    assert_panic!(u32::rounding_from(&x, Exact));

    let x = Rational::from_str("1000000000000").unwrap();
    assert_panic!(u32::rounding_from(&x, Ceiling));
    assert_panic!(u32::rounding_from(&x, Up));
    assert_panic!(u32::rounding_from(&x, Exact));

    let x = Rational::from_str("22/7").unwrap();
    assert_panic!(i32::rounding_from(&x, Exact));

    let x = Rational::from_str("-1000000000000").unwrap();
    assert_panic!(i32::rounding_from(&x, Floor));
    assert_panic!(i32::rounding_from(&x, Up));
    assert_panic!(i32::rounding_from(&x, Exact));

    let x = Rational::from_str("1000000000000").unwrap();
    assert_panic!(i32::rounding_from(&x, Ceiling));
    assert_panic!(i32::rounding_from(&x, Up));
    assert_panic!(i32::rounding_from(&x, Exact));
}

fn try_from_rational_properties_helper<
    T: for<'a> TryFrom<&'a Rational> + for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>()
where
    Rational: TryFrom<T> + PartialOrd<T>,
{
    rational_gen().test_properties(|x| {
        let p_x = T::try_from(&x);
        assert_eq!(p_x.is_ok(), x >= T::MIN && x <= T::MAX && x.is_integer());
        assert_eq!(p_x.is_ok(), T::convertible_from(&x));
        if let Ok(n) = p_x {
            assert_eq!(n.to_string(), x.to_string());
            assert_eq!(T::exact_from(&x), n);
            assert!(PartialEq::<Rational>::eq(&Rational::exact_from(n), &x));
        }
    });
}

#[test]
fn try_from_rational_properties() {
    apply_fn_to_primitive_ints!(try_from_rational_properties_helper);
}

fn convertible_from_rational_properties_helper<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>()
where
    Rational: PartialOrd<T>,
{
    rational_gen().test_properties(|x| {
        let convertible = T::convertible_from(&x);
        assert_eq!(convertible, x >= T::MIN && x <= T::MAX && x.is_integer());
    });
}

#[test]
fn convertible_from_rational_properties() {
    apply_fn_to_primitive_ints!(convertible_from_rational_properties_helper);
}

fn rounding_from_rational_properties_helper<
    T: for<'a> ConvertibleFrom<&'a Rational>
        + PartialEq<Integer>
        + PartialOrd<Rational>
        + PrimitiveInt
        + for<'a> RoundingFrom<&'a Rational>,
>()
where
    Rational: From<T> + PartialOrd<T>,
{
    rational_rounding_mode_pair_gen_var_3::<T>().test_properties(|(x, rm)| {
        let (n, o) = T::rounding_from(&x, rm);
        if x >= T::MIN && x <= T::MAX {
            assert!((Rational::from(n) - &x).lt_abs(&1));
        }

        assert_eq!(n.partial_cmp(&x), Some(o));
        match (x >= T::ZERO, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }
    });

    // TODO use range
    rational_gen_var_3().test_properties(|x| {
        if x < T::MIN || x > T::MAX {
            return;
        }
        let floor = T::rounding_from(&x, Floor);
        assert_eq!(floor.0, (&x).floor());
        assert!(floor.0 <= x);
        if floor.0 < T::MAX {
            assert!(floor.0 + T::ONE > x);
        }
        let ceiling = T::rounding_from(&x, Ceiling);
        assert_eq!(ceiling.0, (&x).ceiling());
        assert!(ceiling.0 >= x);
        if ceiling.0 > T::MIN {
            assert!(ceiling.0 - T::ONE < x);
        }

        if x >= T::ZERO {
            assert_eq!(T::rounding_from(&x, Down), floor);
            assert_eq!(T::rounding_from(&x, Up), ceiling);
        } else {
            assert_eq!(T::rounding_from(&x, Down), ceiling);
            assert_eq!(T::rounding_from(&x, Up), floor);
        }

        let nearest = T::rounding_from(&x, Nearest);
        assert!(nearest == floor || nearest == ceiling);
        assert!((Rational::from(nearest.0) - x).le_abs(&Rational::ONE_HALF));
    });
}

fn rounding_from_rational_properties_unsigned_helper<
    T: PrimitiveUnsigned + for<'a> RoundingFrom<&'a Rational>,
>()
where
    Rational: From<T>,
{
    unsigned_gen::<T>().test_properties(|n| {
        let no = (n, Equal);
        let x = Rational::from(n);
        assert_eq!(T::rounding_from(&x, Floor), no);
        assert_eq!(T::rounding_from(&x, Down), no);
        assert_eq!(T::rounding_from(&x, Ceiling), no);
        assert_eq!(T::rounding_from(&x, Up), no);
        assert_eq!(T::rounding_from(&x, Nearest), no);
        assert_eq!(T::rounding_from(&x, Exact), no);
    });
}

fn rounding_from_rational_properties_signed_helper<
    T: PrimitiveSigned + for<'a> RoundingFrom<&'a Rational>,
>()
where
    Rational: From<T>,
{
    signed_gen::<T>().test_properties(|n| {
        let no = (n, Equal);
        let x = Rational::from(n);
        assert_eq!(T::rounding_from(&x, Floor), no);
        assert_eq!(T::rounding_from(&x, Down), no);
        assert_eq!(T::rounding_from(&x, Ceiling), no);
        assert_eq!(T::rounding_from(&x, Up), no);
        assert_eq!(T::rounding_from(&x, Nearest), no);
        assert_eq!(T::rounding_from(&x, Exact), no);
    });
}

#[test]
fn rounding_from_rational_properties() {
    apply_fn_to_primitive_ints!(rounding_from_rational_properties_helper);
    apply_fn_to_unsigneds!(rounding_from_rational_properties_unsigned_helper);
    apply_fn_to_signeds!(rounding_from_rational_properties_signed_helper);
}
