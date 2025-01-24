// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rational_sequences::RationalSequence;
use malachite_base::test_util::generators::{
    unsigned_rational_sequence_gen, unsigned_rational_sequence_pair_gen,
};

#[test]
#[allow(clippy::redundant_clone)]
fn test_clone() {
    let test = |nr: &[u8], r: &[u8]| {
        let xs = RationalSequence::from_slices(nr, r);
        let xs_clone = xs.clone();
        assert!(xs.is_valid());
        assert_eq!(xs_clone, xs);
    };
    test(&[], &[]);
    test(&[1, 2, 3], &[]);
    test(&[], &[1, 2, 3]);
    test(&[1, 2, 3], &[4, 5, 5]);
}

#[test]
fn test_clone_from() {
    let test = |nr_1: &[u8], r_1: &[u8], nr_2: &[u8], r_2: &[u8]| {
        let mut x = RationalSequence::from_slices(nr_1, r_1);
        let y = RationalSequence::from_slices(nr_2, r_2);
        x.clone_from(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
    };
    test(&[], &[], &[1, 2, 3], &[4, 5, 6]);
    test(&[], &[1, 2, 3], &[1, 2, 3], &[]);
}

#[allow(clippy::redundant_clone)]
#[test]
fn clone_and_clone_from_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        let mut_xs = xs.clone();
        assert!(mut_xs.is_valid());
        assert_eq!(mut_xs, xs);
    });

    unsigned_rational_sequence_pair_gen::<u8>().test_properties(|(xs, ys)| {
        let mut mut_xs = xs.clone();
        mut_xs.clone_from(&ys);
        assert!(mut_xs.is_valid());
        assert_eq!(mut_xs, ys);
    });
}
