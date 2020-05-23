use malachite_base::crement::Crementable;

use malachite_test::common::test_properties_no_limit_exhaustive_no_special;
use malachite_test::inputs::base::{chars_not_max, chars_not_min};

#[test]
fn char_increment_properties() {
    test_properties_no_limit_exhaustive_no_special(chars_not_max, |&c| {
        let mut mut_c = c;
        mut_c.increment();
        assert_ne!(mut_c, c);
        mut_c.decrement();
        assert_eq!(mut_c, c);
    });
}

#[test]
fn char_decrement_properties() {
    test_properties_no_limit_exhaustive_no_special(chars_not_min, |&c| {
        let mut mut_c = c;
        mut_c.decrement();
        assert_ne!(mut_c, c);
        mut_c.increment();
        assert_eq!(mut_c, c);
    });
}
