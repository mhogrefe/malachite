use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::traits::{ModPowerOfTwo, SignificantBits};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
#[cfg(feature = "32_bit_limbs")]
use rug;

use common::test_properties;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::natural_to_rug_integer;
use malachite_test::inputs::natural::naturals;

#[test]
fn test_limb_checked_from_natural() {
    let test = |n, out| {
        assert_eq!(Limb::checked_from(Natural::from_str(n).unwrap()), out);
        assert_eq!(Limb::checked_from(&Natural::from_str(n).unwrap()), out);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000", None);
        test("4294967295", Some(Limb::MAX));
        test("4294967296", None);
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test("1000000000000", Some(1000000000000));
        test("18446744073709551615", Some(Limb::MAX));
        test("18446744073709551616", None);
    }
}

#[test]
fn test_limb_wrapping_from_natural() {
    let test = |n, out| {
        assert_eq!(Limb::wrapping_from(Natural::from_str(n).unwrap()), out);
        assert_eq!(Limb::wrapping_from(&Natural::from_str(n).unwrap()), out);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    #[cfg(feature = "32_bit_limbs")]
    {
        test("1000000000000", 3_567_587_328);
        test("4294967296", 0);
        test("4294967297", 1);
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test("1000000000000", 1000000000000);
        test("18446744073709551616", 0);
        test("18446744073709551617", 1);
    }
}

#[test]
fn limb_checked_from_natural_properties() {
    test_properties(naturals, |x| {
        let result = Limb::checked_from(x);
        assert_eq!(Limb::checked_from(x.clone()), result);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(natural_to_rug_integer(x).to_u32(), result);
        if x.significant_bits() <= u64::from(Limb::WIDTH) {
            assert_eq!(Natural::from(result.unwrap()), *x);
            assert_eq!(result, Some(Limb::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
    });
}

#[test]
fn limb_wrapping_from_natural_properties() {
    test_properties(naturals, |x| {
        let result = Limb::wrapping_from(x);
        assert_eq!(Limb::wrapping_from(x.clone()), result);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(natural_to_rug_integer(x).to_u32_wrapping(), result);
        assert_eq!(
            result,
            Limb::checked_from((&x).mod_power_of_two(Limb::WIDTH.into())).unwrap()
        );
    });
}
