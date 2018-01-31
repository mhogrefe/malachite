use common::LARGE_LIMIT;
use malachite_base::chars::{char_to_contiguous_range, contiguous_range_to_char,
                            CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES,
                            NUMBER_OF_CHARS};
use malachite_test::common::GenerationMode;
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
    // contiguous_range_to_char(u).is_some() == (u < NUMBER_OF_CHARS)
    let one_u32 = |u| {
        assert_eq!(contiguous_range_to_char(u).is_some(), u < NUMBER_OF_CHARS);
    };

    // If u < NUMBER_OF_CHARS, char_to_contiguous_range(contiguous_range_to_char(u).unwrap()) == u
    let one_u32_in_range = |u| {
        assert_eq!(
            char_to_contiguous_range(contiguous_range_to_char(u).unwrap()),
            u
        );
    };

    // If u and v are both less than NUMBER_OF_CHARS,
    //      u.cmp(v) == contiguous_range_to_char(u).cmp(contiguous_range_to_char(v))
    let two_u32s_in_range = |u: u32, v: u32| {
        assert_eq!(
            u.cmp(&v),
            contiguous_range_to_char(u).cmp(&contiguous_range_to_char(v))
        );
    };

    for u in unsigneds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(u);
    }

    for u in unsigneds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(u);
    }

    for u in u32s_range_1(GenerationMode::Exhaustive) {
        one_u32_in_range(u);
    }

    for u in u32s_range_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32_in_range(u);
    }

    for (u, v) in pairs_of_u32s_range_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_u32s_in_range(u, v);
    }

    for (u, v) in pairs_of_u32s_range_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_u32s_in_range(u, v);
    }
}
