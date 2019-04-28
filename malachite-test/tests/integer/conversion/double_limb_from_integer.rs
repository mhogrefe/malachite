use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::traits::{ModPowerOfTwo, Sign, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_nz::platform::{DoubleLimb, Limb};

use common::test_properties;
use malachite_test::inputs::integer::integers;

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
    #[cfg(feature = "64_bit_limbs")]
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
    #[cfg(feature = "64_bit_limbs")]
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
    });
}
