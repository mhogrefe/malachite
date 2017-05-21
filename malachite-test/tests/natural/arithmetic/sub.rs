use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use std::str::FromStr;

#[test]
fn test_sub_assign_natural() {
    let test = |u, v, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n -= native::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n -= gmp::Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
}

#[test]
fn test_sub_natural() {
    let test = |u, v, out| {
        let on = native::Natural::from_str(u).unwrap() - native::Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = gmp::Natural::from_str(u).unwrap() - gmp::Natural::from_str(v).unwrap();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "123", "None");
    test("123", "0", "Some(123)");
    test("456", "123", "Some(333)");
    test("1000000000000", "123", "Some(999999999877)");
    test("123", "1000000000000", "None");
    test("12345678987654321",
         "314159265358979",
         "Some(12031519722295342)");
    test("4294967296", "1", "Some(4294967295)");
    test("4294967295", "4294967295", "Some(0)");
    test("4294967296", "4294967295", "Some(1)");
    test("4294967296", "4294967296", "Some(0)");
    test("4294967295", "4294967296", "None");
    test("18446744073709551616", "1", "Some(18446744073709551615)");
    test("18446744073709551615", "18446744073709551615", "Some(0)");
    test("18446744073709551616", "18446744073709551615", "Some(1)");
    test("18446744073709551615", "18446744073709551616", "None");
}
