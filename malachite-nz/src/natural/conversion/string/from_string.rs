use malachite_base::num::conversion::string::from_string::digit_from_display_byte;
use malachite_base::num::conversion::traits::{Digits, FromStringBase, WrappingFrom};
use natural::Natural;
use std::str::FromStr;

impl FromStringBase for Natural {
    #[inline]
    fn from_string_base(base: u64, s: &str) -> Option<Natural> {
        assert!((2..=36).contains(&base), "base out of range");
        if s.is_empty() {
            None
        } else {
            for b in s.bytes() {
                digit_from_display_byte(b)?;
            }
            Natural::from_digits_desc(
                &u8::wrapping_from(base),
                s.bytes().map(|b| digit_from_display_byte(b).unwrap()),
            )
        }
    }
}

impl FromStr for Natural {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Natural, ()> {
        Natural::from_string_base(10, s).ok_or(())
    }
}
