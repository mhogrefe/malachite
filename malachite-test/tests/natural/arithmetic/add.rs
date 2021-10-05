use std::cmp::max;

use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::arithmetic::add::{
    limbs_add, limbs_add_greater, limbs_add_greater_to_out, limbs_add_limb, limbs_add_limb_to_out,
    limbs_add_same_length_to_out, limbs_add_same_length_with_carry_in_in_place_left,
    limbs_add_same_length_with_carry_in_to_out, limbs_add_to_out, limbs_add_to_out_aliased,
    limbs_slice_add_greater_in_place_left, limbs_slice_add_in_place_either,
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
    limbs_vec_add_in_place_either, limbs_vec_add_in_place_left, limbs_vec_add_limb_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec,
    pairs_of_unsigned_vec_and_unsigned, pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_3,
    pairs_of_unsigneds, quadruples_of_three_unsigned_vecs_and_bool_var_1,
    triples_of_two_unsigned_vecs_and_bool_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
    triples_of_unsigned_vec_usize_and_unsigned_vec_var_1, triples_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_var_4, triples_of_unsigned_vec_var_9,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};

#[test]
fn limbs_add_limb_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_add_limb(limbs, limb)),
            Natural::from_limbs_asc(limbs) + Natural::from(limb)
        );
    });
}

#[test]
fn limbs_add_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let carry = limbs_add_limb_to_out(&mut out, in_limbs, limb);
            let n = Natural::from_limbs_asc(in_limbs) + Natural::from(limb);
            let len = in_limbs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_add_limb_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        let carry = limbs_slice_add_limb_in_place(&mut limbs, limb);
        let n = Natural::from_limbs_asc(&old_limbs) + Natural::from(limb);
        let mut expected_limbs = n.into_limbs_asc();
        assert_eq!(carry, expected_limbs.len() == limbs.len() + 1);
        expected_limbs.resize(limbs.len(), 0);
        assert_eq!(limbs, expected_limbs);
    });
}

#[test]
fn limbs_vec_add_limb_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_add_limb_in_place(&mut limbs, limb);
            let n = Natural::from_limbs_asc(&old_limbs) + Natural::from(limb);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

fn limbs_add_helper(f: &dyn Fn(&[Limb], &[Limb]) -> Vec<Limb>, xs: &Vec<Limb>, ys: &Vec<Limb>) {
    assert_eq!(
        Natural::from_owned_limbs_asc(f(xs, ys)),
        Natural::from_limbs_asc(xs) + Natural::from_limbs_asc(ys)
    );
}

#[test]
fn limbs_add_greater_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        limbs_add_helper(&limbs_add_greater, xs, ys);
    });
}

#[test]
fn limbs_add_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        limbs_add_helper(&limbs_add, xs, ys);
    });
}

fn limbs_add_to_out_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb], &[Limb]) -> bool,
    out_len: &dyn Fn(usize, usize) -> usize,
    out: &Vec<Limb>,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    let mut out = out.to_vec();
    let old_out = out.clone();
    let carry = f(&mut out, xs, ys);
    let n = Natural::from_limbs_asc(xs) + Natural::from_limbs_asc(ys);
    let len = out_len(xs.len(), ys.len());
    let mut limbs = n.into_limbs_asc();
    assert_eq!(carry, limbs.len() == len + 1);
    limbs.resize(len, 0);
    assert_eq!(limbs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
}

#[test]
fn limbs_add_greater_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_9,
        |&(ref out, ref xs, ref ys)| {
            limbs_add_to_out_helper(
                &mut limbs_add_greater_to_out,
                &|xs_len, _| xs_len,
                out,
                xs,
                ys,
            );
        },
    );
}

#[test]
fn limbs_add_same_length_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_3,
        |&(ref out, ref xs, ref ys)| {
            limbs_add_to_out_helper(
                &mut limbs_add_same_length_to_out,
                &|xs_len, _| xs_len,
                out,
                xs,
                ys,
            );
        },
    );
}

#[test]
fn limbs_add_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_4,
        |&(ref out, ref xs, ref ys)| {
            limbs_add_to_out_helper(&mut limbs_add_to_out, &max, out, xs, ys);
        },
    );
}

#[test]
fn limbs_add_to_out_aliased_properties() {
    test_properties(
        triples_of_unsigned_vec_usize_and_unsigned_vec_var_1,
        |&(ref xs, in_size, ref ys)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            let carry = limbs_add_to_out_aliased(&mut xs, in_size, ys);
            let n = Natural::from_limbs_asc(&xs_old[..in_size]) + Natural::from_limbs_asc(ys);
            let mut limbs = n.into_limbs_asc();
            let ys_len = ys.len();
            if limbs.len() < ys_len {
                limbs.resize(ys_len, 0);
            }
            assert_eq!(
                Natural::from_limbs_asc(&xs[..ys_len]),
                Natural::from_limbs_asc(&limbs[..ys_len]),
            );
            if carry {
                assert_eq!(limbs.len(), ys_len + 1);
                assert_eq!(*limbs.last().unwrap(), 1);
            } else {
                assert_eq!(limbs.len(), ys_len);
            }
            assert_eq!(&xs[ys_len..], &xs_old[ys_len..]);
        },
    );
}

fn limbs_slice_add_in_place_left_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb]) -> bool,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    let mut xs = xs.to_vec();
    let xs_old = xs.clone();
    let carry = f(&mut xs, ys);
    let n = Natural::from_owned_limbs_asc(xs_old) + Natural::from_limbs_asc(ys);
    let len = xs.len();
    let mut limbs = n.into_limbs_asc();
    assert_eq!(carry, limbs.len() == len + 1);
    limbs.resize(len, 0);
    assert_eq!(limbs, xs);
}

#[test]
fn limbs_slice_add_same_length_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        limbs_slice_add_in_place_left_helper(
            &mut limbs_slice_add_same_length_in_place_left,
            xs,
            ys,
        );
    });
}

#[test]
fn limbs_slice_add_greater_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        limbs_slice_add_in_place_left_helper(&mut limbs_slice_add_greater_in_place_left, xs, ys);
    });
}

#[test]
fn limbs_vec_add_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_vec_add_in_place_left(&mut xs, ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Natural::from_owned_limbs_asc(xs_old) + Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_slice_add_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let (right, b) = limbs_slice_add_in_place_either(&mut xs, &mut ys);
        let len = max(xs_old.len(), ys_old.len());
        let result = Natural::from_limbs_asc(&xs_old) + Natural::from_limbs_asc(&ys_old);
        let mut expected_limbs = result.to_limbs_asc();
        expected_limbs.resize(len, 0);
        assert_eq!(!b, Natural::from_limbs_asc(&expected_limbs) == result);
        if right {
            assert_eq!(ys, expected_limbs.as_slice());
            assert_eq!(xs, xs_old);
        } else {
            assert_eq!(xs, expected_limbs.as_slice());
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn limbs_vec_add_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_vec_add_in_place_either(&mut xs, &mut ys);
        let n = Natural::from_limbs_asc(&xs_old) + Natural::from_limbs_asc(&ys_old);
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
fn limbs_add_same_length_with_carry_in_to_out_properties() {
    test_properties(
        quadruples_of_three_unsigned_vecs_and_bool_var_1,
        |&(ref out, ref xs, ref ys, carry_in)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let carry = limbs_add_same_length_with_carry_in_to_out(&mut out, xs, ys, carry_in);
            let mut n = Natural::from_limbs_asc(xs) + Natural::from_limbs_asc(ys);
            if carry_in {
                n += Natural::ONE;
            }
            let len = xs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_add_same_length_with_carry_in_in_place_left_properties() {
    test_properties(
        triples_of_two_unsigned_vecs_and_bool_var_1,
        |&(ref xs, ref ys, carry_in)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            let carry = limbs_add_same_length_with_carry_in_in_place_left(&mut xs, ys, carry_in);
            let mut n = Natural::from_owned_limbs_asc(xs_old) + Natural::from_limbs_asc(ys);
            if carry_in {
                n += Natural::ONE;
            }
            let len = xs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, xs);
        },
    );
}

#[test]
fn add_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let sum_val_val = x.clone() + y.clone();
        let sum_val_ref = x.clone() + y;
        let sum_ref_val = x + y.clone();
        let sum = x + y;
        assert!(sum_val_val.is_valid());
        assert!(sum_val_ref.is_valid());
        assert!(sum_ref_val.is_valid());
        assert!(sum.is_valid());
        assert_eq!(sum_val_val, sum);
        assert_eq!(sum_val_ref, sum);
        assert_eq!(sum_ref_val, sum);

        let mut mut_x = x.clone();
        mut_x += y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x += y;
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        let mut mut_x = natural_to_rug_integer(x);
        mut_x += natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&mut_x), sum);

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) + natural_to_biguint(y))),
            sum
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) + natural_to_rug_integer(y))),
            sum
        );
        assert_eq!(y + x, sum);
        assert_eq!(&sum - x, *y);
        assert_eq!(&sum - y, *x);

        assert!(sum >= *x);
        assert!(sum >= *y);
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(
            Natural::from(DoubleLimb::from(x) + DoubleLimb::from(y)),
            Natural::from(x) + Natural::from(y)
        );
    });

    test_properties(naturals, |x| {
        assert_eq!(x + Natural::ZERO, *x);
        assert_eq!(Natural::ZERO + x, *x);
        assert_eq!(x + x, x << 1);
    });

    test_properties(triples_of_naturals, |&(ref x, ref y, ref z)| {
        assert_eq!((x + y) + z, x + (y + z));
    });
}
