// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rational_sequences::RationalSequence;
use malachite_base::test_util::generators::{unsigned_rational_sequence_gen, unsigned_vec_gen};

#[test]
pub fn test_is_finite() {
    fn test(non_repeating: &[u8], repeating: &[u8], out: bool) {
        let xs = RationalSequence::from_slices(non_repeating, repeating);
        assert_eq!(xs.is_finite(), out);
    }
    test(&[], &[], true);
    test(&[1, 2, 3], &[], true);
    test(&[], &[1, 2, 3], false);
    test(&[1, 2, 3], &[4, 5, 6], false);
}

#[test]
fn is_finite_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        assert_eq!(xs.is_finite(), xs.slices_ref().1.is_empty());
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        assert!(RationalSequence::from_slice(&xs).is_finite());
    });
}
