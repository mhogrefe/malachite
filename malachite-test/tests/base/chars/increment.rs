use common::test_properties_no_limit_exhaustive_no_special;
use malachite_base::chars::{CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES};
use malachite_base::misc::Walkable;
use malachite_test::inputs::base::chars_not_max;
use std::char;

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
#[should_panic(expected = "Cannot increment char::MAX")]
fn char_increment_fail() {
    let mut c = char::MAX;
    c.increment();
}

#[test]
fn char_increment_properties() {
    test_properties_no_limit_exhaustive_no_special(chars_not_max, |&c| {
        let mut c_mut = c;
        c_mut.increment();
        assert_ne!(c_mut, c);
        c_mut.decrement();
        assert_eq!(c_mut, c);
    });
}
