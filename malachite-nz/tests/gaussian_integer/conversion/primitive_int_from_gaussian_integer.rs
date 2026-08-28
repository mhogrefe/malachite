// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, IsReal};
use malachite_base::strings::ToDebugString;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::gaussian_integer::conversion::primitive_int_from_gaussian_integer::*;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use std::fmt::Debug;
use std::str::FromStr;

#[test]
fn test_try_from_gaussian_integer() {
    fn test<T>(s: &str, out: &str)
    where
        T: Debug
            + for<'a> TryFrom<&'a GaussianInteger, Error = PrimitiveIntFromGaussianIntegerError>,
    {
        let x = GaussianInteger::from_str(s).unwrap();
        assert_eq!(T::try_from(&x).to_debug_string(), out);
    }
    test::<u8>("0", "Ok(0)");
    test::<u8>("123", "Ok(123)");
    test::<u8>("-123", "Err(PrimitiveIntFromGaussianIntegerError)");
    test::<u8>("1000", "Err(PrimitiveIntFromGaussianIntegerError)");
    test::<u8>("i", "Err(PrimitiveIntFromGaussianIntegerError)");
    test::<u8>("2-3i", "Err(PrimitiveIntFromGaussianIntegerError)");

    test::<i8>("0", "Ok(0)");
    test::<i8>("123", "Ok(123)");
    test::<i8>("-123", "Ok(-123)");
    test::<i8>("-1000", "Err(PrimitiveIntFromGaussianIntegerError)");
    test::<i8>("i", "Err(PrimitiveIntFromGaussianIntegerError)");
    test::<i8>("2-3i", "Err(PrimitiveIntFromGaussianIntegerError)");

    test::<u32>("1000000000000", "Err(PrimitiveIntFromGaussianIntegerError)");
    test::<u64>("1000000000000", "Ok(1000000000000)");
    test::<i32>(
        "-1000000000000",
        "Err(PrimitiveIntFromGaussianIntegerError)",
    );
    test::<i64>("-1000000000000", "Ok(-1000000000000)");
}

#[test]
fn test_exact_from_gaussian_integer() {
    let x = GaussianInteger::from_str("123").unwrap();
    assert_eq!(u8::exact_from(&x), 123);
    assert_eq!(i8::exact_from(&x), 123);

    let x = GaussianInteger::from_str("-123").unwrap();
    assert_eq!(i8::exact_from(&x), -123);
}

#[test]
#[should_panic]
fn unsigned_exact_from_gaussian_integer_fail() {
    u8::exact_from(&GaussianInteger::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn signed_exact_from_gaussian_integer_fail() {
    i8::exact_from(&GaussianInteger::from_str("2-3i").unwrap());
}

#[test]
fn test_convertible_from_gaussian_integer() {
    fn test<T: for<'a> ConvertibleFrom<&'a GaussianInteger>>(s: &str, out: bool) {
        let x = GaussianInteger::from_str(s).unwrap();
        assert_eq!(T::convertible_from(&x), out);
    }
    test::<u8>("0", true);
    test::<u8>("123", true);
    test::<u8>("-123", false);
    test::<u8>("1000", false);
    test::<u8>("i", false);
    test::<u8>("2-3i", false);

    test::<i8>("123", true);
    test::<i8>("-123", true);
    test::<i8>("-1000", false);
    test::<i8>("i", false);

    test::<u32>("1000000000000", false);
    test::<u64>("1000000000000", true);
    test::<i32>("-1000000000000", false);
    test::<i64>("-1000000000000", true);
}

fn try_from_gaussian_integer_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    T: for<'a> TryFrom<&'a GaussianInteger, Error = PrimitiveIntFromGaussianIntegerError>
        + for<'a> TryFrom<&'a Integer>
        + for<'a> ConvertibleFrom<&'a GaussianInteger>
        + for<'a> ConvertibleFrom<&'a Integer>,
    Integer: From<T>,
{
    gaussian_integer_gen().test_properties(|x| {
        let ot = T::try_from(&x);
        assert_eq!(T::convertible_from(&x), ot.is_ok());
        assert_eq!(ot.is_ok(), x.is_real() && T::convertible_from(&x.real));
        if let Ok(t) = ot {
            assert_eq!(t, T::try_from(&x.real).ok().unwrap());
            assert_eq!(GaussianInteger::from(t), x);
        }
    });
}

fn try_from_gaussian_integer_properties_helper_signed<T: PrimitiveSigned>()
where
    T: for<'a> TryFrom<&'a GaussianInteger, Error = PrimitiveIntFromGaussianIntegerError>
        + for<'a> TryFrom<&'a Integer>
        + for<'a> ConvertibleFrom<&'a GaussianInteger>
        + for<'a> ConvertibleFrom<&'a Integer>,
    Integer: From<T>,
{
    gaussian_integer_gen().test_properties(|x| {
        let ot = T::try_from(&x);
        assert_eq!(T::convertible_from(&x), ot.is_ok());
        assert_eq!(ot.is_ok(), x.is_real() && T::convertible_from(&x.real));
        if let Ok(t) = ot {
            assert_eq!(t, T::try_from(&x.real).ok().unwrap());
            assert_eq!(GaussianInteger::from(t), x);
        }
    });
}

#[test]
fn try_from_gaussian_integer_properties() {
    apply_fn_to_unsigneds!(try_from_gaussian_integer_properties_helper_unsigned);
    apply_fn_to_signeds!(try_from_gaussian_integer_properties_helper_signed);
}
