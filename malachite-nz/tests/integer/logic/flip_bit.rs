use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::integer::Integer;
use rug;
use std::str::FromStr;

#[test]
fn test_flip_bit() {
    let test = |u, index, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.flip_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n.toggle_bit(u32::exact_from(index));
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "1024");
    test("1024", 10, "0");
    test("100", 0, "101");
    test("101", 0, "100");
    test("1000000000000", 10, "1000000001024");
    test("1000000001024", 10, "1000000000000");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("5", 100, "1267650600228229401496703205381");
    test("1267650600228229401496703205381", 100, "5");
    test("-4294967296", 0, "-4294967295");
    test("-4294967295", 0, "-4294967296");
}
