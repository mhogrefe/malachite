use malachite_base::crement::Crementable;

use common::test_properties_no_limit_exhaustive_no_special;
use malachite_test::inputs::base::chars_not_max;

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
