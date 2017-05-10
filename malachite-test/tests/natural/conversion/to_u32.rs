use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use rugint;
use std::str::FromStr;

#[test]
fn test_to_u32() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().to_u32(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().to_u32(), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().to_u32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", None);
    test("1000000000000", None);
    test("-1000000000000", None);
}

#[test]
fn test_to_u32_wrapping() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", 4294967173);
    test("1000000000000", 3567587328);
    test("-1000000000000", 727379968);
    test("4294967296", 0);
    test("4294967297", 1);
    test("-4294967296", 0);
    test("-4294967295", 1);
}
