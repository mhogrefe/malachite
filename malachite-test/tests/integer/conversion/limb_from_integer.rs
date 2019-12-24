use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, Sign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
#[cfg(feature = "32_bit_limbs")]
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::integer_to_rug_integer;
use malachite_test::common::test_properties;
use malachite_test::inputs::integer::integers;

#[test]
fn test_limb_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(Limb::checked_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(Limb::checked_from(&Integer::from_str(n).unwrap()), out);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", None);
    test("-1000000000000", None);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000", None);
        test("4294967295", Some(Limb::MAX));
        test("4294967296", None);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("1000000000000", Some(1000000000000));
        test("18446744073709551615", Some(Limb::MAX));
        test("18446744073709551616", None);
    }
}

#[test]
fn test_limb_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(Limb::wrapping_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(Limb::wrapping_from(&Integer::from_str(n).unwrap()), out);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("-123", 4_294_967_173);
        test("1000000000000", 3_567_587_328);
        test("-1000000000000", 727_379_968);
        test("4294967296", 0);
        test("4294967297", 1);
        test("-4294967296", 0);
        test("-4294967295", 1);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("-123", 18446744073709551493);
        test("1000000000000", 1000000000000);
        test("-1000000000000", 18446743073709551616);
        test("18446744073709551616", 0);
        test("18446744073709551617", 1);
        test("-18446744073709551616", 0);
        test("-18446744073709551615", 1);
    }
}

#[test]
fn test_limb_saturating_from_integer() {
    let test = |n, out| {
        assert_eq!(Limb::saturating_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(Limb::saturating_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("-123", 0);
        test("1000000000000", 4_294_967_295);
        test("-1000000000000", 0);
        test("4294967296", 4_294_967_295);
        test("4294967297", 4_294_967_295);
        test("-4294967296", 0);
        test("-4294967295", 0);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("-123", 0);
        test("1000000000000", 1_000_000_000_000);
        test("-1000000000000", 0);
        test("18446744073709551616", 18_446_744_073_709_551_615);
        test("18446744073709551617", 18_446_744_073_709_551_615);
        test("-18446744073709551616", 0);
        test("-18446744073709551615", 0);
    }
}

#[test]
fn test_limb_overflowing_from_integer() {
    let test = |n, out| {
        assert_eq!(Limb::overflowing_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(Limb::overflowing_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    #[cfg(feature = "32_bit_limbs")]
    {
        test("-123", (4_294_967_173, true));
        test("1000000000000", (3_567_587_328, true));
        test("-1000000000000", (727_379_968, true));
        test("4294967296", (0, true));
        test("4294967297", (1, true));
        test("-4294967296", (0, true));
        test("-4294967295", (1, true));
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("-123", (18446744073709551493, true));
        test("1000000000000", (1000000000000, false));
        test("-1000000000000", (18446743073709551616, true));
        test("18446744073709551616", (0, true));
        test("18446744073709551617", (1, true));
        test("-18446744073709551616", (0, true));
        test("-18446744073709551615", (1, true));
    }
}

#[test]
fn test_limb_convertible_from_integer() {
    let test = |n, out| {
        assert_eq!(Limb::convertible_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(Limb::convertible_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", true);
    test("123", true);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("-123", false);
        test("1000000000000", false);
        test("-1000000000000", false);
        test("4294967296", false);
        test("4294967297", false);
        test("-4294967296", false);
        test("-4294967295", false);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("-123", false);
        test("1000000000000", true);
        test("-1000000000000", false);
        test("18446744073709551616", false);
        test("18446744073709551617", false);
        test("-18446744073709551616", false);
        test("-18446744073709551615", false);
    }
}

#[test]
fn limb_checked_from_integer_properties() {
    test_properties(integers, |x| {
        let result = Limb::checked_from(x);
        assert_eq!(Limb::checked_from(x.clone()), result);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(integer_to_rug_integer(x).to_u32(), result);
        if x.sign() != Ordering::Less && x.significant_bits() <= u64::from(Limb::WIDTH) {
            assert_eq!(Integer::from(result.unwrap()), *x);
            assert_eq!(result, Some(Limb::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
        assert_eq!(result.is_none(), Limb::overflowing_from(x).1)
    });
}

#[test]
fn limb_wrapping_from_integer_properties() {
    test_properties(integers, |x| {
        let result = Limb::wrapping_from(x);
        assert_eq!(Limb::wrapping_from(x.clone()), result);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(integer_to_rug_integer(x).to_u32_wrapping(), result);
        assert_eq!(result.wrapping_add(Limb::wrapping_from(&-x)), 0);
        assert_eq!(
            result,
            Limb::checked_from(&x.mod_power_of_two(Limb::WIDTH.into())).unwrap()
        );
        assert_eq!(result, Limb::overflowing_from(x).0);
    });
}

#[test]
fn limb_saturating_from_integer_properties() {
    test_properties(integers, |x| {
        let result = Limb::saturating_from(x);
        assert_eq!(Limb::saturating_from(x.clone()), result);
        assert!(result.le_abs(x));
        assert_eq!(result == *x, Limb::convertible_from(x));
    });
}

#[test]
fn limb_overflowing_from_integer_properties() {
    test_properties(integers, |x| {
        let result = Limb::overflowing_from(x);
        assert_eq!(Limb::overflowing_from(x.clone()), result);
        assert_eq!(result, (Limb::wrapping_from(x), !Limb::convertible_from(x)));
    });
}

#[test]
fn limb_convertible_from_integer_properties() {
    test_properties(integers, |x| {
        let convertible = Limb::convertible_from(x.clone());
        assert_eq!(Limb::convertible_from(x), convertible);
        assert_eq!(convertible, *x >= 0 && *x <= Limb::MAX);
    });
}
