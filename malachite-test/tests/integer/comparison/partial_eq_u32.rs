use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::integer::comparison::partial_eq_u32::num_partial_eq_u32;
use num;
use rugint;
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(native::Integer::from_str(u).unwrap() == v, out);
        assert_eq!(gmp::Integer::from_str(u).unwrap() == v, out);
        assert_eq!(num_partial_eq_u32(&num::BigInt::from_str(u).unwrap(), v),
                   out);
        assert_eq!(rugint::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == native::Integer::from_str(u).unwrap(), out);
        assert_eq!(v == gmp::Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rugint::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("-123", 123, false);
    test("1000000000000", 123, false);
    test("-1000000000000", 123, false);
}
