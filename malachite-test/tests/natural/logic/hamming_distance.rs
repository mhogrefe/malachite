use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{CheckedHammingDistance, CountOnes, HammingDistance};
use malachite_nz::natural::logic::hamming_distance::{
    limbs_hamming_distance, limbs_hamming_distance_limb, limbs_hamming_distance_same_length,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::natural_to_rug_integer;
use malachite_nz_test_util::natural::logic::hamming_distance::{
    natural_hamming_distance_alt_1, natural_hamming_distance_alt_2, rug_hamming_distance,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec_var_1,
    pairs_of_unsigned_vec_var_2, pairs_of_unsigneds,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};

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
