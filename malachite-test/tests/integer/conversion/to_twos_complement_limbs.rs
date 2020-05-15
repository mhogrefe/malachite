use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{small_unsigneds, vecs_of_unsigned, vecs_of_unsigned_var_3};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_small_unsigned, pairs_of_integer_and_vec_of_bool_var_1,
};
use malachite_test::integer::conversion::to_twos_complement_limbs::*;

#[test]
fn limbs_twos_complement_properties() {
    test_properties(vecs_of_unsigned_var_3, |limbs| {
        let out_limbs = limbs_twos_complement(limbs);
        if *limbs.last().unwrap() != 0 && out_limbs.last().unwrap().get_highest_bit() {
            let n = -Natural::from_limbs_asc(limbs);
            assert_eq!(n.to_twos_complement_limbs_asc(), out_limbs);
        }
    });
}

#[test]
fn limbs_maybe_sign_extend_non_negative_in_place_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        let mut mut_limbs = limbs.clone();
        limbs_maybe_sign_extend_non_negative_in_place(&mut mut_limbs);
        if !limbs.is_empty() && *limbs.last().unwrap() != 0 {
            let n = Integer::from(Natural::from_limbs_asc(limbs));
            assert_eq!(n.to_twos_complement_limbs_asc(), mut_limbs);
        }
    });
}

#[test]
fn limbs_twos_complement_in_place_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        let mut mut_limbs = limbs.clone();
        limbs_twos_complement_in_place(&mut mut_limbs);
        let mut mut_limbs_alt = limbs.clone();
        limbs_twos_complement_in_place_alt_1(&mut mut_limbs_alt);
        assert_eq!(mut_limbs_alt, mut_limbs);
        let mut mut_limbs_alt = limbs.clone();
        limbs_twos_complement_in_place_alt_2(&mut mut_limbs_alt);
        assert_eq!(mut_limbs_alt, mut_limbs);
        if !limbs.is_empty()
            && *limbs.last().unwrap() != 0
            && mut_limbs.last().unwrap().get_highest_bit()
        {
            let n = -Natural::from_limbs_asc(limbs);
            assert_eq!(n.to_twos_complement_limbs_asc(), mut_limbs);
        }
    });
}

#[test]
fn limbs_twos_complement_and_maybe_sign_extend_negative_in_place_properties() {
    test_properties(vecs_of_unsigned_var_3, |limbs| {
        let mut mut_limbs = limbs.clone();
        limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut mut_limbs);
        if !limbs.is_empty() && *limbs.last().unwrap() != 0 {
            let n = -Natural::from_limbs_asc(limbs);
            assert_eq!(n.to_twos_complement_limbs_asc(), mut_limbs);
        }
    });
}

#[test]
fn to_twos_complement_limbs_asc_properties() {
    test_properties(integers, |x| {
        let limbs = x.to_twos_complement_limbs_asc();
        assert_eq!(x.clone().into_twos_complement_limbs_asc(), limbs);
        assert_eq!(x.twos_complement_limbs().collect::<Vec<Limb>>(), limbs);
        assert_eq!(Integer::from_twos_complement_limbs_asc(&limbs), *x);
        assert_eq!(
            x.to_twos_complement_limbs_desc(),
            limbs.iter().cloned().rev().collect::<Vec<Limb>>()
        );
        match x.sign() {
            Ordering::Equal => assert!(limbs.is_empty()),
            Ordering::Greater => {
                let last = *limbs.last().unwrap();
                assert!(!last.get_highest_bit());
                if last == 0 {
                    assert!(limbs[limbs.len() - 2].get_highest_bit());
                }
            }
            Ordering::Less => {
                let last = *limbs.last().unwrap();
                assert!(last.get_highest_bit());
                if last == !0 && limbs.len() > 1 {
                    assert!(!limbs[limbs.len() - 2].get_highest_bit());
                }
            }
        }
    });
}

#[test]
fn limbs_desc_properties() {
    test_properties(integers, |x| {
        let limbs = x.to_twos_complement_limbs_desc();
        assert_eq!(x.clone().into_twos_complement_limbs_desc(), limbs);
        assert_eq!(
            x.twos_complement_limbs().rev().collect::<Vec<Limb>>(),
            limbs
        );
        assert_eq!(Integer::from_twos_complement_limbs_desc(&limbs), *x);
        assert_eq!(
            x.to_twos_complement_limbs_asc(),
            limbs.iter().cloned().rev().collect::<Vec<Limb>>()
        );
        match x.sign() {
            Ordering::Equal => assert!(limbs.is_empty()),
            Ordering::Greater => {
                let first = limbs[0];
                assert!(!first.get_highest_bit());
                if first == 0 {
                    assert!(limbs[1].get_highest_bit());
                }
            }
            Ordering::Less => {
                let first = limbs[0];
                assert!(first.get_highest_bit());
                if first == !0 && limbs.len() > 1 {
                    assert!(!limbs[1].get_highest_bit());
                }
            }
        }
    });
}

#[test]
fn twos_complement_limbs_properties() {
    test_properties(
        pairs_of_integer_and_vec_of_bool_var_1,
        |&(ref n, ref bs)| {
            let mut limbs = n.twos_complement_limbs();
            let mut limb_vec = Vec::new();
            let mut i = 0;
            for &b in bs {
                if b {
                    limb_vec.insert(i, limbs.next().unwrap());
                    i += 1;
                } else {
                    limb_vec.insert(i, limbs.next_back().unwrap())
                }
            }
            assert!(limbs.next().is_none());
            assert!(limbs.next_back().is_none());
            assert_eq!(n.to_twos_complement_limbs_asc(), limb_vec);
        },
    );

    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        if u < n.unsigned_abs_ref().limb_count() {
            assert_eq!(
                n.twos_complement_limbs().get(u),
                n.to_twos_complement_limbs_asc()[usize::exact_from(u)]
            );
        } else {
            assert_eq!(
                n.twos_complement_limbs().get(u),
                if *n >= 0 { 0 } else { Limb::MAX }
            );
        }
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(Integer::ZERO.twos_complement_limbs().get(u), 0);
    });
}
