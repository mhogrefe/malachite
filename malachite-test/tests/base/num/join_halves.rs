use std::{u16, u8};

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};

use common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigneds, unsigneds};

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

macro_rules! join_halves_properties_helper {
    ($t: ident, $ht: ident) => {
        test_properties(pairs_of_unsigneds, |&(x, y): &($ht, $ht)| {
            let joined = $t::join_halves(x, y);
            assert_eq!(($t::from(x) << ($t::WIDTH >> 1)) + $t::from(y), joined);
            assert_eq!(joined.upper_half(), x);
            assert_eq!(joined.lower_half(), y);
        });

        test_properties(unsigneds, |&x: &$ht| {
            assert_eq!($t::join_halves(0, x), $t::from(x));
        });
    };
}

#[test]
fn join_halves_properties() {
    join_halves_properties_helper!(u16, u8);
    join_halves_properties_helper!(u32, u16);
    join_halves_properties_helper!(u64, u32);
}
