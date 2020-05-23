use chars::constants::{CHAR_JUST_BELOW_SURROGATES, SURROGATE_RANGE_SIZE};

/// Converts a `char` to a `u32`. The standard way to do this, casting, is sometimes inadequate
/// because the set of all `u32`s that can be produced this way is not contiguous; the surrogate
/// range from 0xd800 to 0xdfff, inclusive, doesn't correspond to any valid `char`. This function
/// closes the gap: every `char` is mapped to a distinct `u32` between 0 and `NUMBER_OF_CHARS` - 1,
/// inclusive.
///
/// The inverse of this function is `contiguous_range_to_char`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_base::chars::char_to_contiguous_range::char_to_contiguous_range;
/// use std::char;
///
/// assert_eq!(char_to_contiguous_range('\u{0}'), 0);
/// assert_eq!(char_to_contiguous_range('a'), 97);
/// assert_eq!(char_to_contiguous_range(char::MAX), 1_112_063);
/// ```
pub fn char_to_contiguous_range(c: char) -> u32 {
    match c {
        '\u{0}'..=CHAR_JUST_BELOW_SURROGATES => c as u32,
        _ => c as u32 - SURROGATE_RANGE_SIZE,
    }
}
