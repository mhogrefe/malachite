use common::test_properties_no_limit_exhaustive_no_special;
use malachite_base::chars::{CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES};
use malachite_base::crement::Crementable;
use malachite_test::inputs::base::chars_not_min;
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
#[should_panic]
fn char_decrement_fail() {
    let mut c = '\u{0}';
    c.decrement();
}

#[test]
fn char_increment_properties() {
    test_properties_no_limit_exhaustive_no_special(chars_not_min, |&c| {
        let mut mut_c = c;
        mut_c.decrement();
        assert_ne!(mut_c, c);
        mut_c.increment();
        assert_eq!(mut_c, c);
    });
}
