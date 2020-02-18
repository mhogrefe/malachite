use std::str::FromStr;

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{CheckedHammingDistance, CountOnes, HammingDistance};
use malachite_nz::natural::logic::hamming_distance::{
    limbs_hamming_distance, limbs_hamming_distance_limb, limbs_hamming_distance_same_length,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use malachite_test::common::natural_to_rug_integer;
use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec_var_1,
    pairs_of_unsigned_vec_var_2, pairs_of_unsigneds,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};
use malachite_test::natural::logic::hamming_distance::{
    natural_hamming_distance_alt_1, natural_hamming_distance_alt_2, rug_hamming_distance,
};

#[cfg(feature = "32_bit_limbs")]
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

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_limb_fail() {
    limbs_hamming_distance_limb(&[], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_same_length() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance_same_length(xs, ys), out);
    };
    test(&[], &[], 0);
    test(&[2], &[3], 1);
    test(&[1, 1, 1], &[1, 2, 3], 3);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_limb_same_length_fail() {
    limbs_hamming_distance_same_length(&[1], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance(xs, ys), out);
    };
    test(&[], &[], 0);
    test(&[2], &[3], 1);
    test(&[1, 1, 1], &[1, 2, 3], 3);
    test(&[], &[1, 2, 3], 4);
    test(&[1, 2, 3], &[], 4);
    test(&[1, 1, 1], &[1, 2, 3, 4], 4);
    test(&[1, 2, 3, 4], &[1, 1, 1], 4);
}

#[test]
fn test_hamming_distance() {
    let test = |x, y, out| {
        assert_eq!(
            Natural::from_str(x)
                .unwrap()
                .hamming_distance(&Natural::from_str(y).unwrap()),
            out
        );
        assert_eq!(
            natural_hamming_distance_alt_1(
                &Natural::from_str(x).unwrap(),
                &Natural::from_str(y).unwrap(),
            ),
            out
        );
        assert_eq!(
            natural_hamming_distance_alt_2(
                &Natural::from_str(x).unwrap(),
                &Natural::from_str(y).unwrap(),
            ),
            out
        );
        assert_eq!(
            rug_hamming_distance(
                &rug::Integer::from_str(x).unwrap(),
                &rug::Integer::from_str(y).unwrap(),
            ),
            out
        );
    };
    test("105", "123", 2);
    test("1000000000000", "0", 13);
    test("4294967295", "0", 32);
    test("4294967295", "4294967295", 0);
    test("4294967295", "4294967296", 33);
    test("1000000000000", "1000000000001", 1);
}

#[test]
fn limbs_hamming_distance_limb_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            assert_eq!(
                limbs_hamming_distance_limb(limbs, limb),
                Natural::from_limbs_asc(limbs).hamming_distance(&Natural::from(limb))
            );
        },
    );
}

fn limbs_hamming_distance_helper(
    f: &mut dyn FnMut(&[Limb], &[Limb]) -> u64,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    assert_eq!(
        f(xs, ys),
        Natural::from_limbs_asc(xs).hamming_distance(&Natural::from_limbs_asc(ys))
    );
}

#[test]
fn limbs_hamming_distance_properties_same_length() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        limbs_hamming_distance_helper(&mut limbs_hamming_distance_same_length, xs, ys);
    });
}

#[test]
fn limbs_hamming_distance_properties() {
    test_properties(pairs_of_unsigned_vec_var_2, |&(ref xs, ref ys)| {
        limbs_hamming_distance_helper(&mut limbs_hamming_distance, xs, ys);
    });
}

#[test]
fn hamming_distance_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let distance = x.hamming_distance(y);
        assert_eq!(
            rug_hamming_distance(&natural_to_rug_integer(x), &natural_to_rug_integer(y)),
            distance
        );
        assert_eq!(y.hamming_distance(x), distance);
        assert_eq!(natural_hamming_distance_alt_1(x, y), distance);
        assert_eq!(natural_hamming_distance_alt_2(x, y), distance);
        assert_eq!(distance == 0, x == y);
        assert_eq!((x ^ y).count_ones(), distance);
        assert_eq!((!x).checked_hamming_distance(&!y), Some(distance));
    });

    test_properties(triples_of_naturals, |&(ref a, ref b, ref c)| {
        assert!(a.hamming_distance(c) <= a.hamming_distance(b) + b.hamming_distance(c));
    });

    test_properties(naturals, |n| {
        assert_eq!(n.hamming_distance(n), 0);
        assert_eq!(n.hamming_distance(&Natural::ZERO), n.count_ones());
        assert_eq!(Natural::ZERO.hamming_distance(n), n.count_ones());
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(
            Natural::from(x).hamming_distance(&Natural::from(y)),
            x.hamming_distance(y)
        );
    });
}
