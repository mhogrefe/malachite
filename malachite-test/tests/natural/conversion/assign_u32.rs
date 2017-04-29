use malachite_native::natural as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp::natural as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use malachite_test::natural::conversion::assign_u32::num_assign_u32;
use num;
use std::str::FromStr;

#[test]
fn test_assign_u32() {
    let test = |u, v: u32, out| {
        let mut x = native::Natural::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Natural::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = num::BigUint::from_str(u).unwrap();
        num_assign_u32(&mut x, v);
        assert_eq!(x.to_string(), out);
    };
    test("123", 456, "456");
    test("123", u32::max_value(), "4294967295");
    test("1000000000000", 123, "123");
}
