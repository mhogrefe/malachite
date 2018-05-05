use common::{test_properties_no_limit_exhaustive_no_special, test_properties_no_special};
use malachite_base::chars::{
    char_to_contiguous_range, contiguous_range_to_char, CHAR_JUST_ABOVE_SURROGATES,
    CHAR_JUST_BELOW_SURROGATES, NUMBER_OF_CHARS,
};
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
