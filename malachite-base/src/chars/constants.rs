// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::comparison::traits::{Max, Min};
use crate::named::Named;

/// The number of [Unicode scalar values](https://www.unicode.org/glossary/#unicode_scalar_value).
/// $2^{20}+2^{16}-2^{11} = \mathrm{0x10\\,f800} = 1,\\!112,\\!064$.
pub const NUMBER_OF_CHARS: u32 = (1 << 20) + (1 << 16) - NUMBER_OF_SURROGATE_CODE_POINTS;

/// The number of [surrogate code points](https://www.unicode.org/glossary/#surrogate_code_point);
/// these are code points that do not correspond to any valid [`char`].
///
/// $2^{11} = 2,\\!048$.
pub const NUMBER_OF_SURROGATE_CODE_POINTS: u32 = 1 << 11;

/// The first [surrogate code point](https://www.unicode.org/glossary/#surrogate_code_point).
pub const FIRST_SURROGATE_CODE_POINT: u32 = 0xd800;

/// The [`char`] that comes just before the surrogate range.
///
/// This happens to be an unassigned (as of Unicode 14.0) character in the [Hangul Jamo Extended-B
/// block](https://www.unicode.org/charts/PDF/UD7B0.pdf).
pub const CHAR_JUST_BELOW_SURROGATES: char = '\u{d7ff}';

/// The [`char`] that comes just after the surrogate range.
///
/// This is a character in the [Private Use Area](https://www.unicode.org/charts/PDF/UE000.pdf).
pub const CHAR_JUST_ABOVE_SURROGATES: char = '\u{e000}';

impl Min for char {
    /// The minimum value of a [`char`]: `'\u{0}'`.
    ///
    /// This is the famous NUL character, a [C0
    /// control](https://www.unicode.org/charts/PDF/U0000.pdf).
    const MIN: char = '\u{0}';
}

impl Max for char {
    /// The maximum value of a [`char`]: `'\u{10ffff}'`.
    ///
    /// This is a character in [Supplementary Private Use
    /// Area-B](https://www.unicode.org/charts/PDF/U10FF80.pdf).
    const MAX: char = core::char::MAX;
}
impl_named!(char);
