use common::test_properties;
use malachite_base::num::NotAssign;
use malachite_base::num::{BitAccess, One};
use malachite_nz::integer::logic::bit_access::limbs_set_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::pairs_of_unsigned_vec_and_small_unsigned_var_1;
use malachite_test::inputs::integer::pairs_of_integer_and_small_u64;
use std::str::FromStr;
#[cfg(feature = "32_bit_limbs")]
use std::u32;

#[cfg(feature = "32_bit_limbs")]
#[test]
pub fn test_limbs_set_bit_neg() {
    let test = |limbs: &[u32], index: u64, out_limbs: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_set_bit_neg(&mut mut_limbs, index);
        assert_eq!(mut_limbs, out_limbs);
    };
    test(&[3, 2, 1], 100, &[3, 2, 1]);
    test(&[0, 0, 0b1101, 0b11], 96, &[0, 0, 0b1101, 0b10]);
    test(&[0, 0, 0b1101, 0b11], 66, &[0, 0, 0b1001, 0b11]);
    test(&[0, 0, 0b1100, 0b11], 64, &[0, 0, 0b1011, 0b11]);
    test(&[0, 0, 0b1101, 0b11], 32, &[0, u32::MAX, 0b1100, 0b11]);
}

#[test]
fn test_set_bit() {
    let test = |u, index, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 10, "1024");
    test("100", 0, "101");
    test("1000000000000", 10, "1000000001024");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("5", 100, "1267650600228229401496703205381");
    test("-1", 5, "-1");
    test("-1", 100, "-1");
    test("-33", 5, "-1");
    test("-1267650600228229401496703205377", 100, "-1");
    test("-32", 0, "-31");
    test("-1000000000000", 10, "-999999998976");
    test("-1000000000000", 100, "-1000000000000");
    test("-1267650600228229402496703205376", 100, "-1000000000000");
    test("-18446744078004518912", 0, "-18446744078004518911");
    test("-18446744078004518912", 32, "-18446744078004518912");
    test("-18446744078004518912", 33, "-18446744078004518912");
    test("-18446744078004518912", 64, "-4294967296");
    test("-18446744078004518912", 65, "-18446744078004518912");
    test("-4294967296", 0, "-4294967295");
}

#[test]
fn limbs_set_bit_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, index)| {
            let mut mut_limbs = limbs.clone();
            let mut n = -Natural::from_limbs_asc(limbs);
            limbs_set_bit_neg(&mut mut_limbs, index);
            n.set_bit(index);
            assert_eq!(-Natural::from_limbs_asc(&mut_limbs), n);
        },
    );
}

#[test]
fn set_bit_properties() {
    test_properties(pairs_of_integer_and_small_u64, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.set_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, true);
        assert_eq!(mut_n, result);

        assert_eq!(n | (Integer::ONE << index), result);

        assert_ne!(result, 0 as Limb);
        assert!(result >= *n);
        if n.get_bit(index) {
            assert_eq!(result, *n);
        } else {
            assert_ne!(result, *n);
            let mut mut_result = result.clone();
            mut_result.clear_bit(index);
            assert_eq!(mut_result, *n);
        }

        let mut mut_not_n = !n;
        mut_not_n.clear_bit(index);
        mut_not_n.not_assign();
        assert_eq!(mut_not_n, result);
    });
}
