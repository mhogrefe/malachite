use common::test_properties;
use malachite_base::num::BitAccess;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rug_integer,
                             rug_integer_to_natural};
use malachite_test::inputs::natural::pairs_of_natural_and_small_u64;
use malachite_test::natural::logic::set_bit::num_set_bit;
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_set_bit() {
    let test = |u, index, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = BigUint::from_str(u).unwrap();
        num_set_bit(&mut n, index);
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from_str(u).unwrap();
        n.set_bit(index as u32, true);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "1024");
    test("100", 0, "101");
    test("1000000000000", 10, "1000000001024");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("5", 100, "1267650600228229401496703205381");
}

#[test]
fn set_bit_properties() {
    test_properties(pairs_of_natural_and_small_u64, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.set_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, true);
        assert_eq!(mut_n, result);

        let mut num_n = natural_to_biguint(n);
        num_set_bit(&mut num_n, index);
        assert_eq!(biguint_to_natural(&num_n), result);

        let mut rug_n = natural_to_rug_integer(n);
        rug_n.set_bit(index as u32, true);
        assert_eq!(rug_integer_to_natural(&rug_n), result);

        assert_ne!(result, 0);
        assert!(result >= *n);
        if n.get_bit(index) {
            assert_eq!(result, *n);
        } else {
            assert_ne!(result, *n);
            let mut mut_result = result.clone();
            mut_result.clear_bit(index);
            assert_eq!(mut_result, *n);
        }
    });
}
