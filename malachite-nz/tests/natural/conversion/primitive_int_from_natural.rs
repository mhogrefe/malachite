// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::conversion::primitive_int_from_natural::{
    SignedFromNaturalError, UnsignedFromNaturalError,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use rug;
use std::str::FromStr;

#[test]
fn test_u32_try_from_natural() {
    let test = |n, out| {
        assert_eq!(u32::try_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .to_u32()
                .ok_or(UnsignedFromNaturalError),
            out
        );
    };
    test("0", Ok(0));
    test("123", Ok(123));
    test("1000000000000", Err(UnsignedFromNaturalError));
    test("4294967295", Ok(u32::MAX));
    test("4294967296", Err(UnsignedFromNaturalError));
}

#[test]
fn test_u32_exact_from_natural() {
    let test = |n, out| {
        assert_eq!(u32::exact_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32().unwrap(), out);
    };
    test("0", 0);
    test("123", 123);
    test("4294967295", u32::MAX);
}

#[test]
#[should_panic]
fn u32_exact_from_natural_fail_1() {
    u32::exact_from(&Natural::from_str("1000000000000").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_natural_fail_2() {
    u32::exact_from(&Natural::from_str("4294967296").unwrap());
}

#[test]
fn test_u32_wrapping_from_natural() {
    let test = |n, out| {
        assert_eq!(u32::wrapping_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 3567587328);
    test("4294967296", 0);
    test("4294967297", 1);
}

#[test]
fn test_u32_saturating_from_natural() {
    let test = |n, out| {
        assert_eq!(u32::saturating_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", u32::MAX);
    test("4294967296", u32::MAX);
    test("4294967297", u32::MAX);
}

#[test]
fn test_u32_overflowing_from_natural() {
    let test = |n, out| {
        assert_eq!(u32::overflowing_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000", (3567587328, true));
    test("4294967296", (0, true));
    test("4294967297", (1, true));
}

#[test]
fn test_u32_convertible_from_natural() {
    let test = |n, out| {
        assert_eq!(u32::convertible_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", true);
    test("123", true);
    test("1000000000000", false);
    test("4294967295", true);
    test("4294967296", false);
}

#[test]
fn test_u64_try_from_natural() {
    let test = |n, out| {
        assert_eq!(u64::try_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .to_u64()
                .ok_or(UnsignedFromNaturalError),
            out
        );
    };
    test("0", Ok(0));
    test("123", Ok(123));
    test("1000000000000", Ok(1000000000000));
    test("1000000000000000000000000", Err(UnsignedFromNaturalError));
    test("18446744073709551615", Ok(u64::MAX));
    test("18446744073709551616", Err(UnsignedFromNaturalError));
}

#[test]
fn test_u64_exact_from_natural() {
    let test = |n, out| {
        assert_eq!(u64::exact_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u64().unwrap(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 1000000000000);
    test("18446744073709551615", u64::MAX);
}

#[test]
#[should_panic]
fn u64_exact_from_natural_fail_1() {
    u64::exact_from(&Natural::from_str("1000000000000000000000000").unwrap());
}

#[test]
#[should_panic]
fn u64_exact_from_natural_fail_2() {
    u64::exact_from(&Natural::from_str("18446744073709551616").unwrap());
}

#[test]
fn test_u64_wrapping_from_natural() {
    let test = |n, out| {
        assert_eq!(u64::wrapping_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", 2003764205206896640);
    test("18446744073709551616", 0);
    test("18446744073709551617", 1);
}

#[test]
fn test_u64_saturating_from_natural() {
    let test = |n, out| {
        assert_eq!(u64::saturating_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", u64::MAX);
    test("18446744073709551616", u64::MAX);
    test("18446744073709551617", u64::MAX);
}

#[test]
fn test_u64_overflowing_from_natural() {
    let test = |n, out| {
        assert_eq!(u64::overflowing_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000000000000000", (2003764205206896640, true));
    test("18446744073709551616", (0, true));
    test("18446744073709551617", (1, true));
}

#[test]
fn test_u64_convertible_from_natural() {
    let test = |n, out| {
        assert_eq!(u64::convertible_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", true);
    test("123", true);
    test("1000000000000000000000000", false);
    test("18446744073709551615", true);
    test("18446744073709551616", false);
}

#[test]
fn test_i32_try_from_natural() {
    let test = |n, out| {
        assert_eq!(i32::try_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .to_i32()
                .ok_or(SignedFromNaturalError),
            out
        );
    };
    test("0", Ok(0));
    test("123", Ok(123));
    test("1000000000000", Err(SignedFromNaturalError));
    test("2147483647", Ok(i32::MAX));
    test("2147483648", Err(SignedFromNaturalError));
}

#[test]
fn test_i32_exact_from_natural() {
    let test = |n, out| {
        assert_eq!(i32::exact_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32().unwrap(), out);
    };
    test("0", 0);
    test("123", 123);
    test("2147483647", i32::MAX);
}

#[test]
#[should_panic]
fn i32_exact_from_natural_fail_1() {
    i32::exact_from(&Natural::from_str("1000000000000").unwrap());
}

#[test]
#[should_panic]
fn i32_exact_from_natural_fail_2() {
    i32::exact_from(&Natural::from_str("2147483648").unwrap());
}

#[test]
fn test_i32_wrapping_from_natural() {
    let test = |n, out| {
        assert_eq!(i32::wrapping_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", -727379968);
    test("2147483648", -0x80000000);
    test("2147483649", -0x7fffffff);
}

#[test]
fn test_i32_saturating_from_natural() {
    let test = |n, out| {
        assert_eq!(i32::saturating_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 0x7fffffff);
    test("2147483648", 0x7fffffff);
    test("2147483649", 0x7fffffff);
}

#[test]
fn test_i32_overflowing_from_natural() {
    let test = |n, out| {
        assert_eq!(i32::overflowing_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000", (-727379968, true));
    test("2147483648", (-0x80000000, true));
    test("2147483649", (-0x7fffffff, true));
}

#[test]
fn test_i32_convertible_from_natural() {
    let test = |n, out| {
        assert_eq!(i32::convertible_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", true);
    test("123", true);
    test("1000000000000", false);
    test("2147483647", true);
    test("2147483648", false);
}

#[test]
fn test_i64_try_from_natural() {
    let test = |n, out: Result<i64, SignedFromNaturalError>| {
        assert_eq!(i64::try_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .to_i64()
                .ok_or(SignedFromNaturalError),
            out
        );
    };
    test("0", Ok(0));
    test("123", Ok(123));
    test("1000000000000000000000000", Err(SignedFromNaturalError));
    test("9223372036854775807", Ok(i64::MAX));
    test("9223372036854775808", Err(SignedFromNaturalError));
}

#[test]
fn test_i64_exact_from_natural() {
    let test = |n, out| {
        assert_eq!(i64::exact_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i64().unwrap(), out);
    };
    test("0", 0);
    test("123", 123);
    test("9223372036854775807", i64::MAX);
}

#[test]
#[should_panic]
fn i64_exact_from_natural_fail_1() {
    i64::exact_from(&Natural::from_str("1000000000000000000000000").unwrap());
}

#[test]
#[should_panic]
fn i64_exact_from_natural_fail_2() {
    i64::exact_from(&Natural::from_str("9223372036854775808").unwrap());
}

#[test]
fn test_i64_wrapping_from_natural() {
    let test = |n, out| {
        assert_eq!(i64::wrapping_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", 2003764205206896640);
    test("9223372036854775808", -0x8000000000000000);
    test("9223372036854775809", -0x7fffffffffffffff);
}

#[test]
fn test_i64_saturating_from_natural() {
    let test = |n, out| {
        assert_eq!(i64::saturating_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", 0x7fffffffffffffff);
    test("9223372036854775808", 0x7fffffffffffffff);
    test("9223372036854775809", 0x7fffffffffffffff);
}

#[test]
fn test_i64_overflowing_from_natural() {
    let test = |n, out| {
        assert_eq!(i64::overflowing_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000000000000000", (2003764205206896640, true));
    test("9223372036854775808", (-0x8000000000000000, true));
    test("9223372036854775809", (-0x7fffffffffffffff, true));
}

#[test]
fn test_i64_convertible_from_natural() {
    let test = |n, out| {
        assert_eq!(i64::convertible_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", true);
    test("123", true);
    test("1000000000000000000000000", false);
    test("9223372036854775807", true);
    test("9223372036854775808", false);
}

fn primitive_int_properties<
    T: for<'a> ConvertibleFrom<&'a Natural>
        + for<'a> OverflowingFrom<&'a Natural>
        + PartialEq<Natural>
        + PartialOrd<Natural>
        + PrimitiveInt
        + for<'a> SaturatingFrom<&'a Natural>
        + for<'a> WrappingFrom<&'a Natural>,
>()
where
    Natural: PartialOrd<T>,
{
    natural_gen().test_properties(|x| {
        let result = T::wrapping_from(&x);
        assert_eq!(result, T::overflowing_from(&x).0);

        let result = T::saturating_from(&x);
        assert!(result <= x);
        assert_eq!(result == x, T::convertible_from(&x));

        let result = T::overflowing_from(&x);
        assert_eq!(result, (T::wrapping_from(&x), !T::convertible_from(&x)));

        let convertible = T::convertible_from(&x);
        assert_eq!(convertible, x >= T::MIN && x <= T::MAX);
    });
}

fn unsigned_properties<
    T: for<'a> TryFrom<&'a Natural, Error = UnsignedFromNaturalError>
        + for<'a> OverflowingFrom<&'a Natural>
        + PartialEq<Natural>
        + PrimitiveUnsigned
        + for<'a> WrappingFrom<&'a Natural>,
>()
where
    Natural: From<T>,
{
    natural_gen().test_properties(|x| {
        let result = T::try_from(&x);
        if x.significant_bits() <= T::WIDTH {
            assert_eq!(Natural::from(result.unwrap()), x);
            assert_eq!(result, Ok(T::wrapping_from(&x)));
            assert_eq!(result, Ok(T::exact_from(&x)));
        } else {
            assert!(result.is_err());
        }
        assert_eq!(result.is_err(), T::overflowing_from(&x).1);

        let result = T::wrapping_from(&x);
        assert_eq!(result, T::exact_from(&(&x).mod_power_of_2(T::WIDTH)));
    });
}

fn signed_properties<
    T: for<'a> TryFrom<&'a Natural, Error = SignedFromNaturalError>
        + for<'a> OverflowingFrom<&'a Natural>
        + PartialEq<Natural>
        + PrimitiveSigned
        + for<'a> WrappingFrom<&'a Natural>,
>()
where
    Natural: ExactFrom<T>,
{
    natural_gen().test_properties(|x| {
        let result = T::try_from(&x);
        if x >= 0 && x.significant_bits() < T::WIDTH {
            assert_eq!(Natural::exact_from(result.unwrap()), x);
            assert_eq!(result, Ok(T::wrapping_from(&x)));
            assert_eq!(result, Ok(T::exact_from(&x)));
        } else {
            assert!(result.is_err());
        }
        assert_eq!(result.is_err(), T::overflowing_from(&x).1);
    });
}

#[test]
fn primitive_int_from_natural_properties() {
    apply_fn_to_primitive_ints!(primitive_int_properties);
    apply_fn_to_unsigneds!(unsigned_properties);
    apply_fn_to_signeds!(signed_properties);
}
