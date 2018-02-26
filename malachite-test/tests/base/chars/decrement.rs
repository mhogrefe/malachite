use common::test_properties_no_limit_exhaustive_no_special;
use malachite_base::chars::{CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES};
use malachite_base::misc::Walkable;
use malachite_test::inputs::base::chars_var_1;
use std::char;

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
#[should_panic(expected = "Cannot decrement char '\\u{0}'")]
fn char_decrement_fail() {
    let mut c = '\u{0}';
    c.decrement();
}

#[test]
fn char_increment_properties() {
    test_properties_no_limit_exhaustive_no_special(chars_var_1, |&c| {
        let mut c_mut = c;
        c_mut.decrement();
        assert_ne!(c_mut, c);
        c_mut.increment();
        assert_eq!(c_mut, c);
    });
}
