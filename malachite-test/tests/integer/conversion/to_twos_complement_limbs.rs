use common::{test_properties, test_properties_no_special};
use malachite_base::misc::Max;
use malachite_base::num::{PrimitiveInteger, Sign, Zero};
use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::{small_usizes, vecs_of_unsigned, vecs_of_unsigned_var_3};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_small_usize, pairs_of_integer_and_vec_of_bool_var_1,
};
use malachite_test::integer::conversion::to_twos_complement_limbs::*;
use std::cmp::Ordering;
#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
pub fn test_limbs_maybe_sign_extend_non_negative_in_place() {
    let test = |limbs: &[Limb], out_limbs: &[Limb]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_maybe_sign_extend_non_negative_in_place(&mut mut_limbs);
        assert_eq!(mut_limbs, out_limbs);
    };
    test(&[], &[]);
    test(&[1, 2, 3], &[1, 2, 3]);
    test(&[1, 2, 0xffff_ffff], &[1, 2, 0xffff_ffff, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
pub fn test_limbs_twos_complement_in_place() {
    let test = |limbs: &[Limb], out_limbs: &[Limb], carry: bool| {
        let mut mut_limbs = limbs.to_vec();
        assert_eq!(limbs_twos_complement_in_place(&mut mut_limbs), carry);
        assert_eq!(mut_limbs, out_limbs);

        let mut mut_limbs = limbs.to_vec();
        assert_eq!(limbs_twos_complement_in_place_alt_1(&mut mut_limbs), carry);
        assert_eq!(mut_limbs, out_limbs);

        let mut mut_limbs = limbs.to_vec();
        assert_eq!(limbs_twos_complement_in_place_alt_2(&mut mut_limbs), carry);
        assert_eq!(mut_limbs, out_limbs);
    };
    test(&[], &[], true);
    test(&[1, 2, 3], &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc], false);
    test(&[0, 0, 0], &[0, 0, 0], true);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
pub fn test_limbs_twos_complement_and_maybe_sign_extend_negative_in_place() {
    let test = |limbs: &[Limb], out_limbs: &[Limb]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut mut_limbs);
        assert_eq!(mut_limbs, out_limbs);
    };
    test(&[1, 2, 3], &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]);
    test(&[0, 0xffff_ffff], &[0, 1, 0xffff_ffff]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_twos_complement_and_maybe_sign_extend_negative_in_place_fail() {
    let mut mut_limbs = vec![0, 0, 0];
    limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut mut_limbs);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_twos_complement_limbs_asc() {
    let test = |n, out| {
        let n = Integer::from_str(n).unwrap();
        assert_eq!(n.twos_complement_limbs().collect::<Vec<Limb>>(), out);
        assert_eq!(n.to_twos_complement_limbs_asc(), out);
        assert_eq!(n.into_twos_complement_limbs_asc(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("-123", vec![4_294_967_173]);
    test("1000000000000", vec![3_567_587_328, 232]);
    test("-1000000000000", vec![727_379_968, 4_294_967_063]);
    test(
        "1701411834921604967429270619762735448065",
        vec![1, 2, 3, 4, 5],
    );
    test(
        "-1701411834921604967429270619762735448065",
        vec![
            Limb::MAX,
            Limb::MAX - 2,
            Limb::MAX - 3,
            Limb::MAX - 4,
            Limb::MAX - 5,
        ],
    );
    test("4294967295", vec![Limb::MAX, 0]);
    test("-4294967295", vec![1, Limb::MAX]);
    test("4294967296", vec![0, 1]);
    test("-4294967296", vec![0, Limb::MAX]);
    test("18446744073709551615", vec![Limb::MAX, Limb::MAX, 0]);
    test("-18446744073709551615", vec![1, 0, Limb::MAX]);
    test("18446744073709551616", vec![0, 0, 1]);
    test("-18446744073709551616", vec![0, 0, Limb::MAX]);

    let n = Integer::from_str("-1701411834921604967429270619762735448065").unwrap();
    let mut limbs = n.twos_complement_limbs();
    assert_eq!(Some(Limb::MAX), limbs.next());
    assert_eq!(Some(Limb::MAX - 5), limbs.next_back());
    assert_eq!(Some(Limb::MAX - 4), limbs.next_back());
    assert_eq!(Some(Limb::MAX - 2), limbs.next());
    assert_eq!(Some(Limb::MAX - 3), limbs.next());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());

    assert_eq!(limbs.get(0), Limb::MAX);
    assert_eq!(limbs.get(1), Limb::MAX - 2);
    assert_eq!(limbs.get(2), Limb::MAX - 3);
    assert_eq!(limbs.get(3), Limb::MAX - 4);
    assert_eq!(limbs.get(4), Limb::MAX - 5);
    assert_eq!(limbs.get(5), Limb::MAX);

    let mut limbs = n.twos_complement_limbs();
    assert_eq!(Some(Limb::MAX), limbs.next());
    assert_eq!(Some(Limb::MAX - 2), limbs.next());
    assert_eq!(Some(Limb::MAX - 3), limbs.next());
    assert_eq!(Some(Limb::MAX - 5), limbs.next_back());
    assert_eq!(Some(Limb::MAX - 4), limbs.next_back());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_twos_complement_limbs_desc() {
    let test = |n, out| {
        assert_eq!(
            Integer::from_str(n)
                .unwrap()
                .to_twos_complement_limbs_desc(),
            out
        );
        assert_eq!(
            Integer::from_str(n)
                .unwrap()
                .into_twos_complement_limbs_desc(),
            out
        );
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("-123", vec![4_294_967_173]);
    test("1000000000000", vec![232, 3_567_587_328]);
    test("-1000000000000", vec![4_294_967_063, 727_379_968]);
    test(
        "1701411834921604967429270619762735448065",
        vec![5, 4, 3, 2, 1],
    );
    test(
        "-1701411834921604967429270619762735448065",
        vec![
            Limb::MAX - 5,
            Limb::MAX - 4,
            Limb::MAX - 3,
            Limb::MAX - 2,
            Limb::MAX,
        ],
    );
    test("4294967295", vec![0, Limb::MAX]);
    test("-4294967295", vec![Limb::MAX, 1]);
    test("4294967296", vec![1, 0]);
    test("-4294967296", vec![Limb::MAX, 0]);
    test("18446744073709551615", vec![0, Limb::MAX, Limb::MAX]);
    test("-18446744073709551615", vec![Limb::MAX, 0, 1]);
    test("18446744073709551616", vec![1, 0, 0]);
    test("-18446744073709551616", vec![Limb::MAX, 0, 0]);
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

    test_properties(pairs_of_integer_and_small_usize, |&(ref n, u)| {
        if u < n.unsigned_abs_ref().limb_count() as usize {
            assert_eq!(
                n.twos_complement_limbs().get(u),
                n.to_twos_complement_limbs_asc()[u]
            );
        } else {
            assert_eq!(
                n.twos_complement_limbs().get(u),
                if *n >= 0 as Limb { 0 } else { Limb::MAX }
            );
        }
    });

    test_properties_no_special(small_usizes, |&u| {
        assert_eq!(Integer::ZERO.twos_complement_limbs().get(u), 0);
    });
}
