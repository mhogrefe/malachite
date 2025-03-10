// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rational_sequences::RationalSequence;
use malachite_base::test_util::generators::{unsigned_rational_sequence_gen, unsigned_vec_gen};

#[test]
pub fn test_len() {
    fn test(non_repeating: &[u8], repeating: &[u8], out: Option<usize>) {
        let xs = RationalSequence::from_slices(non_repeating, repeating);
        assert_eq!(xs.len(), out);
    }
    test(&[], &[], Some(0));
    test(&[1, 2, 3], &[], Some(3));
    test(&[], &[1, 2, 3], None);
    test(&[1, 2, 3], &[4, 5, 6], None);
}

#[test]
fn len_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        assert_eq!(xs.len().is_some(), xs.is_finite());
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        assert_eq!(RationalSequence::from_slice(&xs).len(), Some(xs.len()));
    });
}
