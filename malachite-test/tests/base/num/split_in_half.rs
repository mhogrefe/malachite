use common::test_properties;
use malachite_base::num::{JoinHalves, PrimitiveUnsigned, SplitInHalf};
use malachite_test::inputs::base::unsigneds;
use rand::Rand;
use std::{u16, u8};

fn split_in_half_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: (T::Half, T::Half))
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.split_in_half(), out);
}

#[test]
pub fn test_split_in_half() {
    split_in_half_helper(0u64, (0u32, 0u32));
    split_in_half_helper(1u64, (0u32, 1u32));
    split_in_half_helper(u16::from(u8::MAX), (0, u8::MAX));
    split_in_half_helper(u16::from(u8::MAX) + 1, (1, 0));
    split_in_half_helper(u16::MAX, (u8::MAX, u8::MAX));
    split_in_half_helper(258u16, (1u8, 2u8));
    split_in_half_helper(0xabcd_1234u32, (0xabcd, 0x1234));
}

fn split_in_half_properties_helper<T: JoinHalves + PrimitiveUnsigned + SplitInHalf + Rand>() {
    test_properties(unsigneds, |&n: &T| {
        let (upper, lower) = n.split_in_half();
        assert_eq!(T::join_halves(upper, lower), n)
    });
}

#[test]
fn split_in_half_properties() {
    split_in_half_properties_helper::<u16>();
    split_in_half_properties_helper::<u32>();
    split_in_half_properties_helper::<u64>();
}
