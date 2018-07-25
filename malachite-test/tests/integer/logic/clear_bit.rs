use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{BitAccess, NotAssign, One};
use malachite_nz::integer::logic::bit_access::{
    limbs_slice_clear_bit_neg, limbs_vec_clear_bit_neg,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::{
    pairs_of_u32_vec_and_small_u64_var_3, pairs_of_unsigned_vec_and_small_u64_var_1,
};
use malachite_test::inputs::integer::pairs_of_integer_and_small_u64;
use std::str::FromStr;

#[test]
pub fn test_limbs_slice_clear_bit_neg() {
    let test = |limbs: &[u32], index: u64, out_limbs: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_slice_clear_bit_neg(&mut mut_limbs, index);
        assert_eq!(mut_limbs, out_limbs);
    };
    test(&[3, 2, 1], 0, &[4, 2, 1]);
    test(&[0, 0, 3], 32, &[0, 0, 3]);
    test(&[0, 3, 2, 1], 64, &[0, 3, 3, 1]);
    test(&[0, 0, 0xffff_fffd], 64, &[0, 0, 0xffff_fffe]);
    test(&[0xffff_fff7], 3, &[0xffff_ffff]);
}

#[test]
#[should_panic(expected = "Setting bit cannot be done within existing slice")]
fn limbs_slice_clear_bit_fail_1() {
    let mut mut_limbs = vec![0, 0, 0xffff_ffff];
    limbs_slice_clear_bit_neg(&mut mut_limbs, 64);
}

#[test]
#[should_panic(expected = "Setting bit cannot be done within existing slice")]
fn limbs_slice_clear_bit_fail_2() {
    let mut mut_limbs = vec![3, 2, 1];
    limbs_slice_clear_bit_neg(&mut mut_limbs, 100);
}

#[test]
pub fn test_limbs_vec_clear_bit_neg() {
    let test = |limbs: &[u32], index: u64, out_limbs: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_vec_clear_bit_neg(&mut mut_limbs, index);
        assert_eq!(mut_limbs, out_limbs);
    };
    test(&[3, 2, 1], 0, &[4, 2, 1]);
    test(&[0, 0, 3], 32, &[0, 0, 3]);
    test(&[0, 3, 2, 1], 64, &[0, 3, 3, 1]);
    test(&[0, 0, 0xffff_fffd], 64, &[0, 0, 0xffff_fffe]);
    test(&[0, 0, 0xffff_ffff], 64, &[0, 0, 0, 1]);
    test(&[3, 2, 1], 100, &[3, 2, 1, 16]);
    test(&[0xffff_fff7], 3, &[0xffff_ffff]);
    test(&[0xffff_fff8], 3, &[0, 1]);
}

#[test]
fn test_clear_bit() {
    let test = |u, index, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.clear_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 10, "0");
    test("0", 100, "0");
    test("1024", 10, "0");
    test("101", 0, "100");
    test("1000000001024", 10, "1000000000000");
    test("1000000001024", 100, "1000000001024");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("1267650600228229401496703205381", 100, "5");
    test("-1", 5, "-33");
    test("-1", 100, "-1267650600228229401496703205377");
    test("-31", 0, "-32");
    test("-999999998976", 10, "-1000000000000");
    test("-1000000000000", 100, "-1267650600228229402496703205376");
    test("-18446744078004518912", 0, "-18446744078004518912");
    test("-18446744078004518912", 32, "-18446744082299486208");
    test("-18446744078004518912", 33, "-18446744086594453504");
    test("-18446744078004518912", 64, "-18446744078004518912");
    test("-18446744078004518912", 65, "-55340232225423622144");
    test("-36893488143124135936", 32, "-36893488147419103232");
    test("-4294967295", 0, "-4294967296");
    test("-4294967287", 3, "-4294967295");
    test("-4294967288", 3, "-4294967296");
}

macro_rules! limbs_clear_bit_neg_helper {
    ($f:ident, $limbs:ident, $index:ident) => {
        |&(ref $limbs, $index)| {
            let mut mut_limbs = $limbs.clone();
            let mut n = -Natural::from_limbs_asc($limbs);
            $f(&mut mut_limbs, $index);
            n.clear_bit($index);
            assert_eq!(-Natural::from_limbs_asc(&mut_limbs), n);
        }
    };
}

#[test]
fn limbs_slice_clear_bit_neg_properties() {
    test_properties(
        pairs_of_u32_vec_and_small_u64_var_3,
        limbs_clear_bit_neg_helper!(limbs_slice_clear_bit_neg, limbs, index),
    );
}

#[test]
fn limbs_vec_clear_bit_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_u64_var_1,
        limbs_clear_bit_neg_helper!(limbs_vec_clear_bit_neg, limbs, index),
    );
}

#[test]
fn clear_bit_properties() {
    test_properties(pairs_of_integer_and_small_u64, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.clear_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, false);
        assert_eq!(mut_n, result);

        assert_eq!(
            n & !(Integer::ONE << u32::checked_from(index).unwrap()),
            result
        );

        assert!(result <= *n);
        if n.get_bit(index) {
            assert_ne!(result, *n);
            let mut mut_result = result.clone();
            mut_result.set_bit(index);
            assert_eq!(mut_result, *n);
        } else {
            assert_eq!(result, *n);
        }

        let mut mut_not_n = !n;
        mut_not_n.set_bit(index);
        mut_not_n.not_assign();
        assert_eq!(mut_not_n, result);
    });
}
