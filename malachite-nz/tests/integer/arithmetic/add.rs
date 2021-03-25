use malachite_nz::integer::Integer;
use num::BigInt;
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut n = u.clone();
        n += v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n += &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() + v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u + v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() + &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u + &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigInt::from_str(s).unwrap() + BigInt::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(s).unwrap() + rug::Integer::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
    test("0", "-123", "-123");
    test("123", "-123", "0");
    test("123", "-456", "-333");
    test("1000000000000", "-123", "999999999877");
    test("123", "-1000000000000", "-999999999877");
    test("12345678987654321", "-314159265358979", "12031519722295342");
}
