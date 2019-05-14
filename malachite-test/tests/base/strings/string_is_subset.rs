use malachite_base::strings::{string_is_subset, string_nub, string_sort};

use common::test_properties_no_special;
use malachite_test::inputs::base::pairs_of_strings;

#[test]
fn string_is_subset_properties() {
    test_properties_no_special(pairs_of_strings, |(s, t)| {
        let is_subset = string_is_subset(s, t);
        assert_eq!(
            string_is_subset(&string_sort(s), &string_sort(t)),
            is_subset
        );
        assert_eq!(string_is_subset(&string_nub(s), &string_nub(t)), is_subset);
        assert_eq!(
            is_subset && string_is_subset(t, s),
            string_sort(&string_nub(s)) == string_sort(&string_nub(t))
        );
    });
}