use common::LARGE_LIMIT;
use malachite_base::chars::{CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES};
use malachite_base::num::Walkable;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::chars_var_1;
use std::char;

#[test]
fn test_decrement() {
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
#[should_panic(expected = "Cannot decrement char '\\u{0}'")]
fn char_decrement_fail() {
    let mut c = '\u{0}';
    c.decrement();
}

#[test]
fn char_increment() {
    // if c != '\u{0}', c.decrement() changes c; c.decrement() followed by c.increment() leaves it
    // unchanged.
    let one_char = |mut c: char| {
        let c_old = c;
        c.decrement();
        assert_ne!(c, c_old);
        c.increment();
        assert_eq!(c, c_old);
    };

    for c in chars_var_1(GenerationMode::Exhaustive) {
        one_char(c);
    }

    for c in chars_var_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_char(c);
    }
}
