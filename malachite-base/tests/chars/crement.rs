use malachite_base::chars::constants::{CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES};
use malachite_base::comparison::traits::Max;
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
