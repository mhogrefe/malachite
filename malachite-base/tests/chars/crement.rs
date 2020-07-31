use malachite_base::chars::{CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES};
use malachite_base::comparison::traits::Max;

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
