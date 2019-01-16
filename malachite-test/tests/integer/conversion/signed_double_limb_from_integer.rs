use common::test_properties;
use malachite_base::misc::{CheckedFrom, Max, Min, WrappingFrom};
use malachite_base::num::{
    ModPowerOfTwo, One, PrimitiveInteger, PrimitiveUnsigned, SignificantBits,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, SignedDoubleLimb, SignedLimb};
use malachite_test::inputs::integer::integers;
use std::str::FromStr;

#[test]
fn test_signed_double_limb_checked_from_integer() {
    let test = |n, out| {
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
    });
}

#[test]
fn signed_double_limb_wrapping_from_integer_properties() {
    test_properties(integers, |x| {
        let result = SignedDoubleLimb::wrapping_from(x);
        assert_eq!(SignedDoubleLimb::wrapping_from(x.clone()), result);
        assert_eq!(-result, SignedDoubleLimb::wrapping_from(&-x));
        assert_eq!(
            result,
            DoubleLimb::checked_from(&x.mod_power_of_two(DoubleLimb::WIDTH.into()))
                .unwrap()
                .to_signed_bitwise()
        );
    });
}
