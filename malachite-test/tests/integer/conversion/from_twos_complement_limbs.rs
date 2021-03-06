use std::cmp::Ordering;

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::vecs::vec_delete_left;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

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
        let mut trimmed_limbs = limbs.iter().cloned().rev().collect_vec();
        trim_be_limbs(&mut trimmed_limbs);
        trimmed_limbs.reverse();
        assert_eq!(x.to_twos_complement_limbs_asc(), trimmed_limbs);
        assert_eq!(
            Integer::from_owned_twos_complement_limbs_desc(limbs.iter().cloned().rev().collect()),
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
            Integer::from_owned_twos_complement_limbs_asc(limbs.iter().cloned().rev().collect()),
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
