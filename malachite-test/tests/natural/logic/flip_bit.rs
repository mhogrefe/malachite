use std::str::FromStr;

use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{BitAccess, One};
use malachite_nz::natural::Natural;
use rug;

use common::test_properties;
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn test_flip_bit() {
    let test = |u, index, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.flip_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n.toggle_bit(u32::checked_from(index).unwrap());
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
}

#[test]
fn flip_bit_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.flip_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;
        assert_ne!(result, *n);

        let mut rug_n = natural_to_rug_integer(n);
        rug_n.toggle_bit(u32::checked_from(index).unwrap());
        assert_eq!(rug_integer_to_natural(&rug_n), result);

        let mut mut_result = result.clone();
        mut_result.flip_bit(index);
        assert_eq!(mut_result, *n);

        assert_eq!(n ^ (Natural::ONE << index), result);
    });
}
