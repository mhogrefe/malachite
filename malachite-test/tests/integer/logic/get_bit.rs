use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use rugint;
use std::str::FromStr;

#[test]
pub fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().get_bit(index as u32),
                   out);
    };

    test("0", 0, false);
    test("0", 100, false);
    test("123", 2, false);
    test("123", 3, true);
    test("123", 100, false);
    test("-123", 0, true);
    test("-123", 1, false);
    test("-123", 100, true);
    test("1000000000000", 12, true);
    test("1000000000000", 100, false);
    test("-1000000000000", 12, true);
    test("-1000000000000", 100, true);
    test("4294967295", 31, true);
    test("4294967295", 32, false);
    test("4294967296", 31, false);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("-4294967295", 0, true);
    test("-4294967295", 1, false);
    test("-4294967295", 31, false);
    test("-4294967295", 32, true);
    test("-4294967295", 33, true);
    test("-4294967296", 0, false);
    test("-4294967296", 31, false);
    test("-4294967296", 32, true);
    test("-4294967296", 33, true);
}
