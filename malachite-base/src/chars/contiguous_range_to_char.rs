use std::char;

use chars::constants::{FIRST_SURROGATE_CODE_POINT, NUMBER_OF_CHARS, SURROGATE_RANGE_SIZE};

/// The inverse of this function is `char_to_contiguous_range`: every `u32` between 0 and
/// `NUMBER_OF_CHARS` - 1, inclusive, is mapped to a distinct `char`; larger `u32`s return `None`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_base::chars::contiguous_range_to_char::contiguous_range_to_char;
/// use std::char;
///
/// assert_eq!(contiguous_range_to_char(0), Some('\u{0}'));
/// assert_eq!(contiguous_range_to_char(97), Some('a'));
/// assert_eq!(contiguous_range_to_char(1_112_063), Some(char::MAX));
/// ```
pub fn contiguous_range_to_char(u: u32) -> Option<char> {
    const ONE_BELOW_FIRST_SURROGATE_CODE_POINT: u32 = FIRST_SURROGATE_CODE_POINT - 1;
    const ONE_BELOW_NUMBER_OF_CHARS: u32 = NUMBER_OF_CHARS - 1;
    match u {
        0..=ONE_BELOW_FIRST_SURROGATE_CODE_POINT => char::from_u32(u),
        FIRST_SURROGATE_CODE_POINT..=ONE_BELOW_NUMBER_OF_CHARS => {
            char::from_u32(u + SURROGATE_RANGE_SIZE)
        }
        _ => None,
    }
}
