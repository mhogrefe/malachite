// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::constants::{CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES};
use malachite_base::chars::crement::{decrement_char, increment_char};
use malachite_base::test_util::generators::{char_gen_var_1, char_gen_var_2};

#[test]
fn test_increment_char() {
    let test = |mut c: char, out| {
        increment_char(&mut c);
        assert_eq!(c, out);
    };
    test('\u{0}', '\u{1}');
    test('a', 'b');
    test('A', 'B');
    test(CHAR_JUST_BELOW_SURROGATES, CHAR_JUST_ABOVE_SURROGATES);
    test('\u{10fffe}', char::MAX);
}

#[test]
#[should_panic]
fn increment_char_fail() {
    let mut c = char::MAX;
    increment_char(&mut c);
}

#[test]
fn test_decrement_char() {
    let test = |mut c: char, out| {
        decrement_char(&mut c);
        assert_eq!(c, out);
    };
    test('\u{1}', '\u{0}');
    test('b', 'a');
    test('B', 'A');
    test(CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES);
    test(char::MAX, '\u{10fffe}');
}

#[test]
#[should_panic]
fn decrement_char_fail() {
    let mut c = '\u{0}';
    decrement_char(&mut c);
}

#[test]
fn increment_char_properties() {
    char_gen_var_1().test_properties_no_exhaustive_limit(|c| {
        let mut mut_c = c;
        increment_char(&mut mut_c);
        assert_ne!(mut_c, c);
        decrement_char(&mut mut_c);
        assert_eq!(mut_c, c);
    });
}

#[test]
fn decrement_char_properties() {
    char_gen_var_2().test_properties_no_exhaustive_limit(|c| {
        let mut mut_c = c;
        decrement_char(&mut mut_c);
        assert_ne!(mut_c, c);
        increment_char(&mut mut_c);
        assert_eq!(mut_c, c);
    });
}
