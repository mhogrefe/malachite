use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;
use std::str::FromStr;

#[test]
fn test_shl_assign_u32() {
    let test = |u, v: u32, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "123");
    test("123", 1, "246");
    test("123", 2, "492");
    test("123", 25, "4127195136");
    test("123", 26, "8254390272");
    test("123", 100, "155921023828072216384094494261248");
    test("2147483648", 1, "4294967296");
    test("1000000000000", 0, "1000000000000");
    test("1000000000000", 3, "8000000000000");
    test("1000000000000", 24, "16777216000000000000");
    test("1000000000000", 25, "33554432000000000000");
    test("1000000000000", 31, "2147483648000000000000");
    test("1000000000000", 32, "4294967296000000000000");
    test("1000000000000", 33, "8589934592000000000000");
    test("1000000000000",
         100,
         "1267650600228229401496703205376000000000000");
}

#[test]
fn test_shl_u32() {
    let test = |u, v: u32, out| {
        let n = native::Natural::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num::BigUint::from_str(u).unwrap() << v as usize;
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "123");
    test("123", 1, "246");
    test("123", 2, "492");
    test("123", 25, "4127195136");
    test("123", 26, "8254390272");
    test("123", 100, "155921023828072216384094494261248");
    test("2147483648", 1, "4294967296");
    test("1000000000000", 0, "1000000000000");
    test("1000000000000", 3, "8000000000000");
    test("1000000000000", 24, "16777216000000000000");
    test("1000000000000", 25, "33554432000000000000");
    test("1000000000000", 31, "2147483648000000000000");
    test("1000000000000", 32, "4294967296000000000000");
    test("1000000000000", 33, "8589934592000000000000");
    test("1000000000000",
         100,
         "1267650600228229401496703205376000000000000");
}
