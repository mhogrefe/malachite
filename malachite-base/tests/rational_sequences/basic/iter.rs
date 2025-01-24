// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::{count_is_at_least, prefix_to_string};
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::test_util::generators::{unsigned_rational_sequence_gen, unsigned_vec_gen};

#[test]
pub fn test_iter() {
    fn test(non_repeating: &[u8], repeating: &[u8], out: &str) {
        let xs = RationalSequence::from_slices(non_repeating, repeating);
        assert_eq!(prefix_to_string(xs.iter(), 10), out);
    }
    test(&[], &[], "[]");
    test(&[1, 2, 3], &[], "[1, 2, 3]");
    test(&[], &[1, 2, 3], "[1, 2, 3, 1, 2, 3, 1, 2, 3, 1, ...]");
    test(
        &[1, 2, 3],
        &[4, 5, 6],
        "[1, 2, 3, 4, 5, 6, 4, 5, 6, 4, ...]",
    );
}

#[test]
fn iter_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        let mut it = xs.iter();
        if let Some(len) = xs.len() {
            assert_eq!(it.count(), len);
        } else {
            assert!(count_is_at_least(&mut it, 20));
        }

        let non_rep_len = xs.slices_ref().0.len();
        let rep_len = xs.slices_ref().1.len();
        assert!(Iterator::eq(
            xs.iter().take(non_rep_len),
            xs.slices_ref().0.iter()
        ));
        assert!(Iterator::eq(
            xs.iter().skip(non_rep_len).take(rep_len),
            xs.slices_ref().1.iter()
        ));
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        assert!(Iterator::eq(
            xs.iter(),
            RationalSequence::from_slice(&xs).iter()
        ));
    });
}
