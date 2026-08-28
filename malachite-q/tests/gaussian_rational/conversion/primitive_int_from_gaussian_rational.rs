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
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::gaussian_rational::conversion::primitive_int_from_gaussian_rational::*;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::fmt::Debug;
use std::str::FromStr;

#[test]
fn test_try_from_gaussian_rational() {
    fn test<T>(s: &str, out: &str)
    where
        T: Debug
            + for<'a> TryFrom<&'a GaussianRational, Error = PrimitiveIntFromGaussianRationalError>,
    {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(T::try_from(&x).to_debug_string(), out);
    }
    test::<u8>("0", "Ok(0)");
    test::<u8>("123", "Ok(123)");
    test::<u8>("-123", "Err(PrimitiveIntFromGaussianRationalError)");
    test::<u8>("1000", "Err(PrimitiveIntFromGaussianRationalError)");
    test::<u8>("22/7", "Err(PrimitiveIntFromGaussianRationalError)");
    test::<u8>("i", "Err(PrimitiveIntFromGaussianRationalError)");
    test::<u8>("2-3i", "Err(PrimitiveIntFromGaussianRationalError)");

    test::<i8>("0", "Ok(0)");
    test::<i8>("123", "Ok(123)");
    test::<i8>("-123", "Ok(-123)");
    test::<i8>("-1000", "Err(PrimitiveIntFromGaussianRationalError)");
    test::<i8>("-22/7", "Err(PrimitiveIntFromGaussianRationalError)");
    test::<i8>("i/2", "Err(PrimitiveIntFromGaussianRationalError)");
    test::<i8>("2/3-5i/6", "Err(PrimitiveIntFromGaussianRationalError)");

    test::<u32>(
        "1000000000000",
        "Err(PrimitiveIntFromGaussianRationalError)",
    );
    test::<u64>("1000000000000", "Ok(1000000000000)");
    test::<i32>(
        "-1000000000000",
        "Err(PrimitiveIntFromGaussianRationalError)",
    );
    test::<i64>("-1000000000000", "Ok(-1000000000000)");
}

#[test]
fn test_exact_from_gaussian_rational() {
    let x = GaussianRational::from_str("123").unwrap();
    assert_eq!(u8::exact_from(&x), 123);
    assert_eq!(i8::exact_from(&x), 123);

    let x = GaussianRational::from_str("-123").unwrap();
    assert_eq!(i8::exact_from(&x), -123);
}

#[test]
#[should_panic]
fn unsigned_exact_from_gaussian_rational_fail() {
    u8::exact_from(&GaussianRational::from_str("22/7").unwrap());
}

#[test]
#[should_panic]
fn signed_exact_from_gaussian_rational_fail() {
    i8::exact_from(&GaussianRational::from_str("2-3i").unwrap());
}

#[test]
fn test_convertible_from_gaussian_rational() {
    fn test<T: for<'a> ConvertibleFrom<&'a GaussianRational>>(s: &str, out: bool) {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(T::convertible_from(&x), out);
    }
    test::<u8>("0", true);
    test::<u8>("123", true);
    test::<u8>("-123", false);
    test::<u8>("1000", false);
    test::<u8>("22/7", false);
    test::<u8>("i", false);
    test::<u8>("2-3i", false);

    test::<i8>("123", true);
    test::<i8>("-123", true);
    test::<i8>("-1000", false);
    test::<i8>("-22/7", false);
    test::<i8>("i/2", false);

    test::<u32>("1000000000000", false);
    test::<u64>("1000000000000", true);
    test::<i32>("-1000000000000", false);
    test::<i64>("-1000000000000", true);
}

fn try_from_gaussian_rational_properties_helper_unsigned<T>()
where
    T: PrimitiveUnsigned
        + for<'a> TryFrom<&'a GaussianRational, Error = PrimitiveIntFromGaussianRationalError>
        + for<'a> TryFrom<&'a Rational>
        + for<'a> ConvertibleFrom<&'a GaussianRational>
        + for<'a> ConvertibleFrom<&'a Rational>,
    GaussianRational: From<T>,
{
    gaussian_rational_gen().test_properties(|x| {
        let ot = T::try_from(&x);
        assert_eq!(T::convertible_from(&x), ot.is_ok());
        assert_eq!(ot.is_ok(), x.is_real() && T::convertible_from(&x.real));
        if let Ok(t) = ot {
            assert_eq!(t, T::try_from(&x.real).ok().unwrap());
            assert_eq!(GaussianRational::from(t), x);
        }
    });
}

fn try_from_gaussian_rational_properties_helper_signed<T>()
where
    T: PrimitiveSigned
        + for<'a> TryFrom<&'a GaussianRational, Error = PrimitiveIntFromGaussianRationalError>
        + for<'a> TryFrom<&'a Rational>
        + for<'a> ConvertibleFrom<&'a GaussianRational>
        + for<'a> ConvertibleFrom<&'a Rational>,
    GaussianRational: From<T>,
{
    gaussian_rational_gen().test_properties(|x| {
        let ot = T::try_from(&x);
        assert_eq!(T::convertible_from(&x), ot.is_ok());
        assert_eq!(ot.is_ok(), x.is_real() && T::convertible_from(&x.real));
        if let Ok(t) = ot {
            assert_eq!(t, T::try_from(&x.real).ok().unwrap());
            assert_eq!(GaussianRational::from(t), x);
        }
    });
}

#[test]
fn try_from_gaussian_rational_properties() {
    apply_fn_to_unsigneds!(try_from_gaussian_rational_properties_helper_unsigned);
    apply_fn_to_signeds!(try_from_gaussian_rational_properties_helper_signed);
}
