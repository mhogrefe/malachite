use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::ModPowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::DoubleLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::natural::naturals;

#[test]
fn test_double_limb_checked_from_natural() {
    let test = |n, out| {
        assert_eq!(DoubleLimb::checked_from(Natural::from_str(n).unwrap()), out);
        assert_eq!(
            DoubleLimb::checked_from(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", Some(0));
    test("123", Some(123));
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000000000000", None);
        test("18446744073709551615", Some(DoubleLimb::MAX));
        test("18446744073709551616", None);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("1000000000000000000000", Some(1000000000000000000000));
        test(
            "340282366920938463463374607431768211455",
            Some(DoubleLimb::MAX),
        );
        test("340282366920938463463374607431768211456", None);
    }
}

#[test]
fn test_double_limb_wrapping_from_natural() {
    let test = |n, out| {
        assert_eq!(
            DoubleLimb::wrapping_from(Natural::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            DoubleLimb::wrapping_from(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", 0);
    test("123", 123);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000000000000", 3_875_820_019_684_212_736);
        test("18446744073709551616", 0);
        test("18446744073709551617", 1);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("1000000000000000000000", 1_000_000_000_000_000_000_000);
        test("340282366920938463463374607431768211456", 0);
        test("340282366920938463463374607431768211457", 1);
    }
}

#[test]
fn test_double_limb_saturating_from_natural() {
    let test = |n, out| {
        assert_eq!(
            DoubleLimb::saturating_from(Natural::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            DoubleLimb::saturating_from(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", 0);
    test("123", 123);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000000000000", 18_446_744_073_709_551_615);
        test("18446744073709551616", 18_446_744_073_709_551_615);
        test("18446744073709551617", 18_446_744_073_709_551_615);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("1000000000000000000000", 1_000_000_000_000_000_000_000);
        test(
            "340282366920938463463374607431768211456",
            340_282_366_920_938_463_463_374_607_431_768_211_455,
        );
        test(
            "340282366920938463463374607431768211457",
            340_282_366_920_938_463_463_374_607_431_768_211_455,
        );
    }
}

#[test]
fn test_double_limb_overflowing_from_natural() {
    let test = |n, out| {
        assert_eq!(
            DoubleLimb::overflowing_from(Natural::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            DoubleLimb::overflowing_from(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", (0, false));
    test("123", (123, false));
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000000000000", (3_875_820_019_684_212_736, true));
        test("18446744073709551616", (0, true));
        test("18446744073709551617", (1, true));
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test(
            "1000000000000000000000",
            (1_000_000_000_000_000_000_000, false),
        );
        test("340282366920938463463374607431768211456", (0, true));
        test("340282366920938463463374607431768211457", (1, true));
    }
}

#[test]
fn test_double_limb_convertible_from_natural() {
    let test = |n, out| {
        assert_eq!(
            DoubleLimb::convertible_from(Natural::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            DoubleLimb::convertible_from(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", true);
    test("123", true);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000000000000", false);
        test("18446744073709551615", true);
        test("18446744073709551616", false);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("1000000000000000000000", true);
        test("340282366920938463463374607431768211455", true);
        test("340282366920938463463374607431768211456", false);
    }
}

#[test]
fn double_limb_checked_from_natural_properties() {
    test_properties(naturals, |x| {
        let result = DoubleLimb::checked_from(x);
        assert_eq!(DoubleLimb::checked_from(x.clone()), result);
        if x.significant_bits() <= u64::from(DoubleLimb::WIDTH) {
            assert_eq!(Natural::from(result.unwrap()), *x);
            assert_eq!(result, Some(DoubleLimb::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
        assert_eq!(result.is_none(), DoubleLimb::overflowing_from(x).1)
    });
}

#[test]
fn double_limb_wrapping_from_natural_properties() {
    test_properties(naturals, |x| {
        let result = DoubleLimb::wrapping_from(x);
        assert_eq!(DoubleLimb::wrapping_from(x.clone()), result);
        assert_eq!(
            DoubleLimb::checked_from((&x).mod_power_of_two(DoubleLimb::WIDTH.into())).unwrap(),
            result,
        );
        assert_eq!(result, DoubleLimb::overflowing_from(x).0);
    });
}

#[test]
fn double_limb_saturating_from_natural_properties() {
    test_properties(naturals, |x| {
        let result = DoubleLimb::saturating_from(x);
        assert_eq!(DoubleLimb::saturating_from(x.clone()), result);
        assert!(Natural::from(result) <= *x);
        assert_eq!(Natural::from(result) == *x, DoubleLimb::convertible_from(x));
    });
}

#[test]
fn double_limb_overflowing_from_natural_properties() {
    test_properties(naturals, |x| {
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
fn double_limb_convertible_from_natural_properties() {
    test_properties(naturals, |x| {
        let convertible = DoubleLimb::convertible_from(x.clone());
        assert_eq!(DoubleLimb::convertible_from(x), convertible);
        assert_eq!(convertible, *x <= Natural::from(DoubleLimb::MAX));
    });
}
