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
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_2, unsigned_pair_gen_var_1,
};

#[test]
fn test_contiguous_range_to_char() {
    let test = |u, out| {
        assert_eq!(contiguous_range_to_char(u), out);
    };
    test(0, Some('\u{0}'));
    test(97, Some('a'));
    test(65, Some('A'));
    test(55295, Some(CHAR_JUST_BELOW_SURROGATES));
    test(55296, Some(CHAR_JUST_ABOVE_SURROGATES));
    test(NUMBER_OF_CHARS - 1, Some(char::MAX));
    test(NUMBER_OF_CHARS, None);
    test(u32::MAX, None);
}

#[test]
fn contiguous_range_to_char_properties() {
    unsigned_gen().test_properties(|u| {
        assert_eq!(contiguous_range_to_char(u).is_some(), u < NUMBER_OF_CHARS);
    });

    unsigned_gen_var_2().test_properties(|u| {
        assert_eq!(
            char_to_contiguous_range(contiguous_range_to_char(u).unwrap()),
            u
        );
    });

    unsigned_pair_gen_var_1().test_properties(|(u, v)| {
        assert_eq!(
            u.cmp(&v),
            contiguous_range_to_char(u).cmp(&contiguous_range_to_char(v))
        );
    });
}
