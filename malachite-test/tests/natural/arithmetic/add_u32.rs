use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::natural::arithmetic::add_u32::num_add_u32;
use num;
use std::str::FromStr;

#[test]
fn test_add_assign_u32() {
    let test = |u, v: u32, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "579");
    test("1000000000000", 123, "1000000000123");
    test("4294967295", 1, "4294967296");
}

#[test]
fn test_add_u32() {
    let test = |u, v: u32, out| {
        assert_eq!((native::Natural::from_str(u).unwrap() + v).to_string(), out);
        assert_eq!((gmp::Natural::from_str(u).unwrap() + v).to_string(), out);
        assert_eq!(num_add_u32(num::BigUint::from_str(u).unwrap(), v).to_string(),
                   out);

        assert_eq!((v + native::Natural::from_str(u).unwrap()).to_string(), out);
        assert_eq!((v + gmp::Natural::from_str(u).unwrap()).to_string(), out);
    };
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "579");
    test("1000000000000", 123, "1000000000123");
    test("4294967295", 1, "4294967296");
}
