// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::ToDebugString;

#[cfg(feature = "test_build")]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CharType {
    AsciiLower,
    AsciiUpper,
    AsciiNumeric,
    AsciiNonAlphanumericGraphic,
    NonAsciiGraphic,
    NonGraphic,
}

#[cfg(not(feature = "test_build"))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum CharType {
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

/// Determines whether a [`char`] is graphic.
///
/// There is an [official Unicode
/// definition](https://www.unicode.org/versions/Unicode14.0.0/ch03.pdf#G30602) of _graphic
/// character_, but that definition is not followed here. In Malachite, a [`char`] is considered
/// graphic if it is ASCII and not a [C0 control](https://unicode.org/charts/PDF/U0000.pdf), or
/// non-ASCII and its debug string does not begin with a backslash. This function can be used as a
/// guide to whether a [`char`] can be displayed on a screen without resorting to some sort of
/// escape sequence. Of course, many typefaces will not be able to render many graphic [`char`]s.
///
/// The ASCII space `' '` is the only graphic whitespace [`char`].
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
    pub_crate_test! {contains(self, c: char) -> bool {
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
    }}
}

/// Constants associated with [`char`]s.
///
/// Apart from the constants visibile on this page, the trait-based constants
/// [`MIN`](crate::comparison::traits::Min::MIN), [`MAX`](crate::comparison::traits::Max::MAX), and
/// [`NAME`](crate::named::Named::NAME) are also defined.
pub mod constants;
/// Functions for incrementing and decrementing [`char`]s.
pub mod crement;
/// Iterators that generate [`char`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`char`]s randomly.
pub mod random;
