use itertools::Itertools;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable};
use malachite_base_test_util::num::logic::bit_convertible::{to_bits_asc_alt, to_bits_desc_alt};
use malachite_nz::integer::logic::bit_convertible::{
    bits_slice_to_twos_complement_bits_negative, bits_to_twos_complement_bits_non_negative,
    bits_vec_to_twos_complement_bits_negative,
};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::integer::logic::to_bits::{to_bits_asc_naive, to_bits_desc_naive};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, vecs_of_bool, vecs_of_bool_var_1};
use malachite_test::inputs::integer::integers;

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
fn to_bits_asc_properties() {
    test_properties(integers, |x| {
        let bits = x.to_bits_asc();
        assert_eq!(to_bits_asc_naive(x), bits);
        assert_eq!(to_bits_asc_alt(x), bits);
        assert_eq!(x.bits().collect_vec(), bits);
        assert_eq!(Integer::from_bits_asc(bits.iter().cloned()), *x);
        if *x != 0 {
            assert_eq!(*bits.last().unwrap(), *x < 0);
        }
        let bit_len = bits.len();
        if bit_len > 1 {
            assert_ne!(bits[bit_len - 1], bits[bit_len - 2]);
        }
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(i.to_bits_asc(), Integer::from(i).to_bits_asc());
    });
}

#[test]
fn to_bits_desc_properties() {
    test_properties(integers, |x| {
        let bits = x.to_bits_desc();
        assert_eq!(to_bits_desc_naive(x), bits);
        assert_eq!(to_bits_desc_alt(x), bits);
        assert_eq!(x.bits().rev().collect_vec(), bits);
        assert_eq!(Integer::from_bits_desc(bits.iter().cloned()), *x);
        if *x != 0 {
            assert_eq!(bits[0], *x < 0);
        }
        if bits.len() > 1 {
            assert_ne!(bits[0], bits[1]);
        }
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(i.to_bits_desc(), Integer::from(i).to_bits_desc());
    });
}
