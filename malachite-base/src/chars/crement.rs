// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::constants::{
    CHAR_JUST_BELOW_SURROGATES, FIRST_SURROGATE_CODE_POINT, NUMBER_OF_CHARS,
    NUMBER_OF_SURROGATE_CODE_POINTS,
};

/// Converts a [`char`] to a [`u32`].
///
/// The conversion is done in such a way that if the next largest [`char`] after $x$ is $y$, then
/// $\mathrm{char\\_to\\_contiguous\\_range(x)}+1 = \mathrm{char\\_to\\_contiguous\\_range(y)}$.
/// This can't be accomplished just through casting, because there is a range of [`u32`]s (the
/// [surrogate code points](https://www.unicode.org/glossary/#surrogate_code_point)) that do not
/// correspond to any [`char`].
///
/// The inverse of this function is [`contiguous_range_to_char`].
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::crement::char_to_contiguous_range;
/// use std::char;
///
/// assert_eq!(char_to_contiguous_range('\u{0}'), 0);
/// assert_eq!(char_to_contiguous_range('a'), 97);
/// assert_eq!(char_to_contiguous_range(char::MAX), 1112063);
/// ```
pub const fn char_to_contiguous_range(c: char) -> u32 {
    match c {
        '\u{0}'..=CHAR_JUST_BELOW_SURROGATES => c as u32,
        _ => c as u32 - NUMBER_OF_SURROGATE_CODE_POINTS,
    }
}

/// Converts a [`u32`] to a [`char`]; if all [`char`]s were arranged in ascending order, passing $u$
/// to this function would return the $u$th [`char`].
///
/// This function is the inverse of [`char_to_contiguous_range`]. Every [`u32`] between $0$ and
/// $\mathrm{NUMBER\\_OF\\_CHARS} - 1$, inclusive, is mapped to a distinct [`char`]. Passing a
/// larger [`u32`] yields `None`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::crement::contiguous_range_to_char;
/// use std::char;
///
/// assert_eq!(contiguous_range_to_char(0), Some('\u{0}'));
/// assert_eq!(contiguous_range_to_char(97), Some('a'));
/// assert_eq!(contiguous_range_to_char(1112063), Some(char::MAX));
/// ```
pub const fn contiguous_range_to_char(u: u32) -> Option<char> {
    const ONE_BELOW_FIRST_SURROGATE_CODE_POINT: u32 = FIRST_SURROGATE_CODE_POINT - 1;
    const ONE_BELOW_NUMBER_OF_CHARS: u32 = NUMBER_OF_CHARS - 1;
    match u {
        0..=ONE_BELOW_FIRST_SURROGATE_CODE_POINT => core::char::from_u32(u),
        FIRST_SURROGATE_CODE_POINT..=ONE_BELOW_NUMBER_OF_CHARS => {
            core::char::from_u32(u + NUMBER_OF_SURROGATE_CODE_POINTS)
        }
        _ => None,
    }
}

/// Increments this [`char`], skipping over the [surrogate code
/// points](https://www.unicode.org/glossary/#surrogate_code_point).
///
/// # Panics
/// Panics if `self` is `char::MAX`.
///
/// # Examples
/// ```
/// use malachite_base::chars::crement::increment_char;
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
        .expect("Cannot increment char::MAX");
}

/// Decrements this [`char`], skipping over the [surrogate code
/// points](https://www.unicode.org/glossary/#surrogate_code_point).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `self` is `'\u{0}'`.
///
/// # Examples
/// ```
/// use malachite_base::chars::crement::decrement_char;
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
