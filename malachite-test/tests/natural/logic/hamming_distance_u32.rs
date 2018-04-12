use common::test_properties;
use malachite_base::num::{HammingDistance, Zero};
use malachite_nz::natural::logic::hamming_distance_u32::limbs_hamming_distance_limb;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::{pairs_of_nonempty_unsigned_vec_and_unsigned, unsigneds};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::logic::hamming_distance_u32::natural_hamming_distance_u32_alt;
use std::str::FromStr;
use std::u32;

#[test]
fn test_limbs_hamming_distance_limb() {
    let test = |limbs, limb, out| {
        assert_eq!(limbs_hamming_distance_limb(limbs, limb), out);
    };
    test(&[2], 3, 1);
    test(&[1, 1, 1], 1, 2);
    test(&[1, 1, 1], 2, 4);
    test(&[1, 2, 3], 0, 4);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_hamming_distance_limb_fail() {
    limbs_hamming_distance_limb(&[], 0);
}

#[test]
fn test_hamming_distance_u32() {
    let test = |n, u, out| {
        assert_eq!(Natural::from_str(n).unwrap().hamming_distance(u), out);
    };
    test("105", 123, 2);
    test("1000000000000", 0, 13);
    test("4294967295", 0, 32);
    test("4294967295", u32::MAX, 0);
}

#[test]
fn limbs_hamming_distance_limb_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            assert_eq!(
                limbs_hamming_distance_limb(limbs, limb),
                Natural::from_limbs_asc(limbs).hamming_distance(limb)
            );
        },
    );
}

#[test]
fn hamming_distance_u32_properties() {
    test_properties(pairs_of_natural_and_unsigned, |&(ref n, u)| {
        let distance = n.hamming_distance(u);
        assert_eq!(natural_hamming_distance_u32_alt(n, u), distance);
        //TODO xor
        //TODO assert_eq!((!n).checked_hamming_distance(!Natural::from(u)), Some(distance));
    });

    test_properties(naturals, |n| {
        assert_eq!(n.hamming_distance(0), n.count_ones());
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Natural::ZERO.hamming_distance(u), u64::from(u.count_ones()));
    });
}
