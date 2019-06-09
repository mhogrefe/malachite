use std::u16;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use rand::Rand;

use common::test_properties;
use malachite_test::inputs::base::unsigneds;

fn upper_half_helper<T: JoinHalves + PrimitiveUnsigned + SplitInHalf + Rand>() {
    test_properties(unsigneds, |&n: &T| {
        let upper = n.upper_half();
        assert_eq!(T::join_halves(upper, n.lower_half()), n)
    });
}

#[test]
fn upper_half_properties() {
    upper_half_helper::<u16>();
    upper_half_helper::<u32>();
    upper_half_helper::<u64>();
}
