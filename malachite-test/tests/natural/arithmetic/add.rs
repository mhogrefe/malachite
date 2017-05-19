use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;
use std::str::FromStr;

#[test]
fn test_add_assign_natural() {
    let test = |u, v, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n += native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n += gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
}

#[test]
fn test_add_natural() {
    let test = |u, v, out| {
        let n = native::Natural::from_str(u).unwrap() + native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Natural::from_str(u).unwrap() + gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num::BigUint::from_str(u).unwrap() + num::BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
}
