use comparison::traits::{Max, Min};
use named::Named;
use strings::ToDebugString;

// The number of Unicode scalar values, or 1,112,064.
pub const NUMBER_OF_CHARS: u32 = (1 << 20) + (1 << 16) - SURROGATE_RANGE_SIZE;

// The size of the surrogate range; this is a range of code points that do not correspond to any
// valid `char` and must be skipped.
pub const SURROGATE_RANGE_SIZE: u32 = 1 << 11;

// The first code point in the surrogate range.
pub const FIRST_SURROGATE_CODE_POINT: u32 = 0xd800;

// The `char` that comes just before the surrogate range.
pub const CHAR_JUST_BELOW_SURROGATES: char = '\u{d7ff}';

// The `char` that comes just after the surrogate range.
pub const CHAR_JUST_ABOVE_SURROGATES: char = '\u{e000}';

/// The minimum value of a `'\u{0}'`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Min for char {
    const MIN: char = '\u{0}';
}

/// The maximum value of a `char`, `'\u{10ffff}'`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Max for char {
    const MAX: char = std::char::MAX;
}

impl_named!(char);

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
/// use malachite_base::chars::char_to_contiguous_range;
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

/// The inverse of this function is `char_to_contiguous_range`: every `u32` between 0 and
/// `NUMBER_OF_CHARS` - 1, inclusive, is mapped to a distinct `char`; larger `u32`s return `None`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_base::chars::contiguous_range_to_char;
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
        0..=ONE_BELOW_FIRST_SURROGATE_CODE_POINT => std::char::from_u32(u),
        FIRST_SURROGATE_CODE_POINT..=ONE_BELOW_NUMBER_OF_CHARS => {
            std::char::from_u32(u + SURROGATE_RANGE_SIZE)
        }
        _ => None,
    }
}

/// Increments this `char`, skipping over the surrogate range.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `self` is `char::MAX`.
///
/// # Examples
/// ```
/// use malachite_base::chars::increment_char;
///
/// let mut c = '\u{0}';
/// increment_char(&mut c);
/// assert_eq!(c, '\u{1}');
///
/// let mut c = 'a';
/// increment_char(&mut c);
/// assert_eq!(c, 'b');
/// ```
#[inline]
pub fn increment_char(c: &mut char) {
    *c = contiguous_range_to_char(char_to_contiguous_range(*c) + 1)
        .expect("Cannot increment char::MAX")
}

/// Decrements this `char`, skipping over the surrogate range.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `self` is `'\u{0}'`.
///
/// # Examples
/// ```
/// use malachite_base::chars::decrement_char;
///
/// let mut c = '\u{1}';
/// decrement_char(&mut c);
/// assert_eq!(c, '\u{0}');
///
/// let mut c = 'b';
/// decrement_char(&mut c);
/// assert_eq!(c, 'a');
/// ```
#[inline]
pub fn decrement_char(c: &mut char) {
    if *c == char::MIN {
        panic!("Cannot decrement char '{}'", *c);
    } else {
        *c = contiguous_range_to_char(char_to_contiguous_range(*c) - 1).unwrap();
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CharType {
    AsciiLower,
    AsciiUpper,
    AsciiNumeric,
    AsciiNonAlphanumericGraphic,
    NonAsciiGraphic,
    NonGraphic,
}

fn debug_starts_with_slash(c: char) -> bool {
    // Skip the first `char`, which is always a single quote
    c.to_debug_string().chars().nth(1) == Some('\\')
}

impl CharType {
    pub fn contains(self, c: char) -> bool {
        match self {
            CharType::AsciiLower => c.is_ascii_lowercase(),
            CharType::AsciiUpper => c.is_ascii_uppercase(),
            CharType::AsciiNumeric => c.is_ascii_digit(),
            CharType::AsciiNonAlphanumericGraphic => {
                c.is_ascii() && !c.is_ascii_alphanumeric() && !c.is_ascii_control()
            }
            CharType::NonAsciiGraphic => !c.is_ascii() && !debug_starts_with_slash(c),
            CharType::NonGraphic => {
                c.is_ascii_control() || !c.is_ascii() && debug_starts_with_slash(c)
            }
        }
    }
}

pub mod exhaustive;
