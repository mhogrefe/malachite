use std::char;

use comparison::{Max, Min};
use crement::Crementable;

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
        0..=ONE_BELOW_FIRST_SURROGATE_CODE_POINT => char::from_u32(u),
        FIRST_SURROGATE_CODE_POINT..=ONE_BELOW_NUMBER_OF_CHARS => {
            char::from_u32(u + SURROGATE_RANGE_SIZE)
        }
        _ => None,
    }
}

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

impl Crementable for char {
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
    /// use malachite_base::crement::Crementable;
    ///
    /// let mut c = '\u{0}';
    /// c.increment();
    /// assert_eq!(c, '\u{1}');
    ///
    /// let mut c = 'a';
    /// c.increment();
    /// assert_eq!(c, 'b');
    /// ```
    fn increment(&mut self) {
        *self = contiguous_range_to_char(char_to_contiguous_range(*self) + 1)
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
    /// use malachite_base::crement::Crementable;
    ///
    /// let mut c = '\u{1}';
    /// c.decrement();
    /// assert_eq!(c, '\u{0}');
    ///
    /// let mut c = 'b';
    /// c.decrement();
    /// assert_eq!(c, 'a');
    /// ```
    #[allow(clippy::panic_params)]
    fn decrement(&mut self) {
        if *self == char::MIN {
            panic!("Cannot decrement char '{}'", *self);
        } else {
            *self = contiguous_range_to_char(char_to_contiguous_range(*self) - 1).unwrap();
        }
    }
}
