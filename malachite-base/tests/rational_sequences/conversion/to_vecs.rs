// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rational_sequences::RationalSequence;
use malachite_base::test_util::generators::unsigned_rational_sequence_gen;

#[test]
pub fn test_to_vecs_into_vecs_and_slices_ref() {
    fn test(non_repeating: &[u8], repeating: &[u8], o_xs: &[u8], o_ys: &[u8]) {
        let xs = RationalSequence::from_slices(non_repeating, repeating);
        let (out_xs, out_ys) = xs.to_vecs();
        assert_eq!(out_xs, o_xs);
        assert_eq!(out_ys, o_ys);
        let (out_xs, out_ys) = xs.clone().into_vecs();
        assert_eq!(out_xs, o_xs);
        assert_eq!(out_ys, o_ys);
        let (out_xs, out_ys) = xs.slices_ref();
        assert_eq!(out_xs, o_xs);
        assert_eq!(out_ys, o_ys);
    }
    test(&[], &[], &[], &[]);
    test(&[1, 2, 3], &[], &[1, 2, 3], &[]);
    test(&[], &[1, 2, 3], &[], &[1, 2, 3]);
    test(&[1, 2, 3], &[4, 5, 6], &[1, 2, 3], &[4, 5, 6]);
}

#[test]
fn to_vecs_into_vecs_and_slices_ref_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|rxs| {
        let (xs, ys) = rxs.to_vecs();
        let (xs_alt, ys_alt) = rxs.clone().into_vecs();
        assert_eq!(xs_alt, xs);
        assert_eq!(ys_alt, ys);
        let (xs_alt, ys_alt) = rxs.slices_ref();
        assert_eq!(xs_alt, xs);
        assert_eq!(ys_alt, ys);
        assert_eq!(RationalSequence::from_vecs(xs, ys), rxs);
    });
}
