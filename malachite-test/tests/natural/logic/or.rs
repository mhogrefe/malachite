use std::cmp::max;

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::CountOnes;
use malachite_nz::natural::logic::or::{
    limbs_or, limbs_or_in_place_either, limbs_or_in_place_left, limbs_or_limb,
    limbs_or_limb_in_place, limbs_or_limb_to_out, limbs_or_same_length,
    limbs_or_same_length_in_place_left, limbs_or_same_length_to_out, limbs_or_to_out,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::natural::logic::or::{natural_or_alt_1, natural_or_alt_2};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec,
    pairs_of_unsigned_vec_var_1, pairs_of_unsigneds,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2, triples_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_var_4,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};

#[test]
fn limbs_or_limb_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_or_limb(limbs, limb)),
                Natural::from_limbs_asc(limbs) | Natural::from(limb)
            );
        },
    );
}

#[test]
fn limbs_or_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            limbs_or_limb_to_out(&mut out, in_limbs, limb);
            let len = in_limbs.len();
            assert_eq!(
                Natural::from_limbs_asc(&out[..len]),
                Natural::from_limbs_asc(in_limbs) | Natural::from(limb)
            );
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_or_limb_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_or_limb_in_place(&mut limbs, limb);
            assert_eq!(
                Natural::from_limbs_asc(&limbs),
                Natural::from_limbs_asc(&old_limbs) | Natural::from(limb)
            );
        },
    );
}

fn limbs_or_helper(
    f: &mut dyn FnMut(&[Limb], &[Limb]) -> Vec<Limb>,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    assert_eq!(
        Natural::from_owned_limbs_asc(f(xs, ys)),
        Natural::from_limbs_asc(xs) | Natural::from_limbs_asc(ys)
    );
}

#[test]
fn limbs_or_same_length_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        limbs_or_helper(&mut limbs_or_same_length, xs, ys);
    });
}

#[test]
fn limbs_or_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        limbs_or_helper(&mut limbs_or, xs, ys);
    });
}

#[test]
fn limbs_or_same_length_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_3,
        |&(ref xs, ref ys, ref zs)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            limbs_or_same_length_to_out(&mut xs, ys, zs);
            let len = ys.len();
            assert_eq!(
                Natural::from_limbs_asc(&xs[..len]),
                Natural::from_limbs_asc(ys) | Natural::from_limbs_asc(zs)
            );
            assert_eq!(&xs[len..], &xs_old[len..]);
        },
    );
}

#[test]
fn limbs_or_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_4,
        |&(ref xs, ref ys, ref zs)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            limbs_or_to_out(&mut xs, ys, zs);
            let len = max(ys.len(), zs.len());
            assert_eq!(
                Natural::from_limbs_asc(&xs[..len]),
                Natural::from_limbs_asc(ys) | Natural::from_limbs_asc(zs)
            );
            assert_eq!(&xs[len..], &xs_old[len..]);
        },
    );
}

#[test]
fn limbs_or_same_length_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_or_same_length_in_place_left(&mut xs, ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Natural::from_owned_limbs_asc(xs_old) | Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_or_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_or_in_place_left(&mut xs, ys);
        let n = Natural::from_limbs_asc(&xs_old) | Natural::from_limbs_asc(ys);
        assert_eq!(Natural::from_owned_limbs_asc(xs), n);
    });
}

#[test]
fn limbs_or_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_or_in_place_either(&mut xs, &mut ys);
        let n = Natural::from_limbs_asc(&xs_old) | Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(Natural::from_owned_limbs_asc(ys), n);
        } else {
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn or_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let result_val_val = x.clone() | y.clone();
        let result_val_ref = x.clone() | y;
        let result_ref_val = x | y.clone();
        let result = x | y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x |= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x |= y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = natural_to_rug_integer(x);
        mut_x |= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&mut_x), result);

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) | natural_to_biguint(y))),
            result
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) | natural_to_rug_integer(y))),
            result
        );

        assert_eq!(natural_or_alt_1(&x, y), result);
        assert_eq!(natural_or_alt_2(&x, y), result);

        assert_eq!(y | x, result);
        assert_eq!(&result | x, result);
        assert_eq!(&result | y, result);

        assert!(result >= *x);
        assert!(result >= *y);

        let ones = result.count_ones();
        assert!(ones >= x.count_ones());
        assert!(ones >= y.count_ones());
    });

    test_properties(naturals, |x| {
        assert_eq!(x | Natural::ZERO, *x);
        assert_eq!(Natural::ZERO | x, *x);
        assert_eq!(x | x, *x);
    });

    test_properties(triples_of_naturals, |&(ref x, ref y, ref z)| {
        assert_eq!((x | y) | z, x | (y | z));
        assert_eq!(x & (y | z), (x & y) | (x & z));
        assert_eq!((x & y) | z, (x | z) & (y | z));
        assert_eq!(x | (y & z), (x | y) & (x | z));
        assert_eq!((x | y) & z, (x & z) | (y & z));
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(Natural::from(x) | Natural::from(y), x | y);
    });
}
