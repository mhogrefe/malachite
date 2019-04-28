use std::{u16, u8};

use malachite_base::num::traits::{JoinHalves, SplitInHalf};
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use common::test_properties;
use malachite_test::inputs::base::unsigneds;

fn lower_half_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: T::Half)
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.lower_half(), out);
}

#[test]
pub fn test_lower_half() {
    lower_half_helper(0u64, 0u32);
    lower_half_helper(1u64, 1u32);
    lower_half_helper(u16::from(u8::MAX), u8::MAX);
    lower_half_helper(u16::from(u8::MAX) + 1, 0);
    lower_half_helper(u16::MAX, u8::MAX);
    lower_half_helper(258u16, 2u8);
    lower_half_helper(0xabcd_1234u32, 0x1234);
}

fn lower_half_properties_helper<T: JoinHalves + PrimitiveUnsigned + SplitInHalf + Rand>() {
    test_properties(unsigneds, |&n: &T| {
        let lower = n.lower_half();
        assert_eq!(T::join_halves(n.upper_half(), lower), n)
    });
}

#[test]
fn lower_half_properties() {
    lower_half_properties_helper::<u16>();
    lower_half_properties_helper::<u32>();
    lower_half_properties_helper::<u64>();
}
