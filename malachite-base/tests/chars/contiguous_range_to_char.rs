use malachite_base::chars::{
    contiguous_range_to_char, CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES,
};
use malachite_base::comparison::traits::Max;

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
