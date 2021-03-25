use malachite_nz::integer::Integer;
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_sub() {
    let test = |i, j, out| {
        let mut n = Integer::from_str(i).unwrap();
        n -= Integer::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(i).unwrap();
        n -= &Integer::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(i).unwrap() - Integer::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(i).unwrap() - Integer::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(i).unwrap() - &Integer::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(i).unwrap() - &Integer::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigInt::from_str(i).unwrap() - BigInt::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(i).unwrap() - rug::Integer::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "-123", "123");
    test("123", "0", "123");
    test("123", "-456", "579");
    test("1000000000000", "-123", "1000000000123");
    test("123", "-1000000000000", "1000000000123");
    test("12345678987654321", "-314159265358979", "12659838253013300");
    test("0", "123", "-123");
    test("123", "123", "0");
    test("123", "456", "-333");
    test("1000000000000", "123", "999999999877");
    test("123", "1000000000000", "-999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
}
