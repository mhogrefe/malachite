use std::cmp::Ordering;

use malachite_nz::natural::comparison::ord::{limbs_cmp, limbs_cmp_same_length};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{natural_to_biguint, natural_to_rug_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_2, pairs_of_unsigneds,
    triples_of_unsigned_vec_var_1, triples_of_unsigned_vec_var_2, vecs_of_unsigned_var_1,
    vecs_of_unsigned_var_2,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};

#[test]
fn limbs_cmp_same_length_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        let ord = limbs_cmp_same_length(xs, ys);
        assert_eq!(
            Natural::from_limbs_asc(xs).cmp(&Natural::from_limbs_asc(ys)),
            ord
        );
        assert_eq!(limbs_cmp_same_length(ys, xs).reverse(), ord);
    });

    test_properties(vecs_of_unsigned_var_1, |xs| {
        assert_eq!(limbs_cmp_same_length(xs, xs), Ordering::Equal);
    });

    test_properties(
        triples_of_unsigned_vec_var_1,
        |&(ref xs, ref ys, ref zs)| {
            if limbs_cmp_same_length(xs, ys) == Ordering::Less
                && limbs_cmp_same_length(ys, zs) == Ordering::Less
            {
                assert_eq!(limbs_cmp_same_length(xs, zs), Ordering::Less);
            } else if limbs_cmp_same_length(xs, ys) == Ordering::Greater
                && limbs_cmp_same_length(ys, zs) == Ordering::Greater
            {
                assert_eq!(limbs_cmp_same_length(xs, zs), Ordering::Greater);
            }
        },
    );
}

#[test]
fn limbs_cmp_properties() {
    test_properties(pairs_of_unsigned_vec_var_2, |&(ref xs, ref ys)| {
        let ord = limbs_cmp(xs, ys);
        assert_eq!(
            Natural::from_limbs_asc(xs).cmp(&Natural::from_limbs_asc(ys)),
            ord
        );
        assert_eq!(limbs_cmp(ys, xs).reverse(), ord);
    });

    test_properties(vecs_of_unsigned_var_2, |xs| {
        assert_eq!(limbs_cmp(xs, xs), Ordering::Equal);
    });

    test_properties(
        triples_of_unsigned_vec_var_2,
        |&(ref xs, ref ys, ref zs)| {
            if limbs_cmp(xs, ys) == Ordering::Less && limbs_cmp(ys, zs) == Ordering::Less {
                assert_eq!(limbs_cmp(xs, zs), Ordering::Less);
            } else if limbs_cmp(xs, ys) == Ordering::Greater
                && limbs_cmp(ys, zs) == Ordering::Greater
            {
                assert_eq!(limbs_cmp(xs, zs), Ordering::Greater);
            }
        },
    );
}

#[test]
fn cmp_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let ord = x.cmp(y);
        assert_eq!(natural_to_biguint(x).cmp(&natural_to_biguint(y)), ord);
        assert_eq!(
            natural_to_rug_integer(x).cmp(&natural_to_rug_integer(y)),
            ord
        );
        assert_eq!(y.cmp(x).reverse(), ord);
        assert_eq!((-y).cmp(&(-x)), ord);
    });

    test_properties(naturals, |x| {
        assert_eq!(x.cmp(x), Ordering::Equal);
    });

    test_properties(triples_of_naturals, |&(ref x, ref y, ref z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(Natural::from(x).cmp(&Natural::from(y)), x.cmp(&y));
    });
}
