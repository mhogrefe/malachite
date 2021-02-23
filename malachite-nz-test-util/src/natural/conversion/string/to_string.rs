use malachite_base::num::conversion::string::to_string::digit_to_display_byte_lower;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

pub fn _to_string_base_naive(x: &Natural, base: u64) -> String {
    assert!((2..=36).contains(&base), "base out of range");
    let base = Limb::wrapping_from(base);
    if *x == 0 {
        "0".to_string()
    } else {
        let mut x = x.clone();
        let mut cs = Vec::new();
        while x != 0 {
            cs.push(char::from(digit_to_display_byte_lower(u8::wrapping_from(
                x.div_assign_mod_limb(base),
            ))));
        }
        cs.into_iter().rev().collect()
    }
}
