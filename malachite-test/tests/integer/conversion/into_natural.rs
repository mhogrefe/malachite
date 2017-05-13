use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use std::str::FromStr;

#[test]
fn test_into_natural() {
    let test = |n, out| {
        assert_eq!(format!("{:?}", native::Integer::from_str(n).unwrap().into_natural()),
                   out);
        assert_eq!(format!("{:?}", gmp::Integer::from_str(n).unwrap().into_natural()),
                   out);
    };
    test("0", "Some(0)");
    test("123", "Some(123)");
    test("-123", "None");
    test("1000000000000", "Some(1000000000000)");
    test("-1000000000000", "None");
    test("2147483647", "Some(2147483647)");
    test("2147483648", "Some(2147483648)");
    test("-2147483648", "None");
    test("-2147483649", "None");
}
