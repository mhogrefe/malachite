use common::test_properties;
use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, Sign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_test::inputs::integer::integers;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_double_limb_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(DoubleLimb::checked_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(
            DoubleLimb::checked_from(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", None);
    test("1000000000000", Some(1_000_000_000_000));
    test("-1000000000000", None);
    test("-1000000000000000000000", None);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("4294967295", Some(Limb::MAX.into()));
        test("4294967296", Some(DoubleLimb::from(Limb::MAX) + 1));
        test("18446744073709551615", Some(DoubleLimb::MAX));
        test("18446744073709551616", None);
        test("1000000000000000000000", None);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("18446744073709551615", Some(Limb::MAX.into()));
        test(
            "18446744073709551616",
            Some(DoubleLimb::from(Limb::MAX) + 1),
        );
        test(
            "340282366920938463463374607431768211455",
            Some(DoubleLimb::MAX),
        );
        test("340282366920938463463374607431768211456", None);
        test("1000000000000000000000", Some(1000000000000000000000));
    }
}

#[test]
fn test_double_limb_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(
            DoubleLimb::wrapping_from(Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            DoubleLimb::wrapping_from(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 1_000_000_000_000);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("-123", 18_446_744_073_709_551_493);
        test("-1000000000000", 18_446_743_073_709_551_616);
        test("1000000000000000000000", 3_875_820_019_684_212_736);
        test("-1000000000000000000000", 14_570_924_054_025_338_880);
        test("4294967296", DoubleLimb::from(Limb::MAX) + 1);
        test("4294967297", DoubleLimb::from(Limb::MAX) + 2);
        test("-4294967296", 0xffff_ffff_0000_0000);
        test("-4294967295", 18_446_744_069_414_584_321);
        test("18446744073709551616", 0);
        test("18446744073709551617", 1);
        test("-18446744073709551616", 0);
        test("-18446744073709551615", 1);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("-123", 340282366920938463463374607431768211333);
        test("-1000000000000", 340282366920938463463374606431768211456);
        test("1000000000000000000000", 1000000000000000000000);
        test(
            "-1000000000000000000000",
            340282366920938462463374607431768211456,
        );
        test("18446744073709551616", DoubleLimb::from(Limb::MAX) + 1);
        test("18446744073709551617", DoubleLimb::from(Limb::MAX) + 2);
        test("-4294967296", 0xffff_ffff_ffff_ffff_ffff_ffff_0000_0000);
        test("-4294967295", 340282366920938463463374607427473244161);
        test("340282366920938463463374607431768211456", 0);
        test("340282366920938463463374607431768211457", 1);
        test("-340282366920938463463374607431768211456", 0);
        test("-340282366920938463463374607431768211455", 1);
    }
}

#[test]
fn test_double_limb_saturating_from_integer() {
    let test = |n, out| {
        assert_eq!(
            DoubleLimb::saturating_from(Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            DoubleLimb::saturating_from(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 1_000_000_000_000);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("-123", 0);
        test("-1000000000000", 0);
        test("1000000000000000000000", 18446744073709551615);
        test("-1000000000000000000000", 0);
        test("4294967296", DoubleLimb::from(Limb::MAX) + 1);
        test("4294967297", DoubleLimb::from(Limb::MAX) + 2);
        test("-4294967296", 0);
        test("-4294967295", 0);
        test("18446744073709551616", 18446744073709551615);
        test("18446744073709551617", 18446744073709551615);
        test("-18446744073709551616", 0);
        test("-18446744073709551615", 0);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("-123", 0);
        test("-1000000000000", 0);
        test("1000000000000000000000", 1000000000000000000000);
        test("-1000000000000000000000", 0);
        test("18446744073709551616", DoubleLimb::from(Limb::MAX) + 1);
        test("18446744073709551617", DoubleLimb::from(Limb::MAX) + 2);
        test("-4294967296", 0);
        test("-4294967295", 0);
        test(
            "340282366920938463463374607431768211456",
            340282366920938463463374607431768211455,
        );
        test(
            "340282366920938463463374607431768211457",
            340282366920938463463374607431768211455,
        );
        test("-340282366920938463463374607431768211456", 0);
        test("-340282366920938463463374607431768211455", 0);
    }
}

#[test]
fn test_double_limb_overflowing_from_integer() {
    let test = |n, out| {
        assert_eq!(
            DoubleLimb::overflowing_from(Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            DoubleLimb::overflowing_from(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000", (1_000_000_000_000, false));
    #[cfg(feature = "32_bit_limbs")]
    {
        test("-123", (18_446_744_073_709_551_493, true));
        test("-1000000000000", (18_446_743_073_709_551_616, true));
        test("1000000000000000000000", (3_875_820_019_684_212_736, true));
        test(
            "-1000000000000000000000",
            (14_570_924_054_025_338_880, true),
        );
        test("4294967296", (DoubleLimb::from(Limb::MAX) + 1, false));
        test("4294967297", (DoubleLimb::from(Limb::MAX) + 2, false));
        test("-4294967296", (0xffff_ffff_0000_0000, true));
        test("-4294967295", (18_446_744_069_414_584_321, true));
        test("18446744073709551616", (0, true));
        test("18446744073709551617", (1, true));
        test("-18446744073709551616", (0, true));
        test("-18446744073709551615", (1, true));
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("-123", (340282366920938463463374607431768211333, true));
        test(
            "-1000000000000",
            (340282366920938463463374606431768211456, true),
        );
        test("1000000000000000000000", (1000000000000000000000, false));
        test(
            "-1000000000000000000000",
            (340282366920938462463374607431768211456, true),
        );
        test(
            "18446744073709551616",
            (DoubleLimb::from(Limb::MAX) + 1, false),
        );
        test(
            "18446744073709551617",
            (DoubleLimb::from(Limb::MAX) + 2, false),
        );
        test(
            "-4294967296",
            (0xffff_ffff_ffff_ffff_ffff_ffff_0000_0000, true),
        );
        test(
            "-4294967295",
            (340282366920938463463374607427473244161, true),
        );
        test("340282366920938463463374607431768211456", (0, true));
        test("340282366920938463463374607431768211457", (1, true));
        test("-340282366920938463463374607431768211456", (0, true));
        test("-340282366920938463463374607431768211455", (1, true));
    }
}

#[test]
fn test_double_limb_convertible_from_integer() {
    let test = |n, out| {
        assert_eq!(
            DoubleLimb::convertible_from(Integer::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            DoubleLimb::convertible_from(&Integer::from_str(n).unwrap()),
            out
        );
    };
    test("0", true);
    test("123", true);
    test("1000000000000", true);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("-123", false);
        test("-1000000000000", false);
        test("1000000000000000000000", false);
        test("-1000000000000000000000", false);
        test("4294967296", true);
        test("4294967297", true);
        test("-4294967296", false);
        test("-4294967295", false);
        test("18446744073709551616", false);
        test("18446744073709551617", false);
        test("-18446744073709551616", false);
        test("-18446744073709551615", false);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("-123", false);
        test("-1000000000000", false);
        test("1000000000000000000000", true);
        test("-1000000000000000000000", false);
        test("18446744073709551616", true);
        test("18446744073709551617", true);
        test("-4294967296", false);
        test("-4294967295", false);
        test("340282366920938463463374607431768211456", false);
        test("340282366920938463463374607431768211457", false);
        test("-340282366920938463463374607431768211456", false);
        test("-340282366920938463463374607431768211455", false);
    }
}

#[test]
fn double_limb_checked_from_integer_properties() {
    test_properties(integers, |x| {
        let result = DoubleLimb::checked_from(x);
        assert_eq!(DoubleLimb::checked_from(x.clone()), result);
        if x.sign() != Ordering::Less && x.significant_bits() <= u64::from(DoubleLimb::WIDTH) {
            assert_eq!(Integer::from(result.unwrap()), *x);
            assert_eq!(result, Some(DoubleLimb::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
        assert_eq!(result.is_none(), DoubleLimb::overflowing_from(x).1)
    });
}

#[test]
fn double_limb_wrapping_from_integer_properties() {
    test_properties(integers, |x| {
        let result = DoubleLimb::wrapping_from(x);
        assert_eq!(DoubleLimb::wrapping_from(x.clone()), result);
        assert_eq!(result.wrapping_add(DoubleLimb::wrapping_from(&-x)), 0);
        assert_eq!(
            result,
            DoubleLimb::checked_from(&(&x).mod_power_of_two(DoubleLimb::WIDTH.into())).unwrap()
        );
        assert_eq!(result, DoubleLimb::overflowing_from(x).0);
    });
}

#[test]
fn double_limb_saturating_from_integer_properties() {
    test_properties(integers, |x| {
        let result = DoubleLimb::saturating_from(x);
        assert_eq!(DoubleLimb::saturating_from(x.clone()), result);
        let result = Natural::from(result);
        assert!(result.le_abs(x));
        assert_eq!(result == *x, DoubleLimb::convertible_from(x));
    });
}

#[test]
fn double_limb_overflowing_from_integer_properties() {
    test_properties(integers, |x| {
        let result = DoubleLimb::overflowing_from(x);
        assert_eq!(DoubleLimb::overflowing_from(x.clone()), result);
        assert_eq!(
            result,
            (
                DoubleLimb::wrapping_from(x),
                !DoubleLimb::convertible_from(x)
            )
        );
    });
}

#[test]
fn double_limb_convertible_from_integer_properties() {
    test_properties(integers, |x| {
        let convertible = DoubleLimb::convertible_from(x.clone());
        assert_eq!(DoubleLimb::convertible_from(x), convertible);
        assert_eq!(convertible, *x >= 0 && *x <= Natural::from(DoubleLimb::MAX));
    });
}
