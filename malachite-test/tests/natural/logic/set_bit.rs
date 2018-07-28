use common::test_properties;
use malachite_base::num::{BitAccess, One};
use malachite_nz::natural::logic::bit_access::{limbs_slice_set_bit, limbs_vec_set_bit};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_u32_vec_and_small_u64_var_2, pairs_of_unsigned_vec_and_small_u64,
};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;
use malachite_test::natural::logic::set_bit::num_set_bit;
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
pub fn test_limbs_slice_set_bit() {
    let test = |limbs: &[u32], index: u64, out_limbs: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_slice_set_bit(&mut mut_limbs, index);
        assert_eq!(mut_limbs, out_limbs);
    };
    test(&[0, 1], 0, &[1, 1]);
    test(&[1, 1], 0, &[1, 1]);
    test(&[1, 1], 1, &[3, 1]);
    test(&[3, 1], 33, &[3, 3]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn limbs_slice_set_bit_fail() {
    let mut mut_limbs = vec![1, 2, 3];
    limbs_slice_set_bit(&mut mut_limbs, 100);
}

#[test]
pub fn test_limbs_vec_set_bit() {
    let test = |limbs: &[u32], index: u64, out_limbs: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_vec_set_bit(&mut mut_limbs, index);
        assert_eq!(mut_limbs, out_limbs);
    };
    test(&[0, 1], 0, &[1, 1]);
    test(&[1, 1], 0, &[1, 1]);
    test(&[1, 1], 1, &[3, 1]);
    test(&[3, 1], 33, &[3, 3]);
    test(&[3, 3], 100, &[3, 3, 0, 16]);
    test(&[3, 3], 128, &[3, 3, 0, 0, 1]);
    test(&[], 32, &[0, 1]);
}

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
fn limbs_slice_set_bit_properties() {
    test_properties(
        pairs_of_u32_vec_and_small_u64_var_2,
        |&(ref limbs, index)| {
            let mut mut_limbs = limbs.clone();
            let mut n = Natural::from_limbs_asc(limbs);
            limbs_slice_set_bit(&mut mut_limbs, index);
            n.set_bit(index);
            assert_eq!(Natural::from_limbs_asc(&mut_limbs), n);
        },
    );
}

#[test]
fn limbs_vec_set_bit_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_u64,
        |&(ref limbs, index)| {
            let mut mut_limbs = limbs.clone();
            let mut n = Natural::from_limbs_asc(limbs);
            limbs_vec_set_bit(&mut mut_limbs, index);
            n.set_bit(index);
            assert_eq!(Natural::from_limbs_asc(&mut_limbs), n);
        },
    );
}

#[test]
fn set_bit_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, index)| {
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

        assert_eq!(n | (Natural::ONE << index), result);

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
