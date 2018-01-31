use common::LARGE_LIMIT;
use malachite_base::chars::{CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES};
use malachite_base::misc::Walkable;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::chars_var_2;
use std::char;

#[test]
fn test_increment() {
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
#[should_panic(expected = "Cannot increment char::MAX")]
fn char_increment_fail() {
    let mut c = char::MAX;
    c.increment();
}

#[test]
fn char_increment() {
    // if c != char::MAX, c.increment() changes c; c.increment() followed by c.decrement() leaves it
    // unchanged.
    let one_char = |mut c: char| {
        let c_old = c;
        c.increment();
        assert_ne!(c, c_old);
        c.decrement();
        assert_eq!(c, c_old);
    };

    for c in chars_var_2(GenerationMode::Exhaustive) {
        one_char(c);
    }

    for c in chars_var_2(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_char(c);
    }
}
