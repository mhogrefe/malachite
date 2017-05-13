use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::natural::logic::set_bit::num_set_bit;
use num;
use std::str::FromStr;

#[test]
fn test_set_bit() {
    let test = |u, index, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = num::BigUint::from_str(u).unwrap();
        num_set_bit(&mut n, index);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "1024");
    test("100", 0, "101");
    test("1000000000000", 10, "1000000001024");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("5", 100, "1267650600228229401496703205381");
}
