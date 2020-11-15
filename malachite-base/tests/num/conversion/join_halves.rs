use malachite_base_test_util::generators::{unsigned_gen, unsigned_pair_gen};

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{HasHalf, JoinHalves, SplitInHalf};

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
    join_halves_helper(0xabcd, 0x1234, 0xabcd1234u32);
}

fn join_halves_properties_helper<
    T: From<HT> + HasHalf<Half = HT> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
    HT: PrimitiveUnsigned,
>() {
    unsigned_pair_gen::<HT>().test_properties(|(x, y)| {
        let joined = T::join_halves(x, y);
        assert_eq!((T::from(x) << (T::WIDTH >> 1)) + T::from(y), joined);
        assert_eq!(joined.upper_half(), x);
        assert_eq!(joined.lower_half(), y);
    });

    unsigned_gen::<HT>().test_properties(|x| {
        assert_eq!(T::join_halves(HT::ZERO, x), T::from(x));
    });
}

#[test]
fn join_halves_properties() {
    join_halves_properties_helper::<u16, u8>();
    join_halves_properties_helper::<u32, u16>();
    join_halves_properties_helper::<u64, u32>();
    join_halves_properties_helper::<u128, u64>();
}
