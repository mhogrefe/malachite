use malachite_base::num::arithmetic::traits::{DivAssignMod, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::string::to_string::digit_to_display_byte_lower;
use malachite_base::num::conversion::traits::WrappingFrom;

pub fn _to_string_base_unsigned_naive<T: PrimitiveUnsigned>(mut x: T, base: u64) -> String
where
    u8: WrappingFrom<T>,
{
    assert!((2..=36).contains(&base), "base out of range");
    if x == T::ZERO {
        "0".to_string()
    } else {
        let base = T::wrapping_from(base);
        let mut cs = Vec::new();
        while x != T::ZERO {
            cs.push(char::from(digit_to_display_byte_lower(u8::wrapping_from(
                x.div_assign_mod(base),
            ))));
        }
        cs.into_iter().rev().collect()
    }
}

pub fn _to_string_base_signed_naive<T: PrimitiveSigned>(x: T, base: u64) -> String
where
    u8: WrappingFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    assert!((2..=36).contains(&base), "base out of range");
    if x == T::ZERO {
        "0".to_string()
    } else {
        let base = <T as UnsignedAbs>::Output::wrapping_from(base);
        let mut cs = Vec::new();
        let mut abs_x = x.unsigned_abs();
        while abs_x != <T as UnsignedAbs>::Output::ZERO {
            cs.push(char::from(digit_to_display_byte_lower(u8::wrapping_from(
                abs_x.div_assign_mod(base),
            ))));
        }
        if x < T::ZERO {
            cs.push('-');
        }
        cs.into_iter().rev().collect()
    }
}
