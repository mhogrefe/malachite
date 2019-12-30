use std::{u16, u8};

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigneds, unsigneds};

macro_rules! join_halves_helper {
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
    join_halves_helper!(u16, u8);
    join_halves_helper!(u32, u16);
    join_halves_helper!(u64, u32);
}
