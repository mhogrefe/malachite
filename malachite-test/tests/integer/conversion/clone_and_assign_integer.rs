use malachite_native::integer as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use num;
use rugint;
use std::str::FromStr;

#[test]
fn test_assign_integer() {
    let test = |u, v, out| {
        let mut x = native::Integer::from_str(u).unwrap();
        x.assign(&native::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Integer::from_str(u).unwrap();
        x.assign(&gmp::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test("-123", "456", "456");
    test("-123", "1000000000000", "1000000000000");
    test("1000000000000", "-123", "-123");
    test("1000000000000", "2000000000000", "2000000000000");
}

#[test]
fn test_clone() {
    let test = |u| {
        let x = native::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = gmp::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = num::BigInt::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rugint::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
}

#[test]
fn test_clone_from() {
    let test = |u, v, out| {
        let mut x = native::Integer::from_str(u).unwrap();
        x.clone_from(&native::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Integer::from_str(u).unwrap();
        x.clone_from(&gmp::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = num::BigInt::from_str(u).unwrap();
        x.clone_from(&num::BigInt::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.clone_from(&rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
    };
    test("123", "456", "456");
    test("123", "1000000000000", "1000000000000");
    test("1000000000000", "123", "123");
    test("1000000000000", "2000000000000", "2000000000000");
}
