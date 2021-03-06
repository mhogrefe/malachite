use num::conversion::traits::{ExactFrom, FromStringBase};

/// Produces a digit from a byte corresponding to a numeric or alphabetic (lower- or uppercase)
/// `char` that represents the digit.
///
/// Bytes corresponding to `char`s from '0' to '9' become digits 0 to 9. Bytes corresponding to
/// `char`s from 'a' to 'z' become digits 10 to 35. Bytes corresponding to `char`s from 'A' to 'Z'
/// also become digits 10 to 35. Passing a byte that does not correspond to any of these `char`s
/// yields `None`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::conversion::string::from_string::digit_from_display_byte;
///
/// assert_eq!(digit_from_display_byte(b'0'), Some(0));
/// assert_eq!(digit_from_display_byte(b'9'), Some(9));
/// assert_eq!(digit_from_display_byte(b'a'), Some(10));
/// assert_eq!(digit_from_display_byte(b'z'), Some(35));
/// assert_eq!(digit_from_display_byte(b'A'), Some(10));
/// assert_eq!(digit_from_display_byte(b'Z'), Some(35));
/// assert_eq!(digit_from_display_byte(b' '), None);
/// assert_eq!(digit_from_display_byte(b'!'), None);
/// ```
pub const fn digit_from_display_byte(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'z' => Some(b - b'a' + 10),
        b'A'..=b'Z' => Some(b - b'A' + 10),
        _ => None,
    }
}

macro_rules! impl_from_string_base {
    ($t:ident) => {
        impl FromStringBase for $t {
            #[inline]
            fn from_string_base(base: u64, s: &str) -> Option<Self> {
                $t::from_str_radix(s, u32::exact_from(base)).ok()
            }
        }
    };
}
apply_to_primitive_ints!(impl_from_string_base);
