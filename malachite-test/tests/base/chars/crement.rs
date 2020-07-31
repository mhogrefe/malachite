use malachite_base::chars::{decrement_char, increment_char};

use malachite_test::common::test_properties_no_limit_exhaustive_no_special;
use malachite_test::inputs::base::{chars_not_max, chars_not_min};

#[test]
fn increment_char_properties() {
    test_properties_no_limit_exhaustive_no_special(chars_not_max, |&c| {
        let mut mut_c = c;
        increment_char(&mut mut_c);
        assert_ne!(mut_c, c);
        decrement_char(&mut mut_c);
        assert_eq!(mut_c, c);
    });
}

#[test]
fn decrement_char_properties() {
    test_properties_no_limit_exhaustive_no_special(chars_not_min, |&c| {
        let mut mut_c = c;
        decrement_char(&mut mut_c);
        assert_ne!(mut_c, c);
        increment_char(&mut mut_c);
        assert_eq!(mut_c, c);
    });
}
