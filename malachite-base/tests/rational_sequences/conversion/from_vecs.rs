// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rational_sequences::{rational_sequence_is_reduced, RationalSequence};
use malachite_base::test_util::generators::unsigned_vec_pair_gen;

#[test]
pub fn test_from_vecs_and_from_slices() {
    fn test(non_repeating: &[u8], repeating: &[u8], out: &str) {
        assert_eq!(
            RationalSequence::from_slices(non_repeating, repeating).to_string(),
            out
        );
        assert_eq!(
            RationalSequence::from_vecs(non_repeating.to_vec(), repeating.to_vec()).to_string(),
            out
        );
    }
    test(&[], &[], "[]");
    test(&[1, 2, 3], &[], "[1, 2, 3]");
    test(&[], &[1, 2, 3], "[[1, 2, 3]]");
    test(&[1, 2, 3], &[4, 5, 6], "[1, 2, 3, [4, 5, 6]]");
    test(&[1, 2], &[3, 4, 3, 4], "[1, 2, [3, 4]]");
    test(&[1, 2, 3], &[4, 3, 4, 3], "[1, 2, [3, 4]]");
    test(&[1, 2, 3, 4], &[3, 4, 3, 4], "[1, 2, [3, 4]]");
}

#[test]
fn from_vec_and_from_slice_properties() {
    unsigned_vec_pair_gen::<u8>().test_properties(|(xs, ys)| {
        let rs = RationalSequence::from_slices(&xs, &ys);
        assert!(rs.is_valid());
        assert_eq!(RationalSequence::from_vecs(xs.clone(), ys.clone()), rs);
        if rational_sequence_is_reduced(&xs, &ys) {
            assert_eq!(rs.into_vecs(), (xs, ys));
        }
    });
}
