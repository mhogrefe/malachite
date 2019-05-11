use malachite_base::chars::{char_to_contiguous_range, contiguous_range_to_char, NUMBER_OF_CHARS};

use common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{pairs_of_limbs_range_1, u32s_range_1, unsigneds};

#[test]
fn contiguous_range_to_char_properties() {
    test_properties(unsigneds, |&u| {
        assert_eq!(contiguous_range_to_char(u).is_some(), u < NUMBER_OF_CHARS);
    });

    test_properties_no_special(u32s_range_1, |&u| {
        assert_eq!(
            char_to_contiguous_range(contiguous_range_to_char(u).unwrap()),
            u
        );
    });

    test_properties_no_special(pairs_of_limbs_range_1, |&(u, v)| {
        assert_eq!(
            u.cmp(&v),
            contiguous_range_to_char(u).cmp(&contiguous_range_to_char(v))
        );
    });
}
