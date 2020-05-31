use std::cmp::max;

use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::logic::xor::{
    limbs_neg_xor_limb, limbs_neg_xor_limb_neg, limbs_neg_xor_limb_neg_in_place,
    limbs_neg_xor_limb_neg_to_out, limbs_neg_xor_limb_to_out, limbs_pos_xor_limb_neg,
    limbs_pos_xor_limb_neg_to_out, limbs_slice_neg_xor_limb_in_place,
    limbs_slice_pos_xor_limb_neg_in_place, limbs_vec_neg_xor_limb_in_place,
    limbs_vec_pos_xor_limb_neg_in_place, limbs_xor_neg_neg, limbs_xor_neg_neg_in_place_either,
    limbs_xor_neg_neg_in_place_left, limbs_xor_neg_neg_to_out, limbs_xor_pos_neg,
    limbs_xor_pos_neg_in_place_either, limbs_xor_pos_neg_in_place_left,
    limbs_xor_pos_neg_in_place_right, limbs_xor_pos_neg_to_out,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz_test_util::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_nz_test_util::integer::logic::xor::{integer_xor_alt_1, integer_xor_alt_2};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_signeds,
    pairs_of_unsigned_vec_and_unsigned_var_2, pairs_of_unsigned_vec_var_6,
    triples_of_limb_vec_var_7, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
};
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use malachite_test::inputs::natural::pairs_of_naturals;

#[test]
fn limbs_neg_xor_limb_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_unsigned_var_2,
        |&(ref limbs, limb)| {
            assert_eq!(
                -Natural::from_owned_limbs_asc(limbs_neg_xor_limb(limbs, limb)),
                -Natural::from_limbs_asc(limbs) ^ Integer::from(limb)
            );
        },
    );
}

#[test]
fn limbs_neg_xor_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let len = in_limbs.len();
            if limbs_neg_xor_limb_to_out(&mut out, in_limbs, limb) {
                let mut result_limbs = Natural::exact_from(
                    -(Integer::from(Natural::from_limbs_asc(in_limbs)) ^ Integer::from(limb)),
                )
                .to_limbs_asc();
                result_limbs.resize(len, 0);
                assert_eq!(result_limbs, &out[..len]);
            } else {
                assert_eq!(
                    -Natural::from_limbs_asc(&out[..len]),
                    -Natural::from_limbs_asc(in_limbs) ^ Integer::from(limb),
                );
            }
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_neg_xor_limb_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_unsigned_var_2,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            if limbs_slice_neg_xor_limb_in_place(&mut limbs, limb) {
                let mut result_limbs = Natural::exact_from(
                    -(Integer::from(Natural::from_owned_limbs_asc(old_limbs))
                        ^ Integer::from(limb)),
                )
                .to_limbs_asc();
                result_limbs.resize(limbs.len(), 0);
                assert_eq!(result_limbs, limbs);
            } else {
                assert_eq!(
                    -Natural::from_limbs_asc(&limbs),
                    -Natural::from_owned_limbs_asc(old_limbs) ^ Integer::from(limb)
                );
            }
        },
    );
}

#[test]
fn limbs_vec_neg_xor_limb_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_unsigned_var_2,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_neg_xor_limb_in_place(&mut limbs, limb);
            assert_eq!(
                -Natural::from_limbs_asc(&limbs),
                -Natural::from_owned_limbs_asc(old_limbs) ^ Integer::from(limb)
            );
        },
    );
}

#[test]
fn limbs_pos_xor_limb_neg_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let out = limbs_pos_xor_limb_neg(limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(limbs))
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            assert_eq!(Natural::from_owned_limbs_asc(out), Natural::exact_from(-n));
        },
    );
}

#[test]
fn limbs_pos_xor_limb_neg_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            limbs_pos_xor_limb_neg_to_out(&mut out, in_limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(in_limbs))
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            let len = in_limbs.len();
            let mut limbs = Natural::exact_from(-n).into_limbs_asc();
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_pos_xor_limb_neg_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut mut_limbs = limbs.to_vec();
            let carry = limbs_slice_pos_xor_limb_neg_in_place(&mut mut_limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(&limbs))
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            if carry {
                let result_limbs = Natural::exact_from(-n).to_limbs_asc();
                assert_eq!(mut_limbs, &result_limbs[..limbs.len()]);
            } else {
                assert_eq!(
                    Natural::from_owned_limbs_asc(mut_limbs),
                    Natural::exact_from(-n)
                );
            }
        },
    );
}

#[test]
fn limbs_vec_pos_xor_limb_neg_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut mut_limbs = limbs.to_vec();
            limbs_vec_pos_xor_limb_neg_in_place(&mut mut_limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(&limbs))
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            assert_eq!(
                Natural::from_owned_limbs_asc(mut_limbs),
                Natural::exact_from(-n)
            );
        },
    );
}

#[test]
fn limbs_neg_xor_limb_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_unsigned_var_2,
        |&(ref limbs, limb)| {
            let out = limbs_neg_xor_limb_neg(limbs, limb);
            let n = -Natural::from_limbs_asc(limbs)
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            assert_eq!(Natural::from_owned_limbs_asc(out), Natural::exact_from(n));
        },
    );
}

#[test]
fn limbs_neg_xor_limb_neg_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            limbs_neg_xor_limb_neg_to_out(&mut out, in_limbs, limb);
            let n = -Natural::from_limbs_asc(in_limbs)
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            let len = in_limbs.len();
            let mut limbs = Natural::exact_from(n).into_limbs_asc();
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_neg_xor_limb_neg_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_unsigned_var_2,
        |&(ref limbs, limb)| {
            let mut mut_limbs = limbs.to_vec();
            limbs_neg_xor_limb_neg_in_place(&mut mut_limbs, limb);
            let n = -Natural::from_limbs_asc(&limbs)
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            let mut expected_limbs = Natural::exact_from(n).into_limbs_asc();
            expected_limbs.resize(limbs.len(), 0);
            assert_eq!(mut_limbs, expected_limbs);
        },
    );
}

#[test]
fn limbs_xor_pos_neg_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_xor_pos_neg(xs, ys)),
            Integer::from(Natural::from_limbs_asc(xs)) ^ -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_pos_neg_to_out_properties() {
    test_properties(triples_of_limb_vec_var_7, |&(ref out, ref xs, ref ys)| {
        let mut out = out.to_vec();
        let out_old = out.clone();
        limbs_xor_pos_neg_to_out(&mut out, xs, ys);
        let len = max(xs.len(), ys.len());
        assert_eq!(
            -Natural::from_limbs_asc(&out[..len]),
            Integer::from(Natural::from_limbs_asc(xs)) ^ -Natural::from_limbs_asc(ys)
        );
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_xor_pos_neg_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_xor_pos_neg_in_place_left(&mut xs, ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            Integer::from(Natural::from_owned_limbs_asc(xs_old)) ^ -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_pos_neg_in_place_right_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        limbs_xor_pos_neg_in_place_right(xs, &mut ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(ys),
            Integer::from(Natural::from_limbs_asc(xs)) ^ -Natural::from_owned_limbs_asc(ys_old)
        );
    });
}

#[test]
fn limbs_xor_pos_neg_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let right = limbs_xor_pos_neg_in_place_either(&mut xs, &mut ys);
        let expected =
            Integer::from(Natural::from_limbs_asc(&xs_old)) ^ -Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(-Natural::from_owned_limbs_asc(ys), expected);
        } else {
            assert_eq!(-Natural::from_owned_limbs_asc(xs), expected);
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn limbs_xor_neg_neg_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_xor_neg_neg(xs, ys)),
            -Natural::from_limbs_asc(xs) ^ -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_neg_neg_to_out_properties() {
    test_properties(triples_of_limb_vec_var_7, |&(ref xs, ref ys, ref zs)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_xor_neg_neg_to_out(&mut xs, ys, zs);
        let len = max(ys.len(), zs.len());
        let result =
            Natural::exact_from(-Natural::from_limbs_asc(ys) ^ -Natural::from_limbs_asc(zs));
        let mut expected_limbs = result.to_limbs_asc();
        expected_limbs.resize(len, 0);
        assert_eq!(&xs[..len], expected_limbs.as_slice());
        assert_eq!(&xs[len..], &xs_old[len..]);
    });
}

#[test]
fn limbs_xor_neg_neg_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_xor_neg_neg_in_place_left(&mut xs, ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            -Natural::from_owned_limbs_asc(xs_old) ^ -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_neg_neg_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let right = limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys);
        let expected = -Natural::from_limbs_asc(&xs_old) ^ -Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(Natural::from_owned_limbs_asc(ys), expected);
        } else {
            assert_eq!(Natural::from_owned_limbs_asc(xs), expected);
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn xor_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let result_val_val = x.clone() ^ y.clone();
        let result_val_ref = x.clone() ^ y;
        let result_ref_val = x ^ y.clone();
        let result = x ^ y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result, "{} {}", x, y);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x ^= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x ^= y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x ^= integer_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), result);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) ^ integer_to_rug_integer(y))),
            result
        );

        assert_eq!(integer_xor_alt_1(&x, y), result);
        assert_eq!(integer_xor_alt_2(&x, y), result);

        assert_eq!(y ^ x, result);
        assert_eq!(&result ^ x, *y);
        assert_eq!(&result ^ y, *x);
        assert_eq!(!x ^ !y, result);
        assert_eq!(!(x ^ !y), result);
        assert_eq!(!(!x ^ y), result);
    });

    test_properties(integers, |x| {
        assert_eq!(x ^ Integer::ZERO, *x);
        assert_eq!(Integer::ZERO ^ x, *x);
        assert_eq!(x ^ Integer::NEGATIVE_ONE, !x);
        assert_eq!(Integer::NEGATIVE_ONE ^ x, !x);
        assert_eq!(x ^ x, 0);
        assert_eq!(x ^ !x, -1);
        assert_eq!(!x ^ x, -1);
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        assert_eq!((x ^ y) ^ z, x ^ (y ^ z));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(i, j)| {
        assert_eq!(Integer::from(i) ^ Integer::from(j), i ^ j);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x) ^ Integer::from(y), x ^ y);
    });
}
