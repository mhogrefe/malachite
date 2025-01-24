// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::constants::{
    CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES, NUMBER_OF_CHARS,
};
use malachite_base::chars::crement::{char_to_contiguous_range, contiguous_range_to_char};
use malachite_base::test_util::generators::{char_gen, char_pair_gen};

#[test]
fn test_char_to_contiguous_range() {
    let test = |c, out| {
        assert_eq!(char_to_contiguous_range(c), out);
    };
    test('\u{0}', 0);
    test('a', 97);
    test('A', 65);
    test(CHAR_JUST_BELOW_SURROGATES, 55295);
    test(CHAR_JUST_ABOVE_SURROGATES, 55296);
    test(char::MAX, 1112063);
}

#[test]
fn char_to_contiguous_range_properties() {
    char_gen().test_properties_no_exhaustive_limit(|c| {
        let u = char_to_contiguous_range(c);
        assert_eq!(contiguous_range_to_char(u), Some(c));
        assert!(u < NUMBER_OF_CHARS);
    });

    char_pair_gen().test_properties(|(c, d)| {
        assert_eq!(
            c.cmp(&d),
            char_to_contiguous_range(c).cmp(&char_to_contiguous_range(d))
        );
    });
}
