use std::str::FromStr;

use num::BigInt;
use rug;

use malachite_nz::integer::Integer;

#[test]
fn test_clone() {
    let test = |u| {
        let x = Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = BigInt::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rug::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
}

#[test]
fn test_clone_and_clone_from() {
    let test = |u, v| {
        // clone_from
        let mut x = Integer::from_str(u).unwrap();
        x.clone_from(&Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = BigInt::from_str(u).unwrap();
        x.clone_from(&BigInt::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        let mut x = rug::Integer::from_str(u).unwrap();
        x.clone_from(&rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("-123", "456");
    test("-123", "1000000000000");
    test("1000000000000", "-123");
    test("1000000000000", "2000000000000");
}
