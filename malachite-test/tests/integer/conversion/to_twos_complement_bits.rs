use common::{test_properties, test_properties_no_special};
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{SignificantBits, Zero};
use malachite_nz::integer::conversion::to_twos_complement_bits::*;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::{small_unsigneds, vecs_of_bool, vecs_of_bool_var_1};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_small_u64, pairs_of_integer_and_vec_of_bool_var_2,
};
use std::str::FromStr;

#[test]
pub fn test_bits_to_twos_complement_bits_non_negative() {
    let test = |bits: &[bool], out_bits: &[bool]| {
        let mut mut_bits = bits.to_vec();
        bits_to_twos_complement_bits_non_negative(&mut mut_bits);
        assert_eq!(mut_bits, out_bits);
    };
    test(&[], &[]);
    test(&[false, true, false], &[false, true, false]);
    test(&[true, false, true], &[true, false, true, false]);
}

#[test]
pub fn test_bits_slice_to_twos_complement_bits_negative() {
    let test = |bits: &[bool], out_bits: &[bool], carry: bool| {
        let mut mut_bits = bits.to_vec();
        assert_eq!(
            bits_slice_to_twos_complement_bits_negative(&mut mut_bits),
            carry
        );
        assert_eq!(mut_bits, out_bits);
    };
    test(&[], &[], true);
    test(&[true, false, true], &[true, true, false], false);
    test(&[false, false, false], &[false, false, false], true);
}

#[test]
pub fn test_bits_vec_to_twos_complement_bits_negative() {
    let test = |bits: &[bool], out_bits: &[bool]| {
        let mut mut_bits = bits.to_vec();
        bits_vec_to_twos_complement_bits_negative(&mut mut_bits);
        assert_eq!(mut_bits, out_bits);
    };
    test(&[true, false, false], &[true, true, true]);
    test(&[true, false, true], &[true, true, false, true]);
}

#[test]
#[should_panic]
fn bits_vec_to_twos_complement_bits_negative_fail() {
    let mut mut_bits = vec![false, false];
    bits_vec_to_twos_complement_bits_negative(&mut mut_bits);
}

#[test]
fn test_to_twos_complement_bits_asc() {
    let test = |n, out| {
        let n = Integer::from_str(n).unwrap();
        assert_eq!(n.twos_complement_bits().collect::<Vec<bool>>(), out);
        assert_eq!(n.to_twos_complement_bits_asc(), out);
    };
    test("0", vec![]);
    test("1", vec![true, false]);
    test("-1", vec![true]);
    test("6", vec![false, true, true, false]);
    test("-6", vec![false, true, false, true]);
    test(
        "105",
        vec![true, false, false, true, false, true, true, false],
    );
    test(
        "-105",
        vec![true, true, true, false, true, false, false, true],
    );
    test(
        "1000000000000",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true, false,
        ],
    );
    test(
        "-1000000000000",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, true, true, true, false, true, false, true, true, false, true, false, true, true,
            false, true, false, true, false, false, true, true, true, false, true, false, false,
            false, true,
        ],
    );
    test(
        "4294967295",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, false,
        ],
    );
    test(
        "-4294967295",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true,
        ],
    );
    test(
        "4294967296",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true, false,
        ],
    );
    test(
        "-4294967296",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true,
        ],
    );
    test(
        "18446744073709551615",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, false,
        ],
    );
    test(
        "-18446744073709551615",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true,
        ],
    );
    test(
        "18446744073709551616",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true, false,
        ],
    );
    test(
        "-18446744073709551616",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true,
        ],
    );
    test(
        "-10725406948920873257320529212268773241779870075",
        vec![
            true, false, true, false, false, false, false, true, false, true, true, false, false,
            true, false, true, false, false, true, false, false, true, false, false, true, true,
            true, true, false, false, false, false, true, true, true, true, false, false, true,
            false, true, false, true, true, false, true, false, false, true, false, false, false,
            true, false, true, true, true, true, false, false, true, true, false, false, false,
            false, true, true, true, false, false, true, true, true, true, true, false, false,
            true, true, true, true, true, false, false, false, true, false, false, true, true,
            false, false, false, false, true, true, false, false, true, true, false, true, false,
            true, true, true, true, false, true, true, false, true, false, true, true, true, false,
            false, true, true, false, true, true, false, false, true, true, true, false, true,
            true, true, false, true, false, false, true, true, true, false, false, false, false,
            true, true, true, true, true, false, false, false, false, true,
        ],
    );

    let n = Integer::from(-105);
    let mut bits = n.twos_complement_bits();
    assert_eq!(Some(true), bits.next());
    assert_eq!(Some(true), bits.next_back());
    assert_eq!(Some(false), bits.next_back());
    assert_eq!(Some(false), bits.next_back());
    assert_eq!(Some(true), bits.next());
    assert_eq!(Some(true), bits.next());
    assert_eq!(Some(false), bits.next());
    assert_eq!(Some(true), bits.next());
    assert_eq!(None, bits.next());
    assert_eq!(None, bits.next_back());

    assert_eq!(bits[0], true);
    assert_eq!(bits[1], true);
    assert_eq!(bits[2], true);
    assert_eq!(bits[3], false);
    assert_eq!(bits[4], true);
    assert_eq!(bits[5], false);
    assert_eq!(bits[6], false);
    assert_eq!(bits[7], true);
    assert_eq!(bits[8], true);

    let mut bits = n.twos_complement_bits();
    assert_eq!(Some(true), bits.next_back());
    assert_eq!(Some(true), bits.next());
    assert_eq!(Some(true), bits.next());
    assert_eq!(Some(true), bits.next());
    assert_eq!(Some(false), bits.next_back());
    assert_eq!(Some(false), bits.next_back());
    assert_eq!(Some(true), bits.next_back());
    assert_eq!(Some(false), bits.next_back());
    assert_eq!(None, bits.next());
    assert_eq!(None, bits.next_back());
}

#[test]
fn test_to_twos_complement_bits_desc() {
    let test = |n, out| {
        let n = Integer::from_str(n).unwrap();
        assert_eq!(n.twos_complement_bits().rev().collect::<Vec<bool>>(), out);
        assert_eq!(n.to_twos_complement_bits_desc(), out);
    };
    test("0", vec![]);
    test("1", vec![false, true]);
    test("-1", vec![true]);
    test("6", vec![false, true, true, false]);
    test("-6", vec![true, false, true, false]);
    test(
        "105",
        vec![false, true, true, false, true, false, false, true],
    );
    test(
        "-105",
        vec![true, false, false, true, false, true, true, true],
    );
    test(
        "1000000000000",
        vec![
            false, true, true, true, false, true, false, false, false, true, true, false, true,
            false, true, false, false, true, false, true, false, false, true, false, true, false,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ],
    );
    test(
        "-1000000000000",
        vec![
            true, false, false, false, true, false, true, true, true, false, false, true, false,
            true, false, true, true, false, true, false, true, true, false, true, false, true,
            true, true, true, false, false, false, false, false, false, false, false, false, false,
            false, false,
        ],
    );
    test(
        "4294967295",
        vec![
            false, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true,
        ],
    );
    test(
        "-4294967295",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true,
        ],
    );
    test(
        "4294967296",
        vec![
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false,
        ],
    );
    test(
        "-4294967296",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false,
        ],
    );
    test(
        "18446744073709551615",
        vec![
            false, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true,
        ],
    );
    test(
        "-18446744073709551615",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true,
        ],
    );
    test(
        "18446744073709551616",
        vec![
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
        ],
    );
    test(
        "-18446744073709551616",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false,
        ],
    );
}

#[test]
fn bits_to_twos_complement_bits_non_negative_properties() {
    test_properties(vecs_of_bool, |bits| {
        let mut mut_bits = bits.clone();
        bits_to_twos_complement_bits_non_negative(&mut mut_bits);
    });
}

#[test]
fn bits_slice_to_twos_complement_bits_negative_properties() {
    test_properties(vecs_of_bool, |bits| {
        let mut mut_bits = bits.clone();
        bits_slice_to_twos_complement_bits_negative(&mut mut_bits);
    });
}

#[test]
fn bits_vec_to_twos_complement_bits_negative_properties() {
    test_properties(vecs_of_bool_var_1, |bits| {
        let mut mut_bits = bits.clone();
        bits_vec_to_twos_complement_bits_negative(&mut mut_bits);
    });
}

#[test]
fn to_twos_complement_bits_asc_properties() {
    test_properties(integers, |x| {
        let bits = x.to_twos_complement_bits_asc();
        assert_eq!(
            x.twos_complement_bits().collect::<Vec<bool>>(),
            bits,
            "{}",
            x
        );
        assert_eq!(Integer::from_twos_complement_bits_asc(&bits), *x);
        if *x != 0 as Limb {
            assert_eq!(*bits.last().unwrap(), *x < 0 as Limb);
        }
    });
}

#[test]
fn to_twos_complement_bits_desc_properties() {
    test_properties(integers, |x| {
        let bits = x.to_twos_complement_bits_desc();
        assert_eq!(x.twos_complement_bits().rev().collect::<Vec<bool>>(), bits);
        assert_eq!(Integer::from_twos_complement_bits_desc(&bits), *x);
        if *x != 0 as Limb {
            assert_eq!(bits[0], *x < 0 as Limb);
        }
    });
}

#[test]
fn twos_complement_bits_properties() {
    test_properties(
        pairs_of_integer_and_vec_of_bool_var_2,
        |&(ref n, ref bs)| {
            let mut bits = n.twos_complement_bits();
            let mut bit_vec = Vec::new();
            let mut i = 0;
            for &b in bs {
                if b {
                    bit_vec.insert(i, bits.next().unwrap());
                    i += 1;
                } else {
                    bit_vec.insert(i, bits.next_back().unwrap())
                }
            }
            assert!(bits.next().is_none());
            assert!(bits.next_back().is_none());
            assert_eq!(n.to_twos_complement_bits_asc(), bit_vec);
        },
    );

    test_properties(pairs_of_integer_and_small_u64, |&(ref n, u)| {
        if u < n.significant_bits() {
            assert_eq!(
                n.twos_complement_bits()[u],
                n.to_twos_complement_bits_asc()[usize::checked_from(u).unwrap()]
            );
        } else {
            assert_eq!(n.twos_complement_bits()[u], *n < 0 as Limb);
        }
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(Integer::ZERO.twos_complement_bits()[u], false);
    });
}
