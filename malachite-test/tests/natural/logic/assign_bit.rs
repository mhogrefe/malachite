use std::str::FromStr;

use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::natural::Natural;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::natural::triples_of_natural_small_u64_and_bool;

#[test]
fn test_assign_bit() {
    let test = |u, index, bit, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.assign_bit(index, bit);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n.set_bit(u32::checked_from(index).unwrap(), bit);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, true, "1024");
    test("100", 0, true, "101");
    test("1000000000000", 10, true, "1000000001024");
    test(
        "1000000000000",
        100,
        true,
        "1267650600228229402496703205376",
    );
    test("5", 100, true, "1267650600228229401496703205381");
    test("0", 10, false, "0");
    test("0", 100, false, "0");
    test("1024", 10, false, "0");
    test("101", 0, false, "100");
    test("1000000001024", 10, false, "1000000000000");
    test("1000000001024", 100, false, "1000000001024");
    test(
        "1267650600228229402496703205376",
        100,
        false,
        "1000000000000",
    );
    test("1267650600228229401496703205381", 100, false, "5");
}

#[test]
fn assign_bit_properties() {
    test_properties(
        triples_of_natural_small_u64_and_bool,
        |&(ref n, index, bit)| {
            let mut mut_n = n.clone();
            mut_n.assign_bit(index, bit);
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = natural_to_rug_integer(n);
            rug_n.set_bit(u32::checked_from(index).unwrap(), bit);
            assert_eq!(rug_integer_to_natural(&rug_n), result);
        },
    );
}
