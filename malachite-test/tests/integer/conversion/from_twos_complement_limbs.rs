use std::cmp::Ordering;

use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::vecs::vec_delete_left;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_twos_complement_limbs_asc() {
    let test = |limbs: &[Limb], out| {
        let x = Integer::from_twos_complement_limbs_asc(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        let x = Integer::from_owned_twos_complement_limbs_asc(limbs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[123, 0], "123");
    test(&[4_294_967_173], "-123");
    test(&[4_294_967_173, Limb::MAX], "-123");
    test(&[3_567_587_328, 232], "1000000000000");
    test(&[727_379_968, 4_294_967_063], "-1000000000000");
    test(&[1, 2, 3, 4, 5], "1701411834921604967429270619762735448065");
    test(
        &[
            Limb::MAX,
            Limb::MAX - 2,
            Limb::MAX - 3,
            Limb::MAX - 4,
            Limb::MAX - 5,
        ],
        "-1701411834921604967429270619762735448065",
    );
    test(&[Limb::MAX, 0], "4294967295");
    test(&[1, Limb::MAX], "-4294967295");
    test(&[0, 1], "4294967296");
    test(&[0, Limb::MAX], "-4294967296");
    test(&[Limb::MAX, Limb::MAX, 0], "18446744073709551615");
    test(&[1, 0, Limb::MAX], "-18446744073709551615");
    test(&[0, 0, 1], "18446744073709551616");
    test(&[0, 0, Limb::MAX], "-18446744073709551616");
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_twos_complement_limbs_desc() {
    let test = |limbs: &[Limb], out| {
        let x = Integer::from_twos_complement_limbs_desc(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        let x = Integer::from_owned_twos_complement_limbs_desc(limbs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[0, 123], "123");
    test(&[4_294_967_173], "-123");
    test(&[Limb::MAX, 4_294_967_173], "-123");
    test(&[232, 3_567_587_328], "1000000000000");
    test(&[4_294_967_063, 727_379_968], "-1000000000000");
    test(&[5, 4, 3, 2, 1], "1701411834921604967429270619762735448065");
    test(
        &[
            Limb::MAX - 5,
            Limb::MAX - 4,
            Limb::MAX - 3,
            Limb::MAX - 2,
            Limb::MAX,
        ],
        "-1701411834921604967429270619762735448065",
    );
    test(&[0, Limb::MAX], "4294967295");
    test(&[Limb::MAX, 1], "-4294967295");
    test(&[1, 0], "4294967296");
    test(&[Limb::MAX, 0], "-4294967296");
    test(&[0, Limb::MAX, Limb::MAX], "18446744073709551615");
    test(&[Limb::MAX, 0, 1], "-18446744073709551615");
    test(&[1, 0, 0], "18446744073709551616");
    test(&[Limb::MAX, 0, 0], "-18446744073709551616");
}

fn trim_be_limbs(xs: &mut Vec<Limb>) {
    if xs.is_empty() {
        return;
    }
    if xs[0].get_highest_bit() {
        match xs.iter().position(|&limb| limb != Limb::MAX) {
            None => *xs = vec![Limb::MAX],
            Some(i) => {
                let i = if !xs[i].get_highest_bit() { i - 1 } else { i };
                vec_delete_left(xs, i);
            }
        }
    } else {
        match xs.iter().position(|&limb| limb != 0) {
            None => xs.clear(),
            Some(i) => {
                let i = if xs[i].get_highest_bit() { i - 1 } else { i };
                vec_delete_left(xs, i);
            }
        }
    }
}

#[test]
fn from_twos_complement_limbs_asc_properties() {
    test_properties(vecs_of_unsigned, |limbs: &Vec<Limb>| {
        let x = Integer::from_twos_complement_limbs_asc(limbs);
        assert_eq!(
            Integer::from_owned_twos_complement_limbs_asc(limbs.clone()),
            x
        );
        let mut trimmed_limbs: Vec<Limb> = limbs.iter().cloned().rev().collect();
        trim_be_limbs(&mut trimmed_limbs);
        trimmed_limbs.reverse();
        assert_eq!(x.to_twos_complement_limbs_asc(), trimmed_limbs);
        assert_eq!(
            Integer::from_twos_complement_limbs_desc(
                &limbs.iter().cloned().rev().collect::<Vec<Limb>>()
            ),
            x
        );
        if match x.sign() {
            Ordering::Equal => limbs.is_empty(),
            Ordering::Greater => {
                let last = *limbs.last().unwrap();
                !last.get_highest_bit() && (last != 0 || limbs[limbs.len() - 2].get_highest_bit())
            }
            Ordering::Less => {
                let last = *limbs.last().unwrap();
                last.get_highest_bit()
                    && (last != Limb::MAX
                        || limbs.len() <= 1
                        || !limbs[limbs.len() - 2].get_highest_bit())
            }
        } {
            assert_eq!(x.to_twos_complement_limbs_asc(), *limbs);
        }
    });
}

#[test]
fn from_twos_complement_limbs_desc_properties() {
    test_properties(vecs_of_unsigned, |limbs: &Vec<Limb>| {
        let x = Integer::from_twos_complement_limbs_desc(limbs);
        assert_eq!(
            Integer::from_owned_twos_complement_limbs_desc(limbs.clone()),
            x
        );
        let mut trimmed_limbs: Vec<Limb> = limbs.to_vec();
        trim_be_limbs(&mut trimmed_limbs);
        assert_eq!(x.to_twos_complement_limbs_desc(), trimmed_limbs);
        assert_eq!(
            Integer::from_twos_complement_limbs_asc(
                &limbs.iter().cloned().rev().collect::<Vec<Limb>>()
            ),
            x
        );
        if match x.sign() {
            Ordering::Equal => limbs.is_empty(),
            Ordering::Greater => {
                let first = limbs[0];
                !first.get_highest_bit() && (first != 0 || limbs[1].get_highest_bit())
            }
            Ordering::Less => {
                let first = limbs[0];
                first.get_highest_bit()
                    && (first != Limb::MAX || limbs.len() <= 1 || !limbs[1].get_highest_bit())
            }
        } {
            assert_eq!(x.to_twos_complement_limbs_desc(), *limbs);
        }
    });
}
