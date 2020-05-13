use std::{char, u32};

use malachite_base::chars::{
    char_to_contiguous_range, contiguous_range_to_char, CHAR_JUST_ABOVE_SURROGATES,
    CHAR_JUST_BELOW_SURROGATES,
};
use malachite_base::comparison::traits::Min;
use malachite_base::crement::Crementable;

#[test]
fn test_char_increment() {
    let test = |mut c: char, out| {
        c.increment();
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
fn char_increment_fail() {
    let mut c = char::MAX;
    c.increment();
}

#[test]
fn test_char_decrement() {
    let test = |mut c: char, out| {
        c.decrement();
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
fn char_decrement_fail() {
    let mut c = '\u{0}';
    c.decrement();
}

#[test]
fn test_char_to_contiguous_range() {
    let test = |c, out| {
        assert_eq!(char_to_contiguous_range(c), out);
    };
    test('\u{0}', 0);
    test('a', 97);
    test('A', 65);
    test(CHAR_JUST_BELOW_SURROGATES, 55_295);
    test(CHAR_JUST_ABOVE_SURROGATES, 55_296);
    test(char::MAX, 1_112_063);
}

#[test]
fn test_contiguous_range_to_char() {
    let test = |u, out| {
        assert_eq!(contiguous_range_to_char(u), out);
    };
    test(0, Some('\u{0}'));
    test(97, Some('a'));
    test(65, Some('A'));
    test(55_295, Some(CHAR_JUST_BELOW_SURROGATES));
    test(55_296, Some(CHAR_JUST_ABOVE_SURROGATES));
    test(1_112_063, Some(char::MAX));
    test(1_112_064, None);
    test(u32::MAX, None);
}

#[test]
fn test_min() {
    assert_eq!(char::MIN, '\u{0}');
}

#[test]
fn test_max() {
    assert_eq!(char::MAX, '\u{10ffff}');
}
