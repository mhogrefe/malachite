use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_nz::integer::Integer;
use rug;
use std::str::FromStr;

#[test]
fn test_u32_checked_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u32::checked_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_u32(), out);
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
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u32::exact_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_u32().unwrap(), out);
    };
    test("0", 0);
    test("123", 123);
    test("4294967295", u32::MAX);
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_1() {
    u32::exact_from(&Integer::from_str("1000000000000").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_2() {
    u32::exact_from(&Integer::from_str("4294967296").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_3() {
    u32::exact_from(&Integer::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_4() {
    u32::exact_from(&Integer::from_str("-1000000000000").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_5() {
    u32::exact_from(&Integer::from_str("-4294967295").unwrap());
}

#[test]
#[should_panic]
fn u32_exact_from_integer_fail_6() {
    u32::exact_from(&Integer::from_str("-4294967296").unwrap());
}

#[test]
fn test_u32_wrapping_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u32::wrapping_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 3567587328);
    test("4294967296", 0);
    test("4294967297", 1);
    test("-123", 4294967173);
    test("-1000000000000", 727379968);
    test("-4294967296", 0);
    test("-4294967297", u32::MAX);
}

#[test]
fn test_u32_saturating_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(u32::saturating_from(&u), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", u32::MAX);
    test("4294967296", u32::MAX);
    test("4294967297", u32::MAX);
    test("-123", 0);
    test("-1000000000000", 0);
    test("-4294967296", 0);
    test("-4294967297", 0);
}

#[test]
fn test_u32_overflowing_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(u32::overflowing_from(&u), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000", (3567587328, true));
    test("4294967296", (0, true));
    test("4294967297", (1, true));
    test("-123", (4294967173, true));
    test("-1000000000000", (727379968, true));
    test("-4294967296", (0, true));
    test("-4294967297", (u32::MAX, true));
}

#[test]
fn test_u32_convertible_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(u32::convertible_from(&u), out);
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
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(u64::checked_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_u64(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000", Some(1000000000000));
    test("18446744073709551615", Some(u64::MAX));
    test("18446744073709551616", None);
    test("-123", None);
    test("-1000000000000", None);
    test("-18446744073709551615", None);
    test("-18446744073709551616", None);
}

#[test]
fn test_u64_exact_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(u64::exact_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_u64().unwrap(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 1000000000000);
    test("18446744073709551615", u64::MAX);
}

#[test]
#[should_panic]
fn u64_exact_from_integer_fail_1() {
    u64::exact_from(&Integer::from_str("18446744073709551616").unwrap());
}

#[test]
#[should_panic]
fn u64_exact_from_integer_fail_2() {
    u64::exact_from(&Integer::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn u64_exact_from_integer_fail_3() {
    u64::exact_from(&Integer::from_str("-1000000000000").unwrap());
}

#[test]
#[should_panic]
fn u64_exact_from_integer_fail_4() {
    u64::exact_from(&Integer::from_str("-18446744073709551615").unwrap());
}

#[test]
#[should_panic]
fn u64_exact_from_integer_fail_5() {
    u64::exact_from(&Integer::from_str("-18446744073709551616").unwrap());
}

#[test]
fn test_u64_wrapping_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(u64::wrapping_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_u64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", 2003764205206896640);
    test("18446744073709551616", 0);
    test("18446744073709551617", 1);
    test("-123", 18446744073709551493);
    test("-1000000000000000000000000", 16442979868502654976);
    test("-18446744073709551616", 0);
    test("-18446744073709551617", u64::MAX);
}

#[test]
fn test_u64_saturating_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(u64::saturating_from(&u), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", u64::MAX);
    test("18446744073709551616", u64::MAX);
    test("18446744073709551617", u64::MAX);
    test("-123", 0);
    test("-1000000000000000000000000", 0);
    test("-18446744073709551616", 0);
    test("-18446744073709551617", 0);
}

#[test]
fn test_u64_overflowing_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(u64::overflowing_from(&u), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000000000000000", (2003764205206896640, true));
    test("18446744073709551616", (0, true));
    test("18446744073709551617", (1, true));
    test("-123", (18446744073709551493, true));
    test("-1000000000000000000000000", (16442979868502654976, true));
    test("-18446744073709551616", (0, true));
    test("-18446744073709551617", (u64::MAX, true));
}

#[test]
fn test_u64_convertible_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(u64::convertible_from(&u), out);
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
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i32::checked_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_i32(), out);
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
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i32::exact_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_i32().unwrap(), out);
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
    i32::exact_from(&Integer::from_str("1000000000000").unwrap());
}

#[test]
#[should_panic]
fn i32_exact_from_integer_fail_2() {
    i32::exact_from(&Integer::from_str("2147483648").unwrap());
}

#[test]
#[should_panic]
fn i32_exact_from_integer_fail_3() {
    i32::exact_from(&Integer::from_str("-1000000000000").unwrap());
}

#[test]
#[should_panic]
fn i32_exact_from_integer_fail_4() {
    i32::exact_from(&Integer::from_str("-2147483649").unwrap());
}

#[test]
fn test_i32_wrapping_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i32::wrapping_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_i32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", -727379968);
    test("2147483648", -0x80000000);
    test("2147483649", -0x7fffffff);
    test("-123", -123);
    test("-1000000000000", 727379968);
    test("-2147483649", 0x7fffffff);
    test("-2147483650", 0x7ffffffe);
}

#[test]
fn test_i32_saturating_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i32::saturating_from(&u), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 0x7fffffff);
    test("2147483648", 0x7fffffff);
    test("2147483649", 0x7fffffff);
    test("-123", -123);
    test("-1000000000000", -0x80000000);
    test("-2147483648", -0x80000000);
    test("-2147483649", -0x80000000);
}

#[test]
fn test_i32_overflowing_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i32::overflowing_from(&u), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000", (-727379968, true));
    test("2147483648", (-0x80000000, true));
    test("2147483649", (-0x7fffffff, true));
    test("-123", (-123, false));
    test("-1000000000000", (727379968, true));
    test("-2147483649", (0x7fffffff, true));
    test("-2147483650", (0x7ffffffe, true));
}

#[test]
fn test_i32_convertible_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i32::convertible_from(&u), out);
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
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i64::checked_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_i64(), out);
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
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i64::exact_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_i64().unwrap(), out);
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
    i64::exact_from(&Integer::from_str("1000000000000000000000000").unwrap());
}

#[test]
#[should_panic]
fn i64_exact_from_integer_fail_2() {
    i64::exact_from(&Integer::from_str("9223372036854775808").unwrap());
}

#[test]
#[should_panic]
fn i64_exact_from_integer_fail_3() {
    i64::exact_from(&Integer::from_str("-1000000000000000000000000").unwrap());
}

#[test]
#[should_panic]
fn i64_exact_from_integer_fail_4() {
    i64::exact_from(&Integer::from_str("-9223372036854775809").unwrap());
}

#[test]
fn test_i64_wrapping_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i64::wrapping_from(&u), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().to_i64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", 2003764205206896640);
    test("9223372036854775808", -0x8000000000000000);
    test("9223372036854775809", -0x7fffffffffffffff);
    test("-123", -123);
    test("-1000000000000000000000000", -2003764205206896640);
    test("-9223372036854775809", 0x7fffffffffffffff);
    test("-9223372036854775810", 0x7ffffffffffffffe);
}

#[test]
fn test_i64_saturating_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i64::saturating_from(&u), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000000", 0x7fffffffffffffff);
    test("9223372036854775808", 0x7fffffffffffffff);
    test("9223372036854775809", 0x7fffffffffffffff);
    test("-123", -123);
    test("-1000000000000000000000000", -0x8000000000000000);
    test("-9223372036854775808", -0x8000000000000000);
    test("-9223372036854775809", -0x8000000000000000);
}

#[test]
fn test_i64_overflowing_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i64::overflowing_from(&u), out);
    };
    test("0", (0, false));
    test("123", (123, false));
    test("1000000000000000000000000", (2003764205206896640, true));
    test("9223372036854775808", (-0x8000000000000000, true));
    test("9223372036854775809", (-0x7fffffffffffffff, true));
    test("-123", (-123, false));
    test("-1000000000000000000000000", (-2003764205206896640, true));
    test("-9223372036854775809", (0x7fffffffffffffff, true));
    test("-9223372036854775810", (0x7ffffffffffffffe, true));
}

#[test]
fn test_i64_convertible_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();
        assert_eq!(i64::convertible_from(&u), out);
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
