use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::natural::Natural;

pub fn _from_string_base_naive(small_base: u64, s: &str) -> Option<Natural> {
    let mut x = Natural::ZERO;
    let base = Natural::from(small_base);
    for c in s.chars() {
        x *= &base;
        x += Natural::from(c.to_digit(u32::checked_from(small_base)?)?);
    }
    Some(x)
}
