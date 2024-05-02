// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rational_sequences::RationalSequence;
use malachite_base::strings::string_is_subset;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::{unsigned_rational_sequence_gen, unsigned_vec_gen};

#[test]
pub fn test_to_string() {
    fn test(non_repeating: &[u8], repeating: &[u8], out: &str) {
        let xs = RationalSequence::from_slices(non_repeating, repeating);
        assert_eq!(xs.to_string(), out);
        assert_eq!(xs.to_debug_string(), out);
    }
    test(&[], &[], "[]");
    test(&[1, 2, 3], &[], "[1, 2, 3]");
    test(&[], &[1, 2, 3], "[[1, 2, 3]]");
    test(&[1, 2, 3], &[4, 5, 6], "[1, 2, 3, [4, 5, 6]]");
}

#[test]
fn to_string_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        let s = xs.to_string();
        assert_eq!(xs.to_debug_string(), s);
        assert!(string_is_subset(&s, " ,0123456789[]"));
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        assert_eq!(
            RationalSequence::from_slice(&xs).to_string(),
            xs.to_debug_string()
        );
    });
}
