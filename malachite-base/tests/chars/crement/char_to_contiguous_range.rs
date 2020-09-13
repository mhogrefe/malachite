use malachite_base::chars::constants::{CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES};
use malachite_base::chars::crement::char_to_contiguous_range;
use malachite_base::comparison::traits::Max;

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
