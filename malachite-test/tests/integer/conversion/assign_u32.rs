use malachite_native::integer as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use malachite_test::integer::conversion::assign_u32::num_assign_u32;
use num;
use rugint::{self, Assign};
use std::str::FromStr;

#[test]
fn test_assign_u32() {
    let test = |u, v: u32, out| {
        let mut x = native::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = num::BigInt::from_str(u).unwrap();
        num_assign_u32(&mut x, v);
        assert_eq!(x.to_string(), out);

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
    };
    test("-123", 456, "456");
    test("123", u32::max_value(), "4294967295");
    test("1000000000000", 123, "123");
}
