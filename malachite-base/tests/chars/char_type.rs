// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::chars::exhaustive::chars_increasing;
use malachite_base::chars::CharType;
use std::collections::HashSet;

#[test]
fn test_char_type() {
    let char_types = [
        CharType::AsciiLower,
        CharType::AsciiUpper,
        CharType::AsciiNumeric,
        CharType::AsciiNonAlphanumericGraphic,
        CharType::NonAsciiGraphic,
        CharType::NonGraphic,
    ];
    let mut remaining_chars: HashSet<_> = chars_increasing().collect();
    let mut n = remaining_chars.len();
    let mut frequencies = Vec::new();
    for &char_type in &char_types {
        remaining_chars.retain(|&c| !char_type.contains(c));
        let frequency = n - remaining_chars.len();
        n = remaining_chars.len();
        frequencies.push((char_type, frequency));
    }
    assert_eq!(n, 0);
    assert_eq!(
        frequencies,
        &[
            (CharType::AsciiLower, 26),
            (CharType::AsciiUpper, 26),
            (CharType::AsciiNumeric, 10),
            (CharType::AsciiNonAlphanumericGraphic, 33),
            (CharType::NonAsciiGraphic, 152619),
            (CharType::NonGraphic, 959350)
        ]
    );

    let first_chars = char_types.iter().map(|&char_type| {
        (
            char_type,
            chars_increasing().find(|&c| char_type.contains(c)).unwrap(),
        )
    });
    assert_eq!(
        first_chars.collect_vec(),
        &[
            (CharType::AsciiLower, 'a'),
            (CharType::AsciiUpper, 'A'),
            (CharType::AsciiNumeric, '0'),
            (CharType::AsciiNonAlphanumericGraphic, ' '),
            (CharType::NonAsciiGraphic, '¡'),
            (CharType::NonGraphic, '\u{0}')
        ]
    );

    let last_chars = char_types.iter().map(|&char_type| {
        (
            char_type,
            chars_increasing()
                .rev()
                .find(|&c| char_type.contains(c))
                .unwrap(),
        )
    });
    assert_eq!(
        last_chars.collect_vec(),
        &[
            (CharType::AsciiLower, 'z'),
            (CharType::AsciiUpper, 'Z'),
            (CharType::AsciiNumeric, '9'),
            (CharType::AsciiNonAlphanumericGraphic, '~'),
            (CharType::NonAsciiGraphic, '𲎯'),
            (CharType::NonGraphic, '\u{10ffff}')
        ]
    );
}
