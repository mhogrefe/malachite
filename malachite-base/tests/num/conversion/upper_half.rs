use std::{u16, u8};

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::SplitInHalf;

fn upper_half_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: T::Half)
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.upper_half(), out);
}

#[test]
pub fn test_upper_half() {
    upper_half_helper(0u64, 0u32);
    upper_half_helper(1u64, 0u32);
    upper_half_helper(u16::from(u8::MAX), 0);
    upper_half_helper(u16::from(u8::MAX) + 1, 1);
    upper_half_helper(u16::MAX, u8::MAX);
    upper_half_helper(258u16, 1u8);
    upper_half_helper(0xabcd_1234u32, 0xabcd);
}
