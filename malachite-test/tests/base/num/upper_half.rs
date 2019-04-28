use common::test_properties;
use malachite_base::num::traits::{JoinHalves, SplitInHalf};
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use malachite_test::inputs::base::unsigneds;
use rand::Rand;
use std::{u16, u8};

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

fn upper_half_properties_helper<T: JoinHalves + PrimitiveUnsigned + SplitInHalf + Rand>() {
    test_properties(unsigneds, |&n: &T| {
        let upper = n.upper_half();
        assert_eq!(T::join_halves(upper, n.lower_half()), n)
    });
}

#[test]
fn upper_half_properties() {
    upper_half_properties_helper::<u16>();
    upper_half_properties_helper::<u32>();
    upper_half_properties_helper::<u64>();
}
