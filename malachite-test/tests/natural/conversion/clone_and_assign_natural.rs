use malachite_native::natural as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp::natural as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use num;
use std::str::FromStr;

#[test]
fn test_assign_natural() {
    let test = |u, v, out| {
        let mut x = native::Natural::from_str(u).unwrap();
        x.assign(&native::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Natural::from_str(u).unwrap();
        x.assign(&gmp::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test("123", "456", "456");
    test("123", "1000000000000", "1000000000000");
    test("1000000000000", "123", "123");
    test("1000000000000", "2000000000000", "2000000000000");
}

#[test]
fn test_clone() {
    let test = |u| {
        let x = native::Natural::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = gmp::Natural::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = num::BigUint::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
}

#[test]
fn test_clone_from() {
    let test = |u, v, out| {
        let mut x = native::Natural::from_str(u).unwrap();
        x.clone_from(&native::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Natural::from_str(u).unwrap();
        x.clone_from(&gmp::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = num::BigUint::from_str(u).unwrap();
        x.clone_from(&num::BigUint::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
    };
    test("123", "456", "456");
    test("123", "1000000000000", "1000000000000");
    test("1000000000000", "123", "123");
    test("1000000000000", "2000000000000", "2000000000000");
}
