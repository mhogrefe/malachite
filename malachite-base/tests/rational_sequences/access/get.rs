// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rational_sequences::RationalSequence;
use malachite_base::test_util::generators::{
    unsigned_rational_sequence_unsigned_pair_gen_var_1,
    unsigned_rational_sequence_unsigned_pair_gen_var_2,
};

#[test]
pub fn test_get() {
    fn test(non_repeating: &[u8], repeating: &[u8], index: usize, out: Option<&u8>) {
        let xs = RationalSequence::from_slices(non_repeating, repeating);
        assert_eq!(xs.get(index), out);
    }
    test(&[], &[], 0, None);
    test(&[1, 2, 3], &[], 0, Some(&1));
    test(&[1, 2, 3], &[], 1, Some(&2));
    test(&[1, 2, 3], &[], 2, Some(&3));
    test(&[1, 2, 3], &[], 3, None);
    test(&[1, 2, 3], &[4], 3, Some(&4));
    test(&[1, 2, 3], &[4], 100, Some(&4));
}

#[test]
pub fn test_index() {
    fn test(non_repeating: &[u8], repeating: &[u8], index: usize, out: u8) {
        let xs = RationalSequence::from_slices(non_repeating, repeating);
        assert_eq!(xs[index], out);
    }
    test(&[1, 2, 3], &[], 0, 1);
    test(&[1, 2, 3], &[], 1, 2);
    test(&[1, 2, 3], &[], 2, 3);
    test(&[1, 2, 3], &[4], 3, 4);
    test(&[1, 2, 3], &[4], 100, 4);
}

#[allow(clippy::unnecessary_operation)]
#[test]
#[should_panic]
fn index_fail_1() {
    RationalSequence::<u8>::from_vec(vec![])[0];
}

#[allow(clippy::unnecessary_operation)]
#[test]
#[should_panic]
fn index_fail_2() {
    RationalSequence::from_vec(vec![1, 2, 3])[3];
}

#[test]
fn get_properties() {
    unsigned_rational_sequence_unsigned_pair_gen_var_1::<u8, usize>().test_properties(
        |(xs, index)| {
            assert_eq!(xs.get(index), xs.iter().nth(index));
        },
    );
}

#[test]
fn index_properties() {
    unsigned_rational_sequence_unsigned_pair_gen_var_2::<u8>().test_properties(|(xs, index)| {
        let x = xs[index];
        assert_eq!(xs.iter().nth(index).unwrap(), &x);
        assert_eq!(xs.get(index), Some(&x));
    });
}
