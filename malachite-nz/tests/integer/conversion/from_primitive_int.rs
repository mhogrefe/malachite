use malachite_nz::integer::Integer;
use num::BigInt;
use rug;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(u).to_string(), out);
        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::MAX, "4294967295");
}

#[test]
fn test_from_u64() {
    let test = |u: u64, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(u).to_string(), out);
        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u64::MAX, "18446744073709551615");
}

#[test]
fn test_from_i32() {
    let test = |i: i32, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(i).to_string(), out);
        assert_eq!(rug::Integer::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i32::MIN, "-2147483648");
    test(i32::MAX, "2147483647");
}

#[test]
fn test_from_i64() {
    let test = |i: i64, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(i).to_string(), out);
        assert_eq!(rug::Integer::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i64::MIN, "-9223372036854775808");
    test(i64::MAX, "9223372036854775807");
}
