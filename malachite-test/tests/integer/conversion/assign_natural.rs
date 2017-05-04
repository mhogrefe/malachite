use malachite_native as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use std::str::FromStr;

#[test]
fn test_assign_natural() {
    let test = |u, v, out| {
        let mut x = native::integer::Integer::from_str(u).unwrap();
        x.assign(&native::natural::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::integer::Integer::from_str(u).unwrap();
        x.assign(&gmp::natural::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test("-123", "456", "456");
    test("-123", "1000000000000", "1000000000000");
    test("1000000000000", "123", "123");
    test("1000000000000", "2000000000000", "2000000000000");
}
