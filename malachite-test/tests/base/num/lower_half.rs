use std::u16;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use rand::Rand;

use common::test_properties;
use malachite_test::inputs::base::unsigneds;

fn lower_half_helper<T: JoinHalves + PrimitiveUnsigned + SplitInHalf + Rand>() {
    test_properties(unsigneds, |&n: &T| {
        let lower = n.lower_half();
        assert_eq!(T::join_halves(n.upper_half(), lower), n)
    });
}

#[test]
fn lower_half_properties() {
    lower_half_helper::<u16>();
    lower_half_helper::<u32>();
    lower_half_helper::<u64>();
}
