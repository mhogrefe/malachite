use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use malachite_base_test_util::generators::unsigned_gen;

fn upper_half_test_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: T::Half)
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.upper_half(), out);
}

#[test]
pub fn test_upper_half() {
    upper_half_test_helper(0u64, 0u32);
    upper_half_test_helper(1u64, 0u32);
    upper_half_test_helper(u16::from(u8::MAX), 0);
    upper_half_test_helper(u16::from(u8::MAX) + 1, 1);
    upper_half_test_helper(u16::MAX, u8::MAX);
    upper_half_test_helper(258u16, 1u8);
    upper_half_test_helper(0xabcd1234u32, 0xabcd);
}

fn upper_half_properties_helper<T: JoinHalves + PrimitiveUnsigned + SplitInHalf>() {
    unsigned_gen::<T>().test_properties(|n| {
        let upper = n.upper_half();
        assert_eq!(T::join_halves(upper, n.lower_half()), n);
    });
}

#[test]
fn upper_half_properties() {
    upper_half_properties_helper::<u16>();
    upper_half_properties_helper::<u32>();
    upper_half_properties_helper::<u64>();
    upper_half_properties_helper::<u128>();
}
