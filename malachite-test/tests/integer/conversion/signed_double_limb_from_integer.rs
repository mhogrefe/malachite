use std::str::FromStr;

use malachite_base::comparison::{Max, Min};
use malachite_base::conversion::{CheckedFrom, OverflowingFrom, SaturatingFrom, WrappingFrom};
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::traits::{ModPowerOfTwo, One, PartialOrdAbs, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, SignedDoubleLimb, SignedLimb};

use common::test_properties;
use malachite_test::inputs::integer::integers;

#[test]
fn test_signed_double_limb_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(
            SignedDoubleLimb::checked_from(Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            SignedDoubleLimb::checked_from(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", Some(-123));
    test("1000000000000", Some(1_000_000_000_000));
    test("-1000000000000", Some(-1_000_000_000_000));
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000000000000", None);
        test("-1000000000000000000000", None);
        test("2147483647", Some(SignedLimb::MAX.into()));
        test("2147483648", Some(-SignedDoubleLimb::from(SignedLimb::MIN)));
        test("-2147483648", Some(SignedLimb::MIN.into()));
        test(
            "-2147483649",
            Some(SignedDoubleLimb::from(SignedLimb::MIN) - 1),
        );
        test("9223372036854775807", Some(SignedDoubleLimb::MAX));
        test("9223372036854775808", None);
        test("-9223372036854775808", Some(SignedDoubleLimb::MIN));
        test("-9223372036854775809", None);
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test("1000000000000000000000", Some(1000000000000000000000));
        test("-1000000000000000000000", Some(-1000000000000000000000));
        test("9223372036854775807", Some(SignedLimb::MAX.into()));
        test(
            "9223372036854775808",
            Some(-SignedDoubleLimb::from(SignedLimb::MIN)),
        );
        test("-9223372036854775808", Some(SignedLimb::MIN.into()));
        test(
            "-9223372036854775809",
            Some(SignedDoubleLimb::from(SignedLimb::MIN) - 1),
        );
        test(
            "170141183460469231731687303715884105727",
            Some(SignedDoubleLimb::MAX),
        );
        test("170141183460469231731687303715884105728", None);
        test(
            "-170141183460469231731687303715884105728",
            Some(SignedDoubleLimb::MIN),
        );
        test("-170141183460469231731687303715884105729", None);
    }
}

#[test]
fn test_signed_double_limb_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(
            SignedDoubleLimb::wrapping_from(Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            SignedDoubleLimb::wrapping_from(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", 0);
    test("123", 123);
    test("-123", -123);
    test("1000000000000", 1_000_000_000_000);
    test("-1000000000000", -1_000_000_000_000);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000000000000", 3_875_820_019_684_212_736);
        test("-1000000000000000000000", -3_875_820_019_684_212_736);
        test("2147483647", SignedLimb::MAX.into());
        test("2147483648", -SignedDoubleLimb::from(SignedLimb::MIN));
        test("-2147483648", SignedLimb::MIN.into());
        test("-2147483649", SignedDoubleLimb::from(SignedLimb::MIN) - 1);
        test("9223372036854775807", SignedDoubleLimb::MAX);
        test("9223372036854775808", SignedDoubleLimb::MIN);
        test("-9223372036854775808", SignedDoubleLimb::MIN);
        test("-9223372036854775809", SignedDoubleLimb::MAX);
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test("1000000000000000000000", 1000000000000000000000);
        test("-1000000000000000000000", -1000000000000000000000);
        test("9223372036854775807", SignedLimb::MAX.into());
        test(
            "9223372036854775808",
            -SignedDoubleLimb::from(SignedLimb::MIN),
        );
        test("-9223372036854775808", SignedLimb::MIN.into());
        test(
            "-9223372036854775809",
            SignedDoubleLimb::from(SignedLimb::MIN) - 1,
        );
        test(
            "170141183460469231731687303715884105727",
            SignedDoubleLimb::MAX,
        );
        test(
            "170141183460469231731687303715884105728",
            SignedDoubleLimb::MIN,
        );
        test(
            "-170141183460469231731687303715884105728",
            SignedDoubleLimb::MIN,
        );
        test(
            "-170141183460469231731687303715884105729",
            SignedDoubleLimb::MAX,
        );
    }
}

#[test]
fn test_signed_double_limb_saturating_from_integer() {
    let test = |n, out| {
        assert_eq!(
            SignedDoubleLimb::saturating_from(Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            SignedDoubleLimb::saturating_from(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", 0);
    test("123", 123);
    test("-123", -123);
    test("1000000000000", 1_000_000_000_000);
    test("-1000000000000", -1_000_000_000_000);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000000000000", SignedDoubleLimb::MAX);
        test("-1000000000000000000000", SignedDoubleLimb::MIN);
        test("2147483647", SignedLimb::MAX.into());
        test("2147483648", -SignedDoubleLimb::from(SignedLimb::MIN));
        test("-2147483648", SignedLimb::MIN.into());
        test("-2147483649", SignedDoubleLimb::from(SignedLimb::MIN) - 1);
        test("9223372036854775807", SignedDoubleLimb::MAX);
        test("9223372036854775808", SignedDoubleLimb::MAX);
        test("-9223372036854775808", SignedDoubleLimb::MIN);
        test("-9223372036854775809", SignedDoubleLimb::MIN);
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test("1000000000000000000000", 1000000000000000000000);
        test("-1000000000000000000000", -1000000000000000000000);
        test("9223372036854775807", SignedLimb::MAX.into());
        test(
            "9223372036854775808",
            -SignedDoubleLimb::from(SignedLimb::MIN),
        );
        test("-9223372036854775808", SignedLimb::MIN.into());
        test(
            "-9223372036854775809",
            SignedDoubleLimb::from(SignedLimb::MIN) - 1,
        );
        test(
            "170141183460469231731687303715884105727",
            SignedDoubleLimb::MAX,
        );
        test(
            "170141183460469231731687303715884105728",
            SignedDoubleLimb::MAX,
        );
        test(
            "-170141183460469231731687303715884105728",
            SignedDoubleLimb::MIN,
        );
        test(
            "-170141183460469231731687303715884105729",
            SignedDoubleLimb::MIN,
        );
    }
}

#[test]
fn test_signed_double_limb_overflowing_from_integer() {
    let test = |n, out| {
        assert_eq!(
            SignedDoubleLimb::overflowing_from(Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            SignedDoubleLimb::overflowing_from(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", (0, false));
    test("123", (123, false));
    test("-123", (-123, false));
    test("1000000000000", (1_000_000_000_000, false));
    test("-1000000000000", (-1_000_000_000_000, false));
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000000000000", (3_875_820_019_684_212_736, true));
        test(
            "-1000000000000000000000",
            (-3_875_820_019_684_212_736, true),
        );
        test("2147483647", (SignedLimb::MAX.into(), false));
        test(
            "2147483648",
            (-SignedDoubleLimb::from(SignedLimb::MIN), false),
        );
        test("-2147483648", (SignedLimb::MIN.into(), false));
        test(
            "-2147483649",
            (SignedDoubleLimb::from(SignedLimb::MIN) - 1, false),
        );
        test("9223372036854775807", (SignedDoubleLimb::MAX, false));
        test("9223372036854775808", (SignedDoubleLimb::MIN, true));
        test("-9223372036854775808", (SignedDoubleLimb::MIN, false));
        test("-9223372036854775809", (SignedDoubleLimb::MAX, true));
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test("1000000000000000000000", (1000000000000000000000, false));
        test("-1000000000000000000000", (-1000000000000000000000, false));
        test("9223372036854775807", (SignedLimb::MAX.into(), false));
        test(
            "9223372036854775808",
            (-SignedDoubleLimb::from(SignedLimb::MIN), false),
        );
        test("-9223372036854775808", (SignedLimb::MIN.into(), false));
        test(
            "-9223372036854775809",
            (SignedDoubleLimb::from(SignedLimb::MIN) - 1, false),
        );
        test(
            "170141183460469231731687303715884105727",
            (SignedDoubleLimb::MAX, false),
        );
        test(
            "170141183460469231731687303715884105728",
            (SignedDoubleLimb::MIN, true),
        );
        test(
            "-170141183460469231731687303715884105728",
            (SignedDoubleLimb::MIN, false),
        );
        test(
            "-170141183460469231731687303715884105729",
            (SignedDoubleLimb::MAX, true),
        );
    }
}

#[test]
fn signed_double_limb_checked_from_integer_properties() {
    test_properties(integers, |x| {
        let result = SignedDoubleLimb::checked_from(x);
        assert_eq!(SignedDoubleLimb::checked_from(x.clone()), result);
        if x.significant_bits() < u64::from(SignedDoubleLimb::WIDTH)
            || *x == -(Natural::ONE << (SignedDoubleLimb::WIDTH - 1))
        {
            assert_eq!(Integer::from(result.unwrap()), *x);
            assert_eq!(result, Some(SignedDoubleLimb::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
        assert_eq!(result.is_none(), SignedDoubleLimb::overflowing_from(x).1)
    });
}

#[test]
fn signed_double_limb_wrapping_from_integer_properties() {
    test_properties(integers, |x| {
        let result = SignedDoubleLimb::wrapping_from(x);
        assert_eq!(SignedDoubleLimb::wrapping_from(x.clone()), result);
        assert_eq!(result.wrapping_neg(), SignedDoubleLimb::wrapping_from(&-x));
        assert_eq!(
            result,
            SignedDoubleLimb::wrapping_from(
                DoubleLimb::checked_from(&x.mod_power_of_two(DoubleLimb::WIDTH.into())).unwrap()
            )
        );
        assert_eq!(result, SignedDoubleLimb::overflowing_from(x).0);
    });
}

#[test]
fn signed_double_limb_saturating_from_integer_properties() {
    test_properties(integers, |x| {
        let result = SignedDoubleLimb::saturating_from(x);
        assert_eq!(SignedDoubleLimb::saturating_from(x.clone()), result);
        let result = Integer::from(result);
        assert!(result.le_abs(x));
        assert_eq!(result == *x, SignedDoubleLimb::checked_from(x).is_some());
    });
}

#[test]
fn signed_double_limb_overflowing_from_integer_properties() {
    test_properties(integers, |x| {
        let result = SignedDoubleLimb::overflowing_from(x);
        assert_eq!(SignedDoubleLimb::overflowing_from(x.clone()), result);
        assert_eq!(
            result,
            (
                SignedDoubleLimb::wrapping_from(x),
                SignedDoubleLimb::checked_from(x).is_none()
            )
        );
    });
}
