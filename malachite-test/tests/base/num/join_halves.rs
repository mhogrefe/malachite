use common::test_properties;
use malachite_base::num::{JoinHalves, PrimitiveUnsigned, SplitInHalf, Zero};
use malachite_test::inputs::base::{pairs_of_unsigneds, unsigneds};
use rand::Rand;
use std::{u16, u8};

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

fn join_halves_properties_helper<T: JoinHalves + PrimitiveUnsigned + SplitInHalf>()
where
    T::Half: PrimitiveUnsigned + Rand,
{
    test_properties(pairs_of_unsigneds, |&(x, y): &(T::Half, T::Half)| {
        let joined = T::join_halves(x, y);
        assert_eq!(x.into() * (1 << (T::WIDTH >> 1)) + y.into(), joined.into());
        assert_eq!(joined.upper_half(), x);
        assert_eq!(joined.lower_half(), y);
    });

    test_properties(unsigneds, |&x: &T::Half| {
        assert_eq!(T::join_halves(T::Half::ZERO, x).into(), x.into());
    });
}

#[test]
fn join_halves_properties() {
    join_halves_properties_helper::<u16>();
    join_halves_properties_helper::<u32>();
    join_halves_properties_helper::<u64>();
}
