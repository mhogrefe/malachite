use malachite_native as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use std::str::FromStr;

#[test]
fn test_assign_integer() {
    let test = |u, v, out| {
        let mut x = native::natural::Natural::from_str(u).unwrap();
        x.assign(&native::integer::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::natural::Natural::from_str(u).unwrap();
        x.assign(&gmp::integer::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test("123", "456", "456");
    test("123", "1000000000000", "1000000000000");
    test("1000000000000", "123", "123");
    test("1000000000000", "2000000000000", "2000000000000");
}

#[test]
#[should_panic(expected = "Cannot assign from a negative Integer. Invalid other: -456")]
fn assign_integer_fail_native() {
    let mut x = native::natural::Natural::from_str("123").unwrap();
    x.assign(&native::integer::Integer::from_str("-456").unwrap());
}

#[test]
#[should_panic(expected = "Cannot assign from a negative Integer. Invalid other: -456")]
fn assign_integer_fail_gmp() {
    let mut x = gmp::natural::Natural::from_str("123").unwrap();
    x.assign(&gmp::integer::Integer::from_str("-456").unwrap());
}
