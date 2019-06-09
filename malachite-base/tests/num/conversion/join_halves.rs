use std::{u16, u8};

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::JoinHalves;

fn join_halves_helper<T: JoinHalves + PrimitiveUnsigned>(upper: T::Half, lower: T::Half, out: T) {
    assert_eq!(T::join_halves(upper, lower), out);
}

#[test]
pub fn test_join_halves() {
    join_halves_helper(0u32, 0u32, 0u64);
    join_halves_helper(0u32, 1u32, 1u64);
    join_halves_helper(0, u8::MAX, u16::from(u8::MAX));
    join_halves_helper(1, 0, u16::from(u8::MAX) + 1);
    join_halves_helper(u8::MAX, u8::MAX, u16::MAX);
    join_halves_helper(1, 2, 258u16);
    join_halves_helper(0xabcd, 0x1234, 0xabcd_1234u32);
}
