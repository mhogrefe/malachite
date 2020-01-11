use std::str::FromStr;

use malachite_base::comparison::{Max, Min};
use malachite_base::num::arithmetic::traits::ModPowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use rug;

use malachite_test::common::integer_to_rug_integer;
use malachite_test::common::test_properties;
use malachite_test::inputs::integer::integers;

#[test]
fn test_u32_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(u32::checked_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u32::checked_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000", None);
    test("4294967295", Some(u32::MAX));
    test("4294967296", None);
    test("-123", None);
    test("-1000000000000", None);
    test("-4294967295", None);
    test("-4294967296", None);
}

#[test]
fn test_u32_exact_from_integer() {
    let test = |n, out| {
        assert_eq!(u32::exact_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u32::exact_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32().unwrap(), out);
    };
    test("0", 0);
    test("123", 123);
    test("4294967295", u32::MAX);
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_1() {
    u32::exact_from(Integer::from_str("1000000000000").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_2() {
    u32::exact_from(Integer::from_str("4294967296").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_3() {
    u32::exact_from(Integer::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_4() {
    u32::exact_from(Integer::from_str("-1000000000000").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_5() {
    u32::exact_from(Integer::from_str("-4294967295").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_6() {
    u32::exact_from(Integer::from_str("-4294967296").unwrap());
}

#[test]
fn test_u32_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(u32::wrapping_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u32::wrapping_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 3_567_587_328);
    test("4294967296", 0);
    test("4294967297", 1);
    test("-123", 4_294_967_173);
    test("-1000000000000", 727_379_968);
    test("-4294967296", 0);
    test("-4294967297", 4_294_967_295);
}

#[test]
fn test_u32_saturating_from_integer() {
    let test = |n, out| {
        assert_eq!(u32::saturating_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u32::saturating_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 4_294_967_295);
    test("4294967296", 4_294_967_295);
    test("4294967297", 4_294_967_295);
    test("-123", 0);
    test("-1000000000000", 0);
    test("-4294967296", 0);
    test("-4294967297", 0);
}

#[test]
fn test_u32_overflowing_from_integer() {
    let test = |n, out| {
        assert_eq!(u32::overflowing_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u32::overflowing_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000", (3_567_587_328, true));
    test("4294967296", (0, true));
    test("4294967297", (1, true));
    test("-123", (4_294_967_173, true));
    test("-1000000000000", (727_379_968, true));
    test("-4294967296", (0, true));
    test("-4294967297", (4_294_967_295, true));
}

#[test]
fn test_u32_convertible_from_integer() {
    let test = |n, out| {
        assert_eq!(u32::convertible_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u32::convertible_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", true);
    test("123", true);
    test("1000000000000", false);
    test("4294967295", true);
    test("4294967296", false);
    test("-123", false);
    test("-1000000000000", false);
    test("-4294967295", false);
    test("-4294967296", false);
}

#[test]
fn test_u64_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(u64::checked_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u64::checked_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u64(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000", Some(1_000_000_000_000));
    test("18446744073709551615", Some(u64::MAX));
    test("18446744073709551616", None);
    test("-123", None);
    test("-1000000000000", None);
    test("-18446744073709551615", None);
    test("-18446744073709551616", None);
}

#[test]
fn test_u64_exact_from_integer() {
    let test = |n, out| {
        assert_eq!(u64::exact_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u64::exact_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u64().unwrap(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 1_000_000_000_000);
    test("18446744073709551615", u64::MAX);
}

#[test]
#[should_panic]
fn u64_exact_from_integer_fail_1() {
    u64::exact_from(Integer::from_str("18446744073709551616").unwrap());
}

#[test]
#[should_panic]
fn u64_exact_from_integer_fail_2() {
    u64::exact_from(Integer::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn u64_exact_from_integer_fail_3() {
    u64::exact_from(Integer::from_str("-1000000000000").unwrap());
}

#[test]
#[should_panic]
fn u64_exact_from_integer_fail_4() {
    u64::exact_from(Integer::from_str("-18446744073709551615").unwrap());
}

#[test]
#[should_panic]
fn u64_exact_from_integer_fail_5() {
    u64::exact_from(Integer::from_str("-18446744073709551616").unwrap());
}

#[test]
fn test_u64_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(u64::wrapping_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u64::wrapping_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", 2_003_764_205_206_896_640);
    test("18446744073709551616", 0);
    test("18446744073709551617", 1);
    test("-123", 18_446_744_073_709_551_493);
    test("-1000000000000000000000000", 16_442_979_868_502_654_976);
    test("-18446744073709551616", 0);
    test("-18446744073709551617", 18_446_744_073_709_551_615);
}

#[test]
fn test_u64_saturating_from_integer() {
    let test = |n, out| {
        assert_eq!(u64::saturating_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u64::saturating_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", 18_446_744_073_709_551_615);
    test("18446744073709551616", 18_446_744_073_709_551_615);
    test("18446744073709551617", 18_446_744_073_709_551_615);
    test("-123", 0);
    test("-1000000000000000000000000", 0);
    test("-18446744073709551616", 0);
    test("-18446744073709551617", 0);
}

#[test]
fn test_u64_overflowing_from_integer() {
    let test = |n, out| {
        assert_eq!(u64::overflowing_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u64::overflowing_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test(
        "1000000000000000000000000",
        (2_003_764_205_206_896_640, true),
    );
    test("18446744073709551616", (0, true));
    test("18446744073709551617", (1, true));
    test("-123", (18_446_744_073_709_551_493, true));
    test(
        "-1000000000000000000000000",
        (16_442_979_868_502_654_976, true),
    );
    test("-18446744073709551616", (0, true));
    test("-18446744073709551617", (18_446_744_073_709_551_615, true));
}

#[test]
fn test_u64_convertible_from_integer() {
    let test = |n, out| {
        assert_eq!(u64::convertible_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(u64::convertible_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", true);
    test("123", true);
    test("1000000000000000000000000", false);
    test("18446744073709551615", true);
    test("18446744073709551616", false);
    test("-123", false);
    test("-1000000000000000000000000", false);
    test("-18446744073709551615", false);
    test("-18446744073709551616", false);
}

#[test]
fn test_i32_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(i32::checked_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i32::checked_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000", None);
    test("2147483647", Some(i32::MAX));
    test("2147483648", None);
    test("-123", Some(-123));
    test("-1000000000000", None);
    test("-2147483648", Some(i32::MIN));
    test("-2147483649", None);
}

#[test]
fn test_i32_exact_from_integer() {
    let test = |n, out| {
        assert_eq!(i32::exact_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i32::exact_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32().unwrap(), out);
    };
    test("0", 0);
    test("123", 123);
    test("2147483647", i32::MAX);
    test("-123", -123);
    test("-2147483648", i32::MIN);
}

#[test]
#[should_panic]
fn i32_exact_from_integer_fail_1() {
    i32::exact_from(Integer::from_str("1000000000000").unwrap());
}

#[test]
#[should_panic]
fn i32_exact_from_integer_fail_2() {
    i32::exact_from(Integer::from_str("2147483648").unwrap());
}

#[test]
#[should_panic]
fn i32_exact_from_integer_fail_3() {
    i32::exact_from(Integer::from_str("-1000000000000").unwrap());
}

#[test]
#[should_panic]
fn i32_exact_from_integer_fail_4() {
    i32::exact_from(Integer::from_str("-2147483649").unwrap());
}

#[test]
fn test_i32_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(i32::wrapping_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i32::wrapping_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", -727_379_968);
    test("2147483648", -2_147_483_648);
    test("2147483649", -2_147_483_647);
    test("-123", -123);
    test("-1000000000000", 727_379_968);
    test("-2147483649", 2_147_483_647);
    test("-2147483650", 2_147_483_646);
}

#[test]
fn test_i32_saturating_from_integer() {
    let test = |n, out| {
        assert_eq!(i32::saturating_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i32::saturating_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 2_147_483_647);
    test("2147483648", 2_147_483_647);
    test("2147483649", 2_147_483_647);
    test("-123", -123);
    test("-1000000000000", -2_147_483_648);
    test("-2147483648", -2_147_483_648);
    test("-2147483649", -2_147_483_648);
}

#[test]
fn test_i32_overflowing_from_integer() {
    let test = |n, out| {
        assert_eq!(i32::overflowing_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i32::overflowing_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000", (-727_379_968, true));
    test("2147483648", (-2_147_483_648, true));
    test("2147483649", (-2_147_483_647, true));
    test("-123", (-123, false));
    test("-1000000000000", (727_379_968, true));
    test("-2147483649", (2_147_483_647, true));
    test("-2147483650", (2_147_483_646, true));
}

#[test]
fn test_i32_convertible_from_integer() {
    let test = |n, out| {
        assert_eq!(i32::convertible_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i32::convertible_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", true);
    test("123", true);
    test("1000000000000", false);
    test("2147483647", true);
    test("2147483648", false);
    test("-123", true);
    test("-1000000000000", false);
    test("-2147483648", true);
    test("-2147483649", false);
}

#[test]
fn test_i64_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(i64::checked_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i64::checked_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i64(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000000000000000", None);
    test("9223372036854775807", Some(i64::MAX));
    test("9223372036854775808", None);
    test("-123", Some(-123));
    test("-1000000000000000000000000", None);
    test("-9223372036854775808", Some(i64::MIN));
    test("-9223372036854775809", None);
}

#[test]
fn test_i64_exact_from_integer() {
    let test = |n, out| {
        assert_eq!(i64::exact_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i64::exact_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i64().unwrap(), out);
    };
    test("0", 0);
    test("123", 123);
    test("9223372036854775807", i64::MAX);
    test("-123", -123);
    test("-9223372036854775808", i64::MIN);
}

#[test]
#[should_panic]
fn i64_exact_from_integer_fail_1() {
    i64::exact_from(Integer::from_str("1000000000000000000000000").unwrap());
}

#[test]
#[should_panic]
fn i64_exact_from_integer_fail_2() {
    i64::exact_from(Integer::from_str("9223372036854775808").unwrap());
}

#[test]
#[should_panic]
fn i64_exact_from_integer_fail_3() {
    i64::exact_from(Integer::from_str("-1000000000000000000000000").unwrap());
}

#[test]
#[should_panic]
fn i64_exact_from_integer_fail_4() {
    i64::exact_from(Integer::from_str("-9223372036854775809").unwrap());
}

#[test]
fn test_i64_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(i64::wrapping_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i64::wrapping_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", 2_003_764_205_206_896_640);
    test("9223372036854775808", -9_223_372_036_854_775_808);
    test("9223372036854775809", -9_223_372_036_854_775_807);
    test("-123", -123);
    test("-1000000000000000000000000", -2_003_764_205_206_896_640);
    test("-9223372036854775809", 9_223_372_036_854_775_807);
    test("-9223372036854775810", 9_223_372_036_854_775_806);
}

#[test]
fn test_i64_saturating_from_integer() {
    let test = |n, out| {
        assert_eq!(i64::saturating_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i64::saturating_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", 9_223_372_036_854_775_807);
    test("9223372036854775808", 9_223_372_036_854_775_807);
    test("9223372036854775809", 9_223_372_036_854_775_807);
    test("-123", -123);
    test("-1000000000000000000000000", -9_223_372_036_854_775_808);
    test("-9223372036854775808", -9_223_372_036_854_775_808);
    test("-9223372036854775809", -9_223_372_036_854_775_808);
}

#[test]
fn test_i64_overflowing_from_integer() {
    let test = |n, out| {
        assert_eq!(i64::overflowing_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i64::overflowing_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test(
        "1000000000000000000000000",
        (2_003_764_205_206_896_640, true),
    );
    test("9223372036854775808", (-9_223_372_036_854_775_808, true));
    test("9223372036854775809", (-9_223_372_036_854_775_807, true));
    test("-123", (-123, false));
    test(
        "-1000000000000000000000000",
        (-2_003_764_205_206_896_640, true),
    );
    test("-9223372036854775809", (9_223_372_036_854_775_807, true));
    test("-9223372036854775810", (9_223_372_036_854_775_806, true));
}

#[test]
fn test_i64_convertible_from_integer() {
    let test = |n, out| {
        assert_eq!(i64::convertible_from(Integer::from_str(n).unwrap()), out);
        assert_eq!(i64::convertible_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", true);
    test("123", true);
    test("1000000000000000000000000", false);
    test("9223372036854775807", true);
    test("9223372036854775808", false);
    test("-123", true);
    test("-1000000000000000000000000", false);
    test("-9223372036854775808", true);
    test("-9223372036854775809", false);
}

macro_rules! unsigned_properties {
    ($t: ident) => {
        properties!($t);

        test_properties(integers, |x| {
            let result = $t::checked_from(x);
            assert_eq!($t::checked_from(x.clone()), result);
            if *x >= Integer::ZERO && x.significant_bits() <= u64::from($t::WIDTH) {
                assert_eq!(Integer::from(result.unwrap()), *x);
                assert_eq!(result, Some($t::wrapping_from(x)));
                assert_eq!(result, Some($t::exact_from(x)));
            } else {
                assert!(result.is_none());
            }
            assert_eq!(result.is_none(), $t::overflowing_from(x).1);

            let result = $t::wrapping_from(x);
            assert_eq!(
                result,
                $t::exact_from((&x).mod_power_of_two(u64::from($t::WIDTH)))
            );
        });
    };
}

macro_rules! signed_properties {
    ($t: ident) => {
        properties!($t);

        test_properties(integers, |x| {
            let result = $t::checked_from(x);
            assert_eq!($t::checked_from(x.clone()), result);
            //TODO if *x >= Integer::ZERO && x.significant_bits() <= u64::from($t::WIDTH - 1) {
            //TODO     assert_eq!(Integer::from(result.unwrap()), *x);
            //TODO     assert_eq!(result, Some($t::wrapping_from(x)));
            //TODO     assert_eq!(result, Some($t::exact_from(x)));
            //TODO } else {
            //TODO     assert!(result.is_none());
            //TODO }
            assert_eq!(result.is_none(), $t::overflowing_from(x).1);
        });
    };
}

macro_rules! properties {
    ($t: ident) => {
        test_properties(integers, |x| {
            let result = $t::wrapping_from(x);
            assert_eq!($t::wrapping_from(x.clone()), result);
            assert_eq!(result, $t::overflowing_from(x).0);

            let result = $t::saturating_from(x);
            assert_eq!($t::saturating_from(x.clone()), result);
            //TODO assert!(result <= *x);
            //TODO assert_eq!(result == *x, $t::convertible_from(x));

            let result = $t::overflowing_from(x);
            assert_eq!($t::overflowing_from(x.clone()), result);
            assert_eq!(result, ($t::wrapping_from(x), !$t::convertible_from(x)));

            let convertible = $t::convertible_from(x.clone());
            assert_eq!($t::convertible_from(x), convertible);
            //TODO assert_eq!(convertible, *x >= $t::MIN && *x <= $t::MAX);
        });
    };
}

#[test]
fn primitive_integer_from_integer_properties() {
    test_properties(integers, |x| {
        assert_eq!(integer_to_rug_integer(x).to_u32(), u32::checked_from(x));
        assert_eq!(
            integer_to_rug_integer(x).to_u32_wrapping(),
            u32::wrapping_from(x)
        );
        assert_eq!(integer_to_rug_integer(x).to_u64(), u64::checked_from(x));
        assert_eq!(
            integer_to_rug_integer(x).to_u64_wrapping(),
            u64::wrapping_from(x)
        );
        assert_eq!(integer_to_rug_integer(x).to_i32(), i32::checked_from(x));
        assert_eq!(
            integer_to_rug_integer(x).to_i32_wrapping(),
            i32::wrapping_from(x)
        );
        assert_eq!(integer_to_rug_integer(x).to_i64(), i64::checked_from(x));
        assert_eq!(
            integer_to_rug_integer(x).to_i64_wrapping(),
            i64::wrapping_from(x)
        );
    });

    unsigned_properties!(u8);
    unsigned_properties!(u16);
    unsigned_properties!(u32);
    unsigned_properties!(u64);
    unsigned_properties!(usize);

    signed_properties!(i8);
    signed_properties!(i16);
    signed_properties!(i32);
    signed_properties!(i64);
    signed_properties!(isize);
}
