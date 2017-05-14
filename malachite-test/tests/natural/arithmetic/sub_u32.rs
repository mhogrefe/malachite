use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use std::str::FromStr;

#[test]
fn test_sub_assign_u32() {
    let test = |u, v: u32, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("123", 123, "0");
    test("123", 0, "123");
    test("456", 123, "333");
    test("1000000000000", 123, "999999999877");
    test("4294967296", 1, "4294967295");
    test("18446744073709551616", 1, "18446744073709551615");
}

#[test]
#[should_panic(expected = "Cannot subtract a u32 from a smaller Natural. self: 123, other: 456")]
fn sub_assign_u32_native() {
    let mut x = native::Natural::from_str("123").unwrap();
    x -= 456;
}

#[test]
#[should_panic(expected = "Cannot subtract a u32 from a smaller Natural. self: 123, other: 456")]
fn sub_assign_u32_gmp() {
    let mut x = gmp::Natural::from_str("123").unwrap();
    x -= 456;
}

#[test]
fn test_sub_u32() {
    let test = |u, v: u32, out| {
        let on = native::Natural::from_str(u).unwrap() - v;
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = gmp::Natural::from_str(u).unwrap() - v;
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("123", 123, "Some(0)");
    test("123", 0, "Some(123)");
    test("456", 123, "Some(333)");
    test("123", 456, "None");
    test("1000000000000", 123, "Some(999999999877)");
    test("4294967296", 1, "Some(4294967295)");
    test("18446744073709551616", 1, "Some(18446744073709551615)");
}

#[test]
fn test_u32_sub_natural() {
    let test = |u: u32, v, out| {
        let on = u - native::Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = u - gmp::Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test(123, "123", "Some(0)");
    test(123, "0", "Some(123)");
    test(456, "123", "Some(333)");
    test(123, "456", "None");
    test(123, "1000000000000", "None");
    test(4294967295, "4294967295", "Some(0)");
}
