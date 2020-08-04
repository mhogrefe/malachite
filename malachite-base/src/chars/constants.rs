use comparison::traits::{Max, Min};
use named::Named;

/// The number of Unicode scalar values.
///
/// $2^{20}+2^{16}-2^{11} = \mathrm{0x10\\,f800} = 1,\\!112,\\!064$.
pub const NUMBER_OF_CHARS: u32 = (1 << 20) + (1 << 16) - SURROGATE_RANGE_SIZE;

/// The size of the surrogate range; this is a range of code points that do not correspond to any
/// valid `char`.
///
/// $2^{11} = 2,\\!048$.
pub const SURROGATE_RANGE_SIZE: u32 = 1 << 11;

/// The first code point in the surrogate range.
pub const FIRST_SURROGATE_CODE_POINT: u32 = 0xd800;

/// The `char` that comes just before the surrogate range.
pub const CHAR_JUST_BELOW_SURROGATES: char = '\u{d7ff}';

/// The `char` that comes just after the surrogate range.
pub const CHAR_JUST_ABOVE_SURROGATES: char = '\u{e000}';

impl Min for char {
    /// The minimum value of a `char`: `'\u{0}'`.
    const MIN: char = '\u{0}';
}

impl Max for char {
    /// The maximum value of a `char`: `'\u{10ffff}'`.
    const MAX: char = std::char::MAX;
}

impl_named!(char);
