use common::LARGE_LIMIT;
use malachite_base::chars::{char_to_contiguous_range, contiguous_range_to_char,
                            CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES,
                            NUMBER_OF_CHARS};
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::{chars, pairs_of_chars};
use std::char;

#[test]
fn test_char_to_contiguous_range() {
    let test = |c, out| {
        assert_eq!(char_to_contiguous_range(c), out);
    };
    test('\u{0}', 0);
    test('a', 97);
    test('A', 65);
    test(CHAR_JUST_BELOW_SURROGATES, 55_295);
    test(CHAR_JUST_ABOVE_SURROGATES, 55_296);
    test(char::MAX, 1_112_063);
}

#[test]
fn char_to_contiguous_range_properties() {
    // contiguous_range_to_char(char_to_contiguous_range(c)) == Some(c)
    // char_to_contiguous_range(c) < NUMBER_OF_CHARS
    let one_char = |c| {
        let u = char_to_contiguous_range(c);
        assert_eq!(contiguous_range_to_char(u), Some(c));
        assert!(u < NUMBER_OF_CHARS);
    };

    // c.cmp(d) == char_to_contiguous_range(c).cmp(char_to_contiguous_range(d))
    let two_chars = |c: char, d: char| {
        assert_eq!(
            c.cmp(&d),
            char_to_contiguous_range(c).cmp(&char_to_contiguous_range(d))
        );
    };

    for c in chars(GenerationMode::Exhaustive) {
        one_char(c);
    }

    for c in chars(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_char(c);
    }

    for (c, d) in pairs_of_chars(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_chars(c, d);
    }

    for (c, d) in pairs_of_chars(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_chars(c, d);
    }
}
