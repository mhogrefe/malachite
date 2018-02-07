use common::test_properties;
use malachite_base::chars::{char_to_contiguous_range, contiguous_range_to_char,
                            CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES,
                            NUMBER_OF_CHARS};
use malachite_test::inputs::base::{unsigneds, pairs_of_u32s_range_1, u32s_range_1};
use std::{char, u32};

#[test]
fn test_contiguous_range_to_char() {
    let test = |u, out| {
        assert_eq!(contiguous_range_to_char(u), out);
    };
    test(0, Some('\u{0}'));
    test(97, Some('a'));
    test(65, Some('A'));
    test(55_295, Some(CHAR_JUST_BELOW_SURROGATES));
    test(55_296, Some(CHAR_JUST_ABOVE_SURROGATES));
    test(1_112_063, Some(char::MAX));
    test(1_112_064, None);
    test(u32::MAX, None);
}

#[test]
fn contiguous_range_to_char_properties() {
    test_properties(unsigneds, |&u| {
        assert_eq!(contiguous_range_to_char(u).is_some(), u < NUMBER_OF_CHARS);
    });

    test_properties(u32s_range_1, |&u| {
        assert_eq!(
            char_to_contiguous_range(contiguous_range_to_char(u).unwrap()),
            u
        );
    });

    test_properties(pairs_of_u32s_range_1, |&(u, v)| {
        assert_eq!(
            u.cmp(&v),
            contiguous_range_to_char(u).cmp(&contiguous_range_to_char(v))
        );
    });
}
