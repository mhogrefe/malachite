use malachite_base::strings::{string_is_subset, string_nub};

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::strings;

#[test]
fn string_nub_properties() {
    test_properties_no_special(strings, |s| {
        //TODO is unique
        let t = string_nub(s);
        assert!(t.len() <= s.len());
        assert_eq!(string_nub(&t), t);
        assert!(string_is_subset(s, &t));
        assert!(string_is_subset(&t, s));
    });
}
