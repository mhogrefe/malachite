use malachite_base::num::basic::traits::Zero;
use natural::Natural;

pub fn from_string_base_naive(small_base: u8, s: &str) -> Option<Natural> {
    let mut x = Natural::ZERO;
    let base = Natural::from(small_base);
    for c in s.chars() {
        x *= &base;
        x += Natural::from(c.to_digit(u32::from(small_base))?);
    }
    Some(x)
}
