use common::test_properties;
use malachite_base::num::{BitAccess, One};
use malachite_nz::integer::Integer;
use malachite_nz::natural::logic::bit_access::limbs_clear_bit;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned,
};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;
#[cfg(feature = "32_bit_limbs")]
use rug;
#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
pub fn test_limbs_clear_bit() {
    let test = |limbs: &[Limb], index: u64, out_limbs: &[Limb]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_clear_bit(&mut mut_limbs, index);
        assert_eq!(mut_limbs, out_limbs);
    };
    test(&[3, 3], 33, &[3, 1]);
    test(&[3, 1], 1, &[1, 1]);
    test(&[3, 3], 100, &[3, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_clear_bit() {
    let test = |u, index, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.clear_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n.set_bit(index as u32, false);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "0");
    test("0", 100, "0");
    test("1024", 10, "0");
    test("101", 0, "100");
    test("1000000001024", 10, "1000000000000");
    test("1000000001024", 100, "1000000001024");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("1267650600228229401496703205381", 100, "5");
}

#[test]
fn limbs_clear_bit_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, index)| {
            let mut mut_limbs = limbs.clone();
            let mut n = Natural::from_limbs_asc(limbs);
            limbs_clear_bit(&mut mut_limbs, index);
            n.clear_bit(index);
            assert_eq!(Natural::from_limbs_asc(&mut_limbs), n);
        },
    );
}

#[test]
fn clear_bit_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.clear_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut rug_n = natural_to_rug_integer(n);
        rug_n.set_bit(index as u32, false);
        assert_eq!(rug_integer_to_natural(&rug_n), result);

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, false);
        assert_eq!(mut_n, result);

        assert_eq!(Integer::from(n) & !(Natural::ONE << index), result);

        assert!(result <= *n);
        if n.get_bit(index) {
            assert_ne!(result, *n);
            let mut mut_result = result.clone();
            mut_result.set_bit(index);
            assert_eq!(mut_result, *n);
        } else {
            assert_eq!(result, *n);
        }
    });

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(u, index)| {
            let mut mut_u = u;
            mut_u.clear_bit(index);
            let mut n = Natural::from(u);
            n.clear_bit(index);
            assert_eq!(n, mut_u);
        },
    );
}
