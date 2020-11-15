use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::SplitInHalf;

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
    split_in_half_helper(0xabcd1234u32, (0xabcd, 0x1234));
}

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
    lower_half_helper(0xabcd1234u32, 0x1234);
}

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
    upper_half_helper(0xabcd1234u32, 0xabcd);
}
