use malachite_base::chars::{char_to_contiguous_range, contiguous_range_to_char, NUMBER_OF_CHARS};

use common::{test_properties_no_limit_exhaustive_no_special, test_properties_no_special};
use malachite_test::inputs::base::{chars, pairs_of_chars};

#[test]
fn char_to_contiguous_range_properties() {
    test_properties_no_limit_exhaustive_no_special(chars, |&c| {
        let u = char_to_contiguous_range(c);
        assert_eq!(contiguous_range_to_char(u), Some(c));
        assert!(u < NUMBER_OF_CHARS);
    });

    test_properties_no_special(pairs_of_chars, |&(c, d)| {
        assert_eq!(
            c.cmp(&d),
            char_to_contiguous_range(c).cmp(&char_to_contiguous_range(d))
        );
    });
}
