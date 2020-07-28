use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, ModPowerOfTwoNeg, PowerOfTwo};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_in_place_left,
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_slice_sub_in_place_right, limbs_sub,
    limbs_sub_in_place_left, limbs_sub_limb, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_in_place_with_overlap, limbs_sub_same_length_to_out,
    limbs_sub_same_length_to_out_with_overlap, limbs_sub_to_out, limbs_vec_sub_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::natural::arithmetic::sub::{
    limbs_sub_same_length_in_place_with_overlap_naive,
    limbs_sub_same_length_to_out_with_overlap_naive,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds_var_1;
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_small_usize_var_1, pairs_of_unsigned_vec_and_unsigned,
    pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_3,
    quadruples_of_three_unsigned_vecs_and_bool_var_1, triples_of_two_unsigned_vecs_and_bool_var_1,
    triples_of_unsigned_vec_unsigned_and_small_usize_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1, triples_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_var_9, vecs_of_unsigned,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals_var_1};

#[test]
fn limbs_sub_limb_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let (result_limbs, borrow) = limbs_sub_limb(limbs, limb);
        if borrow {
            if limbs.is_empty() {
                assert_ne!(limb, 0);
                assert!(result_limbs.is_empty());
            } else {
                let mut result_limbs = result_limbs;
                result_limbs.push(Limb::MAX);
                assert_eq!(
                    Integer::from_owned_twos_complement_limbs_asc(result_limbs),
                    Integer::from(Natural::from_limbs_asc(limbs)) - Integer::from(limb)
                );
            }
        } else {
            assert_eq!(
                Natural::from_owned_limbs_asc(result_limbs),
                Natural::from_limbs_asc(limbs) - Natural::from(limb)
            );
        }
    });
}

#[test]
fn limbs_sub_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            if limbs_sub_limb_to_out(&mut out, in_limbs, limb) {
                let n = Integer::from(Natural::from_limbs_asc(in_limbs)) - Integer::from(limb);
                let len = in_limbs.len();
                let mut limbs = n.into_twos_complement_limbs_asc();
                limbs.resize(len, Limb::MAX);
                assert_eq!(limbs, &out[..len]);
                assert_eq!(&out[len..], &old_out[len..]);
            } else {
                let n = Natural::from_limbs_asc(in_limbs) - Natural::from(limb);
                let len = in_limbs.len();
                let mut limbs = n.into_limbs_asc();
                limbs.resize(len, 0);
                assert_eq!(limbs, &out[..len]);
                assert_eq!(&out[len..], &old_out[len..]);
            }
        },
    );
}

#[test]
fn limbs_sub_limb_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        if limbs_sub_limb_in_place(&mut limbs, limb) {
            let n = Integer::from(Natural::from_limbs_asc(&old_limbs)) - Integer::from(limb);
            let mut expected_limbs = n.into_twos_complement_limbs_asc();
            expected_limbs.resize(limbs.len(), Limb::MAX);
            assert_eq!(limbs, expected_limbs);
        } else {
            let n = Natural::from_limbs_asc(&old_limbs) - Natural::from(limb);
            let mut expected_limbs = n.into_limbs_asc();
            expected_limbs.resize(limbs.len(), 0);
            assert_eq!(limbs, expected_limbs);
        }
    });
}

#[test]
fn limbs_sub_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        let (limbs, borrow) = limbs_sub(xs, ys);
        let len = limbs.len();
        let n = Natural::from_owned_limbs_asc(limbs);
        if borrow {
            assert_eq!(
                n,
                Natural::from_limbs_asc(xs)
                    + Natural::from_limbs_asc(ys)
                        .mod_power_of_two_neg(u64::exact_from(len) << Limb::LOG_WIDTH)
            );
        } else {
            assert_eq!(n, Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys));
        }
    });
}

fn limbs_sub_to_out_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb], &[Limb]) -> bool,
    out: &Vec<Limb>,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    let mut out = out.to_vec();
    let old_out = out.clone();
    let len = xs.len();
    let mut limbs = if f(&mut out, xs, ys) {
        let n = Natural::from_limbs_asc(xs)
            + Natural::from_limbs_asc(ys)
                .mod_power_of_two_neg(u64::exact_from(len) << Limb::LOG_WIDTH);
        n.into_limbs_asc()
    } else {
        let n = Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys);
        n.into_limbs_asc()
    };
    limbs.resize(len, 0);
    assert_eq!(limbs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
}

#[test]
fn limbs_sub_same_length_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_3,
        |&(ref out, ref xs, ref ys)| {
            limbs_sub_to_out_helper(&mut limbs_sub_same_length_to_out, out, xs, ys);
        },
    );
}

#[test]
fn limbs_sub_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_9,
        |&(ref out, ref xs, ref ys)| {
            limbs_sub_to_out_helper(&mut limbs_sub_to_out, out, xs, ys);
        },
    );
}

fn limbs_sub_in_place_left_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb]) -> bool,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    let mut xs = xs.to_vec();
    let xs_old = xs.clone();
    let len = xs.len();
    let borrow = f(&mut xs, ys);
    let n = Natural::from_owned_limbs_asc(xs);
    if borrow {
        assert_eq!(
            n,
            Natural::from_owned_limbs_asc(xs_old)
                + Natural::from_limbs_asc(ys)
                    .mod_power_of_two_neg(u64::exact_from(len) << Limb::LOG_WIDTH)
        );
    } else {
        assert_eq!(
            n,
            Natural::from_owned_limbs_asc(xs_old) - Natural::from_limbs_asc(ys)
        );
    }
}

#[test]
fn limbs_sub_same_length_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        limbs_sub_in_place_left_helper(&mut limbs_sub_same_length_in_place_left, xs, ys);
    });
}

#[test]
fn limbs_sub_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        limbs_sub_in_place_left_helper(&mut limbs_sub_in_place_left, xs, ys);
    });
}

#[test]
fn limbs_slice_sub_in_place_right_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_small_usize_var_1,
        |&(ref xs, ref ys, len)| {
            let ys_old = ys;
            let mut ys = ys_old.clone();
            let borrow = limbs_slice_sub_in_place_right(xs, &mut ys, len);
            let xs_len = xs.len();
            let x = Natural::from_limbs_asc(xs);
            let y = Natural::from_limbs_asc(&ys_old[..len]);
            let n = Natural::from_limbs_asc(&ys[..xs_len]);
            if borrow {
                assert_eq!(
                    n,
                    x + y.mod_power_of_two_neg(u64::exact_from(xs_len) << Limb::LOG_WIDTH)
                );
            } else {
                assert_eq!(n, x - y);
            }
        },
    );

    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        let ys_old = ys;
        let mut ys = ys_old.clone();
        assert!(!limbs_slice_sub_in_place_right(xs, &mut ys, 0));
        assert_eq!(*xs, ys);
    });
}

macro_rules! limbs_vec_sub_in_place_right_helper {
    ($f:ident, $xs:ident, $ys:ident) => {
        |&(ref $xs, ref $ys)| {
            let mut ys = $ys.to_vec();
            let ys_old = $ys.clone();
            let borrow = $f($xs, &mut ys);
            let n = Natural::from_limbs_asc(&ys);
            if borrow {
                assert_eq!(
                    n,
                    Natural::from_limbs_asc($xs)
                        + Natural::from_owned_limbs_asc(ys_old)
                            .mod_power_of_two_neg(u64::exact_from($xs.len()) << Limb::LOG_WIDTH)
                );
            } else {
                assert_eq!(
                    n,
                    Natural::from_limbs_asc($xs) - Natural::from_owned_limbs_asc(ys_old)
                );
            }
        }
    };
}

#[test]
fn limbs_sub_same_length_in_place_right_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_1,
        limbs_vec_sub_in_place_right_helper!(limbs_sub_same_length_in_place_right, xs, ys),
    );
}

#[test]
fn limbs_vec_sub_in_place_right_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_3,
        limbs_vec_sub_in_place_right_helper!(limbs_vec_sub_in_place_right, xs, ys),
    );
}

#[test]
fn limbs_sub_same_length_with_borrow_in_to_out_properties() {
    test_properties(
        quadruples_of_three_unsigned_vecs_and_bool_var_1,
        |&(ref out, ref xs, ref ys, borrow_in)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let len = xs.len();
            let n = if _limbs_sub_same_length_with_borrow_in_to_out(&mut out, xs, ys, borrow_in) {
                let mut n = Integer::from(Natural::from_limbs_asc(xs))
                    - Integer::from(Natural::from_limbs_asc(ys));
                if borrow_in {
                    n -= Integer::ONE;
                }
                assert!(n < 0);
                n.mod_power_of_two(u64::exact_from(len) << Limb::LOG_WIDTH)
            } else {
                let mut n = Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys);
                if borrow_in {
                    n -= Natural::ONE;
                }
                n
            };
            let mut limbs = n.into_limbs_asc();
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_sub_same_length_with_borrow_in_in_place_left_properties() {
    test_properties(
        triples_of_two_unsigned_vecs_and_bool_var_1,
        |&(ref xs, ref ys, borrow_in)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            let len = xs.len();
            let borrow =
                _limbs_sub_same_length_with_borrow_in_in_place_left(&mut xs, ys, borrow_in);
            let n = Natural::from_owned_limbs_asc(xs);
            let mut expected_result = if borrow {
                let bit_len = u64::exact_from(len) << Limb::LOG_WIDTH;
                let mut neg_y = Natural::from_limbs_asc(ys).mod_power_of_two_neg(bit_len);
                if neg_y == 0 {
                    neg_y = Natural::power_of_two(bit_len);
                }
                Natural::from_owned_limbs_asc(xs_old) + neg_y
            } else {
                Natural::from_owned_limbs_asc(xs_old) - Natural::from_limbs_asc(ys)
            };
            if borrow_in {
                expected_result -= Natural::ONE;
            }
            assert_eq!(n, expected_result);
        },
    );
}

#[test]
fn limbs_sub_same_length_in_place_with_overlap_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_usize_var_1,
        |&(ref xs, right_start)| {
            let xs_old = xs;
            let mut xs = xs_old.clone();
            let borrow = limbs_sub_same_length_in_place_with_overlap(&mut xs, right_start);
            let len = xs.len() - right_start;
            let x = Natural::from_limbs_asc(&xs_old[..len]);
            let y = Natural::from_limbs_asc(&xs_old[right_start..]);
            let n = Natural::from_limbs_asc(&xs[..len]);
            if borrow {
                assert_eq!(
                    n,
                    x + y.mod_power_of_two_neg(u64::exact_from(len) << Limb::LOG_WIDTH)
                );
            } else {
                assert_eq!(n, x - y);
            }
            assert_eq!(&xs[len..], &xs_old[len..]);

            let mut xs_alt = xs_old.clone();
            assert_eq!(
                limbs_sub_same_length_in_place_with_overlap_naive(&mut xs_alt, right_start),
                borrow
            );
            assert_eq!(xs_alt, xs);
        },
    );

    test_properties(vecs_of_unsigned, |ref xs| {
        let xs_old = xs;
        let mut xs = xs_old.to_vec();
        assert!(!limbs_sub_same_length_in_place_with_overlap(&mut xs, 0));
        assert!(slice_test_zero(&xs));

        let mut xs = xs_old.to_vec();
        assert!(!limbs_sub_same_length_in_place_with_overlap(
            &mut xs,
            xs_old.len(),
        ));
        assert_eq!(xs, **xs_old);
    });
}

#[test]
fn limbs_sub_same_length_to_out_with_overlap_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        let xs_old = xs;
        let mut xs = xs_old.clone();
        let borrow = limbs_sub_same_length_to_out_with_overlap(&mut xs, ys);
        let len = ys.len();
        let x = Natural::from_limbs_asc(&xs_old[xs.len() - len..]);
        let y = Natural::from_limbs_asc(ys);
        let n = Natural::from_limbs_asc(&xs[..len]);
        if borrow {
            assert_eq!(
                n,
                x + y.mod_power_of_two_neg(u64::exact_from(len) << Limb::LOG_WIDTH)
            );
        } else {
            assert_eq!(n, x - y);
        }
        if len <= xs.len() - len {
            assert_eq!(&xs[len..xs.len() - len], &xs_old[len..xs.len() - len]);
        }

        let mut xs_alt = xs_old.clone();
        assert_eq!(
            limbs_sub_same_length_to_out_with_overlap_naive(&mut xs_alt, ys),
            borrow
        );
        assert_eq!(xs_alt, xs);
    });

    test_properties(vecs_of_unsigned, |ref xs| {
        let xs_old = xs;
        let mut xs = xs_old.to_vec();
        assert!(!limbs_sub_same_length_to_out_with_overlap(&mut xs, xs_old));
        assert!(slice_test_zero(&xs));

        let mut xs = xs_old.to_vec();
        assert!(!limbs_sub_same_length_to_out_with_overlap(&mut xs, &[]));
        assert_eq!(xs, **xs_old);
    });
}

#[test]
fn sub_properties() {
    test_properties(pairs_of_naturals_var_1, |&(ref x, ref y)| {
        let mut mut_x = x.clone();
        mut_x -= y.clone();
        assert!(mut_x.is_valid());
        let diff = mut_x;

        let mut mut_x = x.clone();
        mut_x -= y;
        assert!(mut_x.is_valid());
        let diff_alt = mut_x;
        assert_eq!(diff_alt, diff);

        let mut rug_x = natural_to_rug_integer(x);
        rug_x -= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&rug_x), diff);

        let diff_alt = x.clone() - y.clone();
        assert!(diff_alt.is_valid());
        assert_eq!(diff_alt, diff);

        let diff_alt = x.clone() - y;
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = x - y.clone();
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = x - y;
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) - natural_to_biguint(y))),
            diff
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) - natural_to_rug_integer(y))),
            diff
        );

        assert!(diff <= *x);
        assert_eq!(diff + y, *x);
    });

    test_properties(pairs_of_unsigneds_var_1::<Limb>, |&(x, y)| {
        assert_eq!(Natural::from(x - y), Natural::from(x) - Natural::from(y));
    });

    #[allow(unknown_lints, identity_op, eq_op)]
    test_properties(naturals, |x| {
        assert_eq!(x - Natural::ZERO, *x);
        assert_eq!(x - x, Natural::ZERO);
    });
}
