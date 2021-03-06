use strings::ToDebugString;

#[doc(hidden)]
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

/// Determines whether a `char` is "graphic".
///
/// A `char` is considered graphic if it is ASCII and not a control character, or non-ASCII and its
/// debug string does not begin with a backslash. This function can be used as a guide to whether a
/// `char` can be displayed on a screen without resorting to some sort of escape sequence. Of
/// course, many typefaces will not be able to render many graphic `char`s.
///
/// The ASCII space ' ' is the only graphic whitespace `char`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::char_is_graphic;
///
/// assert_eq!(char_is_graphic('a'), true);
/// assert_eq!(char_is_graphic(' '), true);
/// assert_eq!(char_is_graphic('\n'), false);
/// assert_eq!(char_is_graphic('ñ'), true);
/// assert_eq!(char_is_graphic('\u{5f771}'), false);
/// ```
pub fn char_is_graphic(c: char) -> bool {
    if c.is_ascii() {
        !c.is_ascii_control()
    } else {
        !debug_starts_with_slash(c)
    }
}

impl CharType {
    #[doc(hidden)]
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

/// This module contains constants associated with `char`s.
pub mod constants;
/// This module contains functions for incrementing and decrementing `char`s.
pub mod crement;
/// This module contains iterators that generate `char`s without repetition.
pub mod exhaustive;
/// This module contains iterators that generate `char`s randomly.
pub mod random;
